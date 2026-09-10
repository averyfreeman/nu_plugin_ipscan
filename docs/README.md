# `ipscan` project documents

This directory records the scope and implementation decisions for the
standalone `ipscan` scanner and its `nu_plugin_ipscan` adapter.

The codebase has three layers:

1. `ipscan-lib` owns packet construction, response decoding, scan timing,
   interface selection, and vendor lookup.
2. `ipscan` owns the command-line grammar, capability checks, signal handling,
   and plain/JSON/YAML/CSV presentation.
3. `nu-plugin-ipscan` starts the standalone binary without a shell, requests
   JSON output, and converts JSON into native Nushell values.

The implementation is intentionally an ARP/IPv4 discovery tool rather than a
general replacement for Linux `ip`. It should only be used on networks the
operator is authorized to inspect.

## Documents

- [Nu plugin evaluation](nu-plugin-evaluation.md) explains the adapter
  boundary, privileges, output contract, and alternatives.
- [Rust implementation plan](rust-implementation-plan.md) tracks the work
  needed to extend scanning and the plugin safely. Each work item has a
  `Completed: bool` field.

The generated API reference is maintained in `rustdoc-ipscan/` and is rebuilt
with `npm run rustdoc` or as part of `npm run build` in that directory.
