// SigmaOS Graphics Module
// Image processing, rendering, and graphics operations

pub mod compositor;
pub mod gpu_driver;
pub mod image_decoder;

pub use compositor::{
    CompositorError, CompositorResult, CompositorStrategy, FramebufferCompositor,
    LayerBlendMode, RenderLayer, SigmaCompositor,
};
pub use image_decoder::{ColorSpace, DecodedImage, ImageDecoder, ImageFormat, ImageMetadata};
pub use gpu_driver::{Framebuffer, GpuDevice, GpuDriver, GpuState, GpuVendor, PixelFormat};
