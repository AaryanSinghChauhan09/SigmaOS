// Overlay Filesystem (Linux-inspired)
// Provides union filesystem with layer merging

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Overlay layer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayLayerType {
    Lower,  // Read-only base layer
    Upper,  // Read-write layer for modifications
    Work,   // Work directory for overlay operations
}

/// Overlay layer
#[derive(Debug, Clone)]
pub struct OverlayLayer {
    pub id: u64,
    pub layer_type: OverlayLayerType,
    pub path: String,
    pub read_only: bool,
}

impl OverlayLayer {
    pub fn new(id: u64, layer_type: OverlayLayerType, path: String) -> Self {
        let read_only = match layer_type {
            OverlayLayerType::Lower => true,
            OverlayLayerType::Upper => false,
            OverlayLayerType::Work => false,
        };

        Self {
            id,
            layer_type,
            path,
            read_only,
        }
    }

    pub fn is_lower(&self) -> bool {
        self.layer_type == OverlayLayerType::Lower
    }

    pub fn is_upper(&self) -> bool {
        self.layer_type == OverlayLayerType::Upper
    }

    pub fn is_work(&self) -> bool {
        self.layer_type == OverlayLayerType::Work
    }
}

/// Overlay mount configuration
#[derive(Debug, Clone)]
pub struct OverlayConfig {
    pub name: String,
    pub lower_dirs: Vec<String>,
    pub upper_dir: Option<String>,
    pub work_dir: Option<String>,
}

impl OverlayConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            lower_dirs: Vec::new(),
            upper_dir: None,
            work_dir: None,
        }
    }

    pub fn with_lower(mut self, path: String) -> Self {
        self.lower_dirs.push(path);
        self
    }

    pub fn with_upper(mut self, path: String) -> Self {
        self.upper_dir = Some(path);
        self
    }

    pub fn with_work(mut self, path: String) -> Self {
        self.work_dir = Some(path);
        self
    }
}

/// Overlay filesystem
#[derive(Debug, Clone)]
pub struct OverlayFilesystem {
    pub id: u64,
    pub config: OverlayConfig,
    pub layers: HashMap<u64, OverlayLayer>,
    pub next_layer_id: u64,
    pub mounted: bool,
}

impl OverlayFilesystem {
    pub fn new(id: u64, config: OverlayConfig) -> Self {
        Self {
            id,
            config,
            layers: HashMap::new(),
            next_layer_id: 1,
            mounted: false,
        }
    }

    /// Mount the overlay filesystem
    pub fn mount(&mut self) -> Result<(), String> {
        if self.mounted {
            return Err("Overlay already mounted".to_string());
        }

        // Create layers from config
        for lower_path in &self.config.lower_dirs {
            let layer = OverlayLayer::new(self.next_layer_id, OverlayLayerType::Lower, lower_path.clone());
            self.layers.insert(self.next_layer_id, layer);
            self.next_layer_id += 1;
        }

        if let Some(ref upper_path) = self.config.upper_dir {
            let layer = OverlayLayer::new(self.next_layer_id, OverlayLayerType::Upper, upper_path.clone());
            self.layers.insert(self.next_layer_id, layer);
            self.next_layer_id += 1;
        }

        if let Some(ref work_path) = self.config.work_dir {
            let layer = OverlayLayer::new(self.next_layer_id, OverlayLayerType::Work, work_path.clone());
            self.layers.insert(self.next_layer_id, layer);
            self.next_layer_id += 1;
        }

        self.mounted = true;
        Ok(())
    }

    /// Unmount the overlay filesystem
    pub fn unmount(&mut self) -> Result<(), String> {
        if !self.mounted {
            return Err("Overlay not mounted".to_string());
        }
        self.mounted = false;
        self.layers.clear();
        Ok(())
    }

    /// Get a layer by ID
    pub fn get_layer(&self, layer_id: u64) -> Option<&OverlayLayer> {
        self.layers.get(&layer_id)
    }

    /// List all layers
    pub fn list_layers(&self) -> Vec<&OverlayLayer> {
        self.layers.values().collect()
    }

    /// Get lower layers
    pub fn lower_layers(&self) -> Vec<&OverlayLayer> {
        self.layers.values().filter(|l| l.is_lower()).collect()
    }

    /// Get upper layer
    pub fn upper_layer(&self) -> Option<&OverlayLayer> {
        self.layers.values().find(|l| l.is_upper())
    }

    /// Get work layer
    pub fn work_layer(&self) -> Option<&OverlayLayer> {
        self.layers.values().find(|l| l.is_work())
    }

    /// Check if mounted
    pub fn is_mounted(&self) -> bool {
        self.mounted
    }

    /// Get layer count
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

/// Overlay manager for system-wide overlay management
pub struct OverlayManager {
    overlays: Arc<Mutex<HashMap<u64, OverlayFilesystem>>>,
    next_overlay_id: Arc<Mutex<u64>>,
}

impl OverlayManager {
    pub fn new() -> Self {
        Self {
            overlays: Arc::new(Mutex::new(HashMap::new())),
            next_overlay_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new overlay filesystem
    pub fn create_overlay(&self, config: OverlayConfig) -> u64 {
        let mut next_id = self.next_overlay_id.lock().unwrap();
        let overlay_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let overlay = OverlayFilesystem::new(overlay_id, config);
        let mut overlays = self.overlays.lock().unwrap();
        overlays.insert(overlay_id, overlay);

        overlay_id
    }

    /// Get an overlay by ID
    pub fn get_overlay(&self, overlay_id: u64) -> Option<OverlayFilesystem> {
        let overlays = self.overlays.lock().unwrap();
        overlays.get(&overlay_id).cloned()
    }

    /// Remove an overlay
    pub fn remove_overlay(&self, overlay_id: u64) -> Result<(), String> {
        let mut overlays = self.overlays.lock().unwrap();
        match overlays.remove(&overlay_id) {
            Some(overlay) => {
                if overlay.mounted {
                    return Err("Cannot remove mounted overlay".to_string());
                }
                Ok(())
            }
            None => Err(format!("Overlay {} not found", overlay_id)),
        }
    }

    /// Mount an overlay
    pub fn mount(&self, overlay_id: u64) -> Result<(), String> {
        let mut overlays = self.overlays.lock().unwrap();
        match overlays.get_mut(&overlay_id) {
            Some(overlay) => overlay.mount(),
            None => Err(format!("Overlay {} not found", overlay_id)),
        }
    }

    /// Unmount an overlay
    pub fn unmount(&self, overlay_id: u64) -> Result<(), String> {
        let mut overlays = self.overlays.lock().unwrap();
        match overlays.get_mut(&overlay_id) {
            Some(overlay) => overlay.unmount(),
            None => Err(format!("Overlay {} not found", overlay_id)),
        }
    }

    /// Get number of mounted overlays
    pub fn mounted_count(&self) -> usize {
        let overlays = self.overlays.lock().unwrap();
        overlays.values().filter(|o| o.is_mounted()).count()
    }

    /// Get total overlay count
    pub fn overlay_count(&self) -> usize {
        let overlays = self.overlays.lock().unwrap();
        overlays.len()
    }
}

impl Default for OverlayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay_layer_creation() {
        let layer = OverlayLayer::new(1, OverlayLayerType::Lower, "/lower".to_string());
        assert_eq!(layer.id, 1);
        assert!(layer.is_lower());
        assert!(layer.read_only);
    }

    #[test]
    fn test_overlay_layer_upper() {
        let layer = OverlayLayer::new(2, OverlayLayerType::Upper, "/upper".to_string());
        assert!(layer.is_upper());
        assert!(!layer.read_only);
    }

    #[test]
    fn test_overlay_config() {
        let config = OverlayConfig::new("test".to_string())
            .with_lower("/lower1".to_string())
            .with_lower("/lower2".to_string())
            .with_upper("/upper".to_string())
            .with_work("/work".to_string());

        assert_eq!(config.lower_dirs.len(), 2);
        assert_eq!(config.upper_dir, Some("/upper".to_string()));
        assert_eq!(config.work_dir, Some("/work".to_string()));
    }

    #[test]
    fn test_overlay_filesystem_mount() {
        let config = OverlayConfig::new("test".to_string())
            .with_lower("/lower".to_string())
            .with_upper("/upper".to_string())
            .with_work("/work".to_string());

        let mut overlay = OverlayFilesystem::new(1, config);
        overlay.mount().unwrap();

        assert!(overlay.is_mounted());
        assert_eq!(overlay.layer_count(), 3);

        assert_eq!(overlay.lower_layers().len(), 1);
        assert!(overlay.upper_layer().is_some());
        assert!(overlay.work_layer().is_some());
    }

    #[test]
    fn test_overlay_filesystem_unmount() {
        let config = OverlayConfig::new("test".to_string())
            .with_lower("/lower".to_string());

        let mut overlay = OverlayFilesystem::new(1, config);
        overlay.mount().unwrap();
        overlay.unmount().unwrap();

        assert!(!overlay.is_mounted());
        assert_eq!(overlay.layer_count(), 0);
    }

    #[test]
    fn test_overlay_manager() {
        let manager = OverlayManager::new();

        let config = OverlayConfig::new("test".to_string())
            .with_lower("/lower".to_string())
            .with_upper("/upper".to_string())
            .with_work("/work".to_string());

        let overlay_id = manager.create_overlay(config);
        assert_eq!(overlay_id, 1);

        manager.mount(overlay_id).unwrap();
        assert_eq!(manager.mounted_count(), 1);

        manager.unmount(overlay_id).unwrap();
        manager.remove_overlay(overlay_id).unwrap();

        assert_eq!(manager.overlay_count(), 0);
    }

    #[test]
    fn test_overlay_manager_multiple_overlays() {
        let manager = OverlayManager::new();

        let config1 = OverlayConfig::new("overlay1".to_string())
            .with_lower("/lower1".to_string());
        let config2 = OverlayConfig::new("overlay2".to_string())
            .with_lower("/lower2".to_string());

        let overlay_id1 = manager.create_overlay(config1);
        let overlay_id2 = manager.create_overlay(config2);

        manager.mount(overlay_id1).unwrap();
        manager.mount(overlay_id2).unwrap();

        assert_eq!(manager.mounted_count(), 2);
        assert_eq!(manager.overlay_count(), 2);
    }

    #[test]
    fn test_overlay_remove_mounted() {
        let manager = OverlayManager::new();

        let config = OverlayConfig::new("test".to_string())
            .with_lower("/lower".to_string());

        let overlay_id = manager.create_overlay(config);
        manager.mount(overlay_id).unwrap();

        assert!(manager.remove_overlay(overlay_id).is_err());

        manager.unmount(overlay_id).unwrap();
        assert!(manager.remove_overlay(overlay_id).is_ok());
    }
}
