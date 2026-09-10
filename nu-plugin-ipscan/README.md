# `nu_plugin_ipscan`

This Nushell `0.115.1` plugin exposes the standalone `ipscan` ARP discovery
tool as a structured `ipscan` command. It forwards arguments without a shell,
requests JSON output, and converts the result into native Nushell values.

```nu
plugin add ~/.cargo/bin/nu_plugin_ipscan
plugin use ipscan

ipscan --network 192.168.1.0/24 --numeric
ipscan --interface eth0 --output json | where vendor != ''
```

Set `NU_PLUGIN_IPSCAN_BACKEND` when the `ipscan` executable is not on `PATH` or
beside the plugin executable. Use `--raw` when exact text/byte output is
needed.
