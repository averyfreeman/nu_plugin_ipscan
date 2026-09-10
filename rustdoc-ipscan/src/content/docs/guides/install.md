---
title: Install
description: Install and register nu_plugin_ipscan with Nushell.
---

`nu_plugin_ipscan` wraps the standalone `ipscan` ARP discovery binary. Build
both pieces when installing from this repository:

```sh
cargo install --path nu-plugin-ipscan --locked
cargo build --manifest-path Cargo.toml --bin ipscan --locked
```

The plugin targets Nushell `0.115.1` and requires `CAP_NET_RAW` or equivalent
privileges for packet capture and transmission.

## Register in Nushell

```nu
plugin add ~/.cargo/bin/nu_plugin_ipscan
plugin use ipscan
$env.NU_PLUGIN_IPSCAN_BACKEND = '/absolute/path/to/ipscan'
```

The backend is resolved from `--backend`, then
`NU_PLUGIN_IPSCAN_BACKEND`, then a sibling `ipscan` executable, and finally
`PATH`.

## Verify

```nu
ipscan --list
ipscan --network 192.0.2.0/24 --numeric
```

The plugin requests JSON automatically. Use `--raw` when you need exact
stdout bytes from the backend.
