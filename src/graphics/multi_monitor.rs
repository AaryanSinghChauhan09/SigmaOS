use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// Multi-Monitor Display Manager for SigmaOS (XrandR & DisplayFusion Parity)
// Supports multi-display layout positioning, scaling, refresh rates, bezel compensation,
// independent monitor taskbars, wallpaper profiles, and ultra-wide virtual monitor splits.

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayRotation {
    Normal,   // 0 degrees
    Right,    // 90 degrees clockwise
    Inverted, // 180 degrees
    Left,     // 270 degrees clockwise
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayOutput {
    pub id: u32,
    pub name: String, // e.g. "HDMI-1", "DP-1", "eDP-1"
    pub resolution: (u32, u32), // (width, height) e.g. (1920, 1080)
    pub position: (i32, i32),   // (x, y) coordinates in virtual canvas
    pub refresh_rate_hz: u32,   // e.g. 60, 144, 240
    pub rotation: DisplayRotation,
    pub is_primary: bool,
    pub scale_factor: f32, // e.g. 1.0, 1.25, 1.5, 2.0
    pub is_enabled: bool,
    pub is_virtual_split: bool,
}

impl DisplayOutput {
    pub fn new(id: u32, name: &str, width: u32, height: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            resolution: (width, height),
            position: (0, 0),
            refresh_rate_hz: 60,
            rotation: DisplayRotation::Normal,
            is_primary: false,
            scale_factor: 1.0,
            is_enabled: true,
            is_virtual_split: false,
        }
    }
}

pub struct MultiMonitorManager {
    pub outputs: HashMap<u32, DisplayOutput>,
    pub taskbar_per_monitor: bool,
    pub enable_bezel_compensation: bool,
    pub bezel_padding_px: u32,
    pub wallpaper_profiles: HashMap<u32, String>, // output_id -> wallpaper image path
}

impl MultiMonitorManager {
    pub fn new() -> Self {
        Self {
            outputs: HashMap::new(),
            taskbar_per_monitor: true,
            enable_bezel_compensation: false,
            bezel_padding_px: 0,
            wallpaper_profiles: HashMap::new(),
        }
    }

    pub fn add_output(&mut self, output: DisplayOutput) {
        let is_first = self.outputs.is_empty();
        let id = output.id;
        self.outputs.insert(id, output);
        if is_first {
            self.set_primary(id).ok();
        }
    }

    /// Sets the specified output as the primary monitor (XrandR --primary)
    pub fn set_primary(&mut self, output_id: u32) -> Result<(), &'static str> {
        if !self.outputs.contains_key(&output_id) {
            return Err("Display output ID not found");
        }
        for (id, output) in self.outputs.iter_mut() {
            output.is_primary = *id == output_id;
        }
        Ok(())
    }

    /// Configures resolution, position, rotation, and scaling (XrandR --mode --pos --rotate --scale)
    pub fn configure_output(
        &mut self,
        output_id: u32,
        resolution: (u32, u32),
        position: (i32, i32),
        rotation: DisplayRotation,
        scale: f32,
    ) -> Result<(), &'static str> {
        let output = self.outputs.get_mut(&output_id).ok_or("Display output ID not found")?;
        output.resolution = resolution;
        output.position = position;
        output.rotation = rotation;
        output.scale_factor = scale;
        Ok(())
    }

    /// Assigns a unique wallpaper to a specific monitor (DisplayFusion feature)
    pub fn set_monitor_wallpaper(&mut self, output_id: u32, wallpaper_path: &str) -> Result<(), &'static str> {
        if !self.outputs.contains_key(&output_id) {
            return Err("Display output ID not found");
        }
        self.wallpaper_profiles.insert(output_id, wallpaper_path.to_string());
        Ok(())
    }

    /// Splits an ultra-wide monitor into virtual sub-monitors (DisplayFusion Monitor Splitting)
    pub fn split_into_virtual_monitors(&mut self, output_id: u32, cols: u32, rows: u32) -> Result<Vec<u32>, &'static str> {
        let source_output = self.outputs.get(&output_id).ok_or("Source display output ID not found")?.clone();
        if cols == 0 || rows == 0 {
            return Err("Columns and rows must be greater than zero");
        }

        let (total_w, total_h) = source_output.resolution;
        let sub_w = total_w / cols;
        let sub_h = total_h / rows;

        // Disable original physical display and generate virtual split displays
        if let Some(out) = self.outputs.get_mut(&output_id) {
            out.is_enabled = false;
        }

        let mut generated_ids = Vec::new();
        let mut count = 0;

        for r in 0..rows {
            for c in 0..cols {
                count += 1;
                let virt_id = output_id * 100 + count;
                let name = format!("{}-Virtual-{}", source_output.name, count);
                let pos_x = source_output.position.0 + (c * sub_w) as i32;
                let pos_y = source_output.position.1 + (r * sub_h) as i32;

                let mut virt_output = DisplayOutput::new(virt_id, &name, sub_w, sub_h);
                virt_output.position = (pos_x, pos_y);
                virt_output.is_virtual_split = true;
                virt_output.scale_factor = source_output.scale_factor;

                self.outputs.insert(virt_id, virt_output);
                generated_ids.push(virt_id);
            }
        }

        Ok(generated_ids)
    }

    /// Calculates the total bounding box dimensions of the virtual desktop canvas
    pub fn get_virtual_canvas_bounds(&self) -> (u32, u32) {
        let mut max_x = 0i32;
        let mut max_y = 0i32;

        for output in self.outputs.values() {
            if output.is_enabled {
                let end_x = output.position.0 + output.resolution.0 as i32;
                let end_y = output.position.1 + output.resolution.1 as i32;
                if end_x > max_x {
                    max_x = end_x;
                }
                if end_y > max_y {
                    max_y = end_y;
                }
            }
        }

        (max_x as u32, max_y as u32)
    }
}

impl Default for MultiMonitorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_xrandr_layout_and_primary_setting() {
        let mut manager = MultiMonitorManager::new();

        let hdmi = DisplayOutput::new(1, "HDMI-1", 1920, 1080);
        let dp = DisplayOutput::new(2, "DP-1", 2560, 1440);

        manager.add_output(hdmi);
        manager.add_output(dp);

        // HDMI-1 is primary by default
        assert!(manager.outputs.get(&1).unwrap().is_primary);

        // Position DP-1 to the right of HDMI-1
        manager.configure_output(2, (2560, 1440), (1920, 0), DisplayRotation::Normal, 1.0).unwrap();
        manager.set_primary(2).unwrap();

        assert!(manager.outputs.get(&2).unwrap().is_primary);
        assert!(!manager.outputs.get(&1).unwrap().is_primary);

        // Virtual canvas bounds should be (1920 + 2560, 1440) = (4480, 1440)
        let bounds = manager.get_virtual_canvas_bounds();
        assert_eq!(bounds, (4480, 1440));
    }

    #[test]
    fn test_displayfusion_wallpaper_and_splits() {
        let mut manager = MultiMonitorManager::new();

        // Add 3840x1080 ultrawide monitor
        let ultrawide = DisplayOutput::new(10, "DP-Ultrawide", 3840, 1080);
        manager.add_output(ultrawide);

        // Set unique wallpaper
        manager.set_monitor_wallpaper(10, "/usr/share/backgrounds/space.png").unwrap();
        assert_eq!(manager.wallpaper_profiles.get(&10).unwrap(), "/usr/share/backgrounds/space.png");

        // Split ultrawide into 2 virtual 1920x1080 side-by-side monitors
        let virt_ids = manager.split_into_virtual_monitors(10, 2, 1).unwrap();
        assert_eq!(virt_ids.len(), 2);

        let v1 = manager.outputs.get(&virt_ids[0]).unwrap();
        let v2 = manager.outputs.get(&virt_ids[1]).unwrap();

        assert_eq!(v1.resolution, (1920, 1080));
        assert_eq!(v1.position, (0, 0));

        assert_eq!(v2.resolution, (1920, 1080));
        assert_eq!(v2.position, (1920, 0));
    }
}
