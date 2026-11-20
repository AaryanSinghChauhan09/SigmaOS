//! SigmaOS Terminal Text Effects
//!
//! Terminal text animation effects library for SigmaOS. Renders rich text
//! animations and effects in the terminal.
//!
//! # Modules
//!
//! - [`effects`]: Built-in effect definitions (matrix, rain, waves, etc.)
//!   and the trait used to author new effects.
//! - [`renderer`]: Terminal rendering abstraction that drives the animation
//!   loop, cursor control, and color output.

pub mod effects;
pub mod renderer;

/// Initialize the terminal text effects subsystem.
///
/// Call once at startup to probe terminal capabilities (color depth, size,
//! resize support) and prepare the renderer.
pub fn init() {
    // TODO: probe terminal capabilities, initialize renderer
}
