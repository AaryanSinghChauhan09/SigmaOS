<<<<<<< HEAD
// SigmaOS Graphics Module
// Image processing, rendering, and graphics operations

pub mod compositor;
pub mod gpu_driver;
pub mod image_decoder;
pub mod raytracer;
pub mod vector_engine;
pub mod video_timeline;
pub mod zenith;
pub mod zenith_compositor;

pub use raytracer::{Ray, Sphere, Vec3};
pub use vector_engine::{PathCommand, Point2D, VectorPath};
pub use video_timeline::{VideoClip, VideoTimeline, VideoTrack};

pub use compositor::{
    CompositorError, CompositorResult, CompositorStrategy, FramebufferCompositor, LayerBlendMode,
    RenderLayer, SigmaCompositor,
};
pub use gpu_driver::{Framebuffer, GpuDevice, GpuDriver, GpuState, GpuVendor, PixelFormat};
pub use image_decoder::{ColorSpace, DecodedImage, ImageDecoder, ImageFormat, ImageMetadata};
pub use zenith::{
    Animation, AnimationCurve, CompositorError as ZenithError, HighContrastMode, LayoutStyle,
    Magnifier, Panel, PanelOrientation, ScreenReader, Widget, ZenithCompositor,
};
pub use zenith_compositor::{
    Geometry, WindowNode, WindowState, ZenithCompositor as WaylandZenithCompositor, SCREEN_HEIGHT,
    SCREEN_WIDTH,
=======
// Core Graphics and Composition Modules for SigmaOS
pub mod compositor;

pub use compositor::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window,
>>>>>>> origin/jules-15532892492441614180-73ce6847
};
