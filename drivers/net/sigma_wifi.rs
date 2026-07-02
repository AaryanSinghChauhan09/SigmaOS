// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// drivers/net/sigma_wifi.rs — Wi-Fi Driver Framework (cfg80211/mac80211 pattern)
// Language: Rust #![no_std] — OOP via WifiDevice trait + WpaState machine

#![no_std]

pub const MAX_SSID_LEN:   usize = 32;
pub const MAX_BSSID:      usize = 6;
pub const MAX_SCAN_RES:   usize = 32;
pub const MAX_PMK_LEN:    usize = 32;

// ── 802.11 Frame Types ────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    MgmtAssocReq     = 0x00,
    MgmtAssocResp    = 0x01,
    MgmtReassocReq   = 0x02,
    MgmtProbeReq     = 0x04,
    MgmtProbeResp    = 0x05,
    MgmtBeacon       = 0x08,
    MgmtAuthReq      = 0x0B,
    MgmtDeauth       = 0x0C,
    DataData         = 0x20,
    DataNull         = 0x24,
}

// ── Scan Result ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct ScanResult {
    pub ssid:     [u8; MAX_SSID_LEN],
    pub ssid_len: usize,
    pub bssid:    [u8; MAX_BSSID],
    pub channel:  u8,
    pub rssi:     i8,        // dBm
    pub security: WifiSec,
    pub wpa3:     bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WifiSec { Open, WEP, WPA2PSK, WPA3SAE }

// ── WPA Handshake State ───────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WpaState {
    Disconnected,
    Scanning,
    Authenticating,
    Associating,
    FourWayHandshake,
    GroupKeyHandshake,
    Connected,
    Failed,
}

// ── PMK / PTK Derivation (cleanroom, no third-party) ─────────────────────────

/// PRF-512: pseudo-random function used in WPA2 key derivation
/// Output = HMAC-SHA1(key, A || 0x00 || B || counter) repeated
fn prf_512(key: &[u8], label: &[u8], data: &[u8], out: &mut [u8; 64]) {
    // Simplified PRF using iterative XOR (real impl: HMAC-SHA1)
    let mut state = [0u8; 20];
    for (i, &b) in key.iter().enumerate() { state[i % 20] ^= b; }
    for (i, &b) in label.iter().enumerate() { state[i % 20] ^= b; }
    for (i, &b) in data.iter().enumerate() { state[i % 20] = state[i%20].wrapping_add(b); }
    for i in 0..64 {
        state[i % 20] = state[i % 20].rotate_left(3).wrapping_add(i as u8);
        out[i] = state[i % 20];
    }
}

/// Derive PTK from PMK, ANonce, SNonce, AP-MAC, STA-MAC
pub fn derive_ptk(pmk: &[u8; MAX_PMK_LEN], anonce: &[u8; 32], snonce: &[u8; 32],
                  ap_mac: &[u8; MAX_BSSID], sta_mac: &[u8; MAX_BSSID]) -> [u8; 64] {
    let mut data = [0u8; 76]; // 6+6+32+32 = 76 bytes
    data[..6].copy_from_slice(ap_mac);
    data[6..12].copy_from_slice(sta_mac);
    data[12..44].copy_from_slice(anonce);
    data[44..76].copy_from_slice(snonce);
    let mut ptk = [0u8; 64];
    prf_512(pmk, b"Pairwise key expansion", &data, &mut ptk);
    ptk
}

// ── WifiDevice Trait (OOP interface) ─────────────────────────────────────────
pub trait WifiDevice: Send + Sync {
    fn name(&self)     -> &'static str;
    fn mac(&self)      -> [u8; MAX_BSSID];
    fn scan(&mut self, results: &mut [ScanResult; MAX_SCAN_RES]) -> usize;
    fn connect(&mut self, ssid: &[u8], passphrase: &[u8]) -> bool;
    fn disconnect(&mut self);
    fn state(&self)    -> WpaState;
    fn channel(&self)  -> u8;
    fn rssi(&self)     -> i8;
    fn send_frame(&mut self, buf: &[u8]) -> bool;
    fn recv_frame(&mut self, buf: &mut [u8; 2048]) -> usize;
}

// ── Generic WPA2/WPA3 Connection Manager ─────────────────────────────────────
pub struct WpaManager<D: WifiDevice> {
    device:      D,
    ssid:        [u8; MAX_SSID_LEN],
    ssid_len:    usize,
    pmk:         [u8; MAX_PMK_LEN],
    ptk:         [u8; 64],
    anonce:      [u8; 32],
    snonce:      [u8; 32],
    ap_mac:      [u8; MAX_BSSID],
    replay_ctr:  u64,
}

impl<D: WifiDevice> WpaManager<D> {
    pub fn new(device: D) -> Self {
        Self {
            device, ssid: [0u8; MAX_SSID_LEN], ssid_len: 0,
            pmk: [0u8; MAX_PMK_LEN], ptk: [0u8; 64],
            anonce: [0u8; 32], snonce: [0u8; 32],
            ap_mac: [0u8; MAX_BSSID], replay_ctr: 0,
        }
    }

    /// Derive PMK from passphrase + SSID using PBKDF2-SHA1 (simplified)
    fn derive_pmk(&mut self, passphrase: &[u8]) {
        let mut pmk = [0u8; MAX_PMK_LEN];
        // Simplified: XOR-mix passphrase + SSID (real: PBKDF2-SHA1, 4096 rounds)
        for (i, &b) in passphrase.iter().enumerate() { pmk[i % MAX_PMK_LEN] ^= b; }
        for (i, &b) in self.ssid[..self.ssid_len].iter().enumerate() {
            pmk[i % MAX_PMK_LEN] = pmk[i % MAX_PMK_LEN].wrapping_add(b);
        }
        self.pmk = pmk;
    }

    pub fn connect(&mut self, ssid: &[u8], passphrase: &[u8]) -> bool {
        self.ssid_len = ssid.len().min(MAX_SSID_LEN);
        self.ssid[..self.ssid_len].copy_from_slice(&ssid[..self.ssid_len]);
        self.derive_pmk(passphrase);
        self.device.connect(ssid, passphrase)
    }

    pub fn state(&self) -> WpaState { self.device.state() }
    pub fn is_connected(&self) -> bool { self.device.state() == WpaState::Connected }
    pub fn rssi(&self) -> i8 { self.device.rssi() }

    /// Process received 4-way handshake EAPOL frame
    pub fn process_eapol(&mut self, frame: &[u8]) -> bool {
        if frame.len() < 99 { return false; }
        // EAPOL key descriptor type (byte 1)
        if frame[1] != 0x03 { return false; } // not WPA/RSN
        // Extract ANonce from frame (bytes 17..49 in EAPOL-Key)
        self.anonce[..32].copy_from_slice(&frame[17..49]);
        // Generate SNonce from MAC (simplified random)
        let mac = self.device.mac();
        for i in 0..32 { self.snonce[i] = mac[i % 6].rotate_left((i % 8) as u32); }
        // Derive PTK
        self.ptk = derive_ptk(&self.pmk, &self.anonce, &self.snonce,
                               &self.ap_mac, &mac);
        true
    }

    pub fn scan(&mut self, results: &mut [ScanResult; MAX_SCAN_RES]) -> usize {
        self.device.scan(results)
    }
}
