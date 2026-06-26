#![no_std] // Enforce sovereign memory model

//! =========================================================================
//! Σ ZENITH NATIVE RUST BINDINGS (PHASE 9)
//! =========================================================================
//! Safe Rust wrappers around the Zenith C++ Toolkit.
//! Runs in userland. Uses #[no_std] to align with SigmaOS's minimalist,
//! dependency-free architecture.
//! =========================================================================

// FFI bindings to the C++ core
extern "C" {
    fn zenith_theme_init();
    fn zenith_draw_button(
        buf: *mut u8, w: u32, x: i32, y: i32, bw: u32, bh: u32,
        label: *const u8, hover: bool
    );
    fn zenith_theme_set_metrics(r: u32, ig: u32, og: u32);
}

// ─── Safe Rust Abstractions ────────────────────────────────────────────────

pub struct ZenithContext {
    buffer: *mut u8,
    width: u32,
}

impl ZenithContext {
    pub fn new(buffer: *mut u8, width: u32) -> Self {
        unsafe { zenith_theme_init(); }
        Self { buffer, width }
    }

    pub fn set_metrics(&self, radius: u32, inner_gap: u32, outer_gap: u32) {
        unsafe { zenith_theme_set_metrics(radius, inner_gap, outer_gap); }
    }

    pub fn draw_button(&self, x: i32, y: i32, w: u32, h: u32, label: &str, is_hovered: bool) {
        // In a real #![no_std] environment we'd carefully handle string null-termination
        // without allocations. For this mockup, we assume ASCII/UTF-8 passing.
        unsafe {
            zenith_draw_button(
                self.buffer, self.width, x, y, w, h,
                label.as_ptr(), is_hovered
            );
        }
    }
}
