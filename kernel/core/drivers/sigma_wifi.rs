// SPDX-License-Identifier: MIT
// SigmaOS Wi-Fi 802.11ax (Wi-Fi 6) Driver — sigma_wifi.rs
// Software MAC layer stub with full frame structures, scanning, association
// state machine, WPA3-SAE authentication skeleton, and management frame handling.
//
// Hardware HAL: plugs into either iwlwifi-style PCIe or rtl8xxxu-style USB transport.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── 802.11 Frame Types ────────────────────────────────────────────────────────
pub const FRAME_TYPE_MGMT:  u8 = 0x00;
pub const FRAME_TYPE_CTRL:  u8 = 0x01;
pub const FRAME_TYPE_DATA:  u8 = 0x02;

// Management subtypes
pub const MGMT_ASSOC_REQ:   u8 = 0x00;
pub const MGMT_ASSOC_RESP:  u8 = 0x01;
pub const MGMT_REASSOC_REQ: u8 = 0x02;
pub const MGMT_PROBE_REQ:   u8 = 0x04;
pub const MGMT_PROBE_RESP:  u8 = 0x05;
pub const MGMT_BEACON:      u8 = 0x08;
pub const MGMT_AUTH:        u8 = 0x0B;
pub const MGMT_DEAUTH:      u8 = 0x0C;
pub const MGMT_ACTION:      u8 = 0x0D;

// ── 802.11ax Capabilities ────────────────────────────────────────────────────
pub const HE_CAP_MCS_SINGLE:  u8 = 0x01; // Single-user MCS
pub const HE_CAP_OFDMA:       u8 = 0x02; // Orthogonal Frequency Division MA
pub const HE_CAP_TWT:         u8 = 0x04; // Target Wake Time (power saving)
pub const HE_CAP_BSS_COLOR:   u8 = 0x08; // BSS Coloring for spatial reuse

// ── Authentication Algorithms ─────────────────────────────────────────────────
pub const AUTH_OPEN:    u16 = 0;
pub const AUTH_SAE:     u16 = 3; // WPA3-SAE (Simultaneous Authentication of Equals)

// ── Channel Bands ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Band {
    Band2_4GHz,
    Band5GHz,
    Band6GHz,
}

// ── Security Mode ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Security {
    Open,
    WEP,
    WPA2PSK,
    WPA3SAE,
    WPA3Enterprise,
}

// ── Connection State Machine ──────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WiFiState {
    Uninitialized,
    Idle,
    Scanning,
    Authenticating,
    Associating,
    Associated,
    Disconnecting,
    Error(u16),
}

// ── MAC Address ───────────────────────────────────────────────────────────────
pub type MacAddr = [u8; 6];
pub const BCAST_ADDR: MacAddr = [0xFF; 6];
pub const ZERO_ADDR:  MacAddr = [0x00; 6];

// ── BSS (Access Point) Descriptor ────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
pub struct BssEntry {
    pub bssid:     MacAddr,
    pub ssid:      [u8; 33],    // Max SSID length 32 + NUL
    pub ssid_len:  u8,
    pub channel:   u8,
    pub band:      Band,
    pub rssi:      i8,          // dBm
    pub security:  Security,
    pub he_caps:   u8,          // 802.11ax HE capabilities
    pub beacon_interval: u16,   // TUs
}

impl Default for BssEntry {
    fn default() -> Self {
        Self {
            bssid:     ZERO_ADDR,
            ssid:      [0u8; 33],
            ssid_len:  0,
            channel:   0,
            band:      Band::Band2_4GHz,
            rssi:      -100,
            security:  Security::Open,
            he_caps:   0,
            beacon_interval: 100,
        }
    }
}

impl BssEntry {
    pub fn ssid_str(&self) -> &[u8] {
        &self.ssid[..self.ssid_len as usize]
    }
}

// ── 802.11 Frame Header (24 bytes) ────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct Dot11Header {
    pub frame_control: u16,
    pub duration:      u16,
    pub addr1:         MacAddr,  // Destination
    pub addr2:         MacAddr,  // Source
    pub addr3:         MacAddr,  // BSSID
    pub seq_ctrl:      u16,
}

impl Dot11Header {
    pub fn frame_type(&self)    -> u8 { ((self.frame_control >> 2) & 0x3) as u8 }
    pub fn frame_subtype(&self) -> u8 { ((self.frame_control >> 4) & 0xF) as u8 }
    pub fn to_ds(&self)         -> bool { (self.frame_control >> 8) & 1 != 0 }
    pub fn from_ds(&self)       -> bool { (self.frame_control >> 9) & 1 != 0 }
    pub fn more_frags(&self)    -> bool { (self.frame_control >> 10) & 1 != 0 }
    pub fn retry(&self)         -> bool { (self.frame_control >> 11) & 1 != 0 }
    pub fn protected(&self)     -> bool { (self.frame_control >> 14) & 1 != 0 }
}

// ── Beacon Frame (partial) ────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct BeaconFixed {
    pub timestamp:        u64,
    pub beacon_interval:  u16,
    pub capability_info:  u16,
}

// ── Authentication Frame ──────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct AuthFrame {
    pub header:       Dot11Header,
    pub auth_alg:     u16,
    pub auth_seq:     u16,
    pub status_code:  u16,
}

// ── Association Request Frame ─────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct AssocReqFixed {
    pub capability_info: u16,
    pub listen_interval: u16,
}

// ── WPA3-SAE State ────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SaeState {
    Nothing,
    Committed,
    Confirmed,
    Accepted,
    Failed,
}

struct SaeContext {
    state:       SaeState,
    scalar:      [u8; 32],  // Our scalar commitment (ECC P-256)
    element:     [u8; 64],  // Our element commitment (P-256 affine point x||y)
    peer_scalar: [u8; 32],
    peer_elem:   [u8; 64],
    pmk:         [u8; 32],  // Pairwise Master Key
    pmk_ready:   bool,
    token:       [u8; 32],
    send_confirm:u16,
}

impl SaeContext {
    const fn new() -> Self {
        Self {
            state:        SaeState::Nothing,
            scalar:       [0u8; 32],
            element:      [0u8; 64],
            peer_scalar:  [0u8; 32],
            peer_elem:    [0u8; 64],
            pmk:          [0u8; 32],
            pmk_ready:    false,
            token:        [0u8; 32],
            send_confirm: 0,
        }
    }
}

// ── Scan Result Pool ──────────────────────────────────────────────────────────
const MAX_SCAN_RESULTS: usize = 64;

// ── TX Frame Pool ─────────────────────────────────────────────────────────────
const MGMT_FRAME_POOL_SIZE: usize = 4;
const MAX_MGMT_FRAME: usize = 256;

struct MgmtFramePool {
    buf:  [[u8; MAX_MGMT_FRAME]; MGMT_FRAME_POOL_SIZE],
    lens: [usize; MGMT_FRAME_POOL_SIZE],
    head: usize,
}

impl MgmtFramePool {
    const fn new() -> Self {
        Self {
            buf:  [[0u8; MAX_MGMT_FRAME]; MGMT_FRAME_POOL_SIZE],
            lens: [0usize; MGMT_FRAME_POOL_SIZE],
            head: 0,
        }
    }

    fn enqueue(&mut self, frame: &[u8]) -> bool {
        if frame.len() > MAX_MGMT_FRAME { return false; }
        let idx = self.head % MGMT_FRAME_POOL_SIZE;
        self.buf[idx][..frame.len()].copy_from_slice(frame);
        self.lens[idx] = frame.len();
        self.head = self.head.wrapping_add(1);
        true
    }
}

// ── Wi-Fi Driver ──────────────────────────────────────────────────────────────
pub struct SigmaWiFi {
    state:          WiFiState,
    mac_addr:       MacAddr,
    current_bssid:  MacAddr,
    current_channel:u8,
    current_band:   Band,
    scan_results:   [BssEntry; MAX_SCAN_RESULTS],
    scan_count:     usize,
    sae:            SaeContext,
    mgmt_pool:      MgmtFramePool,
    seq_num:        u16,
    tx_packets:     AtomicU32,
    rx_packets:     AtomicU32,
    tx_errors:      AtomicU32,
    assoc_id:       u16,   // AID assigned by AP
    ssid:           [u8; 33],
    ssid_len:       u8,
    password:       [u8; 64],
    password_len:   u8,
    link_up:        AtomicBool,
    rssi:           i8,
}

impl SigmaWiFi {
    pub const fn new() -> Self {
        Self {
            state:          WiFiState::Uninitialized,
            mac_addr:       [0x52, 0x54, 0x00, 0x12, 0x34, 0x56], // Locally administered
            current_bssid:  ZERO_ADDR,
            current_channel:0,
            current_band:   Band::Band2_4GHz,
            scan_results:   [BssEntry {
                bssid:     ZERO_ADDR,
                ssid:      [0u8; 33],
                ssid_len:  0,
                channel:   0,
                band:      Band::Band2_4GHz,
                rssi:      -100,
                security:  Security::Open,
                he_caps:   0,
                beacon_interval: 100,
            }; MAX_SCAN_RESULTS],
            scan_count:     0,
            sae:            SaeContext::new(),
            mgmt_pool:      MgmtFramePool::new(),
            seq_num:        0,
            tx_packets:     AtomicU32::new(0),
            rx_packets:     AtomicU32::new(0),
            tx_errors:      AtomicU32::new(0),
            assoc_id:       0,
            ssid:           [0u8; 33],
            ssid_len:       0,
            password:       [0u8; 64],
            password_len:   0,
            link_up:        AtomicBool::new(false),
            rssi:           -100,
        }
    }

    // ── Initialization ────────────────────────────────────────────────────────

    /// Initialize the Wi-Fi subsystem. HAL transport (PCIe/USB) should be
    /// initialized before calling this.
    pub fn init(&mut self, mac: MacAddr) {
        self.mac_addr = mac;
        self.state    = WiFiState::Idle;
    }

    // ── Frame Helpers ─────────────────────────────────────────────────────────

    fn next_seq(&mut self) -> u16 {
        let s = self.seq_num;
        self.seq_num = self.seq_num.wrapping_add(1);
        s << 4 // sequence number occupies bits[15:4]
    }

    fn make_header(&mut self, subtype: u8, dst: MacAddr, bssid: MacAddr) -> Dot11Header {
        let fc: u16 = ((subtype as u16) << 4) | ((FRAME_TYPE_MGMT as u16) << 2);
        Dot11Header {
            frame_control: fc,
            duration:      0,
            addr1:         dst,
            addr2:         self.mac_addr,
            addr3:         bssid,
            seq_ctrl:      self.next_seq(),
        }
    }

    // ── Scanning ──────────────────────────────────────────────────────────────

    /// Start a passive scan. The transport driver will call `rx_beacon()` for
    /// each received beacon frame.
    pub fn start_scan(&mut self) {
        self.scan_count  = 0;
        self.state       = WiFiState::Scanning;
        // TODO: Signal HAL transport to tune to each channel sequentially.
        // For now this is handled by the transport layer calling rx_beacon().
    }

    /// Called by the transport layer when a beacon or probe response frame
    /// is received.
    pub fn rx_beacon(&mut self, bssid: MacAddr, ssid: &[u8], channel: u8, band: Band,
                     rssi: i8, security: Security, he_caps: u8, bi: u16) {
        // Don't add duplicates
        for i in 0..self.scan_count {
            if self.scan_results[i].bssid == bssid {
                self.scan_results[i].rssi = rssi; // Update RSSI
                return;
            }
        }
        if self.scan_count >= MAX_SCAN_RESULTS { return; }
        let mut entry = BssEntry::default();
        entry.bssid   = bssid;
        let l = ssid.len().min(32);
        entry.ssid[..l].copy_from_slice(&ssid[..l]);
        entry.ssid_len   = l as u8;
        entry.channel    = channel;
        entry.band       = band;
        entry.rssi       = rssi;
        entry.security   = security;
        entry.he_caps    = he_caps;
        entry.beacon_interval = bi;
        self.scan_results[self.scan_count] = entry;
        self.scan_count += 1;
    }

    pub fn scan_done(&mut self) {
        self.state = WiFiState::Idle;
    }

    pub fn scan_results(&self) -> &[BssEntry] {
        &self.scan_results[..self.scan_count]
    }

    // ── Connect / Disconnect ──────────────────────────────────────────────────

    /// Connect to an SSID. Initiates auth + association flow.
    pub fn connect(&mut self, ssid: &[u8], password: &[u8]) -> bool {
        if self.state != WiFiState::Idle { return false; }

        // Copy SSID and password
        let sl = ssid.len().min(32);
        self.ssid[..sl].copy_from_slice(&ssid[..sl]);
        self.ssid_len = sl as u8;
        let pl = password.len().min(63);
        self.password[..pl].copy_from_slice(&password[..pl]);
        self.password_len = pl as u8;

        // Find BSS matching SSID with best RSSI
        let mut best_idx: Option<usize> = None;
        let mut best_rssi: i8 = i8::MIN;
        for (i, bss) in self.scan_results[..self.scan_count].iter().enumerate() {
            if bss.ssid_str() == &self.ssid[..self.ssid_len as usize] {
                if bss.rssi > best_rssi {
                    best_rssi = bss.rssi;
                    best_idx  = Some(i);
                }
            }
        }

        if let Some(idx) = best_idx {
            let bss = self.scan_results[idx];
            self.current_bssid   = bss.bssid;
            self.current_channel = bss.channel;
            self.current_band    = bss.band;

            self.state = WiFiState::Authenticating;
            // Initiate appropriate auth sequence
            match bss.security {
                Security::WPA3SAE => self.sae_start(),
                _                 => self.send_auth_open(),
            }
            return true;
        }
        false
    }

    pub fn disconnect(&mut self) {
        self.state   = WiFiState::Idle;
        self.link_up.store(false, Ordering::Relaxed);
        self.current_bssid   = ZERO_ADDR;
        self.assoc_id        = 0;
        self.sae.state       = SaeState::Nothing;
    }

    // ── Open System Authentication ────────────────────────────────────────────

    fn send_auth_open(&mut self) {
        let hdr = self.make_header(MGMT_AUTH, self.current_bssid, self.current_bssid);
        let auth = AuthFrame {
            header:      hdr,
            auth_alg:    AUTH_OPEN,
            auth_seq:    1,
            status_code: 0,
        };
        // Serialize to managed frame pool
        let bytes = unsafe {
            core::slice::from_raw_parts(
                &auth as *const AuthFrame as *const u8,
                core::mem::size_of::<AuthFrame>(),
            )
        };
        self.mgmt_pool.enqueue(bytes);
    }

    // ── WPA3-SAE Authentication ───────────────────────────────────────────────

    fn sae_start(&mut self) {
        // SAE Commit phase: generate scalar and element using a simplified
        // Dragonfly hunting-and-pecking approach (placeholder — production
        // needs constant-time ECC library).
        //
        // For now, set dummy values to demonstrate the state machine structure.
        self.sae.state  = SaeState::Committed;
        self.sae.scalar = [0xAA; 32]; // Would be generated with CSPRNG + ECC
        self.sae.element= [0xBB; 64];
        self.send_sae_commit();
    }

    fn send_sae_commit(&mut self) {
        // 802.11 auth frame with SAE algorithm + sequence 1 (Commit)
        let hdr = self.make_header(MGMT_AUTH, self.current_bssid, self.current_bssid);
        // Compose commit frame (header + SAE commit payload)
        let mut frame = [0u8; MAX_MGMT_FRAME];
        let hdr_bytes = unsafe {
            core::slice::from_raw_parts(
                &hdr as *const Dot11Header as *const u8,
                core::mem::size_of::<Dot11Header>(),
            )
        };
        let off = hdr_bytes.len();
        frame[..off].copy_from_slice(hdr_bytes);
        frame[off]     = (AUTH_SAE & 0xFF) as u8;
        frame[off + 1] = (AUTH_SAE >> 8) as u8;
        frame[off + 2] = 1; // seq = 1 (Commit)
        frame[off + 3] = 0;
        frame[off + 4] = 0; // status = 0
        frame[off + 5] = 0;
        // Append scalar (32 bytes) + element (64 bytes) — SAE commit fields
        let body_off = off + 6;
        frame[body_off..body_off + 32].copy_from_slice(&self.sae.scalar);
        frame[body_off + 32..body_off + 96].copy_from_slice(&self.sae.element);
        let total = body_off + 96;
        self.mgmt_pool.enqueue(&frame[..total]);
    }

    fn send_sae_confirm(&mut self) {
        self.sae.send_confirm += 1;
        // SAE confirm frame (sequence 2)
        let hdr = self.make_header(MGMT_AUTH, self.current_bssid, self.current_bssid);
        let mut frame = [0u8; MAX_MGMT_FRAME];
        let hdr_bytes = unsafe {
            core::slice::from_raw_parts(
                &hdr as *const Dot11Header as *const u8,
                core::mem::size_of::<Dot11Header>(),
            )
        };
        let off = hdr_bytes.len();
        frame[..off].copy_from_slice(hdr_bytes);
        frame[off]     = (AUTH_SAE & 0xFF) as u8;
        frame[off + 1] = (AUTH_SAE >> 8) as u8;
        frame[off + 2] = 2; // seq = 2 (Confirm)
        frame[off + 3] = 0;
        frame[off + 4] = 0;
        frame[off + 5] = 0;
        // Confirm verifier (32-byte HMAC-SHA256 over send_confirm + PMK) — placeholder
        let body_off = off + 6;
        frame[body_off..body_off + 2].copy_from_slice(&self.sae.send_confirm.to_le_bytes());
        frame[body_off + 2..body_off + 34].copy_from_slice(&self.sae.pmk);
        let total = body_off + 34;
        self.mgmt_pool.enqueue(&frame[..total]);
        self.sae.state = SaeState::Confirmed;
    }

    // ── Rx Management Frame Handler ───────────────────────────────────────────

    /// Call this when the transport layer delivers a management frame.
    pub fn rx_mgmt(&mut self, frame: &[u8]) {
        if frame.len() < core::mem::size_of::<Dot11Header>() { return; }
        let hdr = unsafe { &*(frame.as_ptr() as *const Dot11Header) };
        let subtype = hdr.frame_subtype();
        let off = core::mem::size_of::<Dot11Header>();

        match subtype {
            MGMT_AUTH => self.handle_auth(&frame[off..]),
            MGMT_ASSOC_RESP => self.handle_assoc_resp(&frame[off..]),
            MGMT_DEAUTH => {
                self.state = WiFiState::Idle;
                self.link_up.store(false, Ordering::Relaxed);
            },
            _ => {},
        }
        self.rx_packets.fetch_add(1, Ordering::Relaxed);
    }

    fn handle_auth(&mut self, body: &[u8]) {
        if body.len() < 6 { return; }
        let alg    = u16::from_le_bytes([body[0], body[1]]);
        let seq    = u16::from_le_bytes([body[2], body[3]]);
        let status = u16::from_le_bytes([body[4], body[5]]);

        if status != 0 {
            self.state = WiFiState::Error(status);
            return;
        }

        match alg {
            AUTH_OPEN => {
                if seq == 2 {
                    // Auth successful → now associate
                    self.state = WiFiState::Associating;
                    self.send_assoc_req();
                }
            },
            AUTH_SAE => {
                match seq {
                    1 => {
                        // Received SAE commit from AP
                        if body.len() >= 6 + 96 {
                            self.sae.peer_scalar.copy_from_slice(&body[6..38]);
                            self.sae.peer_elem.copy_from_slice(&body[38..102]);
                            // Derive PMK (placeholder — real: DH point mult + KDF)
                            self.sae.pmk = [0xCC; 32];
                            self.sae.pmk_ready = true;
                            self.sae.state = SaeState::Committed;
                            self.send_sae_confirm();
                        }
                    },
                    2 => {
                        // Received SAE confirm → authentication complete
                        if self.sae.state == SaeState::Confirmed {
                            self.sae.state = SaeState::Accepted;
                            self.state     = WiFiState::Associating;
                            self.send_assoc_req();
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    fn send_assoc_req(&mut self) {
        let hdr = self.make_header(MGMT_ASSOC_REQ, self.current_bssid, self.current_bssid);
        let mut frame = [0u8; MAX_MGMT_FRAME];
        let hdr_bytes = unsafe {
            core::slice::from_raw_parts(
                &hdr as *const Dot11Header as *const u8,
                core::mem::size_of::<Dot11Header>(),
            )
        };
        let off = hdr_bytes.len();
        frame[..off].copy_from_slice(hdr_bytes);

        // Fixed fields
        let cap_info: u16 = 0x0431; // ESS | Short Preamble | Short Slot | WMM
        let listen_iv: u16 = 10;
        frame[off]     = (cap_info & 0xFF) as u8;
        frame[off + 1] = (cap_info >> 8) as u8;
        frame[off + 2] = (listen_iv & 0xFF) as u8;
        frame[off + 3] = (listen_iv >> 8) as u8;
        let mut pos = off + 4;

        // SSID IE
        frame[pos] = 0; pos += 1; // Element ID = SSID
        frame[pos] = self.ssid_len; pos += 1;
        let sl = self.ssid_len as usize;
        frame[pos..pos + sl].copy_from_slice(&self.ssid[..sl]);
        pos += sl;

        // Supported Rates IE (6, 9, 12, 18, 24, 36, 48, 54 Mbps)
        frame[pos] = 1; pos += 1; // Element ID = Supported Rates
        frame[pos] = 8; pos += 1; // Length
        let rates: [u8; 8] = [0x8C, 0x12, 0x98, 0x24, 0xB0, 0x48, 0x60, 0x6C];
        frame[pos..pos + 8].copy_from_slice(&rates);
        pos += 8;

        // RSN IE (WPA2/WPA3) — simplified
        if self.ssid_len > 0 {
            frame[pos] = 48; pos += 1; // Element ID = RSN
            frame[pos] = 20; pos += 1; // Length
            // WPA2 / WPA3 RSN element (AES-CCMP group+pairwise, SAE AKM)
            let rsn: [u8; 20] = [
                0x01, 0x00,             // Version
                0x00, 0x0F, 0xAC, 0x04, // Group cipher: AES-CCMP
                0x01, 0x00,             // Pairwise count
                0x00, 0x0F, 0xAC, 0x04, // Pairwise: AES-CCMP
                0x01, 0x00,             // AKM count
                0x00, 0x0F, 0xAC, 0x08, // AKM: SAE
                0x00, 0x00,             // RSN capabilities
            ];
            frame[pos..pos + 20].copy_from_slice(&rsn);
            pos += 20;
        }

        // HE Capability IE (simplified 802.11ax)
        frame[pos] = 255; pos += 1;   // Extension element
        frame[pos] = 3;   pos += 1;   // Length
        frame[pos] = 35;  pos += 1;   // Extension ID: HE Capabilities
        frame[pos] = HE_CAP_MCS_SINGLE | HE_CAP_OFDMA | HE_CAP_TWT; pos += 1;
        frame[pos] = 0;   pos += 1;

        self.mgmt_pool.enqueue(&frame[..pos]);
    }

    fn handle_assoc_resp(&mut self, body: &[u8]) {
        if body.len() < 6 { return; }
        let _cap_info = u16::from_le_bytes([body[0], body[1]]);
        let status    = u16::from_le_bytes([body[2], body[3]]);
        let aid       = u16::from_le_bytes([body[4], body[5]]) & 0x3FFF;

        if status == 0 {
            self.assoc_id = aid;
            self.state    = WiFiState::Associated;
            self.link_up.store(true, Ordering::Relaxed);
        } else {
            self.state = WiFiState::Error(status);
        }
    }

    // ── Data TX hook ──────────────────────────────────────────────────────────

    /// Wrap an Ethernet frame in an 802.11 data frame header.
    /// Returns the number of bytes written to `out_buf`.
    pub fn wrap_data_frame(&mut self, eth_frame: &[u8], out_buf: &mut [u8]) -> usize {
        if self.state != WiFiState::Associated { return 0; }
        if eth_frame.len() + 26 > out_buf.len() { return 0; }

        // 802.11 data frame FC: type=Data, ToDS=1
        let fc: u16 = (0x08 << 4) | ((FRAME_TYPE_DATA as u16) << 2) | (1 << 8);
        let hdr = Dot11Header {
            frame_control: fc,
            duration: 0,
            addr1: self.current_bssid,
            addr2: self.mac_addr,
            addr3: {
                // Destination MAC from Ethernet header (first 6 bytes)
                let mut a = [0u8; 6];
                if eth_frame.len() >= 6 { a.copy_from_slice(&eth_frame[..6]); }
                a
            },
            seq_ctrl: self.next_seq(),
        };
        let hdr_bytes = unsafe {
            core::slice::from_raw_parts(
                &hdr as *const Dot11Header as *const u8,
                core::mem::size_of::<Dot11Header>(),
            )
        };
        let hl = hdr_bytes.len();
        out_buf[..hl].copy_from_slice(hdr_bytes);
        // LLC/SNAP header (6 bytes) for Ethernet encapsulation
        out_buf[hl + 0] = 0xAA; // DSAP
        out_buf[hl + 1] = 0xAA; // SSAP
        out_buf[hl + 2] = 0x03; // Control
        out_buf[hl + 3] = 0x00; // OUI
        out_buf[hl + 4] = 0x00;
        out_buf[hl + 5] = 0x00;
        // Ether type (bytes 12-13 of Ethernet frame)
        if eth_frame.len() >= 14 {
            out_buf[hl + 6] = eth_frame[12];
            out_buf[hl + 7] = eth_frame[13];
        }
        let payload = if eth_frame.len() >= 14 { &eth_frame[14..] } else { eth_frame };
        let plen = payload.len().min(out_buf.len() - hl - 8);
        out_buf[hl + 8..hl + 8 + plen].copy_from_slice(&payload[..plen]);
        self.tx_packets.fetch_add(1, Ordering::Relaxed);
        hl + 8 + plen
    }

    // ── Status ────────────────────────────────────────────────────────────────

    pub fn is_associated(&self) -> bool {
        self.state == WiFiState::Associated
    }

    pub fn link_up(&self) -> bool {
        self.link_up.load(Ordering::Relaxed)
    }

    pub fn current_rssi(&self) -> i8 {
        self.rssi
    }

    pub fn update_rssi(&mut self, rssi: i8) {
        self.rssi = rssi;
    }

    pub fn mac(&self) -> MacAddr {
        self.mac_addr
    }
}

// ── Global Instance ───────────────────────────────────────────────────────────
static mut G_WIFI: SigmaWiFi = SigmaWiFi::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_wifi_init(mac: *const u8) {
    let mut m = [0u8; 6];
    if !mac.is_null() {
        for i in 0..6 { m[i] = *mac.add(i); }
    }
    G_WIFI.init(m);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_wifi_start_scan() {
    G_WIFI.start_scan();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_wifi_scan_count() -> usize {
    G_WIFI.scan_count
}

#[no_mangle]
pub unsafe extern "C" fn sigma_wifi_connect(
    ssid: *const u8, ssid_len: usize,
    pass: *const u8, pass_len: usize,
) -> i32 {
    if ssid.is_null() { return -1; }
    let s = core::slice::from_raw_parts(ssid, ssid_len);
    let p = if pass.is_null() { &[] } else { core::slice::from_raw_parts(pass, pass_len) };
    if G_WIFI.connect(s, p) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_wifi_disconnect() {
    G_WIFI.disconnect();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_wifi_is_associated() -> i32 {
    if G_WIFI.is_associated() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_wifi_rssi() -> i8 {
    G_WIFI.current_rssi()
}
