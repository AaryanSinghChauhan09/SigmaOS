// Bluetooth GATT (Generic Attribute Profile) Client
// Inspired by Linux BlueZ GATT implementation

use std::collections::HashMap;
use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};

/// GATT characteristic
#[derive(Debug, Clone)]
pub struct GattCharacteristic {
    pub uuid: String,
    pub handle: u16,
    pub properties: u8,
    pub value: Vec<u8>,
}

/// GATT service
#[derive(Debug, Clone)]
pub struct GattService {
    pub uuid: String,
    pub handle: u16,
    pub characteristics: Vec<GattCharacteristic>,
}

/// GATT device
#[derive(Debug, Clone)]
pub struct GattDevice {
    pub id: u64,
    pub mac_address: String,
    pub name: String,
    pub connected: bool,
    pub services: Vec<GattService>,
}

/// GATT client
pub struct GattClient {
    next_device_id: AtomicU64,
    devices: HashMap<u64, GattDevice>,
    next_handle: AtomicU16,
}

impl GattClient {
    pub fn new() -> Self {
        Self {
            next_device_id: AtomicU64::new(1),
            devices: HashMap::new(),
            next_handle: AtomicU16::new(1),
        }
    }

    /// Connect to a GATT device
    pub fn connect(&mut self, mac_address: String, name: String) -> GattDevice {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);

        let device = GattDevice {
            id,
            mac_address,
            name,
            connected: true,
            services: Vec::new(),
        };

        self.devices.insert(id, device.clone());
        device
    }

    /// Disconnect from a device
    pub fn disconnect(&mut self, id: u64) -> Result<(), &'static str> {
        if let Some(device) = self.devices.get_mut(&id) {
            device.connected = false;
            Ok(())
        } else {
            Err("Device not found")
        }
    }

    /// Discover services
    pub fn discover_services(&mut self, device_id: u64) -> Result<(), &'static str> {
        if let Some(device) = self.devices.get_mut(&device_id) {
            // Simulated service discovery
            let service = GattService {
                uuid: "00001800-0000-1000-8000-00805f9b34fb".to_string(), // Generic Access
                handle: self.next_handle.fetch_add(1, Ordering::SeqCst),
                characteristics: Vec::new(),
            };

            device.services.push(service);
            Ok(())
        } else {
            Err("Device not found")
        }
    }

    /// Discover characteristics
    pub fn discover_characteristics(
        &mut self,
        device_id: u64,
        service_handle: u16,
    ) -> Result<(), &'static str> {
        if let Some(device) = self.devices.get_mut(&device_id) {
            if let Some(service) = device
                .services
                .iter_mut()
                .find(|s| s.handle == service_handle)
            {
                // Simulated characteristic discovery
                let characteristic = GattCharacteristic {
                    uuid: "00002a00-0000-1000-8000-00805f9b34fb".to_string(), // Device Name
                    handle: self.next_handle.fetch_add(1, Ordering::SeqCst),
                    properties: 0x02, // Read
                    value: Vec::new(),
                };

                service.characteristics.push(characteristic);
                Ok(())
            } else {
                Err("Service not found")
            }
        } else {
            Err("Device not found")
        }
    }

    /// Read characteristic value
    pub fn read_characteristic(
        &self,
        device_id: u64,
        handle: u16,
    ) -> Result<Vec<u8>, &'static str> {
        if let Some(device) = self.devices.get(&device_id) {
            for service in &device.services {
                if let Some(char) = service.characteristics.iter().find(|c| c.handle == handle) {
                    return Ok(char.value.clone());
                }
            }
            Err("Characteristic not found")
        } else {
            Err("Device not found")
        }
    }

    /// Write characteristic value
    pub fn write_characteristic(
        &mut self,
        device_id: u64,
        handle: u16,
        value: Vec<u8>,
    ) -> Result<(), &'static str> {
        if let Some(device) = self.devices.get_mut(&device_id) {
            for service in &mut device.services {
                if let Some(char) = service
                    .characteristics
                    .iter_mut()
                    .find(|c| c.handle == handle)
                {
                    char.value = value;
                    return Ok(());
                }
            }
            Err("Characteristic not found")
        } else {
            Err("Device not found")
        }
    }

    /// Get device by ID
    pub fn get_device(&self, id: u64) -> Option<&GattDevice> {
        self.devices.get(&id)
    }

    /// Get all devices
    pub fn get_all_devices(&self) -> Vec<&GattDevice> {
        self.devices.values().collect()
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        let mut client = GattClient::new();

        let device = client.connect("00:11:22:33:44:55".to_string(), "Test Device".to_string());
        assert_eq!(device.id, 1);
        assert!(device.connected);
        assert_eq!(client.device_count(), 1);
    }

    #[test]
    fn test_disconnect() {
        let mut client = GattClient::new();

        let device = client.connect("00:11:22:33:44:55".to_string(), "Test Device".to_string());
        assert!(client.disconnect(device.id).is_ok());

        let d = client.get_device(device.id).unwrap();
        assert!(!d.connected);
    }

    #[test]
    fn test_discover_services() {
        let mut client = GattClient::new();

        let device = client.connect("00:11:22:33:44:55".to_string(), "Test Device".to_string());
        assert!(client.discover_services(device.id).is_ok());

        let d = client.get_device(device.id).unwrap();
        assert_eq!(d.services.len(), 1);
    }

    #[test]
    fn test_discover_characteristics() {
        let mut client = GattClient::new();

        let device = client.connect("00:11:22:33:44:55".to_string(), "Test Device".to_string());
        client.discover_services(device.id).unwrap();

        let service_handle = client.get_device(device.id).unwrap().services[0].handle;
        assert!(client
            .discover_characteristics(device.id, service_handle)
            .is_ok());
    }

    #[test]
    fn test_write_read_characteristic() {
        let mut client = GattClient::new();

        let device = client.connect("00:11:22:33:44:55".to_string(), "Test Device".to_string());
        client.discover_services(device.id).unwrap();

        let service_handle = client.get_device(device.id).unwrap().services[0].handle;
        client
            .discover_characteristics(device.id, service_handle)
            .unwrap();

        let char_handle =
            client.get_device(device.id).unwrap().services[0].characteristics[0].handle;
        let value = vec![1, 2, 3, 4];

        assert!(client
            .write_characteristic(device.id, char_handle, value.clone())
            .is_ok());
        client.discover_characteristics(device.id, service_handle).unwrap();

        let char_handle = client.get_device(device.id).unwrap().services[0].characteristics[0].handle;
        let value = vec![1, 2, 3, 4];

        assert!(client.write_characteristic(device.id, char_handle, value.clone()).is_ok());
        let value = vec![1, 2, 3, 4];

        assert!(client
            .write_characteristic(device.id, char_handle, value.clone())
            .is_ok());

        let read_value = client.read_characteristic(device.id, char_handle).unwrap();
        assert_eq!(read_value, value);
    }
}
