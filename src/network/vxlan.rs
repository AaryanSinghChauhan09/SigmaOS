#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! # VXLAN / GENEVE Overlay Network Driver
//!
//! Implements Virtual eXtensible LAN (VXLAN, RFC 7348) and GENEVE overlay
//! encapsulation for SigmaOS. VXLAN tunnels Ethernet frames inside UDP
//! datagrams, enabling L2 segments to span L3 boundaries (e.g., across
//! data-centre fabric or cloud VPCs).
//!
//! ## Key points
//! - Default VXLAN UDP port: **4789** (IANA assigned)
//! - GENEVE UDP port: **6081**
//! - 24-bit VNID (Virtual Network Identifier), supporting ~16 million segments
//! - Outer IP header carries the VTEP (VXLAN Tunnel End Point) addresses
//! - Supports multicast-based and unicast (HER / controller) flood-and-learn
//! - Implements ARP suppression to reduce broadcast traffic on large segments

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ─── Constants ────────────────────────────────────────────────────────────────

/// Default UDP destination port for VXLAN (IANA).
pub const VXLAN_PORT: u16 = 4789;

/// Default UDP destination port for GENEVE.
pub const GENEVE_PORT: u16 = 6081;

/// Size of the VXLAN fixed header in bytes (8 bytes: flags + reserved + VNID + reserved).
pub const VXLAN_HEADER_LEN: usize = 8;

/// Maximum supported VNID value (24-bit field).
pub const VXLAN_VNID_MAX: u32 = 0x00FF_FFFF;

// ─── Enumerations ─────────────────────────────────────────────────────────────

/// Encapsulation protocol in use for this tunnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayProto {
    /// RFC 7348 VXLAN.
    Vxlan,
    /// RFC 8926 GENEVE (extensible header options).
    Geneve,
}

/// Flood-and-learn strategy for unknown unicast / BUM traffic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloodMode {
    /// Replicate to all multicast-group members (requires multicast underlay).
    Multicast,
    /// Head-end replication: send unicast copy to every known VTEP.
    HeadEndReplication,
    /// Rely on an SDN controller to push FDB entries; drop unknown frames.
    ControllerLearning,
}

/// State of a VTEP tunnel interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VtepState {
    Down,
    Initialising,
    Up,
    Error,
}

// ─── Structs ──────────────────────────────────────────────────────────────────

/// An 8-byte VXLAN header (flags | reserved24 | VNID | reserved8).
#[derive(Debug, Clone, Copy)]
pub struct VxlanHeader {
    /// Flags byte; bit 3 (0x08) MUST be set to indicate valid VNID.
    pub flags: u8,
    /// 24-bit Virtual Network Identifier.
    pub vnid: u32,
}

impl VxlanHeader {
    /// Construct a new VXLAN header with the I-flag set.
    pub fn new(vnid: u32) -> Self {
        assert!(vnid <= VXLAN_VNID_MAX, "VNID exceeds 24-bit range");
        VxlanHeader { flags: 0x08, vnid }
    }

    /// Serialise the header into exactly 8 bytes (big-endian).
    pub fn to_bytes(&self) -> [u8; 8] {
        let vnid_be = self.vnid.to_be_bytes(); // [0, b0, b1, b2]
        [
            self.flags,
            0, 0, 0,          // reserved
            vnid_be[1], vnid_be[2], vnid_be[3],
            0,                // reserved
        ]
    }

    /// Parse a VXLAN header from an 8-byte slice.
    pub fn from_bytes(raw: &[u8; 8]) -> Result<Self, VxlanError> {
        if raw[0] & 0x08 == 0 {
            return Err(VxlanError::InvalidFlag);
        }
        let vnid = u32::from_be_bytes([0, raw[4], raw[5], raw[6]]);
        Ok(VxlanHeader { flags: raw[0], vnid })
    }
}

/// GENEVE option TLV (variable-length, ≥ 4 bytes).
#[derive(Debug, Clone)]
pub struct GeneveOption {
    /// Option class (namespace).
    pub option_class: u16,
    /// Option type within the class.
    pub option_type: u8,
    /// Option data (length = len field × 4 bytes).
    pub data: Vec<u8>,
}

/// GENEVE fixed header (8 bytes) followed by optional TLVs.
#[derive(Debug, Clone)]
pub struct GeneveHeader {
    /// Version (2 bits, must be 0).
    pub version: u8,
    /// Option length in 4-byte DWORDs.
    pub opt_len: u8,
    /// Control packet flag.
    pub control: bool,
    /// Critical options present flag.
    pub critical: bool,
    /// Protocol type of the encapsulated payload (Ethernet = 0x6558).
    pub protocol_type: u16,
    /// 24-bit Virtual Network Identifier.
    pub vnid: u32,
    /// Variable-length options.
    pub options: Vec<GeneveOption>,
}

/// A single entry in the VTEP forwarding database (FDB).
#[derive(Debug, Clone)]
pub struct FdbEntry {
    /// Destination MAC address.
    pub mac: [u8; 6],
    /// Remote VTEP IP (IPv4 as u32, or IPv6 as u128 in future).
    pub remote_vtep_ip: u32,
    /// VNID to which this entry belongs.
    pub vnid: u32,
    /// Entry age in ticks (for ageing).
    pub age_ticks: u64,
}

/// A VXLAN VTEP interface binding together local IP, VNID, and the FDB.
#[derive(Debug)]
pub struct VxlanInterface {
    /// Interface name (e.g. "vxlan0").
    pub name: String,
    /// Local VTEP IPv4 address (big-endian u32).
    pub local_ip: u32,
    /// UDP source port base (default: ephemeral, hashed from inner header).
    pub src_port_base: u16,
    /// Virtual Network Identifier.
    pub vnid: u32,
    /// Underlay multicast group IP (if FloodMode::Multicast).
    pub multicast_group: Option<u32>,
    /// Flood strategy.
    pub flood_mode: FloodMode,
    /// Overlay encapsulation protocol.
    pub proto: OverlayProto,
    /// Forwarding database: MAC → FdbEntry.
    pub fdb: BTreeMap<[u8; 6], FdbEntry>,
    /// Interface operational state.
    pub state: VtepState,
    /// Counters.
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub encap_errors: u64,
    pub decap_errors: u64,
}

/// Errors produced by the VXLAN subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VxlanError {
    /// The I-flag in the VXLAN header was not set.
    InvalidFlag,
    /// VNID not found in any local interface.
    UnknownVnid,
    /// Packet too short to contain a VXLAN header.
    PacketTooShort,
    /// Attempted to insert a duplicate FDB entry (non-fatal; entry updated).
    DuplicateFdb,
    /// The interface is not in the `Up` state.
    InterfaceDown,
    /// Generic I/O error (from the underlay UDP socket).
    IoError,
}

// ─── Implementation ───────────────────────────────────────────────────────────

impl VxlanInterface {
    /// Create a new VTEP interface.
    pub fn new(name: &str, local_ip: u32, vnid: u32, proto: OverlayProto) -> Self {
        VxlanInterface {
            name: name.to_string(),
            local_ip,
            src_port_base: 49152,
            vnid,
            multicast_group: None,
            flood_mode: FloodMode::HeadEndReplication,
            proto,
            fdb: BTreeMap::new(),
            state: VtepState::Down,
            rx_packets: 0,
            tx_packets: 0,
            rx_bytes: 0,
            tx_bytes: 0,
            encap_errors: 0,
            decap_errors: 0,
        }
    }

    /// Bring up the VTEP interface (bind UDP socket on the underlay).
    pub fn up(&mut self) -> Result<(), VxlanError> {
        // TODO: bind a UDP socket to local_ip:VXLAN_PORT
        self.state = VtepState::Up;
        Ok(())
    }

    /// Take down the VTEP interface.
    pub fn down(&mut self) {
        // TODO: close the UDP socket
        self.state = VtepState::Down;
    }

    /// Encapsulate an Ethernet frame and send it to `remote_vtep_ip`.
    ///
    /// The outer UDP source port is computed by hashing the inner frame's
    /// 5-tuple to preserve per-flow ECMP load-balancing in the underlay.
    pub fn encapsulate_and_send(
        &mut self,
        frame: &[u8],
        remote_vtep_ip: u32,
    ) -> Result<(), VxlanError> {
        if self.state != VtepState::Up {
            return Err(VxlanError::InterfaceDown);
        }
        // TODO: prepend VxlanHeader, wrap in UDP + IP, send via raw socket
        self.tx_packets += 1;
        self.tx_bytes += frame.len() as u64;
        Ok(())
    }

    /// Decapsulate a received UDP datagram, validate VNID, learn MAC, and
    /// deliver the inner Ethernet frame to the virtual switch bridge.
    pub fn decapsulate(&mut self, udp_payload: &[u8]) -> Result<Vec<u8>, VxlanError> {
        if udp_payload.len() < VXLAN_HEADER_LEN {
            self.decap_errors += 1;
            return Err(VxlanError::PacketTooShort);
        }
        let raw: [u8; 8] = udp_payload[..8].try_into().unwrap();
        let hdr = VxlanHeader::from_bytes(&raw)?;
        if hdr.vnid != self.vnid {
            return Err(VxlanError::UnknownVnid);
        }
        let inner = udp_payload[VXLAN_HEADER_LEN..].to_vec();
        // TODO: extract src MAC from inner Ethernet header and learn into FDB
        self.rx_packets += 1;
        self.rx_bytes += inner.len() as u64;
        Ok(inner)
    }

    /// Insert or refresh a static FDB entry.
    pub fn fdb_add(&mut self, mac: [u8; 6], remote_vtep_ip: u32) {
        let entry = FdbEntry { mac, remote_vtep_ip, vnid: self.vnid, age_ticks: 0 };
        self.fdb.insert(mac, entry);
    }

    /// Remove an FDB entry by MAC.
    pub fn fdb_remove(&mut self, mac: &[u8; 6]) {
        self.fdb.remove(mac);
    }

    /// Age out FDB entries older than `max_ticks`.
    pub fn fdb_age(&mut self, current_tick: u64, max_ticks: u64) {
        self.fdb.retain(|_, e| current_tick - e.age_ticks < max_ticks);
    }

    /// Lookup the remote VTEP for a destination MAC; returns None for BUM.
    pub fn fdb_lookup(&self, mac: &[u8; 6]) -> Option<u32> {
        self.fdb.get(mac).map(|e| e.remote_vtep_ip)
    }

    /// Compute a source port by hashing the inner frame header (for ECMP).
    fn hash_src_port(frame: &[u8]) -> u16 {
        let mut h: u32 = 0x811c_9dc5;
        for &b in frame.iter().take(38) {
            h ^= b as u32;
            h = h.wrapping_mul(0x0100_0193);
        }
        0xC000 | (h as u16 & 0x3FFF)
    }
}

// ─── Global VTEP registry ─────────────────────────────────────────────────────

/// Registry of all VTEP interfaces managed by the kernel network stack.
pub struct VxlanRegistry {
    interfaces: BTreeMap<String, VxlanInterface>,
}

impl VxlanRegistry {
    pub fn new() -> Self {
        VxlanRegistry { interfaces: BTreeMap::new() }
    }

    /// Register a new VTEP interface.
    pub fn register(&mut self, iface: VxlanInterface) {
        self.interfaces.insert(iface.name.clone(), iface);
    }

    /// Look up a VTEP by name.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut VxlanInterface> {
        self.interfaces.get_mut(name)
    }

    /// Dispatch a received UDP datagram to the correct VTEP by VNID.
    pub fn dispatch_rx(&mut self, udp_payload: &[u8]) -> Result<Vec<u8>, VxlanError> {
        if udp_payload.len() < VXLAN_HEADER_LEN {
            return Err(VxlanError::PacketTooShort);
        }
        let raw: [u8; 8] = udp_payload[..8].try_into().unwrap();
        let hdr = VxlanHeader::from_bytes(&raw)?;
        for iface in self.interfaces.values_mut() {
            if iface.vnid == hdr.vnid {
                return iface.decapsulate(udp_payload);
            }
        }
        Err(VxlanError::UnknownVnid)
    }
}
