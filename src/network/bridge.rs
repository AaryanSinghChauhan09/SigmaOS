#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! # IEEE 802.1D Transparent Bridging with Spanning Tree Protocol
//!
//! Implements a software Ethernet bridge for SigmaOS that:
//!
//! - **Learns** source MAC addresses from ingress frames and populates a
//!   forwarding table (MAC table / FDB).
//! - **Forwards** unicast frames to the single port associated with the
//!   destination MAC, or **floods** to all ports on unknown unicast / broadcast
//!   / multicast.
//! - **Runs STP** (IEEE 802.1D-2004) to eliminate Layer-2 loops by exchanging
//!   Bridge Protocol Data Units (BPDUs) and placing ports in one of five states:
//!   `Blocking → Listening → Learning → Forwarding` (or `Disabled`).
//!
//! ## BPDU Format (Configuration BPDU, 35 bytes)
//! ```text
//! [Protocol ID 2B][Version 1B][Type 1B][Flags 1B]
//! [Root BID 8B][Root Path Cost 4B][Bridge BID 8B][Port ID 2B]
//! [Message Age 2B][Max Age 2B][Hello Time 2B][Fwd Delay 2B]
//! ```
//!
//! ## STP Timers (in units of 1/256 seconds)
//! - Hello Time: 2 s (bridge transmits BPDUs every 2 s)
//! - Max Age: 20 s (BPDU lifetime)
//! - Forward Delay: 15 s (time spent in Listening and Learning states)

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ─── Constants ────────────────────────────────────────────────────────────────

/// Multicast destination MAC for STP BPDUs (IEEE 802.1D).
pub const STP_BPDU_DEST_MAC: [u8; 6] = [0x01, 0x80, 0xC2, 0x00, 0x00, 0x00];

/// STP BPDU EtherType / length field.
pub const STP_BPDU_LLC: [u8; 3] = [0x42, 0x42, 0x03];

/// Default bridge priority (in units of 4096, priority 32768 = 0x8000).
pub const STP_DEFAULT_BRIDGE_PRIORITY: u16 = 32768;

/// Default port priority.
pub const STP_DEFAULT_PORT_PRIORITY: u8 = 128;

/// Default Hello Time (2 seconds in 1/256 s units = 512).
pub const STP_HELLO_TIME: u16 = 512;

/// Default Max Age (20 seconds in 1/256 s units = 5120).
pub const STP_MAX_AGE: u16 = 5120;

/// Default Forward Delay (15 seconds in 1/256 s units = 3840).
pub const STP_FORWARD_DELAY: u16 = 3840;

/// FDB entry ageing time in ticks.
pub const FDB_AGEING_TICKS: u64 = 300;

// ─── Enumerations ─────────────────────────────────────────────────────────────

/// STP port states per IEEE 802.1D.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StpState {
    /// Port is administratively disabled.
    Disabled,
    /// Port discards frames; participates in BPDU exchange.
    Blocking,
    /// Port discards frames; participates in BPDU exchange (transitional).
    Listening,
    /// Port discards frames; learns MAC addresses.
    Learning,
    /// Port both learns and forwards frames.
    Forwarding,
}

/// STP port role in the spanning tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StpRole {
    /// Port elected as root port (best path to root bridge).
    Root,
    /// Port elected as designated port on its segment.
    Designated,
    /// Port put into blocking state to break a loop.
    Alternate,
    /// Backup to a designated port on the same segment.
    Backup,
    /// Disabled.
    Disabled,
}

/// Errors from the bridge subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeError {
    /// Port name already registered.
    DuplicatePort,
    /// Port not found in the bridge.
    PortNotFound,
    /// Malformed BPDU (too short or wrong protocol).
    InvalidBpdu,
    /// Frame too short for an Ethernet header.
    FrameTooShort,
    /// Bridge has no ports.
    NoPorts,
}

// ─── Bridge Identifier ───────────────────────────────────────────────────────

/// 8-byte Bridge Identifier: 2-byte priority | 6-byte MAC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeId {
    pub priority: u16,
    pub mac: [u8; 6],
}

impl BridgeId {
    pub fn new(priority: u16, mac: [u8; 6]) -> Self {
        BridgeId { priority, mac }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let p = self.priority.to_be_bytes();
        [
            p[0],
            p[1],
            self.mac[0],
            self.mac[1],
            self.mac[2],
            self.mac[3],
            self.mac[4],
            self.mac[5],
        ]
    }
}

// ─── BPDU ─────────────────────────────────────────────────────────────────────

/// Decoded IEEE 802.1D Configuration BPDU.
#[derive(Debug, Clone)]
pub struct ConfigBpdu {
    pub flags: u8,
    pub root_id: BridgeId,
    pub root_path_cost: u32,
    pub bridge_id: BridgeId,
    pub port_id: u16,
    pub message_age: u16,
    pub max_age: u16,
    pub hello_time: u16,
    pub forward_delay: u16,
}

impl ConfigBpdu {
    /// Serialize to 35-byte wire format (preceded by 3-byte LLC).
    pub fn to_bytes(&self) -> [u8; 35] {
        let mut b = [0u8; 35];
        // Protocol ID = 0x0000, Version = 0x00, Type = 0x00 (Config)
        b[0] = 0;
        b[1] = 0;
        b[2] = 0;
        b[3] = 0;
        b[4] = self.flags;
        let rb = self.root_id.to_bytes();
        b[5..13].copy_from_slice(&rb);
        let rpc = self.root_path_cost.to_be_bytes();
        b[13..17].copy_from_slice(&rpc);
        let bb = self.bridge_id.to_bytes();
        b[17..25].copy_from_slice(&bb);
        let pid = self.port_id.to_be_bytes();
        b[25..27].copy_from_slice(&pid);
        b[27..29].copy_from_slice(&self.message_age.to_be_bytes());
        b[29..31].copy_from_slice(&self.max_age.to_be_bytes());
        b[31..33].copy_from_slice(&self.hello_time.to_be_bytes());
        b[33..35].copy_from_slice(&self.forward_delay.to_be_bytes());
        b
    }

    /// Parse from a 35-byte slice.
    pub fn from_bytes(raw: &[u8]) -> Result<Self, BridgeError> {
        if raw.len() < 35 {
            return Err(BridgeError::InvalidBpdu);
        }
        let root_id = BridgeId {
            priority: u16::from_be_bytes([raw[5], raw[6]]),
            mac: raw[7..13].try_into().unwrap(),
        };
        let bridge_id = BridgeId {
            priority: u16::from_be_bytes([raw[17], raw[18]]),
            mac: raw[19..25].try_into().unwrap(),
        };
        Ok(ConfigBpdu {
            flags: raw[4],
            root_id,
            root_path_cost: u32::from_be_bytes(raw[13..17].try_into().unwrap()),
            bridge_id,
            port_id: u16::from_be_bytes([raw[25], raw[26]]),
            message_age: u16::from_be_bytes([raw[27], raw[28]]),
            max_age: u16::from_be_bytes([raw[29], raw[30]]),
            hello_time: u16::from_be_bytes([raw[31], raw[32]]),
            forward_delay: u16::from_be_bytes([raw[33], raw[34]]),
        })
    }
}

// ─── FDB Entry ────────────────────────────────────────────────────────────────

/// A single entry in the Forwarding Database.
#[derive(Debug, Clone)]
pub struct FdbEntry {
    /// Port index where this MAC was learned.
    pub port_idx: usize,
    /// Tick at which this entry was last refreshed.
    pub last_seen: u64,
    /// True if statically configured (not aged out).
    pub is_static: bool,
}

// ─── Bridge Port ──────────────────────────────────────────────────────────────

/// A port attached to the software bridge.
#[derive(Debug)]
pub struct BridgePort {
    pub name: String,
    pub mac: [u8; 6],
    pub stp_state: StpState,
    pub stp_role: StpRole,
    /// Port path cost (depends on link speed; 100 Mbps → 19, 1 Gbps → 4).
    pub path_cost: u32,
    pub port_id: u16,
    pub designated_root: BridgeId,
    pub designated_cost: u32,
    pub designated_bridge: BridgeId,
    pub designated_port: u16,
    pub message_age_timer: u16,
    pub forward_delay_timer: u16,
    pub hello_timer: u16,
    pub rx_bpdu: u64,
    pub tx_bpdu: u64,
}

impl BridgePort {
    pub fn new(name: &str, mac: [u8; 6], path_cost: u32, port_id: u16) -> Self {
        BridgePort {
            name: name.to_string(),
            mac,
            stp_state: StpState::Blocking,
            stp_role: StpRole::Designated,
            path_cost,
            port_id,
            designated_root: BridgeId::new(STP_DEFAULT_BRIDGE_PRIORITY, [0u8; 6]),
            designated_cost: 0,
            designated_bridge: BridgeId::new(STP_DEFAULT_BRIDGE_PRIORITY, [0u8; 6]),
            designated_port: port_id,
            message_age_timer: 0,
            forward_delay_timer: 0,
            hello_timer: 0,
            rx_bpdu: 0,
            tx_bpdu: 0,
        }
    }

    /// Returns true if the port may forward data frames.
    pub fn is_forwarding(&self) -> bool {
        self.stp_state == StpState::Forwarding
    }

    /// Returns true if the port may learn MAC addresses.
    pub fn is_learning(&self) -> bool {
        matches!(self.stp_state, StpState::Learning | StpState::Forwarding)
    }
}

// ─── Bridge ───────────────────────────────────────────────────────────────────

/// IEEE 802.1D software bridge.
pub struct Bridge {
    pub name: String,
    pub bridge_id: BridgeId,
    pub root_id: BridgeId,
    pub root_path_cost: u32,
    pub root_port_idx: Option<usize>,
    pub ports: Vec<BridgePort>,
    /// MAC forwarding database.
    pub fdb: BTreeMap<[u8; 6], FdbEntry>,
    pub hello_time: u16,
    pub max_age: u16,
    pub forward_delay: u16,
    pub tick: u64,
    pub tx_frames: u64,
    pub rx_frames: u64,
    pub flooded_frames: u64,
    pub dropped_frames: u64,
}

impl Bridge {
    /// Create a new bridge.
    pub fn new(name: &str, mac: [u8; 6]) -> Self {
        let bid = BridgeId::new(STP_DEFAULT_BRIDGE_PRIORITY, mac);
        Bridge {
            name: name.to_string(),
            bridge_id: bid,
            root_id: bid,
            root_path_cost: 0,
            root_port_idx: None,
            ports: Vec::new(),
            fdb: BTreeMap::new(),
            hello_time: STP_HELLO_TIME,
            max_age: STP_MAX_AGE,
            forward_delay: STP_FORWARD_DELAY,
            tick: 0,
            tx_frames: 0,
            rx_frames: 0,
            flooded_frames: 0,
            dropped_frames: 0,
        }
    }

    /// Add a port to the bridge.
    pub fn add_port(&mut self, port: BridgePort) -> Result<(), BridgeError> {
        if self.ports.iter().any(|p| p.name == port.name) {
            return Err(BridgeError::DuplicatePort);
        }
        self.ports.push(port);
        Ok(())
    }

    /// Remove a port by name.
    pub fn remove_port(&mut self, name: &str) -> Result<(), BridgeError> {
        let pos = self
            .ports
            .iter()
            .position(|p| p.name == name)
            .ok_or(BridgeError::PortNotFound)?;
        self.ports.remove(pos);
        // Flush FDB entries learned on this port
        self.fdb.retain(|_, e| e.port_idx != pos);
        Ok(())
    }

    /// Learn a source MAC on an ingress port (if the port allows learning).
    fn learn(&mut self, mac: [u8; 6], port_idx: usize) {
        let entry = FdbEntry {
            port_idx,
            last_seen: self.tick,
            is_static: false,
        };
        self.fdb.insert(mac, entry);
    }

    /// Forward a frame received on `ingress_port_idx`.
    ///
    /// Returns a list of `(port_idx, frame)` pairs representing frames to
    /// transmit on each port.
    pub fn forward(
        &mut self,
        frame: Vec<u8>,
        ingress_port_idx: usize,
    ) -> Result<Vec<(usize, Vec<u8>)>, BridgeError> {
        if frame.len() < 14 {
            self.dropped_frames += 1;
            return Err(BridgeError::FrameTooShort);
        }
        self.rx_frames += 1;

        let src_mac: [u8; 6] = frame[6..12].try_into().unwrap();
        let dst_mac: [u8; 6] = frame[0..6].try_into().unwrap();

        // Learn source MAC
        if let Some(port) = self.ports.get(ingress_port_idx) {
            if port.is_learning() {
                self.learn(src_mac, ingress_port_idx);
            }
        }

        // Lookup destination
        let mut outputs = Vec::new();
        if let Some(entry) = self.fdb.get(&dst_mac) {
            let egress = entry.port_idx;
            if egress != ingress_port_idx {
                if self.ports.get(egress).map_or(false, |p| p.is_forwarding()) {
                    self.tx_frames += 1;
                    outputs.push((egress, frame));
                }
            }
        } else {
            // Flood: send to all forwarding ports except ingress
            self.flooded_frames += 1;
            for i in 0..self.ports.len() {
                if i != ingress_port_idx && self.ports[i].is_forwarding() {
                    self.tx_frames += 1;
                    outputs.push((i, frame.clone()));
                }
            }
        }
        Ok(outputs)
    }

    /// Process a received BPDU on the specified port and run STP decision.
    pub fn receive_bpdu(&mut self, port_idx: usize, bpdu_bytes: &[u8]) -> Result<(), BridgeError> {
        let bpdu = ConfigBpdu::from_bytes(bpdu_bytes)?;
        if port_idx >= self.ports.len() {
            return Err(BridgeError::PortNotFound);
        }
        self.ports[port_idx].rx_bpdu += 1;
        // TODO: full STP algorithm: compare priorities, update root, roles, states
        Ok(())
    }

    /// Generate a Configuration BPDU for the given port.
    pub fn generate_bpdu(&self, port_idx: usize) -> Result<ConfigBpdu, BridgeError> {
        let port = self.ports.get(port_idx).ok_or(BridgeError::PortNotFound)?;
        Ok(ConfigBpdu {
            flags: 0,
            root_id: self.root_id,
            root_path_cost: self.root_path_cost,
            bridge_id: self.bridge_id,
            port_id: port.port_id,
            message_age: 0,
            max_age: self.max_age,
            hello_time: self.hello_time,
            forward_delay: self.forward_delay,
        })
    }

    /// Advance the bridge timer by one tick; age FDB entries and drive STP timers.
    pub fn tick_timer(&mut self) {
        self.tick += 1;
        // Age FDB entries
        self.fdb
            .retain(|_, e| e.is_static || (self.tick - e.last_seen < FDB_AGEING_TICKS));
        // TODO: decrement STP timers per port and trigger state transitions
    }

    /// Set the bridge MAC priority (lower = more likely to become root).
    pub fn set_priority(&mut self, priority: u16) {
        self.bridge_id.priority = priority & 0xF000; // must be multiple of 4096
    }
}
