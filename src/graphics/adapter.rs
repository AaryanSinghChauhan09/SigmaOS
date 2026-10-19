//! Graphics adapter following SigmaOS principles
//! 
//! Implements graphics adapter initialization and management
//! following principles: COMPOSITION_OVER_INHERITANCE, POLYMORPHISM,
//! and ENCAPSULATION.

use crate::graphics::types::{GraphicsAdapter, GraphicsCapabilities, GraphicsFormat, FramebufferConfig, PresentMode};

/// Initialize graphics adapter
pub fn init_adapter() -> Result<GraphicsAdapter, String> {
    // In real implementation, this would probe hardware
    Ok(GraphicsAdapter {
        name: "SigmaOS Graphics".to_string(),
        capabilities: GraphicsCapabilities {
            max_texture_size: 8192,
            max_render_target_size: (4096, 4096),
            supported_formats: vec![
                GraphicsFormat::Rgba8,
                GraphicsFormat::Bgra8,
                GraphicsFormat::Depth24Stencil8,
            ],
            vendor: "SigmaOS".to_string(),
            driver_version: "0.1.0".to_string(),
        },
        vendor: "SigmaOS".to_string(),
        driver_version: "0.1.0".to_string(),
        video_memory_mb: 256,
    })
}

/// Get framebuffer configuration
pub fn get_framebuffer_config() -> Result<FramebufferConfig, String> {
    // In real implementation, this would probe hardware
    Ok(FramebufferConfig {
        width: 1920,
        height: 1080,
        bits_per_pixel: 32,
        bytes_per_row: 7680, // 1920 * 4 (RGBA8)
        base_address: 0, // Would be real framebuffer address
    })
}

/// List available graphics formats
pub fn list_formats() -> Vec<GraphicsFormat> {
    vec![
        GraphicsFormat::Rgba8,
        GraphicsFormat::Bgra8,
        GraphicsFormat::Depth24Stencil8,
    ]
}

/// Check if format is supported
pub fn is_format_supported(format: &GraphicsFormat, capabilities: &GraphicsCapabilities) -> bool {
    capabilities.supported_formats.contains(format)
}

/// Get recommended format for target
pub fn get_recommended_format(width: u32, height: u32, use_depth: bool) -> GraphicsFormat {
    if use_depth {
        GraphicsFormat::Depth24Stencil8
    } else {
        GraphicsFormat::Rgba8
    }
}

/// Present framebuffer to display
pub fn present(framebuffer_address: u64, width: u32, height: u32, mode: PresentMode) -> Result<(), String> {
    // In real implementation, this would program the display hardware
    Ok(())
}

/// Get present modes supported
pub fn get_present_modes() -> Vec<PresentMode> {
    vec![
        PresentMode::Immediate,
        PresentMode::Fifo,
        PresentMode::Mailbox,
    ]
}

/// Set swap interval (vsync)
pub fn set_swap_interval(interval: u32) -> Result<(), String> {
    // In real implementation, this would configure the display hardware
    Ok(())
}
