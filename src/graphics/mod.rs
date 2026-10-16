pub mod advanced_accel;
pub mod bsd_graphics;
pub mod compositor;
pub mod drm_kms;
pub mod gaming_layer;
pub mod gpu_driver;
pub mod image_decoder;
pub mod multi_monitor;
pub mod nvidia_prime;
pub mod paint;
pub mod raytracer;
pub mod render3d;
pub mod sovereign_graphics_display_pr_suite;
pub mod vector_engine;
pub mod video;
pub mod video_editor;
pub mod video_timeline;
pub mod virtio_gpu;
pub mod vulkan_api;
pub mod zenith;
pub mod zenith_compositor;

pub use drm_kms::{
    DrmAtomicCommit, DrmConnector, DrmConnectorStatus, DrmConnectorType, DrmCrtc, DrmEncoder,
    DrmFramebuffer, DrmKmsDevice, DrmMode, DrmNodeType, DrmPlane, DrmPlaneType, GemBuffer,
};
pub use sovereign_graphics_display_pr_suite::*;
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
