//! SigmaDevice - Device Manager
//!
//! Inspired by udev, this module provides comprehensive device management
//! including device detection, enumeration, hot-plug support, and device
//! event handling.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Block,
    Character,
    Network,
    USB,
    PCI,
    Input,
    Audio,
    Video,
    Unknown,
;

/// Device state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Present,
    Removed,
    Configured,
    Failed,
    Unknown,
;

/// Device event type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceEventType {
    Add,
    Remove,
    Change,
    Move,
    Bind,
    Unbind,
    Unknown,
;

/// Device data structure
#[derive(Debug, Clone)]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub device_state: DeviceState,
    pub vendor_id: String,
    pub product_id: String,
    pub device_path: String,
    pub driver: String,
    pub subsystem: String,
;

impl Device {
    /// Create a new device
    pub fn new(
        device_id: String,
        device_name: String,
        device_type: DeviceType,
        vendor_id: String,
        product_id: String,
        device_path: String,
        driver: String,
        subsystem: String,
    ) -> Self {
        Self {
            device_id,
            device_name,
            device_type,
            device_state: DeviceState::Present,
            vendor_id,
            product_id,
            device_path,
            driver,
            subsystem,
        }
    }

    /// Set device state
    pub fn set_state(&mut self, state: DeviceState) {
        self.device_state = state;
    }

    /// Check if device is present
    pub fn is_present(&self) -> bool {
        self.device_state == DeviceState::Present || self.device_state == DeviceState::Configured
    }

    /// Check if device is configured
    pub fn is_configured(&self) -> bool {
        self.device_state == DeviceState::Configured
    }
;

/// Device event data structure
#[derive(Debug, Clone)]
pub struct DeviceEvent {
    pub event_id: String,
    pub event_type: DeviceEventType,
    pub device_id: String,
    pub timestamp: u64,
    pub event_data: String,
;

impl DeviceEvent {
    /// Create a new device event
    pub fn new(
        event_id: String,
        event_type: DeviceEventType,
        device_id: String,
        timestamp: u64,
        event_data: String,
    ) -> Self {
        Self {
            event_id,
            event_type,
            device_id,
            timestamp,
            event_data,
        }
    }
;

/// Device manager
pub struct SigmaDevice {
    devices: BTreeMap<String, Device>,
    events: BTreeMap<String, DeviceEvent>,
    running: AtomicBool,
;

impl SigmaDevice {
    /// Create a new device manager
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            events: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the device manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the device manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a device
    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(device.device_id.clone(), device);
    }

    /// Remove a device
    pub fn remove_device(&mut self, device_id: &str) -> Option<Device> {
        self.devices.remove(device_id)
    }

    /// Get a device by ID
    pub fn get_device(&self, device_id: &str) -> Option<&Device> {
        self.devices.get(device_id)
    }

    /// Update device state
    pub fn update_device_state(&mut self, device_id: &str, state: DeviceState) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_state(state);
            true
        } else {
            false
        }
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&Device> {
        self.devices.values().collect()
    }

    /// List devices by type
    pub fn list_devices_by_type(&self, device_type: DeviceType) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    /// List devices by state
    pub fn list_devices_by_state(&self, state: DeviceState) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|d| d.device_state == state)
            .collect()
    }

    /// List devices by subsystem
    pub fn list_devices_by_subsystem(&self, subsystem: &str) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|d| d.subsystem == subsystem)
            .collect()
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Add a device event
    pub fn add_event(&mut self, event: DeviceEvent) {
        self.events.insert(event.event_id.clone(), event);
    }

    /// Get an event by ID
    pub fn get_event(&self, event_id: &str) -> Option<&DeviceEvent> {
        self.events.get(event_id)
    }

    /// List all events
    pub fn list_events(&self) -> Vec<&DeviceEvent> {
        self.events.values().collect()
    }

    /// List events by type
    pub fn list_events_by_type(&self, event_type: DeviceEventType) -> Vec<&DeviceEvent> {
        self.events
            .values()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// List events for a device
    pub fn list_events_for_device(&self, device_id: &str) -> Vec<&DeviceEvent> {
        self.events
            .values()
            .filter(|e| e.device_id == device_id)
            .collect()
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Clear all events
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Search devices by name
    pub fn search_devices_by_name(&self, name_pattern: &str) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|d| d.device_name.contains(name_pattern))
            .collect()
    }

    /// Search devices by vendor/product ID
    pub fn search_devices_by_ids(&self, vendor_id: &str, product_id: &str) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|d| d.vendor_id == vendor_id && d.product_id == product_id)
            .collect()
    }
;

impl Default for SigmaDevice {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaDevice instance (placeholder for actual global state)
static mut SIGMADEVICE_INSTANCE: Option<SigmaDevice> = None;

/// Initialize the global SigmaDevice instance
pub fn init_sigmadevice() {
    unsafe {
        SIGMADEVICE_INSTANCE = Some(SigmaDevice::new());
    }
;

/// Get the global SigmaDevice instance
pub fn get_sigmadevice() -> Option<&'static SigmaDevice> {
    unsafe { SIGMADEVICE_INSTANCE.as_ref() }
;

/// Get mutable global SigmaDevice instance
pub fn get_sigmadevice_mut() -> Option<&'static mut SigmaDevice> {
    unsafe { SIGMADEVICE_INSTANCE.as_mut() }
;

/// Convenience function to add a device
pub fn add_device(device: Device) {
    if let Some(dev) = get_sigmadevice_mut() {
        dev.add_device(device);
    }
;

/// Convenience function to remove a device
pub fn remove_device(device_id: &str) -> Option<Device> {
    if let Some(dev) = get_sigmadevice_mut() {
        dev.remove_device(device_id)
    } else {
        None
    }
;

/// Convenience function to add a device event
pub fn add_device_event(event: DeviceEvent) {
    if let Some(dev) = get_sigmadevice_mut() {
        dev.add_event(event);
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        let device = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );

        assert_eq!(device.device_id, "dev1");
        assert_eq!(device.device_name, "sda");
        assert_eq!(device.device_type, DeviceType::Block);
        assert_eq!(device.device_state, DeviceState::Present);
    }

    #[test]
    fn test_device_state() {
        let mut device = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );

        device.set_state(DeviceState::Configured);
        assert_eq!(device.device_state, DeviceState::Configured);
        assert!(device.is_configured());
        assert!(device.is_present());
    }

    #[test]
    fn test_device_event_creation() {
        let event = DeviceEvent::new(
            "evt1".to_string(),
            DeviceEventType::Add,
            "dev1".to_string(),
            1234567890,
            "Device added".to_string(),
        );

        assert_eq!(event.event_id, "evt1");
        assert_eq!(event.event_type, DeviceEventType::Add);
        assert_eq!(event.device_id, "dev1");
    }

    #[test]
    fn test_device_manager_creation() {
        let dev_mgr = SigmaDevice::new();
        assert!(!dev_mgr.is_running());
        assert_eq!(dev_mgr.device_count(), 0);
        assert_eq!(dev_mgr.event_count(), 0);
    }

    #[test]
    fn test_device_manager_start_stop() {
        let mut dev_mgr = SigmaDevice::new();
        dev_mgr.start();
        assert!(dev_mgr.is_running());
        dev_mgr.stop();
        assert!(!dev_mgr.is_running());
    }

    #[test]
    fn test_device_manager_add_device() {
        let mut dev_mgr = SigmaDevice::new();
        let device = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );

        dev_mgr.add_device(device);
        assert_eq!(dev_mgr.device_count(), 1);
        assert!(dev_mgr.get_device("dev1").is_some());
    }

    #[test]
    fn test_device_manager_remove_device() {
        let mut dev_mgr = SigmaDevice::new();
        let device = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );

        dev_mgr.add_device(device);
        assert_eq!(dev_mgr.device_count(), 1);

        let removed = dev_mgr.remove_device("dev1");
        assert!(removed.is_some());
        assert_eq!(dev_mgr.device_count(), 0);
    }

    #[test]
    fn test_device_manager_update_state() {
        let mut dev_mgr = SigmaDevice::new();
        let device = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );

        dev_mgr.add_device(device);
        let updated = dev_mgr.update_device_state("dev1", DeviceState::Configured);
        assert!(updated);

        let device = dev_mgr.get_device("dev1").unwrap();
        assert_eq!(device.device_state, DeviceState::Configured);
    }

    #[test]
    fn test_device_manager_list_devices() {
        let mut dev_mgr = SigmaDevice::new();

        dev_mgr.add_device(Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        ));

        dev_mgr.add_device(Device::new(
            "dev2".to_string(),
            "eth0".to_string(),
            DeviceType::Network,
            "0x1234".to_string(),
            "0x5679".to_string(),
            "/sys/class/net/eth0".to_string(),
            "e1000e".to_string(),
            "net".to_string(),
        ));

        let devices = dev_mgr.list_devices();
        assert_eq!(devices.len(), 2);
    }

    #[test]
    fn test_device_manager_list_devices_by_type() {
        let mut dev_mgr = SigmaDevice::new();

        dev_mgr.add_device(Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        ));

        dev_mgr.add_device(Device::new(
            "dev2".to_string(),
            "sdb".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5679".to_string(),
            "/dev/sdb".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        ));

        dev_mgr.add_device(Device::new(
            "dev3".to_string(),
            "eth0".to_string(),
            DeviceType::Network,
            "0x1234".to_string(),
            "0x567a".to_string(),
            "/sys/class/net/eth0".to_string(),
            "e1000e".to_string(),
            "net".to_string(),
        ));

        let block_devices = dev_mgr.list_devices_by_type(DeviceType::Block);
        assert_eq!(block_devices.len(), 2);

        let network_devices = dev_mgr.list_devices_by_type(DeviceType::Network);
        assert_eq!(network_devices.len(), 1);
    }

    #[test]
    fn test_device_manager_list_devices_by_state() {
        let mut dev_mgr = SigmaDevice::new();

        let mut device1 = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );
        device1.set_state(DeviceState::Configured);
        dev_mgr.add_device(device1);

        let device2 = Device::new(
            "dev2".to_string(),
            "sdb".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5679".to_string(),
            "/dev/sdb".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );
        dev_mgr.add_device(device2);

        let configured_devices = dev_mgr.list_devices_by_state(DeviceState::Configured);
        assert_eq!(configured_devices.len(), 1);

        let present_devices = dev_mgr.list_devices_by_state(DeviceState::Present);
        assert_eq!(present_devices.len(), 1);
    }

    #[test]
    fn test_device_manager_add_event() {
        let mut dev_mgr = SigmaDevice::new();
        let event = DeviceEvent::new(
            "evt1".to_string(),
            DeviceEventType::Add,
            "dev1".to_string(),
            1234567890,
            "Device added".to_string(),
        );

        dev_mgr.add_event(event);
        assert_eq!(dev_mgr.event_count(), 1);
        assert!(dev_mgr.get_event("evt1").is_some());
    }

    #[test]
    fn test_device_manager_list_events_by_type() {
        let mut dev_mgr = SigmaDevice::new();

        dev_mgr.add_event(DeviceEvent::new(
            "evt1".to_string(),
            DeviceEventType::Add,
            "dev1".to_string(),
            1234567890,
            "Device added".to_string(),
        ));

        dev_mgr.add_event(DeviceEvent::new(
            "evt2".to_string(),
            DeviceEventType::Remove,
            "dev2".to_string(),
            1234567891,
            "Device removed".to_string(),
        ));

        let add_events = dev_mgr.list_events_by_type(DeviceEventType::Add);
        assert_eq!(add_events.len(), 1);

        let remove_events = dev_mgr.list_events_by_type(DeviceEventType::Remove);
        assert_eq!(remove_events.len(), 1);
    }

    #[test]
    fn test_device_manager_list_events_for_device() {
        let mut dev_mgr = SigmaDevice::new();

        dev_mgr.add_event(DeviceEvent::new(
            "evt1".to_string(),
            DeviceEventType::Add,
            "dev1".to_string(),
            1234567890,
            "Device added".to_string(),
        ));

        dev_mgr.add_event(DeviceEvent::new(
            "evt2".to_string(),
            DeviceEventType::Change,
            "dev1".to_string(),
            1234567891,
            "Device changed".to_string(),
        ));

        dev_mgr.add_event(DeviceEvent::new(
            "evt3".to_string(),
            DeviceEventType::Add,
            "dev2".to_string(),
            1234567892,
            "Device added".to_string(),
        ));

        let dev1_events = dev_mgr.list_events_for_device("dev1");
        assert_eq!(dev1_events.len(), 2);

        let dev2_events = dev_mgr.list_events_for_device("dev2");
        assert_eq!(dev2_events.len(), 1);
    }

    #[test]
    fn test_device_manager_clear_events() {
        let mut dev_mgr = SigmaDevice::new();

        dev_mgr.add_event(DeviceEvent::new(
            "evt1".to_string(),
            DeviceEventType::Add,
            "dev1".to_string(),
            1234567890,
            "Device added".to_string(),
        ));

        assert_eq!(dev_mgr.event_count(), 1);

        dev_mgr.clear_events();
        assert_eq!(dev_mgr.event_count(), 0);
    }

    #[test]
    fn test_device_manager_search_by_name() {
        let mut dev_mgr = SigmaDevice::new();

        dev_mgr.add_device(Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        ));

        dev_mgr.add_device(Device::new(
            "dev2".to_string(),
            "sdb".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5679".to_string(),
            "/dev/sdb".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        ));

        let results = dev_mgr.search_devices_by_name("sd");
        assert_eq!(results.len(), 2);

        let results = dev_mgr.search_devices_by_name("sda");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_device_manager_search_by_ids() {
        let mut dev_mgr = SigmaDevice::new();

        dev_mgr.add_device(Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        ));

        dev_mgr.add_device(Device::new(
            "dev2".to_string(),
            "sdb".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5679".to_string(),
            "/dev/sdb".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        ));

        let results = dev_mgr.search_devices_by_ids("0x1234", "0x5678");
        assert_eq!(results.len(), 1);

        let results = dev_mgr.search_devices_by_ids("0x1234", "0x5679");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmadevice();

        let dev = get_sigmadevice();
        assert!(dev.is_some());

        let mut dev_mut = get_sigmadevice_mut();
        assert!(dev_mut.is_some());

        if let Some(dev) = dev_mut {
            dev.start();
            assert!(dev.is_running());
        }
    }

    #[test]
    fn test_convenience_add_device() {
        init_sigmadevice();

        let device = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );

        add_device(device);

        let dev = get_sigmadevice().unwrap();
        assert_eq!(dev.device_count(), 1);
    }

    #[test]
    fn test_convenience_remove_device() {
        init_sigmadevice();

        let device = Device::new(
            "dev1".to_string(),
            "sda".to_string(),
            DeviceType::Block,
            "0x1234".to_string(),
            "0x5678".to_string(),
            "/dev/sda".to_string(),
            "ahci".to_string(),
            "block".to_string(),
        );

        add_device(device);

        let removed = remove_device("dev1");
        assert!(removed.is_some());

        let dev = get_sigmadevice().unwrap();
        assert_eq!(dev.device_count(), 0);
    }

    #[test]
    fn test_convenience_add_event() {
        init_sigmadevice();

        let event = DeviceEvent::new(
            "evt1".to_string(),
            DeviceEventType::Add,
            "dev1".to_string(),
            1234567890,
            "Device added".to_string(),
        );

        add_device_event(event);

        let dev = get_sigmadevice().unwrap();
        assert_eq!(dev.event_count(), 1);
    }
;
