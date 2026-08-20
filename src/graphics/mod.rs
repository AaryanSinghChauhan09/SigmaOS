// Core Graphics and Composition Modules for SigmaOS
pub mod compositor;
pub mod video_editor;
pub mod paint;
pub mod render3d;
pub mod video;

pub use video::{PixelRgba, VideoFrame, VideoEffect, VideoError, TimelineClip};
pub use compositor::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window,
};
pub use video_editor::{VideoClip, VideoEffect as EditorVideoEffect, VideoTimeline, VideoTrack};
pub use paint::ColorRgba;
