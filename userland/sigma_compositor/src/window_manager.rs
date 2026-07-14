use crate::compositor::Surface;

pub struct WindowManager {
    pub tiling_enabled: bool,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        Self { tiling_enabled: true }
    }

    pub fn arrange_surfaces(&self, surfaces: &[Surface], screen_w: u32, screen_h: u32) -> Vec<(u64, i32, i32, u32, u32)> {
        let count = surfaces.len();
        if count == 0 {
            return Vec::new();
        }

        if self.tiling_enabled {
            // Dynamic tiling: split horizontally
            let width_per_window = screen_w / count as u32;
            surfaces
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let x = (i as u32 * width_per_window) as i32;
                    (s.id, x, 0, width_per_window, screen_h)
                })
                .collect()
        } else {
            // Floating layout (stacked at origin)
            surfaces.iter().map(|s| (s.id, 0, 0, s.width, s.height)).collect()
        }
    }
}
