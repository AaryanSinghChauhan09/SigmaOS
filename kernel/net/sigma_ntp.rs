// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/net/sigma_ntp.rs — NTP/NTS Client (no_std, cleanroom)
// Language: Rust #![no_std] — OOP via NtpClient struct

#![no_std]

// ── NTP Packet (48 bytes) ─────────────────────────────────────────────────────
const NTP_PORT:    u16   = 123;
const NTP_VERSION: u8    = 4;
const NTP_CLIENT:  u8    = 3;   // mode = client
const JAN_1970:    u64   = 2_208_988_800; // NTP epoch offset from Unix

pub struct NtpPacket {
    pub data: [u8; 48],
}

impl NtpPacket {
    pub fn new_request() -> Self {
        let mut p = Self { data: [0u8; 48] };
        // LI=0, VN=4, Mode=3
        p.data[0] = (NTP_VERSION << 3) | NTP_CLIENT;
        p
    }

    pub fn transmit_timestamp_ntp(&self) -> u64 {
        u64::from_be_bytes(self.data[40..48].try_into().unwrap_or([0;8]))
    }

    pub fn receive_timestamp_ntp(&self) -> u64 {
        u64::from_be_bytes(self.data[32..40].try_into().unwrap_or([0;8]))
    }

    pub fn origin_timestamp_ntp(&self) -> u64 {
        u64::from_be_bytes(self.data[24..32].try_into().unwrap_or([0;8]))
    }

    /// NTP timestamp to Unix seconds (subtract NTP epoch 1900→1970)
    pub fn ntp_to_unix(ntp: u64) -> u64 {
        let secs = ntp >> 32;
        secs.saturating_sub(JAN_1970)
    }

    pub fn stratum(&self) -> u8 { self.data[1] }
    pub fn precision(&self) -> i8 { self.data[3] as i8 }
    pub fn root_delay_ms(&self) -> u32 {
        u32::from_be_bytes(self.data[4..8].try_into().unwrap_or([0;4])) * 1000 / 65536
    }
    pub fn mode(&self) -> u8 { self.data[0] & 0x7 }
    pub fn li(&self) -> u8 { (self.data[0] >> 6) & 0x3 }
}

// ── NTP Clock State ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NtpState { Unsynchronised, Synchronising, Synchronised, Alarmed }

#[derive(Clone, Copy, Debug, Default)]
pub struct NtpStats {
    pub offset_ms:     i64,
    pub jitter_ms:     u32,
    pub delay_ms:      u32,
    pub stratum:       u8,
    pub poll_interval: u32,
    pub sync_count:    u32,
}

// ── NTP Client ────────────────────────────────────────────────────────────────
pub struct NtpClient {
    pub server_ip:    [u8; 4],
    pub state:        NtpState,
    pub stats:        NtpStats,
    pub unix_time:    u64,     // last known Unix time (seconds)
    t1_local:         u64,     // local ticks when request sent
    t1_ntp:           u64,     // NTP transmit timestamp
    // Offset filter (8 samples, median)
    offset_buf:       [i64; 8],
    offset_idx:       usize,
}

impl NtpClient {
    pub const fn new(server: [u8; 4]) -> Self {
        Self {
            server_ip: server, state: NtpState::Unsynchronised,
            stats: NtpStats { offset_ms:0,jitter_ms:0,delay_ms:0,
                               stratum:0,poll_interval:64,sync_count:0 },
            unix_time: 0, t1_local: 0, t1_ntp: 0,
            offset_buf: [0i64;8], offset_idx: 0,
        }
    }

    /// Build outgoing NTP request, fill T1 from local_ticks
    pub fn build_request(&mut self, local_ticks: u64) -> NtpPacket {
        self.t1_local = local_ticks;
        let mut pkt = NtpPacket::new_request();
        // Set originate timestamp = 0 (first request)
        self.t1_ntp = (local_ticks + JAN_1970 * 1000) << 22; // approx
        let ts = self.t1_ntp;
        pkt.data[40..48].copy_from_slice(&ts.to_be_bytes());
        pkt
    }

    /// Process server response, return updated Unix time
    pub fn process_response(&mut self, resp: &NtpPacket, local_ticks: u64) -> u64 {
        if resp.mode() != 4 { return self.unix_time; } // not server mode
        if resp.stratum() == 0 { self.state = NtpState::Alarmed; return self.unix_time; }

        let t4 = local_ticks;
        let t3_ntp = resp.transmit_timestamp_ntp();
        let t2_ntp = resp.receive_timestamp_ntp();
        let t3_unix = NtpPacket::ntp_to_unix(t3_ntp);
        let t2_unix = NtpPacket::ntp_to_unix(t2_ntp);

        // Round-trip delay = (T4-T1) - (T3-T2)
        let rtt_ticks = t4.saturating_sub(self.t1_local);
        let server_proc = t3_unix.saturating_sub(t2_unix);
        let delay_ms = (rtt_ticks.saturating_sub(server_proc) / 2) as u32;

        // Clock offset = ((T2-T1) + (T3-T4)) / 2
        let offset_ms = ((t2_unix as i64 - self.t1_local as i64 / 1000)
            + (t3_unix as i64 - t4 as i64 / 1000)) / 2;

        // Store in ring buffer
        self.offset_buf[self.offset_idx % 8] = offset_ms;
        self.offset_idx += 1;

        self.unix_time = t3_unix.wrapping_add(delay_ms as u64 / 1000);
        self.stats.offset_ms     = offset_ms;
        self.stats.delay_ms      = delay_ms;
        self.stats.stratum        = resp.stratum();
        self.stats.sync_count    += 1;
        self.state = NtpState::Synchronised;
        self.unix_time
    }

    /// Tick local clock (called every second)
    pub fn tick_second(&mut self) { self.unix_time += 1; }

    pub fn is_sync(&self) -> bool { self.state == NtpState::Synchronised }
}
