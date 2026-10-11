// SigmaOS Systemd-timesyncd SNTP Client & Clock Synchronization Subsystem
// Reference: systemd-timesyncd, RFC 4330 (SNTPv4), RFC 5905 (NTPv4)

#![allow(dead_code)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const NTP_PORT: u16 = 123;
pub const NTP_VERSION: u8 = 4;
pub const NTP_MODE_CLIENT: u8 = 3;
pub const NTP_MODE_SERVER: u8 = 4;

/// Standard 48-byte NTP Packet Header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NtpPacket {
    pub li_vn_mode: u8, // Leap Indicator (2-bit), Version (3-bit), Mode (3-bit)
    pub stratum: u8,
    pub poll: i8,
    pub precision: i8,
    pub root_delay: u32,
    pub root_dispersion: u32,
    pub ref_id: u32,
    pub ref_timestamp: u64,
    pub origin_timestamp: u64,
    pub recv_timestamp: u64,
    pub tx_timestamp: u64,
}

impl NtpPacket {
    pub fn new_client_request() -> Self {
        Self {
            li_vn_mode: (0 << 6) | (NTP_VERSION << 3) | NTP_MODE_CLIENT,
            stratum: 0,
            poll: 4,
            precision: -6,
            root_delay: 0,
            root_dispersion: 0,
            ref_id: 0,
            ref_timestamp: 0,
            origin_timestamp: 0,
            recv_timestamp: 0,
            tx_timestamp: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8; 48] {
        let mut buf = [0u8; 48];
        buf[0] = self.li_vn_mode;
        buf[1] = self.stratum;
        buf[2] = self.poll as u8;
        buf[3] = self.precision as u8;
        buf[4..8].copy_from_slice(&self.root_delay.to_be_bytes());
        buf[8..12].copy_from_slice(&self.root_dispersion.to_be_bytes());
        buf[12..16].copy_from_slice(&self.ref_id.to_be_bytes());
        buf[16..24].copy_from_slice(&self.ref_timestamp.to_be_bytes());
        buf[24..32].copy_from_slice(&self.origin_timestamp.to_be_bytes());
        buf[32..40].copy_from_slice(&self.recv_timestamp.to_be_bytes());
        buf[40..48].copy_from_slice(&self.tx_timestamp.to_be_bytes());
        buf
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 48 {
            return Err("NTP packet too short (expected 48 bytes)");
        }
        Ok(Self {
            li_vn_mode: bytes[0],
            stratum: bytes[1],
            poll: bytes[2] as i8,
            precision: bytes[3] as i8,
            root_delay: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            root_dispersion: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            ref_id: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
            ref_timestamp: u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
            origin_timestamp: u64::from_be_bytes(bytes[24..32].try_into().unwrap()),
            recv_timestamp: u64::from_be_bytes(bytes[32..40].try_into().unwrap()),
            tx_timestamp: u64::from_be_bytes(bytes[40..48].try_into().unwrap()),
        })
    }
}

/// Timesyncd Clock Sync Engine
pub struct TimesyncdEngine {
    servers: Vec<String>,
    current_server_idx: usize,
    synchronized: bool,
    last_offset_ms: i64,
    poll_interval_sec: u32,
}

impl TimesyncdEngine {
    pub fn new() -> Self {
        Self {
            servers: alloc::vec![
                String::from("time.cloudflare.com"),
                String::from("pool.ntp.org"),
                String::from("time.google.com")
            ],
            current_server_idx: 0,
            synchronized: false,
            last_offset_ms: 0,
            poll_interval_sec: 64,
        }
    }

    pub fn add_server(&mut self, server: &str) {
        self.servers.push(String::from(server));
    }

    /// Calculate clock offset given send, receive, and transmit timestamps (in ms)
    pub fn calculate_offset(&mut self, t1_client_tx: i64, t2_server_rx: i64, t3_server_tx: i64, t4_client_rx: i64) -> i64 {
        // Offset theta = ((t2 - t1) + (t3 - t4)) / 2
        let offset = ((t2_server_rx - t1_client_tx) + (t3_server_tx - t4_client_rx)) / 2;
        self.last_offset_ms = offset;
        self.synchronized = true;
        offset
    }

    pub fn is_synchronized(&self) -> bool {
        self.synchronized
    }

    pub fn last_offset_ms(&self) -> i64 {
        self.last_offset_ms
    }
}
