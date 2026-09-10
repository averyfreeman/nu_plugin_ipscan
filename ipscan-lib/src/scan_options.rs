//! Scan configuration shared by the standalone binary and future front ends.

use ipnetwork::IpNetwork;
use pnet::datalink::MacAddr;
use pnet::packet::arp::{ArpHardwareType, ArpOperation};
use pnet::packet::ethernet::EtherType;
use std::net::Ipv4Addr;

/// Output representation selected by the standalone CLI.
pub enum OutputFormat {
    /// Human-oriented terminal table output.
    Plain,
    /// Structured JSON output used by the Nushell adapter.
    Json,
    /// YAML serialization for files and shell pipelines.
    Yaml,
    /// CSV serialization for tabular tooling.
    Csv,
}

/// Named timing/profile presets accepted by `ipscan`.
pub enum ProfileType {
    /// Balanced default timings.
    Default,
    /// Faster requests with a greater chance of missed replies.
    Fast,
    /// Slower requests intended to reduce network impact.
    Stealth,
    /// Randomized timing and scan parameters.
    Chaos,
}

/// Either an inter-packet interval or a target bandwidth.
pub enum ScanTiming {
    /// Delay between successive requests, in milliseconds.
    Interval(u64),
    /// Approximate request bandwidth, in bits per second.
    Bandwidth(u64),
}

/// Immutable options used throughout one scan invocation.
pub struct ScanOptions {
    /// Interface name selected by the caller.
    pub interface_name: Option<String>,
    /// Interface index selected by the caller.
    pub interface_index: Option<u32>,
    /// Explicit IPv4 ranges to scan.
    pub network_range: Option<Vec<IpNetwork>>,
    /// Minimum response collection time, in milliseconds.
    pub timeout_ms: u64,
    /// Whether reverse-DNS lookup is enabled.
    pub resolve_hostname: bool,
    /// Optional source IPv4 address for crafted ARP requests.
    pub source_ipv4: Option<Ipv4Addr>,
    /// Optional source hardware address for crafted Ethernet frames.
    pub source_mac: Option<MacAddr>,
    /// Optional destination hardware address for crafted frames.
    pub destination_mac: Option<MacAddr>,
    /// Optional 802.1Q VLAN identifier.
    pub vlan_id: Option<u16>,
    /// Number of request passes over the target set.
    pub retry_count: usize,
    /// Request pacing mode.
    pub scan_timing: ScanTiming,
    /// Whether target addresses should be randomized.
    pub randomize_targets: bool,
    /// Selected output representation.
    pub output: OutputFormat,
    /// Path to the optional IEEE OUI database.
    pub oui_file: String,
    /// Optional ARP hardware type override.
    pub hw_type: Option<ArpHardwareType>,
    /// Optional ARP hardware-address length override.
    pub hw_addr: Option<u8>,
    /// Optional ARP protocol type override.
    pub proto_type: Option<EtherType>,
    /// Optional ARP protocol-address length override.
    pub proto_addr: Option<u8>,
    /// Optional ARP operation override.
    pub arp_operation: Option<ArpOperation>,
    /// Whether the packet-layout help should be printed.
    pub packet_help: bool,
}

impl ScanOptions {
    /// Return `true` when the selected output is the terminal table.
    pub fn is_plain_output(&self) -> bool {
        matches!(&self.output, OutputFormat::Plain)
    }

    /// Return `true` when VLAN framing changes the packet size.
    pub fn has_vlan(&self) -> bool {
        self.vlan_id.is_some()
    }

    /// Return `true` when the packet-layout help path was requested.
    pub fn request_protocol_print(&self) -> bool {
        self.packet_help
    }
}
