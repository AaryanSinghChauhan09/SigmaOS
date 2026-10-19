//! SigmaOS Fingerprint Manager
//!
//! Inspired by: Omarchy fingerprint (omancy-hw-fingerprint, omancy-setup-security-fingerprint, omancy-remove-security-fingerprint)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, LEAST_PRIVILEGE, MEMORY_SAFETY, LOW_LEVEL_CONTROL, USER_CONTROL.

/// Fingerprint device
#[derive(Debug, Clone)]
pub struct FingerprintDevice {
    pub id: String,
    pub name: String,
    pub path: String,
    pub is_available: bool,
    pub is_enrolled: bool,
    pub template_count: u32,
}

/// Fingerprint enrollment
#[derive(Debug, Clone)]
pub struct Enrollment {
    pub finger_id: String,
    pub finger_name: String,
    pub enrolled_at: u64,
    pub template_id: u32,
}

/// Fingerprint manager
pub struct FingerprintManager {
    pub devices: Vec<FingerprintDevice>,
    pub active_device: Option<String>,
    pub enrollments: HashMap<String, Vec<Enrollment>>,
    pub enabled: bool,
}

impl FingerprintManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            active_device: None,
            enrollments: HashMap::new(),
            enabled: false,
        }
    }

    /// Add a fingerprint device
    pub fn add_device(&mut self, device: FingerprintDevice) {
        self.devices.push(device);
    }

    /// List devices
    pub fn list_devices(&self) -> Vec<&FingerprintDevice> {
        self.devices.iter().collect()
    }

    /// Set active device
    pub fn set_active_device(&mut self, id: &str) -> Result<(), String> {
        if !self.devices.iter().any(|d| d.id == id) {
            return Err(format!("Fingerprint device {} not found", id));
        }
        self.active_device = Some(id.to_string());
        Ok(())
    }

    /// Get active device
    pub fn get_active_device(&self) -> Option<&FingerprintDevice> {
        self.active_device
            .as_ref()
            .and_then(|id| self.devices.iter().find(|d| d.id == *id))
    }

    /// Enable fingerprint authentication
    pub fn enable(&mut self) -> Result<(), String> {
        if self.active_device.is_none() {
            return Err("No active fingerprint device".to_string());
        }
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.id == self.active_device.as_ref().unwrap())
            .ok_or_else(|| "Device not found".to_string())?;
        if !device.is_available {
            return Err("Fingerprint device not available".to_string());
        }
        self.enabled = true;
        Ok(())
    }

    /// Disable fingerprint authentication
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enroll a finger
    pub fn enroll(
        &mut self,
        device_id: &str,
        finger_id: String,
        finger_name: String,
    ) -> Result<u32, String> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.id == device_id)
            .ok_or_else(|| format!("Device {} not found", device_id))?;

        if !device.is_available {
            return Err("Device not available".to_string());
        }

        if !self.enabled {
            return Err("Fingerprint authentication not enabled".to_string());
        }

        let template_id = device.template_count + 1;
        device.template_count += 1;

        let enrollment = Enrollment {
            finger_id: finger_id.clone(),
            finger_name,
            enrolled_at: now_secs(),
            template_id,
        };

        self.enrollments
            .entry(device_id.to_string())
            .or_default()
            .push(enrollment);

        Ok(template_id)
    }

    /// Remove enrollment
    pub fn remove_enrollment(
        &mut self,
        device_id: &str,
        finger_id: &str,
    ) -> Result<(), String> {
        let enrollments = self
            .enrollments
            .get_mut(device_id)
            .ok_or_else(|| format!("No enrollments for device {}", device_id))?;

        let pos = enrollments
            .iter()
            .position(|e| e.finger_id == finger_id)
            .ok_or_else(|| format!("Fingerprint {} not found", finger_id))?;

        enrollments.remove(pos);

        // Update template count
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == device_id) {
            device.template_count = device.template_count.saturating_sub(1);
        }

        Ok(())
    }

    /// List enrollments for a device
    pub fn list_enrollments(&self, device_id: &str) -> Option<&Vec<Enrollment>> {
        self.enrollments.get(device_id)
    }

    /// Verify fingerprint (simulated)
    pub fn verify(&mut self, device_id: &str, finger_id: &str) -> Result<bool, String> {
        if !self.enabled {
            return Err("Fingerprint authentication not enabled".to_string());
        }

        let device = self
            .devices
            .iter()
            .find(|d| d.id == device_id)
            .ok_or_else(|| format!("Device {} not found", device_id))?;

        if !device.is_available {
            return Err("Device not available".to_string());
        }

        let enrollments = self
            .enrollments
            .get(device_id)
            .ok_or_else(|| format!("No enrollments for device {}", device_id))?;

        Ok(enrollments.iter().any(|e| e.finger_id == finger_id))
    }

    /// Device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for FingerprintManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_device() {
        let mut mgr = FingerprintManager::new();
        let device = FingerprintDevice {
            id: "fpc0".to_string(),
            name: "FPC Sensor".to_string(),
            path: "/dev/fpc0".to_string(),
            is_available: true,
            is_enrolled: false,
            template_count: 0,
        };
        mgr.add_device(device);
        assert_eq!(mgr.device_count(), 1);
    }

    #[test]
    fn test_set_active_device() {
        let mut mgr = FingerprintManager::new();
        let device = FingerprintDevice {
            id: "fpc0".to_string(),
            name: "Sensor".to_string(),
            path: "/dev/fpc0".to_string(),
            is_available: true,
            is_enrolled: false,
            template_count: 0,
        };
        mgr.add_device(device);
        assert!(mgr.set_active_device("fpc0").is_ok());
        assert!(mgr.get_active_device().is_some());
    }

    #[test]
    fn test_enable_fingerprint() {
        let mut mgr = FingerprintManager::new();
        let device = FingerprintDevice {
            id: "fpc0".to_string(),
            name: "Sensor".to_string(),
            path: "/dev/fpc0".to_string(),
            is_available: true,
            is_enrolled: false,
            template_count: 0,
        };
        mgr.add_device(device);
        mgr.set_active_device("fpc0").unwrap();
        assert!(mgr.enable().is_ok());
        assert!(mgr.is_enabled());
    }

    #[test]
    fn test_enroll_finger() {
        let mut mgr = FingerprintManager::new();
        let device = FingerprintDevice {
            id: "fpc0".to_string(),
            name: "Sensor".to_string(),
            path: "/dev/fpc0".to_string(),
            is_available: true,
            is_enrolled: false,
            template_count: 0,
        };
        mgr.add_device(device);
        mgr.set_active_device("fpc0").unwrap();
        mgr.enable().unwrap();
        let template_id = mgr.enroll("fpc0", "thumb".to_string(), "Right Thumb".to_string()).unwrap();
        assert_eq!(template_id, 1);
    }

    #[test]
    fn test_verify_fingerprint() {
        let mut mgr = FingerprintManager::new();
        let device = FingerprintDevice {
            id: "fpc0".to_string(),
            name: "Sensor".to_string(),
            path: "/dev/fpc0".to_string(),
            is_available: true,
            is_enrolled: false,
            template_count: 0,
        };
        mgr.add_device(device);
        mgr.set_active_device("fpc0").unwrap();
        mgr.enable().unwrap();
        mgr.enroll("fpc0", "thumb".to_string(), "Right Thumb".to_string()).unwrap();
        assert!(mgr.verify("fpc0", "thumb").unwrap());
        assert!(!mgr.verify("fpc0", "index").unwrap());
    }

    #[test]
    fn test_remove_enrollment() {
        let mut mgr = FingerprintManager::new();
        let device = FingerprintDevice {
            id: "fpc0".to_string(),
            name: "Sensor".to_string(),
            path: "/dev/fpc0".to_string(),
            is_available: true,
            is_enrolled: false,
            template_count: 0,
        };
        mgr.add_device(device);
        mgr.set_active_device("fpc0").unwrap();
        mgr.enable().unwrap();
        mgr.enroll("fpc0", "thumb".to_string(), "Right Thumb".to_string()).unwrap();
        assert!(mgr.remove_enrollment("fpc0", "thumb").is_ok());
        assert!(mgr.list_enrollments("fpc0").unwrap().is_empty());
    }
}