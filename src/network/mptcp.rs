#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! # Multipath TCP (MPTCP) — RFC 8684
//!
//! Implements the MPTCP extension to TCP for SigmaOS, enabling a single
//! transport connection to simultaneously use multiple network paths.
//!
//! ## Key concepts
//! - **MPTCP connection**: the logical entity shared across all subflows.
//! - **Subflow**: an ordinary TCP connection carrying MPTCP data.
//! - **Data Sequence Number (DSN)**: a 64-bit sequence number at the
//!   connection level, independent of per-subflow TCP sequence numbers.
//! - **Data Sequence Signal (DSS)**: the TCP option (kind=30, sub-type=0x2)
//!   that maps subflow-level data to connection-level DSNs.
//!
//! ## Handshake (MP_CAPABLE)
//! ```text
//! Client → Server: SYN  + MP_CAPABLE(key=0, flags=0x01)
//! Server → Client: SYN-ACK + MP_CAPABLE(key=1, flags=0x01)
//! Client → Server: ACK  + MP_CAPABLE(key=0|key=1, flags=0x01)
//! ```
//!
//! ## Subflow addition (MP_JOIN)
//! ```text
//! Initiator → Responder: SYN  + MP_JOIN(token, rand_A)
//! Responder → Initiator: SYN-ACK + MP_JOIN(HMAC_B, rand_B)
//! Initiator → Responder: ACK  + MP_JOIN(HMAC_A)
//! ```

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ─── Constants ────────────────────────────────────────────────────────────────

/// MPTCP TCP option kind (IANA-assigned).
pub const MPTCP_OPT_KIND: u8 = 30;

/// MPTCP sub-type for MP_CAPABLE.
pub const MPTCP_SUB_CAPABLE: u8 = 0x0;

/// MPTCP sub-type for MP_JOIN.
pub const MPTCP_SUB_JOIN: u8 = 0x1;

/// MPTCP sub-type for DSS (Data Sequence Signal).
pub const MPTCP_SUB_DSS: u8 = 0x2;

/// MPTCP sub-type for ADD_ADDR.
pub const MPTCP_SUB_ADD_ADDR: u8 = 0x3;

/// MPTCP sub-type for REMOVE_ADDR.
pub const MPTCP_SUB_REMOVE_ADDR: u8 = 0x4;

/// MPTCP sub-type for MP_FASTCLOSE.
pub const MPTCP_SUB_FASTCLOSE: u8 = 0x7;

/// MPTCP version 1 (RFC 8684).
pub const MPTCP_VERSION: u8 = 1;

// ─── Enumerations ─────────────────────────────────────────────────────────────

/// State of the overall MPTCP connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MptcpState {
    Closed,
    /// Waiting for MP_CAPABLE exchange.
    Handshake,
    Established,
    /// Remote side sent DATA_FIN.
    CloseWait,
    /// Local side sent DATA_FIN.
    FinWait,
    TimeWait,
}

/// State of a single TCP subflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubflowState {
    Closed,
    /// MP_JOIN handshake in progress.
    Joining,
    Established,
    /// Being gracefully removed.
    Removing,
}

/// Scheduler policy for distributing segments across subflows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheduler {
    /// Round-robin across all established subflows.
    RoundRobin,
    /// Send on lowest-RTT subflow first.
    LowestRtt,
    /// Redundant: send on all subflows (for reliability, wastes bandwidth).
    Redundant,
}

/// MPTCP-specific errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MptcpError {
    /// Received an option with an unrecognised sub-type.
    UnknownSubtype,
    /// HMAC verification for MP_JOIN failed.
    HmacMismatch,
    /// DSN map indicates a gap in sequence space.
    DsnGap,
    /// Subflow ID not found.
    SubflowNotFound,
    /// Connection is not in a valid state for this operation.
    InvalidState,
    /// Too many subflows (implementation limit reached).
    TooManySubflows,
}

// ─── MPTCP Options ───────────────────────────────────────────────────────────

/// Decoded MP_CAPABLE TCP option.
#[derive(Debug, Clone, Copy)]
pub struct MpCapable {
    /// Version (must be 1 for RFC 8684).
    pub version: u8,
    /// Flags byte (bit 0 = checksum required, bit 7 = B flag for fallback).
    pub flags: u8,
    /// Sender's key (64-bit random).
    pub sender_key: u64,
    /// Receiver's key (present in SYN-ACK and third-leg ACK).
    pub receiver_key: Option<u64>,
}

/// Decoded MP_JOIN TCP option (SYN variant).
#[derive(Debug, Clone, Copy)]
pub struct MpJoinSyn {
    /// Backup path flag.
    pub backup: bool,
    /// Address ID of this path.
    pub addr_id: u8,
    /// Receiver's token (truncated SHA-256 of receiver's key).
    pub receiver_token: u32,
    /// Initiator's random nonce.
    pub sender_random: u32,
}

/// Decoded Data Sequence Signal (DSS) option.
#[derive(Debug, Clone)]
pub struct DssOption {
    /// Data FIN flag.
    pub data_fin: bool,
    /// Whether DSN is 8 bytes (true) or 4 bytes.
    pub dsn_long: bool,
    /// Data Sequence Number.
    pub dsn: u64,
    /// Subflow Sequence Number (relative to this subflow).
    pub ssn: u32,
    /// Data-level length of this segment.
    pub data_len: u16,
    /// Optional data-level checksum.
    pub checksum: Option<u16>,
}

// ─── Subflow ──────────────────────────────────────────────────────────────────

/// A single TCP subflow within an MPTCP connection.
#[derive(Debug)]
pub struct Subflow {
    pub id: u8,
    pub local_addr: u32,
    pub local_port: u16,
    pub remote_addr: u32,
    pub remote_port: u16,
    pub state: SubflowState,
    /// Per-subflow send sequence number.
    pub snd_nxt: u32,
    /// Per-subflow receive sequence number.
    pub rcv_nxt: u32,
    /// Estimated round-trip time (microseconds).
    pub rtt_us: u32,
    /// Whether this is the backup path.
    pub is_backup: bool,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
}

impl Subflow {
    pub fn new(
        id: u8,
        local_addr: u32,
        local_port: u16,
        remote_addr: u32,
        remote_port: u16,
    ) -> Self {
        Subflow {
            id,
            local_addr,
            local_port,
            remote_addr,
            remote_port,
            state: SubflowState::Closed,
            snd_nxt: 0,
            rcv_nxt: 0,
            rtt_us: 100_000,
            is_backup: false,
            tx_bytes: 0,
            rx_bytes: 0,
        }
    }
}

// ─── DSN Mapping ─────────────────────────────────────────────────────────────

/// Maps a range of subflow sequence numbers to connection-level DSNs.
#[derive(Debug, Clone)]
pub struct DsnMap {
    pub dsn: u64,
    pub ssn: u32,
    pub len: u16,
}

// ─── Receive Buffer ──────────────────────────────────────────────────────────

/// Out-of-order data reassembly queue keyed by DSN.
pub type ReassemblyQueue = BTreeMap<u64, Vec<u8>>;

// ─── MPTCP Connection ────────────────────────────────────────────────────────

/// The top-level MPTCP connection object.
pub struct MptcpConnection {
    /// Connection identifier (token).
    pub token: u32,
    /// Local key for MP_CAPABLE.
    pub local_key: u64,
    /// Remote key (set after handshake).
    pub remote_key: Option<u64>,
    /// Overall connection state.
    pub state: MptcpState,
    /// Active subflows indexed by subflow ID.
    pub subflows: BTreeMap<u8, Subflow>,
    /// Next subflow ID to assign.
    pub next_subflow_id: u8,
    /// Connection-level send sequence number.
    pub snd_dsn: u64,
    /// Connection-level receive sequence number (next expected).
    pub rcv_dsn: u64,
    /// Reassembly queue for out-of-order segments.
    pub reassembly: ReassemblyQueue,
    /// Scheduler in use.
    pub scheduler: Scheduler,
    /// Cumulative bytes sent at MPTCP level.
    pub tx_bytes: u64,
    /// Cumulative bytes received at MPTCP level.
    pub rx_bytes: u64,
}

impl MptcpConnection {
    /// Create a new MPTCP connection (client-side initiator).
    pub fn new(local_key: u64) -> Self {
        let token = (local_key >> 32) as u32 ^ (local_key & 0xFFFF_FFFF) as u32;
        MptcpConnection {
            token,
            local_key,
            remote_key: None,
            state: MptcpState::Closed,
            subflows: BTreeMap::new(),
            next_subflow_id: 0,
            snd_dsn: 0,
            rcv_dsn: 0,
            reassembly: BTreeMap::new(),
            scheduler: Scheduler::LowestRtt,
            tx_bytes: 0,
            rx_bytes: 0,
        }
    }

    /// Initiate the MP_CAPABLE handshake (build a SYN option).
    pub fn build_mp_capable_syn(&self) -> MpCapable {
        MpCapable {
            version: MPTCP_VERSION,
            flags: 0x00,
            sender_key: self.local_key,
            receiver_key: None,
        }
    }

    /// Process a received MP_CAPABLE in SYN-ACK; record remote key.
    pub fn process_mp_capable_synack(&mut self, opt: MpCapable) -> Result<(), MptcpError> {
        if self.state != MptcpState::Closed {
            return Err(MptcpError::InvalidState);
        }
        self.remote_key = Some(opt.sender_key);
        self.state = MptcpState::Handshake;
        Ok(())
    }

    /// Complete the handshake after the ACK is sent.
    pub fn complete_handshake(&mut self) -> Result<(), MptcpError> {
        if self.state != MptcpState::Handshake {
            return Err(MptcpError::InvalidState);
        }
        self.state = MptcpState::Established;
        Ok(())
    }

    /// Add a new subflow (MP_JOIN initiated).
    pub fn add_subflow(&mut self, subflow: Subflow) -> Result<u8, MptcpError> {
        if self.subflows.len() >= 32 {
            return Err(MptcpError::TooManySubflows);
        }
        let id = self.next_subflow_id;
        self.next_subflow_id = self.next_subflow_id.wrapping_add(1);
        self.subflows.insert(id, subflow);
        Ok(id)
    }

    /// Remove a subflow by ID.
    pub fn remove_subflow(&mut self, id: u8) -> Result<(), MptcpError> {
        self.subflows
            .remove(&id)
            .ok_or(MptcpError::SubflowNotFound)?;
        Ok(())
    }

    /// Select the best subflow for the next transmission according to the scheduler.
    pub fn select_subflow(&self) -> Option<u8> {
        let established: Vec<_> = self
            .subflows
            .iter()
            .filter(|(_, sf)| sf.state == SubflowState::Established && !sf.is_backup)
            .collect();
        match self.scheduler {
            Scheduler::LowestRtt => established
                .iter()
                .min_by_key(|(_, sf)| sf.rtt_us)
                .map(|(&id, _)| id),
            Scheduler::RoundRobin => {
                // TODO: track round-robin index
                established.first().map(|(&id, _)| id)
            }
            Scheduler::Redundant => established.first().map(|(&id, _)| id),
        }
    }

    /// Enqueue data for transmission; returns the DSS option and updated DSN.
    pub fn send(&mut self, data: &[u8]) -> Result<DssOption, MptcpError> {
        if self.state != MptcpState::Established {
            return Err(MptcpError::InvalidState);
        }
        let sf_id = self.select_subflow().ok_or(MptcpError::SubflowNotFound)?;
        let sf = self.subflows.get_mut(&sf_id).unwrap();
        let dss = DssOption {
            data_fin: false,
            dsn_long: true,
            dsn: self.snd_dsn,
            ssn: sf.snd_nxt,
            data_len: data.len() as u16,
            checksum: None,
        };
        sf.snd_nxt = sf.snd_nxt.wrapping_add(data.len() as u32);
        sf.tx_bytes += data.len() as u64;
        self.snd_dsn += data.len() as u64;
        self.tx_bytes += data.len() as u64;
        Ok(dss)
    }

    /// Process a received DSS segment; reassemble and deliver in-order data.
    pub fn receive(
        &mut self,
        dss: DssOption,
        data: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, MptcpError> {
        // Buffer out-of-order data
        self.reassembly.insert(dss.dsn, data);
        // Deliver contiguous data
        let mut delivered: Vec<u8> = Vec::new();
        loop {
            if let Some(chunk) = self.reassembly.remove(&self.rcv_dsn) {
                self.rcv_dsn += chunk.len() as u64;
                self.rx_bytes += chunk.len() as u64;
                delivered.extend_from_slice(&chunk);
            } else {
                break;
            }
        }
        if delivered.is_empty() {
            Ok(None)
        } else {
            Ok(Some(delivered))
        }
    }
}
