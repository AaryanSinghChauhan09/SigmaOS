// SigmaOS Desktop Module
pub mod zenith_compositor;

pub use zenith_compositor::{
    DamageRegion, InputEvent, InputEventData, InputEventType, Output, Surface, SurfaceType,
    WindowGeometry, WindowState, ZenithCompositor, ZenithWindow,
};
