// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Wayland DRM Display Hotplug & Multi-Monitor Layout Engine
//!
//! Inspired by Omarchy's automatic multi-monitor discovery and Arch Linux's
//! kanshi / wlr-randr dynamic monitor layout configurations.
//! Automatically assigns monitor scaling (HiDPI vs standard), positions displays
//! horizontally or vertically, and manages dynamic workspace output binding.



/// Monitor position relative to primary display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitorPosition {
    LeftOf,
    RightOf,
    Above,
    Below,
    Primary,
}

/// Output display descriptor
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorOutput {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub scale: f32,
    pub position: MonitorPosition,
    pub relative_to: Option<String>,
}

impl MonitorOutput {
    pub fn new_primary(name: &str, w: u32, h: u32, hz: u32) -> Self {
        let scale = if w >= 3840 || h >= 2160 { 2.0 } else { 1.0 };
        Self {
            name: name.to_string(),
            width: w,
            height: h,
            refresh_rate: hz,
            scale,
            position: MonitorPosition::Primary,
            relative_to: None,
        }
    }

    pub fn new_secondary(name: &str, w: u32, h: u32, hz: u32, pos: MonitorPosition, rel: &str) -> Self {
        let scale = if w >= 3840 || h >= 2160 { 2.0 } else { 1.0 };
        Self {
            name: name.to_string(),
            width: w,
            height: h,
            refresh_rate: hz,
            scale,
            position: pos,
            relative_to: Some(rel.to_string()),
        }
    }

    /// Render Hyprland monitor directive
    pub fn to_hyprland_directive(&self, x_offset: i32, y_offset: i32) -> String {
        format!(
            "monitor = {}, {}x{}@{}, {}x{}, {:.2}",
            self.name, self.width, self.height, self.refresh_rate, x_offset, y_offset, self.scale
        )
    }
}

/// Multi-monitor arrangement manager
#[derive(Debug, Default)]
pub struct DisplayArrangement {
    pub monitors: Vec<MonitorOutput>,
}

impl DisplayArrangement {
    pub fn new() -> Self {
        Self { monitors: Vec::new() }
    }

    pub fn add_monitor(&mut self, mon: MonitorOutput) {
        self.monitors.push(mon);
    }

    /// Calculate offsets and generate complete Hyprland monitor block
    pub fn generate_hyprland_monitors(&self) -> String {
        let mut out = String::new();
        let mut current_x = 0;

        for mon in &self.monitors {
            out.push_str(&mon.to_hyprland_directive(current_x, 0));
            out.push('\n');
            current_x += (mon.width as f32 / mon.scale) as i32;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_hidpi_auto_scaling_aaa() {
        let mon = MonitorOutput::new_primary("DP-1", 3840, 2160, 144);
        assert_eq!(mon.scale, 2.0);
        assert_eq!(mon.position, MonitorPosition::Primary);
    }

    #[test]
    fn test_standard_display_scaling_aaa() {
        let mon = MonitorOutput::new_primary("eDP-1", 1920, 1080, 60);
        assert_eq!(mon.scale, 1.0);
    }

    #[test]
    fn test_multi_monitor_offset_calculation_aaa() {
        let mut arrangement = DisplayArrangement::new();
        arrangement.add_monitor(MonitorOutput::new_primary("DP-1", 1920, 1080, 144));
        arrangement.add_monitor(MonitorOutput::new_secondary("HDMI-A-1", 1920, 1080, 60, MonitorPosition::RightOf, "DP-1"));

        let conf = arrangement.generate_hyprland_monitors();
        assert!(conf.contains("monitor = DP-1, 1920x1080@144, 0x0, 1.00"));
        assert!(conf.contains("monitor = HDMI-A-1, 1920x1080@60, 1920x0, 1.00"));
    }
}
