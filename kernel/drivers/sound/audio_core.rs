// audio_core.rs: Basic Audio Subsystem Skeleton for SigmaOS

#![no_std]

pub trait AudioDevice {
    /// Start PCM playback
    fn start_playback(&mut self) -> Result<(), &'static str>;
    /// Stop PCM playback
    fn stop_playback(&mut self) -> Result<(), &'static str>;
    /// Write PCM data to DMA buffer
    fn write_buffer(&mut self, data: &[u8]) -> Result<usize, &'static str>;
}

pub struct AudioCoreManager {
    // Current active audio device
    active_device: Option<&'static mut dyn AudioDevice>,
}

impl AudioCoreManager {
    pub fn new() -> Self {
        Self {
            active_device: None,
        }
    }

    pub fn register_device(&mut self, device: &'static mut dyn AudioDevice) {
        self.active_device = Some(device);
    }
}
