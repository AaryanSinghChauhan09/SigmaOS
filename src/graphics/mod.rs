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

pub use raytracer::{Ray, Sphere, Vec3};
pub use vector_engine::{PathCommand, Point2D, VectorPath};
pub use multi_monitor::{DisplayOutput, DisplayRotation, MultiMonitorManager};
pub use video_timeline::{VideoClip, VideoTimeline, VideoTrack};

pub use compositor::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window, DisplayServerProtocol, SovereignWaylandCompositor,
};
