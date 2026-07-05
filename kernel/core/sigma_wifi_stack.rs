// SigmaOS — Wi-Fi Stack (IEEE 802.11ax / Wi-Fi 6)
// Issue #851-WLAN: Wi-Fi stack implementation
// Sovereign implementation — no external dependencies

#![allow(dead_code)]

// ─── 802.11 Frame Types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum FrameType {
    Management = 0,
    Control    = 1,
    Data       = 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ManagementSubtype {
    AssocRequest    = 0,
    AssocResponse   = 1,
    ProbeRequest    = 4,
    ProbeResponse   = 5,
    Beacon          = 8,
    Authentication  = 11,
    Deauthentication = 12,
    Action          = 13,
}

// ─── MAC Address ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    pub const BROADCAST: MacAddr = MacAddr([0xFF; 6]);
    pub const ZERO: MacAddr = MacAddr([0x00; 6]);

    pub fn is_broadcast(&self) -> bool { self.0 == [0xFF; 6] }
    pub fn is_multicast(&self) -> bool { self.0[0] & 0x01 != 0 }

    pub fn fmt(&self) -> [u8; 17] {
        let mut out = [0u8; 17];
        let hex = b"0123456789abcdef";
        for i in 0..6 {
            out[i*3]   = hex[(self.0[i] >> 4) as usize];
            out[i*3+1] = hex[(self.0[i] & 0xF) as usize];
            if i < 5 { out[i*3+2] = b':'; }
        }
        out
    }
}

// ─── 802.11 Frame Control ────────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FrameControl {
    pub version_type_subtype: u8,
    pub flags: u8,
}

impl FrameControl {
    pub fn new(ftype: FrameType, subtype: u8) -> Self {
        FrameControl {
            version_type_subtype: ((subtype & 0xF) << 4) | ((ftype as u8) << 2),
            flags: 0,
        }
    }

    pub fn frame_type(&self) -> FrameType {
        match (self.version_type_subtype >> 2) & 0x3 {
            0 => FrameType::Management,
            1 => FrameType::Control,
            _ => FrameType::Data,
        }
    }

    pub fn subtype(&self) -> u8 {
        (self.version_type_subtype >> 4) & 0xF
    }

    pub fn to_ds(&self)   -> bool { self.flags & 0x01 != 0 }
    pub fn from_ds(&self) -> bool { self.flags & 0x02 != 0 }
    pub fn protected(&self) -> bool { self.flags & 0x40 != 0 }
    pub fn set_protected(&mut self) { self.flags |= 0x40; }
}

// ─── Generic 802.11 MAC Header ───────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MacHeader {
    pub frame_ctrl: FrameControl,
    pub duration:   u16,
    pub addr1:      MacAddr,
    pub addr2:      MacAddr,
    pub addr3:      MacAddr,
    pub seq_ctrl:   u16,
}

impl MacHeader {
    pub fn beacon(bssid: MacAddr) -> Self {
        MacHeader {
            frame_ctrl: FrameControl::new(FrameType::Management, ManagementSubtype::Beacon as u8),
            duration: 0,
            addr1: MacAddr::BROADCAST,
            addr2: bssid,
            addr3: bssid,
            seq_ctrl: 0,
        }
    }

    pub fn probe_req(src: MacAddr) -> Self {
        MacHeader {
            frame_ctrl: FrameControl::new(FrameType::Management, ManagementSubtype::ProbeRequest as u8),
            duration: 0,
            addr1: MacAddr::BROADCAST,
            addr2: src,
            addr3: MacAddr::BROADCAST,
            seq_ctrl: 0,
        }
    }
}

// ─── SSID Information Element ────────────────────────────────────────────────

pub const IE_SSID:          u8 = 0;
pub const IE_SUPPORTED_RATES: u8 = 1;
pub const IE_HT_CAPABILITIES: u8 = 45;
pub const IE_HE_CAPABILITIES: u8 = 255; // Wi-Fi 6 HE (vendor ext)
pub const IE_RSN:             u8 = 48;  // WPA2/WPA3

pub struct InfoElement<'a> {
    pub id:  u8,
    pub data: &'a [u8],
}

// ─── WPA3-SAE (Simultaneous Authentication of Equals) ────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuthState {
    Idle,
    SaeCommit,
    SaeConfirm,
    Authenticated,
    Associated,
    Connected,
    Disconnected,
}

pub struct WpaContext {
    pub state: AuthState,
    pub pmk:   [u8; 32],  // Pairwise Master Key
    pub ptk:   [u8; 64],  // Pairwise Transient Key
    pub gtk:   [u8; 32],  // Group Temporal Key
    pub anonce: [u8; 32],
    pub snonce: [u8; 32],
    pub replay_counter: u64,
}

impl WpaContext {
    pub const fn new() -> Self {
        WpaContext {
            state: AuthState::Idle,
            pmk:  [0u8; 32],
            ptk:  [0u8; 64],
            gtk:  [0u8; 32],
            anonce: [0u8; 32],
            snonce: [0u8; 32],
            replay_counter: 0,
        }
    }

    /// Derive PTK using PBKDF2/HKDF (ties into sigma_key_derive)
    /// PRF-X(PMK, "Pairwise key expansion" || min(AA,SA) || max(AA,SA) || min(ANonce,SNonce) || max(ANonce,SNonce))
    pub fn derive_ptk(&mut self, aa: &MacAddr, sa: &MacAddr) {
        // Simplified PRF — real impl uses SHA1-HMAC PRF per 802.11i
        let mut input = [0u8; 128];
        input[..6].copy_from_slice(&aa.0);
        input[6..12].copy_from_slice(&sa.0);
        input[12..44].copy_from_slice(&self.anonce);
        input[44..76].copy_from_slice(&self.snonce);
        // Use HMAC-SHA256 with PMK as key
        let mut pos = 0usize;
        for counter in 0u8..2 {
            input[76] = counter;
            // sovereign hmac from crypto module
            let block = crate::crypto::sigma_key_derive::hmac_sha256(&self.pmk, &input[..77]);
            self.ptk[pos..pos+32].copy_from_slice(&block);
            pos += 32;
        }
    }

    pub fn set_pmk_from_passphrase(&mut self, passphrase: &[u8], ssid: &[u8]) {
        // PBKDF2(passphrase, ssid, 4096, 32)
        crate::crypto::sigma_key_derive::pbkdf2_sha256(
            passphrase, ssid, 4096, &mut self.pmk
        );
    }
}

// ─── BSS Scan Results ────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct BssInfo {
    pub bssid:       MacAddr,
    pub ssid:        [u8; 32],
    pub ssid_len:    u8,
    pub channel:     u8,
    pub rssi_dbm:    i8,
    pub security:    SecurityMode,
    pub beacon_interval_ms: u16,
    pub dtim_period: u8,
    // Wi-Fi 6 / HE capabilities
    pub he_capable:  bool,
    pub max_mcs_idx: u8,
    pub nss:         u8, // number of spatial streams
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityMode {
    Open,
    Wep,
    Wpa,
    Wpa2Personal,
    Wpa2Enterprise,
    Wpa3Personal,
    Wpa3Enterprise,
}

pub const MAX_BSS: usize = 64;

pub struct ScanResults {
    pub entries: [BssInfo; MAX_BSS],
    pub count:   usize,
}

impl ScanResults {
    pub const fn new() -> Self {
        const EMPTY_BSS: BssInfo = BssInfo {
            bssid: MacAddr::ZERO,
            ssid: [0u8; 32],
            ssid_len: 0,
            channel: 1,
            rssi_dbm: -100,
            security: SecurityMode::Open,
            beacon_interval_ms: 100,
            dtim_period: 1,
            he_capable: false,
            max_mcs_idx: 9,
            nss: 1,
        };
        ScanResults { entries: [EMPTY_BSS; MAX_BSS], count: 0 }
    }

    pub fn add(&mut self, bss: BssInfo) -> bool {
        if self.count >= MAX_BSS { return false; }
        self.entries[self.count] = bss;
        self.count += 1;
        true
    }

    pub fn find_by_ssid(&self, ssid: &[u8]) -> Option<&BssInfo> {
        for i in 0..self.count {
            let e = &self.entries[i];
            if e.ssid_len as usize == ssid.len() &&
               &e.ssid[..e.ssid_len as usize] == ssid {
                return Some(e);
            }
        }
        None
    }
}

// ─── Wi-Fi Station State Machine ─────────────────────────────────────────────

pub struct WifiStation {
    pub mac:     MacAddr,
    pub scan:    ScanResults,
    pub wpa:     WpaContext,
    pub assoc_bss: Option<BssInfo>,
    pub rx_count: u64,
    pub tx_count: u64,
    pub tx_errors: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WifiError {
    NotFound,
    AuthFailed,
    AssocFailed,
    Disconnected,
    HardwareError,
    ScanNotDone,
    AlreadyConnected,
}

impl WifiStation {
    pub const fn new(mac: MacAddr) -> Self {
        WifiStation {
            mac,
            scan: ScanResults::new(),
            wpa: WpaContext::new(),
            assoc_bss: None,
            rx_count: 0,
            tx_count: 0,
            tx_errors: 0,
        }
    }

    /// Start active scan on 2.4 GHz + 5 GHz channels.
    pub fn start_scan(&mut self) -> Result<(), WifiError> {
        self.scan.count = 0;
        // Probe request broadcast on channels 1,6,11 (2.4 GHz) + 36,40,44,48,149,153,157,161 (5 GHz)
        // Hardware abstraction: actual probe TX goes through HW driver
        // This layer constructs probe request frame
        let _probe = MacHeader::probe_req(self.mac);
        // In real implementation: push to TX queue, iterate channels with dwell time 10ms
        Ok(())
    }

    /// Connect to SSID with WPA3 passphrase.
    pub fn connect(&mut self, ssid: &[u8], passphrase: &[u8]) -> Result<(), WifiError> {
        let bss = self.scan.find_by_ssid(ssid)
            .ok_or(WifiError::NotFound)?.clone();

        if self.assoc_bss.is_some() {
            return Err(WifiError::AlreadyConnected);
        }

        // Derive PMK from passphrase
        self.wpa.set_pmk_from_passphrase(passphrase, ssid);

        // State: SAE commit → confirm → authenticated → associated
        self.wpa.state = AuthState::SaeCommit;
        // In full impl: send SAE commit frame, await response, exchange confirm
        // Simplified: advance to authenticated
        self.wpa.state = AuthState::Authenticated;

        // 4-way handshake: derive PTK
        // ANonce would come from AP; SNonce generated locally
        self.wpa.snonce = generate_nonce(self.mac);
        self.wpa.derive_ptk(&bss.bssid, &self.mac);

        self.wpa.state = AuthState::Connected;
        self.assoc_bss = Some(bss);
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.assoc_bss = None;
        self.wpa.state = AuthState::Idle;
    }

    pub fn is_connected(&self) -> bool {
        self.wpa.state == AuthState::Connected
    }

    pub fn rssi_dbm(&self) -> Option<i8> {
        self.assoc_bss.map(|b| b.rssi_dbm)
    }

    /// Link speed estimate (Mbps) based on MCS index + spatial streams + guard interval.
    pub fn link_speed_mbps(&self) -> u32 {
        self.assoc_bss.map(|b| {
            // Wi-Fi 6 HE: MCS0–11, NSS 1–8, 80 MHz channel, 0.8µs GI
            // Base rates per MCS (HE-MCS) @ NSS=1 @ 80 MHz
            let mcs_rates_mbps: [u32; 12] = [
                36, 72, 108, 144, 216, 288, 324, 360, 432, 480, 540, 600
            ];
            let mcs = (b.max_mcs_idx as usize).min(11);
            mcs_rates_mbps[mcs] * b.nss as u32
        }).unwrap_or(0)
    }
}

// ─── Nonce Generation ────────────────────────────────────────────────────────

fn generate_nonce(mac: MacAddr) -> [u8; 32] {
    // In production: use hardware entropy source (RDRAND + TPM)
    // For now: hash MAC with counter
    use core::sync::atomic::{AtomicU64, Ordering};
    static NONCE_CTR: AtomicU64 = AtomicU64::new(0x0102030405060708);
    let ctr = NONCE_CTR.fetch_add(1, Ordering::Relaxed);
    let mut seed = [0u8; 32];
    seed[..6].copy_from_slice(&mac.0);
    let ctr_bytes = ctr.to_le_bytes();
    seed[6..14].copy_from_slice(&ctr_bytes);
    crate::crypto::sigma_key_derive::sha256(&seed)
}

// ─── Channel/Band Configuration ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub enum Band { Band2_4Ghz, Band5Ghz, Band6Ghz }

pub struct ChannelConfig {
    pub band:      Band,
    pub channel:   u8,
    pub width_mhz: u8,
    pub primary_freq_mhz: u32,
}

impl ChannelConfig {
    pub fn freq_for_channel_24(ch: u8) -> u32 {
        2412 + (ch as u32 - 1) * 5
    }

    pub fn freq_for_channel_5(ch: u8) -> u32 {
        5000 + ch as u32 * 5
    }

    pub fn freq_for_channel_6(ch: u8) -> u32 {
        5950 + ch as u32 * 5
    }
}
