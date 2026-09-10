//! Nushell `0.115.1` adapter for the standalone Rust `ipscan` command.
//!
//! The plugin intentionally keeps a narrow process boundary: it forwards
//! typed positional arguments without a shell, requests JSON from the
//! standalone binary, and converts that JSON into native Nushell values.
//! This preserves the standalone CLI for raw streams, namespace execution,
//! and other process-oriented behavior that is not a natural plugin result.

use std::process::{Command, Stdio};

use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, SimplePluginCommand};
use nu_protocol::{LabeledError, Record, Signature, Span, SyntaxShape, Type, Value};

/// Environment variable used to select the standalone backend executable.
pub const BACKEND_ENV: &str = "NU_PLUGIN_IPSCAN_BACKEND";

/// The plugin state is intentionally empty. Each invocation is isolated in a
/// child process, so the backend cannot mutate plugin-global state or the
/// plugin's own namespace.
pub struct IpScanPlugin;

/// A compatibility-oriented structured scan command.
pub struct IpScanCommand;

impl Plugin for IpScanPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(IpScanCommand)]
    }
}

impl SimplePluginCommand for IpScanCommand {
    type Plugin = IpScanPlugin;

    fn name(&self) -> &str {
        "ipscan"
    }

    fn description(&self) -> &str {
        "Scan the local network and return structured host records"
    }

    fn extra_description(&self) -> &str {
        "Arguments are passed to ipscan without a shell. JSON output is requested by default and converted to native Nushell values. Set NU_PLUGIN_IPSCAN_BACKEND or use --backend to select the backend executable. Use --raw when exact stdout bytes are required."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["arp", "scan", "network", "pnet"]
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .rest(
                "arguments",
                SyntaxShape::String,
                "Arguments forwarded to the ipscan backend; JSON output is requested by default",
            )
            .switch(
                "raw",
                "Return backend stdout as a binary value instead of parsing JSON",
                None,
            )
            .named(
                "backend",
                SyntaxShape::String,
                "Backend executable path; defaults to NU_PLUGIN_IPSCAN_BACKEND or ipscan",
                None,
            )
            .input_output_type(Type::Nothing, Type::Any)
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let arguments = call.rest::<String>(0)?;
        let raw = call.has_flag("raw")?;
        let backend = resolve_backend(
            call.get_flag::<String>("backend")?
                .map(std::ffi::OsString::from)
                .or_else(|| std::env::var_os(BACKEND_ENV)),
        );

        run_backend(&backend, &arguments, raw, call.head)
    }
}

/// Resolve the backend executable used by a plugin invocation.
///
/// An explicit `--backend` path wins, followed by [`BACKEND_ENV`].  When the
/// plugin executable lives beside an `ipscan` binary, that sibling is used
/// before falling back to the user's `PATH`; this makes a source checkout or a
/// colocated installation work without an extra environment variable.
pub fn resolve_backend(explicit: Option<std::ffi::OsString>) -> std::ffi::OsString {
    if let Some(explicit) = explicit {
        return explicit;
    }
    if let Some(configured) = std::env::var_os(BACKEND_ENV) {
        return configured;
    }
    if let Ok(executable) = std::env::current_exe()
        && let Some(directory) = executable.parent()
    {
        let sibling = directory.join(if cfg!(windows) {
            "ipscan.exe"
        } else {
            "ipscan"
        });
        if sibling.is_file() {
            return sibling.into_os_string();
        }
    }
    std::ffi::OsString::from("ipscan")
}

/// Execute the backend without invoking a shell.
pub fn run_backend(
    backend: &std::ffi::OsStr,
    arguments: &[String],
    raw: bool,
    span: Span,
) -> Result<Value, LabeledError> {
    let backend_arguments = backend_arguments(arguments, raw);
    let output = Command::new(backend)
        .args(&backend_arguments)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| {
            LabeledError::new(format!(
                "could not execute ipscan backend `{}`: {error}",
                backend.to_string_lossy()
            ))
            .with_label("backend executable", span)
            .with_help(format!(
                "install ipscan or set {BACKEND_ENV} to an executable path"
            ))
            .with_code("nu_plugin_ipscan::backend_unavailable")
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = if stderr.trim().is_empty() {
            format!("exit status {}", format_status(&output.status))
        } else {
            format!(
                "exit status {}: {}",
                format_status(&output.status),
                stderr.trim()
            )
        };

        return Err(
            LabeledError::new(format!("ipscan backend failed: {detail}"))
                .with_label("backend invocation", span)
                .with_code("nu_plugin_ipscan::backend_failed"),
        );
    }

    if raw {
        return Ok(Value::binary(output.stdout, span));
    }

    if explicit_output_format(arguments).is_some_and(|format| format != "json") {
        let text = std::str::from_utf8(&output.stdout).map_err(|error| {
            LabeledError::new(format!("ipscan emitted invalid text output: {error}"))
                .with_label("backend output", span)
                .with_code("nu_plugin_ipscan::invalid_output")
        })?;
        return Ok(Value::string(text.to_owned(), span));
    }

    parse_json_output(&output.stdout, span)
}

/// Add the backend's structured-output switch unless the caller explicitly
/// selected raw output or another output format. The standalone `ipscan` CLI
/// uses `--output json` for the plugin transport.
pub fn backend_arguments(arguments: &[String], raw: bool) -> Vec<String> {
    if raw || explicit_output_format(arguments).is_some() {
        arguments.to_vec()
    } else {
        let mut result = Vec::with_capacity(arguments.len() + 2);
        result.push("--output".to_owned());
        result.push("json".to_owned());
        result.extend(arguments.iter().cloned());
        result
    }
}

/// Return the explicitly selected standalone output format, if present.
fn explicit_output_format(arguments: &[String]) -> Option<&str> {
    for (index, argument) in arguments.iter().enumerate() {
        if argument == "-o" || argument == "--output" {
            return Some(
                arguments
                    .get(index + 1)
                    .map(String::as_str)
                    .unwrap_or_default(),
            );
        }
        if let Some(format) = argument.strip_prefix("--output=") {
            return Some(format);
        }
    }
    None
}

/// Convert the JSON emitted by ipscan into native Nu values.
pub fn parse_json_output(output: &[u8], span: Span) -> Result<Value, LabeledError> {
    let text = std::str::from_utf8(output).map_err(|error| {
        LabeledError::new(format!("ipscan emitted invalid UTF-8: {error}"))
            .with_label("backend JSON output", span)
            .with_code("nu_plugin_ipscan::invalid_output")
    })?;

    if text.trim().is_empty() {
        return Ok(Value::nothing(span));
    }

    let json: serde_json::Value = serde_json::from_str(text).map_err(|error| {
        let preview = text.trim().chars().take(512).collect::<String>();
        LabeledError::new(format!(
            "ipscan returned output that is not valid JSON: {error}; output begins with {preview:?}"
        ))
        .with_label("backend JSON output", span)
        .with_help("use `--raw` to inspect backend stdout while diagnosing a compatibility issue")
        .with_code("nu_plugin_ipscan::invalid_output")
    })?;

    Ok(json_to_value(json, span))
}

fn json_to_value(json: serde_json::Value, span: Span) -> Value {
    match json {
        serde_json::Value::Null => Value::nothing(span),
        serde_json::Value::Bool(value) => Value::bool(value, span),
        serde_json::Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                Value::int(value, span)
            } else if let Some(value) = value.as_u64() {
                i64::try_from(value).map_or_else(
                    |_| Value::string(value.to_string(), span),
                    |value| Value::int(value, span),
                )
            } else if let Some(value) = value.as_f64() {
                Value::float(value, span)
            } else {
                Value::string(value.to_string(), span)
            }
        }
        serde_json::Value::String(value) => Value::string(value, span),
        serde_json::Value::Array(values) => Value::list(
            values
                .into_iter()
                .map(|value| json_to_value(value, span))
                .collect(),
            span,
        ),
        serde_json::Value::Object(values) => {
            let mut record = Record::with_capacity(values.len());
            for (key, value) in values {
                record.push(key, json_to_value(value, span));
            }
            Value::record(record, span)
        }
    }
}

fn format_status(status: &std::process::ExitStatus) -> String {
    status.code().map_or_else(
        || "terminated by signal".to_owned(),
        |code| code.to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_output_becomes_native_values() {
        let value = parse_json_output(
            br#"[{"ifname":"lo","index":1,"up":true,"nested":{"mtu":65536},"missing":null}]"#,
            Span::test_data(),
        )
        .expect("JSON should parse");

        let Value::List { vals, .. } = value else {
            panic!("expected a list")
        };
        assert_eq!(vals.len(), 1);
        let Value::Record { val, .. } = &vals[0] else {
            panic!("expected a record")
        };
        assert_eq!(val.get("ifname").unwrap().as_str().unwrap(), "lo");
        assert_eq!(val.get("index").unwrap().as_int().unwrap(), 1);
        assert!(val.get("up").unwrap().as_bool().unwrap());
        assert!(matches!(val.get("missing"), Some(Value::Nothing { .. })));
    }

    #[test]
    fn empty_backend_output_is_nothing() {
        assert!(matches!(
            parse_json_output(b"\n", Span::test_data()).unwrap(),
            Value::Nothing { .. }
        ));
    }

    #[test]
    fn large_json_integers_are_not_rounded() {
        let value = parse_json_output(b"18446744073709551615", Span::test_data())
            .expect("JSON should parse");
        assert_eq!(value.as_str().unwrap(), "18446744073709551615");
    }

    #[test]
    fn structured_backend_arguments_prepend_json() {
        let args = vec!["--network".to_owned(), "192.0.2.0/24".to_owned()];
        assert_eq!(
            backend_arguments(&args, false),
            vec!["--output", "json", "--network", "192.0.2.0/24"]
        );
        assert_eq!(backend_arguments(&args, true), args);
    }

    #[test]
    fn an_explicit_json_switch_is_not_duplicated() {
        let args = vec!["--output".to_owned(), "json".to_owned()];
        assert_eq!(backend_arguments(&args, false), args);
    }

    #[test]
    fn an_explicit_text_format_is_not_replaced() {
        let args = vec![
            "--output".to_owned(),
            "plain".to_owned(),
            "--list".to_owned(),
        ];
        assert_eq!(backend_arguments(&args, false), args);
        assert_eq!(explicit_output_format(&args), Some("plain"));
    }
}
