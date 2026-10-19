//! SigmaOS Bluetooth Manager
//!
//! Inspired by: Omarchy bluetooth controls (omancy-bluetooth-*, bluez)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, USER_CONTROL.

/// Bluetooth device
#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub id: String,
    pub name: String,
    pub address: String,
    pub is_paired: bool,
    pub is_connected: bool,
    pub is_trusted: bool,
    pub device_class: String,
    pub rssi: Option<i32>,
}

/// Bluetooth manager
pub struct BluetoothManager {
    pub devices: HashMap<String, BluetoothDevice>,
    pub powered: bool,
    pub discoverable: bool,
    pub pairable: bool,
}

impl BluetoothManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            powered: false,
            discoverable: false,
            pairable: true,
        }
    }

    /// Power on Bluetooth
    pub fn power_on(&mut self) {
        self.powered = true;
    }

    /// Power off Bluetooth
    pub fn power_off(&mut self) {
        self.powered = false;
    }

    /// Is powered
    pub fn is_powered(&self) -> bool {
        self.powered
    }

    /// Set discoverable
    pub fn set_discoverable(&mut self, discoverable: bool) {
        self.discoverable = discoverable;
    }

    /// Set pairable
    pub fn set_pairable(&mut self, pairable: bool) {
        self.pairable = pairable;
    }

    /// Add a discovered device
    pub fn add_device(&mut self, device: BluetoothDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    /// Pair with a device
    pub fn pair(&mut self, id: &str) -> Result<(), String> {
        let device = self
            .devices
            .get_mut(id)
            .ok_or_else(|| format!("Device {} not found", id))?;
        if !self.powered {
            return Err("Bluetooth is not powered".to_string());
        }
        device.is_paired = true;
        Ok(())
    }

    /// Connect to a device
    pub fn connect(&mut self, id: &str) -> Result<(), String> {
        let device = self
            .devices
            .get_mut(id)
            .ok_or_else(|| format!("Device {} not found", id))?;
        if !device.is_paired {
            return Err("Device must be paired before connecting".to_string());
        }
        device.is_connected = true;
        Ok(())
    }

    /// Disconnect from a device
    pub fn disconnect(&mut self, id: &str) -> Result<(), String> {
        let device = self
            .devices
            .get_mut(id)
            .ok_or_else(|| format!("Device {} not found", id))?;
        device.is_connected = false;
        Ok(())
    }

    /// Trust a device
    pub fn trust(&mut self, id: &str) -> Result<(), String> {
        let device = self
            .devices
            .get_mut(id)
            .ok_or_else(|| format!("Device {} not found", id))?;
        device.is_trusted = true;
        Ok(())
    }

    /// Unpair a device
    pub fn unpair(&mut self, id: &str) -> Result<(), String> {
        let device = self
            .devices
            .get_mut(id)
            .ok_or_else(|| format!("Device {} not found", id))?;
        device.is_paired = false;
        device.is_connected = false;
        device.is_trusted = false;
        Ok(())
    }

    /// Remove a device
    pub fn remove_device(&mut self, id: &str) -> Result<(), String> {
        self.devices
            .remove(id)
            .ok_or_else(|| format!("Device {} not found", id))?;
        Ok(())
    }

    /// List devices
    pub fn list_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.values().collect()
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&BluetoothDevice> {
        self.devices.get(id)
    }

    /// Get connected devices
    pub fn get_connected_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices
            .values()
            .filter(|d| d.is_connected)
            .collect()
    }

    /// Get paired devices
    pub fn get_paired_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.values().filter(|d| d.is_paired).collect()
    }

    /// Device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for BluetoothManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_on_off() {
        let mut mgr = BluetoothManager::new();
        assert!(!mgr.is_powered());
        mgr.power_on();
        assert!(mgr.is_powered());
        mgr.power_off();
        assert!(!mgr.is_powered());
    }

    #[test]
    fn test_add_and_pair_device() {
        let mut mgr = BluetoothManager::new();
        let device = BluetoothDevice {
            id: "dev0".to_string(),
            name: "Headphones".to_string(),
            address: "AA:BB:CC:DD:EE:FF".to_string(),
            is_paired: false,
            is_connected: false,
            is_trusted: false,
            device_class: "headphones".to_string(),
            rssi: None,
        };
        mgr.add_device(device);
        mgr.power_on();
        assert!(mgr.pair("dev0").is_ok());
        assert!(mgr.get_device("dev0").unwrap().is_paired);
    }

    #[test]
    fn test_pair_without_power_fails() {
        let mut mgr = BluetoothManager::new();
        mgr.add_device(BluetoothDevice {
            id: "dev0".to_string(),
            name: "Headphones".to_string(),
            address: "AA:BB:CC:DD:EE:FF".to_string(),
            is_paired: false,
            is_connected: false,
            is_trusted: false,
            device_class: "headphones".to_string(),
            rssi: None,
        });
        assert!(mgr.pair("dev0").is_err());
    }

    #[test]
    fn test_connect_without_pairing_fails() {
        let mut mgr = BluetoothManager::new();
        let device = BluetoothDevice {
            id: "dev0".to_string(),
            name: "Headphones".to_string(),
            address: "AA:BB:CC:DD:EE:FF".to_string(),
            is_paired: false,
            is_connected: false,
            is_trusted: false,
            device_class: "headphones".to_string(),
            rssi: None,
        };
        mgr.add_device(device);
        mgr.power_on();
        assert!(mgr.connect("dev0").is_err());
    }

    #[test]
    fn test_connect_and_disconnect() {
        let mut mgr = BluetoothManager::new();
        let device = BluetoothDevice {
            id: "dev0".to_string(),
            name: "Headphones".to_string(),
            address: "AA:BB:CC:DD:EE:FF".to_string(),
            is_paired: true,
            is_connected: false,
            is_trusted: false,
            device_class: "headphones".to_string(),
            rssi: None,
        };
        mgr.add_device(device);
        mgr.power_on();
        assert!(mgr.connect("dev0").is_ok());
        assert!(mgr.get_device("dev0").unwrap().is_connected);
        assert!(mgr.disconnect("dev0").is_ok());
        assert!(!mgr.get_device("dev0").unwrap().is_connected);
    }

    #[test]
    fn test_trust_and_unpair() {
        let mut mgr = BluetoothManager::new();
        let device = BluetoothDevice {
            id: "dev0".to_string(),
            name: "Headphones".to_string(),
            address: "AA:BB:CC:DD:EE:FF".to_string(),
            is_paired: true,
            is_connected: false,
            is_trusted: false,
            device_class: "headphones".to_string(),
            rssi: None,
        };
        mgr.add_device(device);
        assert!(mgr.trust("dev0").is_ok());
        assert!(mgr.get_device("dev0").unwrap().is_trusted);
        assert!(mgr.unpair("dev0").is_ok());
        assert!(!mgr.get_device("dev0").unwrap().is_paired);
        assert!(!mgr.get_device("dev0").unwrap().is_trusted);
    }

    #[test]
    fn test_get_connected_devices() {
        let mut mgr = BluetoothManager::new();
        mgr.add_device(BluetoothDevice {
            id: "dev0".to_string(),
            name: "Headphones".to_string(),
            address: "AA:BB:CC:DD:EE:FF".to_string(),
            is_paired: true,
            is_connected: true,
            is_trusted: false,
            device_class: "headphones".to_string(),
            rssi: None,
        });
        mgr.add_device(BluetoothDevice {
            id: "dev1".to_string(),
            name: "Mouse".to_string(),
            address: "11:22:33:44:55:66".to_string(),
            is_paired: true,
            is_connected: false,
            is_trusted: false,
            device_class: "mouse".to_string(),
            rssi: None,
        });
        let connected = mgr.get_connected_devices();
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].id, "dev0");
    }
}