//! # IEEE 802.11 Wi-Fi Stack
//!
//! Wireless networking stack implementing 802.11a/b/g/n/ac/ax/be.
//! Inspired by Linux mac80211 and FreeBSD net80211.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// IEEE 802.11 frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
    Management = 0,
    Control = 1,
    Data = 2,
}

/// Management frame subtypes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ManagementSubtype {
    AssociationRequest = 0,
    AssociationResponse = 1,
    ReassociationRequest = 2,
    ReassociationResponse = 3,
    ProbeRequest = 4,
    ProbeResponse = 5,
    Beacon = 8,
    ATIM = 9,
    Disassociation = 10,
    Authentication = 11,
    Deauthentication = 12,
    Action = 13,
}

/// 802.11 MAC header (24 bytes minimum)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Dot11Header {
    pub frame_control: u16, // Frame control field
    pub duration: u16,      // Duration/ID
    pub addr1: [u8; 6],     // Address 1 (receiver)
    pub addr2: [u8; 6],     // Address 2 (transmitter)
    pub addr3: [u8; 6],     // Address 3 (BSSID or destination)
    pub seq_ctrl: u16,      // Sequence control
                            // addr4 present in WDS mode
}

impl Dot11Header {
    pub fn get_type(&self) -> FrameType {
        let fc = u16::from_le(self.frame_control);
        unsafe { core::mem::transmute(((fc >> 2) & 0x03) as u8) }
    }

    pub fn get_subtype(&self) -> u8 {
        let fc = u16::from_le(self.frame_control);
        ((fc >> 4) & 0x0F) as u8
    }

    pub fn is_to_ds(&self) -> bool {
        u16::from_le(self.frame_control) & 0x0100 != 0
    }

    pub fn is_from_ds(&self) -> bool {
        u16::from_le(self.frame_control) & 0x0200 != 0
    }

    pub fn is_protected(&self) -> bool {
        u16::from_le(self.frame_control) & 0x4000 != 0
    }
}

/// Wi-Fi operating modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WifiMode {
    Infrastructure = 0, // Station mode (client)
    AdHoc = 1,          // IBSS (ad-hoc network)
    AP = 2,             // Access Point
    Monitor = 3,        // Monitor mode (promiscuous)
    Mesh = 4,           // 802.11s mesh
}

/// Wi-Fi security types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WifiSecurity {
    Open = 0,
    WEP = 1,
    WPA = 2,
    WPA2 = 3,
    WPA3 = 4,
    WPA2Enterprise = 5,
    WPA3Enterprise = 6,
}

/// Wi-Fi frequency bands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WifiBand {
    Band2_4GHz = 0, // 2.4 GHz (2412-2484 MHz)
    Band5GHz = 1,   // 5 GHz (5170-5835 MHz)
    Band6GHz = 2,   // 6 GHz (5945-7125 MHz) - 802.11ax/be
}

/// Wi-Fi channel bandwidth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelWidth {
    MHz20 = 0,  // 20 MHz
    MHz40 = 1,  // 40 MHz (802.11n)
    MHz80 = 2,  // 80 MHz (802.11ac)
    MHz160 = 3, // 160 MHz (802.11ac/ax)
    MHz320 = 4, // 320 MHz (802.11be)
}

/// Wi-Fi PHY standards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WifiStandard {
    Legacy = 0, // 802.11a/b/g
    N = 1,      // 802.11n (Wi-Fi 4)
    AC = 2,     // 802.11ac (Wi-Fi 5)
    AX = 3,     // 802.11ax (Wi-Fi 6/6E)
    BE = 4,     // 802.11be (Wi-Fi 7)
}

/// Wi-Fi channel configuration
#[derive(Debug, Clone, Copy)]
pub struct WifiChannel {
    pub number: u8,
    pub frequency: u16, // MHz
    pub band: WifiBand,
    pub width: ChannelWidth,
}

impl WifiChannel {
    /// Get 2.4 GHz channel
    pub fn channel_2_4ghz(num: u8) -> Option<Self> {
        if num >= 1 && num <= 14 {
            let freq = match num {
                14 => 2484,
                _ => 2407 + (num as u16 * 5),
            };
            Some(Self {
                number: num,
                frequency: freq,
                band: WifiBand::Band2_4GHz,
                width: ChannelWidth::MHz20,
            })
        } else {
            None
        }
    }

    /// Get 5 GHz channel
    pub fn channel_5ghz(num: u8) -> Option<Self> {
        if num >= 36 && num <= 177 {
            let freq = 5000 + (num as u16 * 5);
            Some(Self {
                number: num,
                frequency: freq,
                band: WifiBand::Band5GHz,
                width: ChannelWidth::MHz20,
            })
        } else {
            None
        }
    }
}

/// BSS (Basic Service Set) information
#[derive(Debug, Clone)]
pub struct BssInfo {
    pub bssid: [u8; 6], // MAC address of AP
    pub ssid: String,   // Network name
    pub channel: WifiChannel,
    pub security: WifiSecurity,
    pub signal_strength: i8,  // dBm
    pub beacon_interval: u16, // TUs (1024 μs)
    pub capability: u16,
}

/// Station information
#[derive(Debug, Clone)]
pub struct StationInfo {
    pub mac_addr: [u8; 6],
    pub connected: bool,
    pub signal: i8,   // dBm
    pub tx_rate: u32, // Mbps
    pub rx_rate: u32, // Mbps
    pub tx_packets: u64,
    pub rx_packets: u64,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub tx_retries: u64,
    pub tx_failed: u64,
}

/// Wi-Fi scan request
#[derive(Debug, Clone)]
pub struct ScanRequest {
    pub ssid: Option<String>, // Specific SSID or broadcast
    pub channels: Vec<WifiChannel>,
    pub passive: bool, // Passive vs active scan
}

/// Wi-Fi connection parameters
#[derive(Debug, Clone)]
pub struct ConnectParams {
    pub ssid: String,
    pub bssid: Option<[u8; 6]>,
    pub security: WifiSecurity,
    pub passphrase: Option<String>,
    pub channel: Option<WifiChannel>,
}

/// Rate control algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateControlAlgo {
    Minstrel,   // Linux default
    MinstrelHT, // HT/VHT extension
    PID,        // Proportional-Integral-Derivative
    AMRR,       // Adaptive Multi Rate Retry (BSD)
}

/// Wi-Fi device capabilities
#[derive(Debug, Clone)]
pub struct WifiCapabilities {
    pub standards: Vec<WifiStandard>,
    pub bands: Vec<WifiBand>,
    pub max_tx_power: i8, // dBm
    pub supports_monitor: bool,
    pub supports_ap: bool,
    pub supports_mesh: bool,
    pub ht_capabilities: Option<HtCapabilities>,
    pub vht_capabilities: Option<VhtCapabilities>,
    pub he_capabilities: Option<HeCapabilities>,
}

/// 802.11n HT capabilities
#[derive(Debug, Clone, Copy)]
pub struct HtCapabilities {
    pub ldpc_coding: bool,
    pub channel_width_40mhz: bool,
    pub sm_power_save: u8,
    pub greenfield: bool,
    pub short_gi_20mhz: bool,
    pub short_gi_40mhz: bool,
    pub tx_stbc: bool,
    pub rx_stbc: u8,
    pub max_amsdu_length: u16,
    pub max_ampdu_length: u32,
}

/// 802.11ac VHT capabilities
#[derive(Debug, Clone, Copy)]
pub struct VhtCapabilities {
    pub max_mpdu_length: u16,
    pub supported_channel_widths: u8,
    pub rx_ldpc: bool,
    pub short_gi_80mhz: bool,
    pub short_gi_160mhz: bool,
    pub tx_stbc: bool,
    pub rx_stbc: u8,
    pub su_beamformer: bool,
    pub su_beamformee: bool,
    pub mu_beamformer: bool,
    pub mu_beamformee: bool,
}

/// 802.11ax HE capabilities
#[derive(Debug, Clone, Copy)]
pub struct HeCapabilities {
    pub dual_band: bool,
    pub channel_width: u8,
    pub rx_pream_puncturing: bool,
    pub device_class: u8,
    pub ldpc_coding: bool,
    pub he_su_ppdu_1x_ltf_0_8us: bool,
    pub max_nc: u8,
    pub stbc_tx: bool,
    pub stbc_rx: bool,
    pub doppler_tx: bool,
    pub doppler_rx: bool,
    pub full_bw_ul_mu_mimo: bool,
    pub partial_bw_ul_mu_mimo: bool,
}

/// Wi-Fi driver interface trait
pub trait WifiDriver {
    fn get_capabilities(&self) -> &WifiCapabilities;
    fn set_mode(&mut self, mode: WifiMode) -> Result<(), WifiError>;
    fn get_mode(&self) -> WifiMode;
    fn scan(&mut self, request: &ScanRequest) -> Result<Vec<BssInfo>, WifiError>;
    fn connect(&mut self, params: &ConnectParams) -> Result<(), WifiError>;
    fn disconnect(&mut self) -> Result<(), WifiError>;
    fn get_station_info(&self) -> Result<StationInfo, WifiError>;
    fn get_signal_strength(&self) -> Result<i8, WifiError>;
    fn transmit(&mut self, frame: &[u8]) -> Result<(), WifiError>;
    fn receive(&mut self) -> Option<Vec<u8>>;
}

/// Wi-Fi rate information
#[derive(Debug, Clone, Copy)]
pub struct WifiRate {
    pub bitrate: u32, // Kbps
    pub mcs_index: Option<u8>,
    pub nss: u8, // Number of spatial streams
    pub gi: u8,  // Guard interval (0=long, 1=short)
}

/// Wi-Fi transmit parameters
#[derive(Debug, Clone, Copy)]
pub struct TxParams {
    pub rate: WifiRate,
    pub power: i8, // dBm
    pub retry_limit: u8,
    pub use_rts: bool,
    pub use_cts: bool,
}

/// Wi-Fi errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiError {
    NotConnected,
    ScanFailed,
    AuthenticationFailed,
    AssociationFailed,
    InvalidChannel,
    InvalidMode,
    Timeout,
    NoHardware,
    FirmwareError,
    BufferFull,
}

/// WPA2/WPA3 key management
pub struct WpaKeyManagement {
    pub ptk: [u8; 64], // Pairwise Transient Key
    pub gtk: [u8; 32], // Group Temporal Key
}

impl WpaKeyManagement {
    pub fn new() -> Self {
        Self {
            ptk: [0u8; 64],
            gtk: [0u8; 32],
        }
    }

    /// Derive PTK from PMK and nonces (simplified)
    pub fn derive_ptk(
        &mut self,
        pmk: &[u8; 32],
        aa: &[u8; 6],
        sa: &[u8; 6],
        anonce: &[u8; 32],
        snonce: &[u8; 32],
    ) {
        // In production: use PRF-512 for actual key derivation
        // This is a placeholder
        for i in 0..32 {
            self.ptk[i] = pmk[i] ^ anonce[i] ^ snonce[i % snonce.len()];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wifi_channel_2_4ghz() {
        let ch1 = WifiChannel::channel_2_4ghz(1).unwrap();
        assert_eq!(ch1.frequency, 2412);
        assert_eq!(ch1.band, WifiBand::Band2_4GHz);

        let ch11 = WifiChannel::channel_2_4ghz(11).unwrap();
        assert_eq!(ch11.frequency, 2462);

        let ch14 = WifiChannel::channel_2_4ghz(14).unwrap();
        assert_eq!(ch14.frequency, 2484);
    }

    #[test]
    fn test_wifi_channel_5ghz() {
        let ch36 = WifiChannel::channel_5ghz(36).unwrap();
        assert_eq!(ch36.frequency, 5180);
        assert_eq!(ch36.band, WifiBand::Band5GHz);

        let ch165 = WifiChannel::channel_5ghz(165).unwrap();
        assert_eq!(ch165.frequency, 5825);
    }

    #[test]
    fn test_dot11_header() {
        let mut header = Dot11Header {
            frame_control: 0x0040u16.to_le(), // Data frame
            duration: 0,
            addr1: [0xFF; 6],
            addr2: [0x00; 6],
            addr3: [0x11; 6],
            seq_ctrl: 0,
        };

        assert_eq!(header.get_type(), FrameType::Data);
    }
}
