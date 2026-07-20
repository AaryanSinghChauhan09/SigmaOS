// SigmaOS Graphics Module
// Image processing, rendering, and graphics operations

pub mod compositor;
pub mod gpu_driver;
pub mod image_decoder;
pub mod zenith;
pub mod zenith_compositor;

pub use compositor::{
    CompositorError, CompositorResult, CompositorStrategy, FramebufferCompositor,
    LayerBlendMode, RenderLayer, SigmaCompositor,
};
pub use image_decoder::{ColorSpace, DecodedImage, ImageDecoder, ImageFormat, ImageMetadata};
pub use gpu_driver::{Framebuffer, GpuDevice, GpuDriver, GpuState, GpuVendor, PixelFormat};
pub use zenith::{
    Animation, AnimationCurve, CompositorError as ZenithError, HighContrastMode, LayoutStyle,
    Magnifier, Panel, PanelOrientation, ScreenReader, Widget, ZenithCompositor,
};
pub use zenith_compositor::{
    Geometry, WindowNode, WindowState, ZenithCompositor as WaylandZenithCompositor,
    SCREEN_WIDTH, SCREEN_HEIGHT,
};
