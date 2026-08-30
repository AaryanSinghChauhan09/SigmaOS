// Core Graphics and Composition Modules for SigmaOS
pub mod compositor;
pub mod video;
pub mod video_editor;
pub mod advanced_accel;
pub mod vector_engine;
pub mod country_flags_svg;

pub use video::{PixelRgba, VideoFrame};

pub use compositor::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window,
};
pub use video_editor::{VideoClip, VideoEffect, VideoTimeline, VideoTrack};
pub use advanced_accel::{GpuDevice as AccelGpuDevice, GraphicsBackendApi, GraphicsManager as AccelGraphicsManager, RenderPipeline};

pub use country_flags_svg::{
    CountryFlagMetadata, DistroEmblemKind, DistroEmblemMetadata, FlagRenderMode,
    IsoCountryFlagsSvgCollection, SystemFeatureFlagMetadata, WorldRegion,
};
