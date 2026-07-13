// Network Drivers Module
// Implements network drivers for Intel iwlwifi, MediaTek MT7921, and Realtek RTW88
// Zero-dependency Rust implementation for SigmaOS

pub mod iwlwifi;
pub mod mt7921;
pub mod rtw88;

pub use iwlwifi::IwlwifiDriver;
pub use mt7921::Mt7921Driver;
pub use rtw88::Rtw88Driver;

// Common network types
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum WifiVendor {
    Intel,
    MediaTek,
    Realtek,
    Unknown,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WifiInfo {
    pub vendor: WifiVendor,
    pub device_id: u32,
    pub mac_address: [u8; 6],
    pub supports_5ghz: bool,
    pub supports_wifi6: bool,
}

// Common network trait
pub trait WifiDriver {
    fn detect(&self) -> Option<WifiInfo>;
    fn initialize(&mut self) -> Result<(), WifiError>;
    fn scan(&mut self) -> Result<(), WifiError>;
    fn connect(&mut self, ssid: &str, password: &str) -> Result<(), WifiError>;
    fn disconnect(&mut self) -> Result<(), WifiError>;
    fn get_info(&self) -> WifiInfo;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum WifiError {
    NotFound,
    InitializationFailed,
    ScanFailed,
    ConnectionFailed,
    DisconnectionFailed,
    InvalidCredentials,
}
