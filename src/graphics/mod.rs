pub mod gaming_layer;
pub use gaming_layer::*;

// Core Graphics and Composition Modules for SigmaOS
pub mod advanced_accel;
pub mod compositor;
pub mod nvidia_prime;
pub mod video;
pub mod video_editor;

pub use advanced_accel::{
    GpuDevice as AccelGpuDevice, GraphicsBackendApi, GraphicsManager as AccelGraphicsManager,
    RenderPipeline,
};
pub use compositor::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window,
};
pub use video_editor::{VideoClip, VideoEffect, VideoTimeline, VideoTrack};
