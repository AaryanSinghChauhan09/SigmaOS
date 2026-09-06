#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
pub mod gaming_layer;
pub use gaming_layer::*;

// Core Graphics and Composition Modules for SigmaOS
pub mod compositor;
pub mod video;
pub mod video_editor;
pub mod advanced_accel;
pub mod nvidia_prime;

pub use video::{PixelRgba, VideoFrame};
pub use nvidia_prime::{
    GpuPowerState, NvidiaPrimeEngine, NvidiaPrimeOffloadConfig, PrimeDmaBufShare, PrimeProfile,
};

pub use compositor::{
    BitmapSurface, Color, Compositor, Position, Rectangle, SimpleCompositor, SimpleWindow, Size,
    Surface, Window,
};
pub use video_editor::{VideoClip, VideoEffect, VideoTimeline, VideoTrack};
pub use advanced_accel::{GpuDevice as AccelGpuDevice, GraphicsBackendApi, GraphicsManager as AccelGraphicsManager, RenderPipeline};
