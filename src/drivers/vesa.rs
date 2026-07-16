// SigmaOS VESA Framebuffer Driver
// Hardware abstraction for VESA BIOS extensions

use crate::security::CapabilityToken;

/// VESA mode info
#[derive(Debug, Clone)]
pub struct VesaModeInfo {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub pitch: u32,
    pub framebuffer_addr: u64,
}

/// VESA driver interface
pub struct VesaDriver {
    pub mode_info: VesaModeInfo,
    pub capabilities: CapabilityToken,
    pub current_mode: u16,
}

impl VesaDriver {
    pub fn new() -> Self {
        Self {
            mode_info: VesaModeInfo {
                width: 1024,
                height: 768,
                bpp: 32,
                pitch: 4096,
                framebuffer_addr: 0xE0000000,
            },
            capabilities: CapabilityToken::new(),
            current_mode: 0,
        }
    }

    pub fn with_mode(width: u32, height: u32, bpp: u32) -> Self {
        let pitch = width * (bpp / 8);
        Self {
            mode_info: VesaModeInfo {
                width,
                height,
                bpp,
                pitch,
                framebuffer_addr: 0xE0000000,
            },
            capabilities: CapabilityToken::new(),
            current_mode: 0,
        }
    }

    pub fn initialize(&mut self) -> Result<(), VesaError> {
        // Simulate VESA initialization
        self.current_mode = 0x118; // Common VESA mode
        Ok(())
    }

    pub fn set_mode(&mut self, mode: u16) -> Result<(), VesaError> {
        // Simulate mode setting
        self.current_mode = mode;

        // Update mode info based on mode
        match mode {
            0x112 => {
                self.mode_info.width = 640;
                self.mode_info.height = 480;
                self.mode_info.bpp = 32;
            }
            0x115 => {
                self.mode_info.width = 800;
                self.mode_info.height = 600;
                self.mode_info.bpp = 32;
            }
            0x118 => {
                self.mode_info.width = 1024;
                self.mode_info.height = 768;
                self.mode_info.bpp = 32;
            }
            _ => {
                return Err(VesaError::InvalidMode);
            }
        }

        self.mode_info.pitch = self.mode_info.width * (self.mode_info.bpp / 8);
        Ok(())
    }

    pub fn get_mode_info(&self) -> &VesaModeInfo {
        &self.mode_info
    }

    pub fn write_pixel(&self, x: u32, y: u32, color: u32) -> Result<(), VesaError> {
        if x >= self.mode_info.width || y >= self.mode_info.height {
            return Err(VesaError::OutOfBounds);
        }

        // Calculate pixel offset
        let offset = (y * self.mode_info.pitch + x * (self.mode_info.bpp / 8)) as usize;

        // In production, this would write to actual framebuffer
        // For now, just validate the operation
        Ok(())
    }

    pub fn clear_screen(&self, color: u32) -> Result<(), VesaError> {
        // Simulate screen clear
        Ok(())
    }

    pub fn set_capabilities(&mut self, capabilities: CapabilityToken) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities.bits & capability) != 0
    }
}

impl Default for VesaDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// VESA errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VesaError {
    InvalidMode,
    OutOfBounds,
    PermissionDenied,
    InitializationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vesa_creation() {
        let vesa = VesaDriver::new();
        assert_eq!(vesa.mode_info.width, 1024);
        assert_eq!(vesa.mode_info.height, 768);
        assert_eq!(vesa.mode_info.bpp, 32);
    }

    #[test]
    fn test_vesa_with_mode() {
        let vesa = VesaDriver::with_mode(1920, 1080, 32);
        assert_eq!(vesa.mode_info.width, 1920);
        assert_eq!(vesa.mode_info.height, 1080);
    }

    #[test]
    fn test_set_mode() {
        let mut vesa = VesaDriver::new();
        assert!(vesa.set_mode(0x112).is_ok());
        assert_eq!(vesa.mode_info.width, 640);
        assert_eq!(vesa.mode_info.height, 480);
    }

    #[test]
    fn test_invalid_mode() {
        let mut vesa = VesaDriver::new();
        assert!(vesa.set_mode(0x999).is_err());
    }

    #[test]
    fn test_write_pixel() {
        let vesa = VesaDriver::new();
        assert!(vesa.write_pixel(100, 100, 0xFFFFFF).is_ok());
    }

    #[test]
    fn test_out_of_bounds() {
        let vesa = VesaDriver::new();
        assert!(vesa.write_pixel(9999, 9999, 0xFFFFFF).is_err());
    }
}
