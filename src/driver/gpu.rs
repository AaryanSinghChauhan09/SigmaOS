// Sovereign GPU Acceleration & DRM/KMS Framework for SigmaOS
// Provides zero-dependency GPU mode setting, display pipeline control, and buffer object management.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    GenericVesa,
}

pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

pub struct SovereignGpuDriver {
    pub vendor: GpuVendor,
    pub card_name: String,
    pub active_mode: DisplayMode,
    pub vram_mb: usize,
}

impl SovereignGpuDriver {
    pub fn new(vendor: GpuVendor, card_name: &str, vram_mb: usize) -> Self {
        Self {
            vendor,
            card_name: String::from(card_name),
            active_mode: DisplayMode {
                width: 1920,
                height: 1080,
                refresh_rate: 60,
            },
            vram_mb,
        }
    }

    pub fn set_display_mode(&mut self, width: u32, height: u32, refresh_rate: u32) {
        self.active_mode = DisplayMode {
            width,
            height,
            refresh_rate,
        };
    }
}
