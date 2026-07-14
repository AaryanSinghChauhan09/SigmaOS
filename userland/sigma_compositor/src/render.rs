use std::collections::HashSet;

/// Tracks regions of the screen that need to be redrawn (damage tracking).
/// Absorbs Wayland/X11 damage concepts.
pub struct DamageTracker {
    damaged_surfaces: HashSet<u64>,
}

impl Default for DamageTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl DamageTracker {
    pub fn new() -> Self {
        Self {
            damaged_surfaces: HashSet::new(),
        }
    }

    /// Mark a surface as damaged, requiring a redraw.
    pub fn damage(&mut self, surface_id: u64) {
        self.damaged_surfaces.insert(surface_id);
    }

    /// Retrieve the set of damaged surfaces and clear the tracking state.
    pub fn take_damage(&mut self) -> HashSet<u64> {
        let current = self.damaged_surfaces.clone();
        self.damaged_surfaces.clear();
        current
    }
}

/// Abstraction for swapping rendered buffers to the display.
pub struct RenderBackend;

impl RenderBackend {
    pub fn swap_buffers(&self) -> Result<(), String> {
        // In a real implementation, this would perform a page flip via DRM/KMS.
        // println!("Flipping buffers to display...");
        Ok(())
    }
}
