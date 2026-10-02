//! Zenith Desktop Configuration — Sovereign settings engine

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorBackend {
    Drm,
    Softbuffer,
    Vulkan,
}
impl Default for CompositorBackend {
    fn default() -> Self {
        Self::Drm
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputScale {
    X1,
    X1_5,
    X2,
    X3,
}
impl Default for OutputScale {
    fn default() -> Self {
        Self::X1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    SigmaDark,
    SigmaLight,
    HighContrast,
    SolarizedDark,
}
impl Default for Theme {
    fn default() -> Self {
        Self::SigmaDark
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAcceleration {
    Flat,
    Adaptive,
    Custom,
}
impl Default for MouseAcceleration {
    fn default() -> Self {
        Self::Adaptive
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompositorConfig {
    pub backend: CompositorBackend,
    pub vsync: bool,
    pub triple_buffering: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InputConfig {
    pub mouse_acceleration: MouseAcceleration,
    pub natural_scroll: bool,
    pub tap_to_click: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AppearanceConfig {
    pub theme: Theme,
    pub font_size: u32,
    pub icon_size: u32,
}

#[derive(Debug, Clone, Default)]
pub struct ZenithConfig {
    pub compositor: CompositorConfig,
    pub input: InputConfig,
    pub appearance: AppearanceConfig,
    pub output_scale: OutputScale,
}

impl ZenithConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
