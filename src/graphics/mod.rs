pub mod drm_kms;
pub mod video;

pub use drm_kms::{DrmAtomicCommit, DrmCrtc, DrmConnector, DrmConnectorStatus, DrmConnectorType, DrmEncoder, DrmFramebuffer, DrmKmsDevice, DrmMode, DrmNodeType, DrmPlane, DrmPlaneType, GemBuffer};
pub use video::{PixelRgba, VideoFrame};

// Color type for UI compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}
