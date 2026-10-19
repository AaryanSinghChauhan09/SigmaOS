//! Graphics types following SigmaOS principles
//! 
//! Following principles: COMPOSITION_OVER_INHERITANCE, POLYMORPHISM,
//! DISCOVERABILITY, and CLEAR_ABSTRACTION.

/// Graphics adapter capabilities
#[derive(Debug, Clone)]
pub struct GraphicsCapabilities {
    /// Maximum texture size
    pub max_texture_size: u32,
    /// Maximum render target size
    pub max_render_target_size: (u32, u32),
    /// Supported formats
    pub supported_formats: Vec<GraphicsFormat>,
    /// GPU vendor
    pub vendor: String,
    /// GPU driver version
    pub driver_version: String,
}

/// Graphics format
#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum GraphicsFormat {
    /// RGBA 8-bit
    Rgba8,
    /// BGRA 8-bit
    Bgra8,
    /// Depth 24-stencil 8
    Depth24Stencil8,
    /// R11F_G11F_B10F
    R11F_G11F_B10F,
    /// RGBA16F
    Rgba16F,
    /// RGBA32F
    Rgba32F,
}

impl fmt::Display for GraphicsFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicsFormat::Rgba8 => write!(f, "Rgba8"),
            GraphicsFormat::Bgra8 => write!(f, "Bgra8"),
            GraphicsFormat::Depth24Stencil8 => write!(f, "Depth24Stencil8"),
            GraphicsFormat::R11F_G11F_B10F => write!(f, "R11F_G11F_B10F"),
            GraphicsFormat::Rgba16F => write!(f, "Rgba16F"),
            GraphicsFormat::Rgba32F => write!(f, "Rgba32F"),
        }
    }
}

/// Graphics adapter information
#[derive(Debug, Clone)]
pub struct GraphicsAdapter {
    /// Adapter name
    pub name: String,
    /// Capabilities
    pub capabilities: GraphicsCapabilities,
    /// Vendor string
    pub vendor: String,
    /// Driver version
    pub driver_version: String,
    /// Total video memory in MB
    pub video_memory_mb: u64,
}

/// Graphics framebuffer configuration
#[derive(Debug, Clone)]
pub struct FramebufferConfig {
    /// Framebuffer width
    pub width: u32,
    /// Framebuffer height
    pub height: u32,
    /// Bits per pixel
    pub bits_per_pixel: u32,
    /// Bytes per row
    pub bytes_per_row: u32,
    /// Framebuffer base address
    pub base_address: u64,
}

/// Present mode for framebuffer swapping
#[derive(Debug, Clone, PartialEq)]
pub enum PresentMode {
    /// Immediate swap
    Immediate,
    /// FIFO swap
    Fifo,
    /// Mailbox swap
    Mailbox,
    /// Copies swap
    Copies,
}

/// Swap interval hint
pub type SwapInterval = u32;
