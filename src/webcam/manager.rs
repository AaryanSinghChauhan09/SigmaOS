//! SigmaOS Webcam Manager
//!
//! Inspired by: Omarchy webcam (omancy-capture-webcam-list, omancy-capture-webcam-resize)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, MEMORY_SAFETY.

/// Webcam device
#[derive(Debug, Clone)]
pub struct WebcamDevice {
    pub id: String,
    pub name: String,
    pub path: String,
    pub is_available: bool,
    pub current_resolution: (u32, u32),
    pub supported_resolutions: Vec<(u32, u32)>,
    pub current_fps: u32,
    pub supported_fps: Vec<u32>,
}

/// Webcam manager
pub struct WebcamManager {
    pub devices: Vec<WebcamDevice>,
    pub active_device: Option<String>,
}

impl WebcamManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            active_device: None,
        }
    }

    /// List available webcam devices
    pub fn list_devices(&self) -> Vec<&WebcamDevice> {
        self.devices.iter().collect()
    }

    /// Add a discovered device
    pub fn add_device(&mut self, device: WebcamDevice) {
        self.devices.push(device);
    }

    /// Set active device
    pub fn set_active_device(&mut self, id: &str) -> Result<(), String> {
        if !self.devices.iter().any(|d| d.id == id) {
            return Err(format!("Webcam device {} not found", id));
        }
        self.active_device = Some(id.to_string());
        Ok(())
    }

    /// Get active device
    pub fn get_active_device(&self) -> Option<&WebcamDevice> {
        self.active_device
            .as_ref()
            .and_then(|id| self.devices.iter().find(|d| d.id == *id))
    }

    /// Set device resolution
    pub fn set_resolution(&mut self, id: &str, width: u32, height: u32) -> Result<(), String> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| format!("Webcam device {} not found", id))?;

        if !device.supported_resolutions.contains(&(width, height)) {
            return Err(format!("Resolution {}x{} not supported by device {}", width, height, id));
        }

        device.current_resolution = (width, height);
        Ok(())
    }

    /// Set device FPS
    pub fn set_fps(&mut self, id: &str, fps: u32) -> Result<(), String> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| format!("Webcam device {} not found", id))?;

        if !device.supported_fps.contains(&fps) {
            return Err(format!("FPS {} not supported by device {}", fps, id));
        }

        device.current_fps = fps;
        Ok(())
    }

    /// Resize webcam feed (for overlay use)
    pub fn resize_feed(&mut self, id: &str, width: u32, height: u32) -> Result<(), String> {
        self.set_resolution(id, width, height)
    }

    /// Remove a device
    pub fn remove_device(&mut self, id: &str) -> Result<(), String> {
        let pos = self
            .devices
            .iter()
            .position(|d| d.id == id)
            .ok_or_else(|| format!("Webcam device {} not found", id))?;
        self.devices.remove(pos);
        if self.active_device.as_deref() == Some(id) {
            self.active_device = None;
        }
        Ok(())
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for WebcamManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_list_devices() {
        let mut mgr = WebcamManager::new();
        let device = WebcamDevice {
            id: "cam0".to_string(),
            name: "Integrated Camera".to_string(),
            path: "/dev/video0".to_string(),
            is_available: true,
            current_resolution: (1920, 1080),
            supported_resolutions: vec![(640, 480), (1280, 720), (1920, 1080)],
            current_fps: 30,
            supported_fps: vec![15, 30, 60],
        };
        mgr.add_device(device);
        assert_eq!(mgr.device_count(), 1);
        assert_eq!(mgr.list_devices().len(), 1);
    }

    #[test]
    fn test_set_active_device() {
        let mut mgr = WebcamManager::new();
        let device = WebcamDevice {
            id: "cam0".to_string(),
            name: "Camera".to_string(),
            path: "/dev/video0".to_string(),
            is_available: true,
            current_resolution: (1280, 720),
            supported_resolutions: vec![(640, 480), (1280, 720)],
            current_fps: 30,
            supported_fps: vec![30],
        };
        mgr.add_device(device);
        assert!(mgr.set_active_device("cam0").is_ok());
        assert!(mgr.get_active_device().is_some());
        assert_eq!(mgr.get_active_device().unwrap().name, "Camera");
    }

    #[test]
    fn test_set_active_nonexistent_fails() {
        let mut mgr = WebcamManager::new();
        assert!(mgr.set_active_device("nonexistent").is_err());
    }

    #[test]
    fn test_set_resolution() {
        let mut mgr = WebcamManager::new();
        let device = WebcamDevice {
            id: "cam0".to_string(),
            name: "Camera".to_string(),
            path: "/dev/video0".to_string(),
            is_available: true,
            current_resolution: (640, 480),
            supported_resolutions: vec![(640, 480), (1920, 1080)],
            current_fps: 30,
            supported_fps: vec![30],
        };
        mgr.add_device(device);
        assert!(mgr.set_resolution("cam0", 1920, 1080).is_ok());
        assert_eq!(mgr.devices[0].current_resolution, (1920, 1080));
    }

    #[test]
    fn test_set_unsupported_resolution_fails() {
        let mut mgr = WebcamManager::new();
        let device = WebcamDevice {
            id: "cam0".to_string(),
            name: "Camera".to_string(),
            path: "/dev/video0".to_string(),
            is_available: true,
            current_resolution: (640, 480),
            supported_resolutions: vec![(640, 480)],
            current_fps: 30,
            supported_fps: vec![30],
        };
        mgr.add_device(device);
        assert!(mgr.set_resolution("cam0", 3840, 2160).is_err());
    }

    #[test]
    fn test_set_fps() {
        let mut mgr = WebcamManager::new();
        let device = WebcamDevice {
            id: "cam0".to_string(),
            name: "Camera".to_string(),
            path: "/dev/video0".to_string(),
            is_available: true,
            current_resolution: (640, 480),
            supported_resolutions: vec![(640, 480)],
            current_fps: 30,
            supported_fps: vec![15, 30, 60],
        };
        mgr.add_device(device);
        assert!(mgr.set_fps("cam0", 60).is_ok());
        assert_eq!(mgr.devices[0].current_fps, 60);
    }

    #[test]
    fn test_resize_feed() {
        let mut mgr = WebcamManager::new();
        let device = WebcamDevice {
            id: "cam0".to_string(),
            name: "Camera".to_string(),
            path: "/dev/video0".to_string(),
            is_available: true,
            current_resolution: (640, 480),
            supported_resolutions: vec![(640, 480), (320, 240)],
            current_fps: 30,
            supported_fps: vec![30],
        };
        mgr.add_device(device);
        assert!(mgr.resize_feed("cam0", 320, 240).is_ok());
        assert_eq!(mgr.devices[0].current_resolution, (320, 240));
    }
}