---
title: Use `ipscan`
description: Discover local IPv4 hosts and compose the results in Nushell.
---

## List interfaces

The plugin preserves the standalone interface inventory as structured records:

```nu
ipscan --list | select name index mac ips is_up is_loopback
ipscan --list | where is_up and not is_loopback
```

## Scan a network

```nu
ipscan --network 192.168.1.0/24 --numeric
ipscan --interface eth0 --timeout 5s --retry 2
```

Each result contains an IPv4 address, MAC address, optional reverse-DNS name,
and optional vendor lookup. The summary also reports packets received,
filtered ARP packets, and elapsed time.

## Filter and export

Because the plugin returns native values, use ordinary Nushell pipelines:

```nu
ipscan --network 192.168.1.0/24 --numeric |
  where vendor != '' |
  select ipv4 mac vendor

ipscan --network 192.168.1.0/24 --numeric |
  to json | save hosts.json
```

The scanner sends Ethernet/ARP frames rather than using the kernel neighbour
cache. Run it only on networks you are authorized to inspect, and expect
interfaces, VLANs, firewalls, and container capabilities to affect results.
