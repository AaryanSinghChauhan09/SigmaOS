//! IoT Platform (IoT Device Management Inspiration)
//! Large-scale device management, protocol support, and digital twin integration

// #![no_std]

extern crate alloc;

use crate::klib::{Vec, String};
use alloc::vec::Vec;
use alloc::string::String;

/// Device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Online,
    Offline,
    Provisioning,
    Updating,
    Error,
}

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoTDeviceType {
    Sensor,
    Actuator,
    Gateway,
    Controller,
    EdgeDevice,
}

/// Protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolType {
    MQTT,
    CoAP,
    HTTP,
    LoRaWAN,
    Modbus,
    OPCUA,
}

/// IoT device
#[derive(Debug, Clone)]
pub struct IoTDevice {
    pub id: String,
    pub name: String,
    pub device_type: IoTDeviceType,
    pub state: DeviceState,
    pub protocol: ProtocolType,
    pub firmware_version: String,
    pub last_seen: u64,
    pub telemetry: Vec<TelemetryData>,
}

#[derive(Debug, Clone)]
pub struct TelemetryData {
    pub timestamp: u64,
    pub metric: String,
    pub value: f64,
    pub unit: String,
}

impl IoTDevice {
    pub fn new(name: &str, device_type: IoTDeviceType, protocol: ProtocolType) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            device_type,
            state: DeviceState::Provisioning,
            protocol,
            firmware_version: "1.0.0".to_string(),
            last_seen: 0,
            telemetry: Vec::new(),
        }
    }

    fn generate_id() -> String {
        "device_abcdef1234567890".to_string()
    }

    pub fn set_online(&mut self) {
        self.state = DeviceState::Online;
    }

    pub fn set_offline(&mut self) {
        self.state = DeviceState::Offline;
    }

    pub fn add_telemetry(&mut self, telemetry: TelemetryData) {
        self.telemetry.push(telemetry);
    }

    pub fn update_firmware(&mut self, version: &str) -> Result<(), IoTError> {
        self.state = DeviceState::Updating;
        self.firmware_version = version.to_string();
        self.state = DeviceState::Online;
        Ok(())
    }
}

/// IoT gateway
#[derive(Debug, Clone)]
pub struct IoTGateway {
    pub id: String,
    pub name: String,
    pub location: String,
    pub connected_devices: Vec<String>,
    pub protocols: Vec<ProtocolType>,
}

impl IoTGateway {
    pub fn new(name: &str, location: &str) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            location: location.to_string(),
            connected_devices: Vec::new(),
            protocols: Vec::new(),
        }
    }

    fn generate_id() -> String {
        "gateway_abcdef1234567890".to_string()
    }

    pub fn connect_device(&mut self, device_id: &str) {
        self.connected_devices.push(device_id.to_string());
    }

    pub fn add_protocol(&mut self, protocol: ProtocolType) {
        self.protocols.push(protocol);
    }
}

/// Data lake
#[derive(Debug, Clone)]
pub struct DataLake {
    pub id: String,
    pub name: String,
    pub storage_capacity: u64,
    pub data_retention: u64,
    pub partitions: Vec<String>,
}

impl DataLake {
    pub fn new(name: &str, capacity: u64) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            storage_capacity: capacity,
            data_retention: 90 * 24 * 3600, // 90 days in seconds
            partitions: Vec::new(),
        }
    }

    fn generate_id() -> String {
        "lake_abcdef1234567890".to_string()
    }

    pub fn add_partition(&mut self, partition: &str) {
        self.partitions.push(partition.to_string());
    }

    pub fn query(&self, query: &str) -> Result<Vec<TelemetryData>, IoTError> {
        // Query data lake
        Ok(Vec::new())
    }
}

/// Digital twin
#[derive(Debug, Clone)]
pub struct DigitalTwin {
    pub id: String,
    pub device_id: String,
    pub name: String,
    pub properties: Vec<(String, String)>,
    pub state: TwinState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwinState {
    Synced,
    Desynced,
    Error,
}

impl DigitalTwin {
    pub fn new(device_id: &str, name: &str) -> Self {
        Self {
            id: Self::generate_id(),
            device_id: device_id.to_string(),
            name: name.to_string(),
            properties: Vec::new(),
            state: TwinState::Synced,
        }
    }

    fn generate_id() -> String {
        "twin_abcdef1234567890".to_string()
    }

    pub fn add_property(&mut self, key: &str, value: &str) {
        self.properties.push((key.to_string(), value.to_string()));
    }

    pub fn sync(&mut self) -> Result<(), IoTError> {
        // Sync digital twin with physical device
        self.state = TwinState::Synced;
        Ok(())
    }
}

/// SigmaIoT - IoT Platform
pub struct SigmaIoT {
    pub devices: Vec<IoTDevice>,
    pub gateways: Vec<IoTGateway>,
    pub data_lakes: Vec<DataLake>,
    pub digital_twins: Vec<DigitalTwin>,
}

impl SigmaIoT {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            gateways: Vec::new(),
            data_lakes: Vec::new(),
            digital_twins: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: IoTDevice) {
        self.devices.push(device);
    }

    pub fn get_device(&mut self, id: &str) -> Option<&mut IoTDevice> {
        self.devices.iter_mut().find(|d| d.id == id || d.name == id)
    }

    pub fn add_gateway(&mut self, gateway: IoTGateway) {
        self.gateways.push(gateway);
    }

    pub fn get_gateway(&mut self, id: &str) -> Option<&mut IoTGateway> {
        self.gateways.iter_mut().find(|g| g.id == id || g.name == id)
    }

    pub fn add_data_lake(&mut self, lake: DataLake) {
        self.data_lakes.push(lake);
    }

    pub fn add_digital_twin(&mut self, twin: DigitalTwin) {
        self.digital_twins.push(twin);
    }

    pub fn provision_device(&mut self, device: IoTDevice) -> Result<String, IoTError> {
        let device_id = device.id.clone();
        self.add_device(device);
        Ok(device_id)
    }

    pub fn update_device_firmware(&mut self, device_id: &str, version: &str) -> Result<(), IoTError> {
        if let Some(device) = self.get_device(device_id) {
            device.update_firmware(version)
        } else {
            Err(IoTError::DeviceNotFound)
        }
    }

    pub fn create_digital_twin(&mut self, device_id: &str, name: &str) -> Result<String, IoTError> {
        let twin = DigitalTwin::new(device_id, name);
        let twin_id = twin.id.clone();
        self.add_digital_twin(twin);
        Ok(twin_id)
    }

    pub fn get_iot_stats(&self) -> IoTStats {
        IoTStats {
            total_devices: self.devices.len(),
            online_devices: self.devices.iter().filter(|d| d.state == DeviceState::Online).count(),
            total_gateways: self.gateways.len(),
            total_data_lakes: self.data_lakes.len(),
            total_twins: self.digital_twins.len(),
            synced_twins: self.digital_twins.iter().filter(|t| t.state == TwinState::Synced).count(),
        }
    }

    pub fn list_devices(&self) -> Vec<&IoTDevice> {
        self.devices.iter().collect()
    }

    pub fn list_gateways(&self) -> Vec<&IoTGateway> {
        self.gateways.iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct IoTStats {
    pub total_devices: usize,
    pub online_devices: usize,
    pub total_gateways: usize,
    pub total_data_lakes: usize,
    pub total_twins: usize,
    pub synced_twins: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoTError {
    DeviceNotFound,
    GatewayNotFound,
    ProvisioningFailed,
    UpdateFailed,
    SyncFailed,
    QueryFailed,
}

impl Default for SigmaIoT {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iot_device_creation() {
        let device = IoTDevice::new("sensor-1", IoTDeviceType::Sensor, ProtocolType::MQTT);
        assert_eq!(device.name, "sensor-1");
        assert_eq!(device.device_type, IoTDeviceType::Sensor);
    }

    #[test]
    fn test_iot_gateway() {
        let mut gateway = IoTGateway::new("gateway-1", "location-1");
        gateway.connect_device("device-1");
        assert_eq!(gateway.connected_devices.len(), 1);
    }

    #[test]
    fn test_data_lake() {
        let lake = DataLake::new("data-lake-1", 1024000);
        assert_eq!(lake.name, "data-lake-1");
    }

    #[test]
    fn test_digital_twin() {
        let twin = DigitalTwin::new("device-1", "twin-1");
        assert_eq!(twin.device_id, "device-1");
    }

    #[test]
    fn test_sigmaiot() {
        let mut iot = SigmaIoT::new();
        let device = IoTDevice::new("sensor-1", IoTDeviceType::Sensor, ProtocolType::MQTT);
        iot.add_device(device);
        assert_eq!(iot.list_devices().len(), 1);
    }
}