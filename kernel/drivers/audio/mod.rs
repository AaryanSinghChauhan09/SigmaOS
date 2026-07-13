// Audio Drivers Module
// Implements audio drivers for Intel HDA and USB Audio
// Zero-dependency Rust implementation for SigmaOS

pub mod hda;
pub mod usb_audio;

pub use hda::HdaDriver;
pub use usb_audio::UsbAudioDriver;

// Common audio types
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum AudioVendor {
    Intel,
    Realtek,
    Creative,
    USBGeneric,
    Unknown,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioInfo {
    pub vendor: AudioVendor,
    pub device_id: u32,
    pub max_channels: u8,
    pub max_sample_rate: u32,
    pub supports_hdmi: bool,
}

// Common audio trait
pub trait AudioDriver {
    fn detect(&self) -> Option<AudioInfo>;
    fn initialize(&mut self) -> Result<(), AudioError>;
    fn set_volume(&mut self, volume: u8) -> Result<(), AudioError>;
    fn play(&mut self, buffer: &[u8]) -> Result<(), AudioError>;
    fn record(&mut self, buffer: &mut [u8]) -> Result<(), AudioError>;
    fn get_info(&self) -> AudioInfo;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum AudioError {
    NotFound,
    InitializationFailed,
    InvalidBuffer,
    HardwareError,
    UnsupportedFormat,
}
