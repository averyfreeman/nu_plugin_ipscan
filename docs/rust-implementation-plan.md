# Rust implementation plan

Each item describes a concrete coding path and records whether the work is
complete. A completed item still needs regression coverage when the packet
path is changed.

## Interface inventory

`Completed: true`

1. Use `pnet_datalink::interfaces()` to collect names, indices, MAC addresses,
   CIDR addresses, and capability flags.
2. Keep the plain table for humans and add a JSON serializer with stable field
   names for the plugin.
3. Validate `--list --output json` and the Nu projection examples without
   opening a datalink channel.

## ARP request construction

`Completed: true`

1. Select the requested interface or the first usable non-loopback interface.
2. Build Ethernet and ARP request frames using the configured source/dest MAC,
   source IPv4, VLAN, hardware/protocol types, and operation.
3. Iterate the requested IPv4 networks with retry, interval, bandwidth, and
   randomization settings.
4. Keep capability checks before opening the packet channel.

## Response collection and result model

`Completed: true`

1. Receive Ethernet frames on a worker thread while the sender transmits the
   target set.
2. Decode ARP replies, deduplicate by IPv4 address, optionally resolve names,
   and optionally map OUIs to vendors.
3. Return packet counters, filtered ARP counters, duration, and target rows.
4. Stop cleanly on timeout or Ctrl-C and preserve partial results when safe.

## Output formats

`Completed: true`

1. Keep plain output readable for terminal use.
2. Serialize one stable JSON object containing summary counters and `results`.
3. Preserve YAML and CSV exports for compatibility.
4. In the plugin, request JSON by default and convert values using
   `nu_protocol::Value::from_json`.

## Nu plugin adapter

`Completed: true`

1. Implement `Plugin` and `PluginCommand` against Nu `0.115.1`.
2. Resolve the backend from `--backend`, `NU_PLUGIN_IPSCAN_BACKEND`, a sibling
   executable, or `PATH`.
3. Pass arguments with `Command::arg` rather than a shell command string.
4. Add `--output json` only when the caller did not choose a format; provide
   `--raw` for exact bytes.
5. Test command metadata, backend resolution, argument construction, and JSON
   conversion without requiring packet privileges.

## IPv6 neighbour discovery

`Completed: false`

1. Add an explicit IPv6 scan mode rather than silently treating IPv6 as ARP.
2. Use ICMPv6 neighbour solicitation or a documented library abstraction.
3. Define capability requirements, multicast handling, and result schema.
4. Add namespace-isolated integration tests before exposing it through Nu.

## Streaming and cancellation

`Completed: false`

1. Decide whether a future `--stream` mode emits one record per response.
2. Add bounded channels and cancellation propagation from the Nu process.
3. Keep the current finite JSON result as the stable default.

## Privilege-free discovery fallback

`Completed: false`

1. Investigate a read-only neighbour-cache mode for environments where raw
   packet access is unavailable.
2. Mark cache-derived results separately from active ARP results.
3. Never report a cache snapshot as equivalent to an active scan.

## Cross-platform support

`Completed: false`

1. Keep interface discovery and serialization portable where pnet supports it.
2. Document platform-specific datalink backends and privilege models.
3. Gate Linux capability checks and add CI builds for supported targets.
