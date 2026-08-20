// SigmaOS Graphics Module
// Image processing, rendering, and graphics operations

pub mod compositor;
pub mod gpu_driver;
pub mod image_decoder;
pub mod raytracer;
pub mod multi_monitor;
pub mod vector_engine;
pub mod video_timeline;
pub mod zenith;
pub mod zenith_compositor;
pub mod bsd_graphics;

pub use raytracer::{Ray, Sphere, Vec3};
pub use vector_engine::{PathCommand, Point2D, VectorPath};
pub use multi_monitor::{DisplayOutput, DisplayRotation, MultiMonitorManager};
pub use video_timeline::{VideoClip, VideoTimeline, VideoTrack};

pub use compositor::{
    AnimationType, BitmapSurface, Color, Compositor, CompositorCapability, CompositorStats,
    GraphicsError, PixelFormat as CompositorPixelFormat, Position, Rectangle, SimpleCompositor,
    SimpleWindow, Size, Surface, SurfaceCapability, SurfaceInfo, Window, WindowCapability,
    WindowInfo,
};
pub use gpu_driver::{
    Framebuffer, GpuDevice, GpuDriver, GpuState, GpuVendor, PixelFormat,
    DrmAtomicPlaneState, WaylandDmaBuf, OpenBsdWsdisplayVt,
};
pub use image_decoder::{ColorSpace, DecodedImage, ImageDecoder, ImageFormat, ImageMetadata};
pub use zenith::{
    Animation, AnimationCurve, CompositorError as ZenithError, HighContrastMode, LayoutStyle,
    Magnifier, Panel, PanelOrientation, ScreenReader, Widget, ZenithCompositor,
};
pub use zenith_compositor::{
    Geometry, WindowNode, WindowState, WaylandZenithCompositor, SCREEN_HEIGHT,
    SCREEN_WIDTH,
};
pub use bsd_graphics::{
    ConsoleDisplayMode, FreeBsdWsconsFbEngine, DrmDumbBuffer, OpenBsdDrmKmsSovereignShim,
    RenderCommand, DragonFlySmpGraphicsRing, SovereignWaylandFreeBsdCompositor,
};
