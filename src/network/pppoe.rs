#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! # PPPoE — Point-to-Point Protocol over Ethernet (RFC 2516)
//!
//! Implements the PPPoE client for SigmaOS DSL/broadband connectivity.
//!
//! ## Discovery phase (EtherType 0x8863)
//! ```text
//! Client → broadcast : PADI  (PPPoE Active Discovery Initiation)
//! Server → client    : PADO  (PPPoE Active Discovery Offer)
//! Client → server    : PADR  (PPPoE Active Discovery Request)
//! Server → client    : PADS  (PPPoE Active Discovery Session-confirmation)
//! Either → either    : PADT  (PPPoE Active Discovery Terminate)
//! ```
//!
//! ## Session phase (EtherType 0x8864)
//! Once a Session ID is assigned (PADS), PPP frames are encapsulated directly
//! in PPPoE session frames.  LCP negotiates the link; NCP (IPCP/IPv6CP)
//! negotiates network-layer parameters.
//!
//! ## PPPoE Header (6 bytes)
//! ```text
//! Ver(4b) | Type(4b) | Code(8b) | Session-ID(16b) | Length(16b)
//! ```

use std::string::{String, ToString};
use std::vec::Vec;

// ─── Constants ────────────────────────────────────────────────────────────────

/// EtherType for PPPoE Discovery.
pub const PPPOE_ETH_DISC: u16 = 0x8863;

/// EtherType for PPPoE Session.
pub const PPPOE_ETH_SESS: u16 = 0x8864;

/// PPPoE version/type field (both 1 → byte 0x11).
pub const PPPOE_VER_TYPE: u8 = 0x11;

/// PADI code.
pub const PPPOE_CODE_PADI: u8 = 0x09;

/// PADO code.
pub const PPPOE_CODE_PADO: u8 = 0x07;

/// PADR code.
pub const PPPOE_CODE_PADR: u8 = 0x19;

/// PADS code.
pub const PPPOE_CODE_PADS: u8 = 0x65;

/// PADT code.
pub const PPPOE_CODE_PADT: u8 = 0xa7;

/// PPPoE session data code (0x00 = data).
pub const PPPOE_CODE_DATA: u8 = 0x00;

/// Broadcast MAC address.
pub const MAC_BROADCAST: [u8; 6] = [0xFF; 6];

/// PPP LCP protocol number.
pub const PPP_PROTO_LCP: u16 = 0xC021;

/// PPP IPCP protocol number.
pub const PPP_PROTO_IPCP: u16 = 0x8021;

/// PPP IPv6CP protocol number.
pub const PPP_PROTO_IPV6CP: u16 = 0x8057;

/// PPP IP protocol number.
pub const PPP_PROTO_IP: u16 = 0x0021;

// ─── Tag Types ────────────────────────────────────────────────────────────────

/// PPPoE TAG type values (RFC 2516 §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum TagType {
    EndOfList = 0x0000,
    ServiceName = 0x0101,
    AcName = 0x0102,
    HostUniq = 0x0103,
    AcCookie = 0x0104,
    VendorSpec = 0x0105,
    RelaySessionId = 0x0110,
    ServiceNameError = 0x0201,
    AcSystemError = 0x0202,
    GenericError = 0x0203,
}

// ─── Discovery FSM states ─────────────────────────────────────────────────────

/// PPPoE client FSM states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PppoeState {
    /// No connection.
    Idle,
    /// PADI sent; waiting for PADO.
    Initiating,
    /// PADO received; PADR sent; waiting for PADS.
    Requesting,
    /// Session established; PPP LCP/NCP in progress.
    SessionUp,
    /// PADT sent/received; tearing down.
    Terminating,
}

/// LCP negotiation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcpState {
    Initial,
    Starting,
    Closed,
    Stopped,
    Closing,
    Stopping,
    RequestSent,
    AckReceived,
    AckSent,
    Opened,
}

/// PPPoE errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PppoeError {
    InvalidPacket,
    WrongState,
    SessionIdMismatch,
    LcpNegotiationFailed,
    Timeout,
    IoError,
}

// ─── Structs ──────────────────────────────────────────────────────────────────

/// A PPPoE TAG TLV.
#[derive(Debug, Clone)]
pub struct PppoeTag {
    pub tag_type: u16,
    pub value: Vec<u8>,
}

impl PppoeTag {
    pub fn new(tag_type: TagType, value: Vec<u8>) -> Self {
        PppoeTag {
            tag_type: tag_type as u16,
            value,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(4 + self.value.len());
        b.extend_from_slice(&self.tag_type.to_be_bytes());
        b.extend_from_slice(&(self.value.len() as u16).to_be_bytes());
        b.extend_from_slice(&self.value);
        b
    }
}

/// A decoded PPPoE discovery/session header.
#[derive(Debug, Clone)]
pub struct PppoeHeader {
    /// Combined ver/type byte (0x11).
    pub ver_type: u8,
    /// Packet code (PADI, PADO, etc.).
    pub code: u8,
    /// Session ID (0 for discovery, assigned by server on PADS).
    pub session_id: u16,
    /// Length of the payload in bytes.
    pub payload_len: u16,
}

impl PppoeHeader {
    pub fn new(code: u8, session_id: u16, payload_len: u16) -> Self {
        PppoeHeader {
            ver_type: PPPOE_VER_TYPE,
            code,
            session_id,
            payload_len,
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        let sid = self.session_id.to_be_bytes();
        let len = self.payload_len.to_be_bytes();
        [self.ver_type, self.code, sid[0], sid[1], len[0], len[1]]
    }

    pub fn from_bytes(raw: &[u8]) -> Result<Self, PppoeError> {
        if raw.len() < 6 {
            return Err(PppoeError::InvalidPacket);
        }
        Ok(PppoeHeader {
            ver_type: raw[0],
            code: raw[1],
            session_id: u16::from_be_bytes([raw[2], raw[3]]),
            payload_len: u16::from_be_bytes([raw[4], raw[5]]),
        })
    }
}

/// PPPoE client session context.
pub struct PppoeClient {
    /// Local Ethernet MAC address.
    pub local_mac: [u8; 6],
    /// Server (AC) MAC address (learned from PADO).
    pub server_mac: [u8; 6],
    /// Assigned session ID (from PADS).
    pub session_id: u16,
    /// FSM state.
    pub state: PppoeState,
    /// LCP state.
    pub lcp_state: LcpState,
    /// Service name to request (empty = any).
    pub service_name: String,
    /// Host-Uniq nonce for correlating replies.
    pub host_uniq: [u8; 8],
    /// AC cookie (returned in PADO, echoed in PADR).
    pub ac_cookie: Vec<u8>,
    /// Negotiated MRU.
    pub mru: u16,
    /// Assigned local IP (from IPCP).
    pub local_ip: Option<u32>,
    /// Assigned remote (gateway) IP (from IPCP).
    pub remote_ip: Option<u32>,
    /// Assigned DNS server (from IPCP option 129).
    pub dns_server: Option<u32>,
    pub tx_packets: u64,
    pub rx_packets: u64,
}

impl PppoeClient {
    /// Create a new PPPoE client.
    pub fn new(local_mac: [u8; 6], service_name: &str) -> Self {
        PppoeClient {
            local_mac,
            server_mac: [0u8; 6],
            session_id: 0,
            state: PppoeState::Idle,
            lcp_state: LcpState::Initial,
            service_name: service_name.to_string(),
            host_uniq: [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE],
            ac_cookie: Vec::new(),
            mru: 1492,
            local_ip: None,
            remote_ip: None,
            dns_server: None,
            tx_packets: 0,
            rx_packets: 0,
        }
    }

    /// Build a PADI packet (broadcast to find access concentrators).
    pub fn build_padi(&self) -> Vec<u8> {
        let svc_tag = PppoeTag::new(TagType::ServiceName, self.service_name.as_bytes().to_vec());
        let uniq_tag = PppoeTag::new(TagType::HostUniq, self.host_uniq.to_vec());
        let mut payload = svc_tag.to_bytes();
        payload.extend_from_slice(&uniq_tag.to_bytes());
        let hdr = PppoeHeader::new(PPPOE_CODE_PADI, 0, payload.len() as u16);
        let mut pkt = hdr.to_bytes().to_vec();
        pkt.extend_from_slice(&payload);
        pkt
    }

    /// Process a received PADO; extract AC info and AC cookie.
    pub fn process_pado(&mut self, src_mac: [u8; 6], payload: &[u8]) -> Result<(), PppoeError> {
        if self.state != PppoeState::Initiating {
            return Err(PppoeError::WrongState);
        }
        self.server_mac = src_mac;
        // TODO: parse tags to extract AcName, AcCookie
        self.state = PppoeState::Requesting;
        Ok(())
    }

    /// Build a PADR packet directed at the selected AC.
    pub fn build_padr(&self) -> Vec<u8> {
        let svc_tag = PppoeTag::new(TagType::ServiceName, self.service_name.as_bytes().to_vec());
        let mut payload = svc_tag.to_bytes();
        if !self.ac_cookie.is_empty() {
            let cookie = PppoeTag::new(TagType::AcCookie, self.ac_cookie.clone());
            payload.extend_from_slice(&cookie.to_bytes());
        }
        let uniq_tag = PppoeTag::new(TagType::HostUniq, self.host_uniq.to_vec());
        payload.extend_from_slice(&uniq_tag.to_bytes());
        let hdr = PppoeHeader::new(PPPOE_CODE_PADR, 0, payload.len() as u16);
        let mut pkt = hdr.to_bytes().to_vec();
        pkt.extend_from_slice(&payload);
        pkt
    }

    /// Process a received PADS; record session ID and transition to SessionUp.
    pub fn process_pads(&mut self, raw: &[u8]) -> Result<(), PppoeError> {
        if self.state != PppoeState::Requesting {
            return Err(PppoeError::WrongState);
        }
        let hdr = PppoeHeader::from_bytes(raw)?;
        if hdr.session_id == 0 {
            return Err(PppoeError::InvalidPacket);
        }
        self.session_id = hdr.session_id;
        self.state = PppoeState::SessionUp;
        self.lcp_state = LcpState::Starting;
        Ok(())
    }

    /// Build and send a PADT (terminate the session).
    pub fn terminate(&mut self) -> Vec<u8> {
        self.state = PppoeState::Terminating;
        let hdr = PppoeHeader::new(PPPOE_CODE_PADT, self.session_id, 0);
        hdr.to_bytes().to_vec()
    }

    /// Encapsulate a PPP frame in a PPPoE session packet.
    pub fn encapsulate(
        &mut self,
        ppp_proto: u16,
        ppp_payload: &[u8],
    ) -> Result<Vec<u8>, PppoeError> {
        if self.state != PppoeState::SessionUp {
            return Err(PppoeError::WrongState);
        }
        // PPP frame: [proto 2B][payload]
        let mut ppp = Vec::with_capacity(2 + ppp_payload.len());
        ppp.extend_from_slice(&ppp_proto.to_be_bytes());
        ppp.extend_from_slice(ppp_payload);
        let hdr = PppoeHeader::new(PPPOE_CODE_DATA, self.session_id, ppp.len() as u16);
        let mut pkt = hdr.to_bytes().to_vec();
        pkt.extend_from_slice(&ppp);
        self.tx_packets += 1;
        Ok(pkt)
    }

    /// Decapsulate a PPPoE session frame; returns (ppp_proto, ppp_payload).
    pub fn decapsulate<'a>(&mut self, raw: &'a [u8]) -> Result<(u16, &'a [u8]), PppoeError> {
        let hdr = PppoeHeader::from_bytes(raw)?;
        if hdr.session_id != self.session_id {
            return Err(PppoeError::SessionIdMismatch);
        }
        if raw.len() < 6 + 2 {
            return Err(PppoeError::InvalidPacket);
        }
        let ppp_proto = u16::from_be_bytes([raw[6], raw[7]]);
        self.rx_packets += 1;
        Ok((ppp_proto, &raw[8..]))
    }

    /// Start the discovery phase (sets state to Initiating).
    pub fn start_discovery(&mut self) -> Result<Vec<u8>, PppoeError> {
        if self.state != PppoeState::Idle {
            return Err(PppoeError::WrongState);
        }
        self.state = PppoeState::Initiating;
        Ok(self.build_padi())
    }
}
