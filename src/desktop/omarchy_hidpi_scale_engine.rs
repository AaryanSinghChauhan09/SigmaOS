// SPDX-License-Identifier: MIT
// SigmaOS — Omarchy 1Password HiDPI Scale Engine
// Inspired by Omarchy branch: 1password-scale-factor
// Zero external dependencies, Safe Rust

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

#[cfg(any(feature = "standalone_test", test))]
use std::{string::String, format};
#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{string::String, format};

/// HiDPI scale profiles for 1Password and Wayland apps
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HiDpiScale {
    /// 1x (96 DPI)
    Scale1x,
    /// 1.5x (144 DPI)
    Scale1_5x,
    /// 2x (192 DPI, standard 4K)
    Scale2x,
    /// 2.5x (240 DPI)
    Scale2_5x,
    /// 3x (288 DPI, high-end mobile/tablet displays)
    Scale3x,
    /// Custom fractional scale (numerator/denominator)
    Custom(u32, u32),
}

impl HiDpiScale {
    /// Returns the scaling factor as a float32-compatible pair (num, den)
    pub fn factor_frac(&self) -> (u32, u32) {
        match self {
            HiDpiScale::Scale1x => (1, 1),
            HiDpiScale::Scale1_5x => (3, 2),
            HiDpiScale::Scale2x => (2, 1),
            HiDpiScale::Scale2_5x => (5, 2),
            HiDpiScale::Scale3x => (3, 1),
            HiDpiScale::Custom(n, d) => (*n, *d),
        }
    }
}

/// Electron/XWayland window that needs explicit scale patching
#[derive(Debug, Clone)]
pub struct ScaleTarget {
    pub app_id: String,
    pub env_var_name: String,
    pub env_var_value: String,
    pub xwayland_dpi_override: u32,
}

/// SigmaOS HiDPI Scale Engine — surpasses Omarchy's 1password-scale-factor
pub struct OmarchyHiDpiScaleEngine {
    global_scale: HiDpiScale,
    targets: [Option<ScaleTarget>; 16],
    target_count: usize,
}

impl OmarchyHiDpiScaleEngine {
    pub fn new() -> Self {
        Self {
            global_scale: HiDpiScale::Scale2x,
            targets: [
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
            ],
            target_count: 0,
        }
    }

    /// Set the global Wayland scale factor
    pub fn set_global_scale(&mut self, scale: HiDpiScale) {
        self.global_scale = scale;
    }

    /// Register an app that requires explicit scale patching
    pub fn register_scale_target(&mut self, app_id: &str, xdpi: u32) -> bool {
        if self.target_count >= 16 {
            return false;
        }
        let (n, d) = self.global_scale.factor_frac();
        let val = format!("{}", (n * 100) / d);
        self.targets[self.target_count] = Some(ScaleTarget {
            app_id: String::from(app_id),
            env_var_name: String::from("GDK_SCALE"),
            env_var_value: val,
            xwayland_dpi_override: xdpi,
        });
        self.target_count += 1;
        true
    }

    /// Generate Hyprland `env` declarations for all registered apps
    pub fn generate_hyprland_env_block(&self) -> String {
        let mut out = String::from("# SigmaOS HiDPI Scale — auto-generated\n");
        let (n, d) = self.global_scale.factor_frac();
        out.push_str(&format!("monitor = ,preferred,auto,{}\n", (n * 10) / d));
        for i in 0..self.target_count {
            if let Some(ref t) = self.targets[i] {
                out.push_str(&format!(
                    "env = {},{}   # {}\n",
                    t.env_var_name, t.env_var_value, t.app_id
                ));
            }
        }
        out
    }

    /// Get the XWayland DPI that should be set via xrandr for a given app_id
    pub fn xwayland_dpi_for(&self, app_id: &str) -> Option<u32> {
        for i in 0..self.target_count {
            if let Some(ref t) = self.targets[i] {
                if t.app_id == app_id {
                    return Some(t.xwayland_dpi_override);
                }
            }
        }
        None
    }

    /// 1Password-specific: returns the GDK_SCALE env needed for crisp rendering
    pub fn onepassword_env_scale(&self) -> (String, String) {
        let (n, d) = self.global_scale.factor_frac();
        let scale_str = if n % d == 0 {
            format!("{}", n / d)
        } else {
            // fractional: use GDK_DPI_SCALE instead
            format!("{}", (n * 96) / d)
        };
        (String::from("GDK_SCALE"), scale_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hidpi_scale_engine() {
        let mut engine = OmarchyHiDpiScaleEngine::new();

        // Default scale is 2x
        assert_eq!(engine.global_scale.factor_frac(), (2, 1));

        // Register 1Password
        assert!(engine.register_scale_target("1password", 192));
        assert_eq!(engine.xwayland_dpi_for("1password"), Some(192));
        assert_eq!(engine.xwayland_dpi_for("nonexistent"), None);

        // Switch to 1.5x
        engine.set_global_scale(HiDpiScale::Scale1_5x);
        let (k, v) = engine.onepassword_env_scale();
        assert_eq!(k, "GDK_SCALE");
        // 3/2 → fractional → 3*96/2 = 144
        assert_eq!(v, "144");

        // Custom scale
        engine.set_global_scale(HiDpiScale::Custom(5, 4));
        assert_eq!(engine.global_scale.factor_frac(), (5, 4));

        // Hyprland env block
        let block = engine.generate_hyprland_env_block();
        assert!(block.contains("monitor"));
        assert!(block.contains("GDK_SCALE"));
    }
}
