// Core Graphics and Composition Modules for SigmaOS
pub mod compositor;
pub mod video;
pub mod video_editor;

pub use video::{PixelRgba, VideoFrame};

pub use compositor::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window,
};
pub use video_editor::{VideoClip, VideoEffect, VideoTimeline, VideoTrack};
