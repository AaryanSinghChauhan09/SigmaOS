// SigmaOS GPU Driver
// Hardware abstraction for graphics rendering

use crate::security::CapabilityToken;

/// GPU command type
#[derive(Debug, Clone)]
pub enum GpuCommand {
    ClearScreen {
        r: u8,
        g: u8,
        b: u8,
    },
    DrawRect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    DrawText {
        x: u32,
        y: u32,
        text: String,
    },
    Present,
}

/// DRM/KMS CRTC configuration
#[derive(Debug, Clone, Copy)]
pub struct DrmCrtc {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub active: bool,
}

/// DRM/KMS Mode Info
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrmModeInfo {
    pub clock: u32,
    pub hdisplay: u16,
    pub vdisplay: u16,
    pub vrefresh: u32,
}

/// DRM/KMS Connector configuration
#[derive(Debug, Clone)]
pub struct DrmConnector {
    pub id: u32,
    pub connected: bool,
    pub modes: Vec<DrmModeInfo>,
}

/// GPU driver interface
pub struct GpuDriver {
    pub width: u32,
    pub height: u32,
    pub capabilities: CapabilityToken,
    pub frame_buffer: Vec<u32>,
    pub crtc: Option<DrmCrtc>,
    pub connector: Option<DrmConnector>,
}

impl GpuDriver {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            capabilities: CapabilityToken::new(),
            frame_buffer: vec![0; size],
            crtc: None,
            connector: None,
        }
    }

    pub fn execute_command(&mut self, command: GpuCommand) -> Result<(), GpuError> {
        match command {
            GpuCommand::ClearScreen { r, g, b } => {
                let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                self.frame_buffer.fill(color);
            }
            GpuCommand::DrawRect {
                x,
                y,
                width,
                height,
                ..
            } => {
                let color = 0xFFFFFF; // White
                for row in y..(y + height).min(self.height) {
                    for col in x..(x + width).min(self.width) {
                        let idx = (row * self.width + col) as usize;
                        if idx < self.frame_buffer.len() {
                            self.frame_buffer[idx] = color;
                        }
                    }
                }
            }
            GpuCommand::Present => {
                // In production, this would swap buffers
            }
            GpuCommand::DrawText { .. } => {
                // Text rendering implementation
            }
        }
        Ok(())
    }

    pub fn set_capabilities(&mut self, capabilities: CapabilityToken) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities.bits() & capability) != 0
    }

    /// DRM/KMS mode setting API
    pub fn set_drm_mode(&mut self, connector_id: u32, crtc_id: u32, mode: DrmModeInfo) -> Result<(), GpuError> {
        self.width = mode.hdisplay as u32;
        self.height = mode.vdisplay as u32;
        self.frame_buffer = vec![0; (self.width * self.height) as usize];

        self.crtc = Some(DrmCrtc {
            id: crtc_id,
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
            active: true,
        });

        self.connector = Some(DrmConnector {
            id: connector_id,
            connected: true,
            modes: vec![mode],
        });

        Ok(())
    }
}

impl Default for GpuDriver {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

/// GPU errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuError {
    InvalidCommand,
    OutOfBounds,
    PermissionDenied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_creation() {
        let gpu = GpuDriver::new(1920, 1080);
        assert_eq!(gpu.width, 1920);
        assert_eq!(gpu.height, 1080);
    }

    #[test]
    fn test_clear_screen() {
        let mut gpu = GpuDriver::new(100, 100);
        let command = GpuCommand::ClearScreen { r: 255, g: 0, b: 0 };
        assert!(gpu.execute_command(command).is_ok());
    }

    #[test]
    fn test_draw_rect() {
        let mut gpu = GpuDriver::new(100, 100);
        let command = GpuCommand::DrawRect {
            x: 10,
            y: 10,
            width: 20,
            height: 20,
        };
        assert!(gpu.execute_command(command).is_ok());
    }
}
