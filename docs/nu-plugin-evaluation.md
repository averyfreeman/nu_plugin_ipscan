# Nushell plugin evaluation

## Conclusion

`ipscan` is a good fit for a Nu plugin because its useful result is naturally a
finite record stream: discovered IPv4 address, MAC address, hostname, vendor,
and scan counters. The plugin does not try to own the raw packet channel or
long-running receive loop; those remain in the standalone process where
capability errors and signals are easier to report.

The adapter targets Nushell `0.115.1`. It forwards arguments directly, avoids
shell interpolation, adds `--output json` unless the caller selected an output
format, and converts JSON arrays/objects into Nu values. `--raw` is the escape
hatch for exact CLI output.

## Boundary and trade-offs

| Concern | Plugin behavior | Reason |
| --- | --- | --- |
| Interface listing | Structured records | Safe to run without raw-packet privileges |
| ARP scan | Structured JSON result | Easy to filter, project, join, and export |
| Plain/YAML/CSV | Forwarded when explicitly requested | Preserves standalone CLI compatibility |
| Packet privileges | Backend responsibility | Keeps capability failures local and understandable |
| Signals and partial results | Standalone process | The CLI already owns Ctrl-C and receive-thread shutdown |
| Streaming every packet | Not exposed initially | ARP scanning produces a final finite result and counters |

## Alternatives

Scores are ease of implementation from 0 to 100, where 100 is easiest.

### 1. Nu wrapper around the standalone binary — 96/100

This is the current design. It has a small protocol adapter, reuses the tested
scanner, and can be installed without making the scanner a Nu-specific library.
The cost is one child process per invocation and a JSON serialization boundary.

### 2. Native Nu command over `ipscan-lib` — 74/100

Move scan orchestration behind a public library API and implement a
`PluginCommand` that returns records directly. This removes the child process,
but requires careful ownership of datalink channels, cancellation, capability
errors, and partial results inside the Nu plugin process.

### 3. External command only — 88/100

Keep `ipscan` as a normal executable and let Nu users pipe its JSON through
`from json`. This is the smallest integration surface and is a useful fallback
for shells other than Nushell, but it does not provide a discoverable `ipscan`
command or automatic JSON selection.

## Recommended workaround

If a host cannot register plugins, use the standalone form:

```nu
^./ipscan --network 192.168.1.0/24 --numeric --output json | from json
```

If the plugin backend is not on `PATH`, set
`NU_PLUGIN_IPSCAN_BACKEND` to an absolute path. If packet access is denied,
grant `CAP_NET_RAW` to the backend or run it through the platform's approved
privilege mechanism; do not make the plugin silently elevate privileges.
