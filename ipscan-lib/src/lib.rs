//! Reusable packet and timing primitives for `ipscan`.
//!
//! The library performs local IPv4 discovery with crafted Ethernet/ARP
//! frames. It is intentionally Linux-friendly but keeps pnet's cross-platform
//! interface model for interface enumeration and packet channels. The CLI
//! owns process exits and presentation; these modules own scan data and packet
//! mechanics.

pub mod network;
pub mod scan_options;
pub mod time;
pub mod utils;
pub mod vendor;
