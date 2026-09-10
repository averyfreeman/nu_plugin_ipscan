---
title: Output schema
description: Structured interface and host records returned by ipscan.
---

The `ipscan` plugin requests the standalone binary's JSON format and converts
it into Nushell records. The exact result depends on whether the command is a
scan or `--list`.

## Scan result

| Field | Type | Meaning |
| --- | --- | --- |
| `packet_count` | `int` | Ethernet packets received during the scan. |
| `arp_count` | `int` | ARP packets accepted by the decoder. |
| `duration_ms` | `int` | Response collection duration. |
| `results` | `list<record>` | Hosts discovered by IPv4 address. |

Each `results` row contains:

| Field | Type | Meaning |
| --- | --- | --- |
| `ipv4` | `string` | Discovered IPv4 address. |
| `mac` | `string` | Sender hardware address. |
| `hostname` | `string` | Reverse-DNS name, or an empty string. |
| `vendor` | `string` | OUI vendor name, or an empty string. |

## Interface inventory

`ipscan --list` returns rows with `name`, `index`, `mac`, `ips`, `is_up`,
`is_loopback`, `is_broadcast`, and `is_multicast`. This form does not open a
packet channel and can be used to choose an interface before a privileged
scan.
