// SPDX-License-Identifier: MIT
// SigmaOS Omarchy Display, DPI Scaling & Multi-Monitor Subsystem Engine (`src/desktop/display_scaling.rs`)
// Inspired by Omarchy Linux (Retina 2x default, fractional 1.6x 4K, Super+/ scale stepper,
// omarchy display text size, extend/mirror toggle, laptop lid sensor, DDC/CI & Apple asdcontrol).

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Display Topology Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayTopologyMode {
    Extended,
    Mirrored,
    SingleInternal,
    SingleExternal,
}

/// Monitor Resolution & Scaling Profile
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorProfile {
    pub output_name: String,
    pub width: u32,
    pub height: u32,
    pub ppi: u32,
    pub gdk_scale: u32,
    pub monitor_scale: f32,
    pub is_apple_display: bool,
    pub is_disabled: bool,
}

impl MonitorProfile {
    pub fn new_retina_5k(output_name: &str) -> Self {
        Self {
            output_name: String::from(output_name),
            width: 5120,
            height: 2880,
            ppi: 218,
            gdk_scale: 2,
            monitor_scale: 2.0,
            is_apple_display: true,
            is_disabled: false,
        }
    }

    pub fn new_4k_fractional(output_name: &str) -> Self {
        Self {
            output_name: String::from(output_name),
            width: 3840,
            height: 2160,
            ppi: 140,
            gdk_scale: 2,
            monitor_scale: 1.6,
            is_apple_display: false,
            is_disabled: false,
        }
    }

    pub fn new_standard_hd(output_name: &str) -> Self {
        Self {
            output_name: String::from(output_name),
            width: 1920,
            height: 1080,
            ppi: 96,
            gdk_scale: 1,
            monitor_scale: 1.0,
            is_apple_display: false,
            is_disabled: false,
        }
    }
}

/// Omarchy Display, DPI Scaling & Multi-Monitor Subsystem Engine
pub struct OmarchyDisplayScalingEngine {
    pub monitors: Vec<MonitorProfile>,
    pub scaling_ratios: Vec<f32>,
    pub current_ratio_index: usize,
    pub current_text_size_px: u32, // Default 11px
    pub topology_mode: DisplayTopologyMode,
    pub is_laptop_lid_closed: bool,
    pub brightness_percent: u32,
}

impl OmarchyDisplayScalingEngine {
    pub fn new() -> Self {
        Self {
            monitors: vec![
                MonitorProfile::new_retina_5k("DP-1"),
                MonitorProfile::new_standard_hd("eDP-1"),
            ],
            scaling_ratios: vec![1.0, 1.25, 1.6, 2.0, 3.0, 4.0],
            current_ratio_index: 3, // Default 2.0x
            current_text_size_px: 11,
            topology_mode: DisplayTopologyMode::Extended,
            is_laptop_lid_closed: false,
            brightness_percent: 80,
        }
    }

    /// Step monitor scaling ratio higher (`Super + /`)
    pub fn step_scale_higher(&mut self) -> f32 {
        if self.current_ratio_index + 1 < self.scaling_ratios.len() {
            self.current_ratio_index += 1;
        }
        self.scaling_ratios[self.current_ratio_index]
    }

    /// Step monitor scaling ratio lower (`Super + Alt + /`)
    pub fn step_scale_lower(&mut self) -> f32 {
        if self.current_ratio_index > 0 {
            self.current_ratio_index -= 1;
        }
        self.scaling_ratios[self.current_ratio_index]
    }

    /// Execute `omarchy display text size <9-20>` CLI command
    pub fn execute_text_size_command(&mut self, args: &[&str]) -> Result<String, &'static str> {
        if args.is_empty() {
            return Ok(format!("Current text size: {}px", self.current_text_size_px));
        }

        match args[0] {
            "reset" => {
                self.current_text_size_px = 11;
                Ok("Text size reset to default (11px). Shell, GTK & terminals updated.".to_string())
            }
            val_str => {
                let px: u32 = val_str.parse().map_err(|_| "Invalid pixel size integer")?;
                if !(9..=20).contains(&px) {
                    return Err("Text size must be between 9 and 20 pixels");
                }
                self.current_text_size_px = px;
                Ok(format!("Text size set to {}px. Shell, GTK & terminals updated.", px))
            }
        }
    }

    /// Toggle display mirroring (`Super + Ctrl + Alt + Delete`)
    pub fn toggle_mirroring(&mut self) -> DisplayTopologyMode {
        if self.topology_mode == DisplayTopologyMode::Extended {
            self.topology_mode = DisplayTopologyMode::Mirrored;
        } else {
            self.topology_mode = DisplayTopologyMode::Extended;
        }
        self.topology_mode
    }

    /// Handle laptop lid open/close trigger (`Super + Ctrl + Delete`)
    pub fn handle_laptop_lid_sensor(&mut self, is_closed: bool) {
        self.is_laptop_lid_closed = is_closed;
        for mon in &mut self.monitors {
            if mon.output_name.starts_with("eDP") {
                mon.is_disabled = is_closed;
            }
        }
    }

    /// Adjust display brightness via DDC/CI or Apple `asdcontrol`
    pub fn adjust_brightness(&mut self, delta: i32, max_shift: bool) -> u32 {
        if max_shift {
            self.brightness_percent = if delta > 0 { 100 } else { 0 };
        } else {
            let new_val = self.brightness_percent as i32 + delta;
            self.brightness_percent = new_val.clamp(0, 100) as u32;
        }
        self.brightness_percent
    }

    /// Export Hyprland `~/.config/hypr/monitors.lua` configuration
    pub fn export_hyprland_monitors_lua(&self) -> String {
        let current_scale = self.scaling_ratios[self.current_ratio_index];
        let gdk_scale = current_scale.round() as u32;

        let mut lua = String::from("-- Omarchy Hyprland Auto-Generated Monitors Config\n");
        lua.push_str(&format!("local omarchy_gdk_scale = {}\n", gdk_scale));
        lua.push_str(&format!("local omarchy_monitor_scale = {:.1}\n\n", current_scale));

        for mon in &self.monitors {
            if mon.is_disabled {
                lua.push_str(&format!("hl.monitor({{ output = \"{}\", disabled = true }})\n", mon.output_name));
            } else {
                lua.push_str(&format!(
                    "hl.monitor({{ output = \"{}\", res = \"{}x{}\", scale = {} }})\n",
                    mon.output_name, mon.width, mon.height, mon.monitor_scale
                ));
            }
        }

        lua
    }
}

impl Default for OmarchyDisplayScalingEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_stepper_higher_and_lower() {
        let mut engine = OmarchyDisplayScalingEngine::new();
        assert_eq!(engine.scaling_ratios[engine.current_ratio_index], 2.0);

        // Step higher
        let new_scale = engine.step_scale_higher();
        assert_eq!(new_scale, 3.0);

        // Step lower twice
        engine.step_scale_lower();
        let min_scale = engine.step_scale_lower();
        assert_eq!(min_scale, 1.6);
    }

    #[test]
    fn test_text_size_command_execution() {
        let mut engine = OmarchyDisplayScalingEngine::new();
        assert_eq!(engine.current_text_size_px, 11);

        // Set text size to 14
        let res = engine.execute_text_size_command(&["14"]).unwrap();
        assert!(res.contains("14px"));
        assert_eq!(engine.current_text_size_px, 14);

        // Out of bounds test
        assert!(engine.execute_text_size_command(&["25"]).is_err());

        // Reset
        let reset_res = engine.execute_text_size_command(&["reset"]).unwrap();
        assert!(reset_res.contains("default"));
        assert_eq!(engine.current_text_size_px, 11);
    }

    #[test]
    fn test_laptop_lid_and_topology_toggle() {
        let mut engine = OmarchyDisplayScalingEngine::new();
        assert_eq!(engine.topology_mode, DisplayTopologyMode::Extended);

        assert_eq!(engine.toggle_mirroring(), DisplayTopologyMode::Mirrored);

        // Close lid
        engine.handle_laptop_lid_sensor(true);
        assert!(engine.is_laptop_lid_closed);
        assert!(engine.monitors.iter().find(|m| m.output_name == "eDP-1").unwrap().is_disabled);
    }

    #[test]
    fn test_brightness_and_lua_export() {
        let mut engine = OmarchyDisplayScalingEngine::new();
        assert_eq!(engine.brightness_percent, 80);

        // Shift + BrightnessUp max
        assert_eq!(engine.adjust_brightness(10, true), 100);

        let lua = engine.export_hyprland_monitors_lua();
        assert!(lua.contains("omarchy_gdk_scale = 2"));
        assert!(lua.contains("DP-1"));
    }
}
