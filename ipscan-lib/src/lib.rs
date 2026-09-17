//! Reusable packet and timing primitives for `ipscan`.
//!
//! The library performs local IPv4 discovery with crafted Ethernet/ARP
//! frames. It is intentionally Linux-friendly but keeps pnet's cross-platform
//! interface model for interface enumeration and packet channels. The CLI
//! owns process exits and presentation; these modules own scan data and packet
//! mechanics.

/// Exposes the public `network` module.
pub mod network;
/// Exposes the public `scan_options` module.
pub mod scan_options;
/// Exposes the public `time` module.
pub mod time;
/// Exposes the public `utils` module.
pub mod utils;
/// Exposes the public `vendor` module.
pub mod vendor;
