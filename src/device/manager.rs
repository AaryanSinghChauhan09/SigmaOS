use core::mem;
/// OOP-based Device Manager for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 91
/// Implements device detection, registration, and management
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(test))]
use alloc::boxed::Box;

pub type DeviceID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Block = 0,
    Character = 1,
    Network = 2,
    Input = 3,
    Output = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceError {
    Success = 0,
    NotFound = 1,
    AlreadyRegistered = 2,
    InitFailed = 3,
}

pub trait Device {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn device_class(&self) -> DeviceClass;
    fn initialize(&mut self) -> Result<(), DeviceError>;
    fn shutdown(&mut self) -> Result<(), DeviceError>;
}

#[repr(C)]
pub struct SimpleDevice {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub device_class: AtomicUsize,
}

impl SimpleDevice {
    pub fn new(id: DeviceID, name: &[u8], device_class: DeviceClass) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleDevice {
            id,
            name: name_array,
            device_class: AtomicUsize::new(device_class as usize),
        }
    }
}

impl Device for SimpleDevice {
    fn id(&self) -> DeviceID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn device_class(&self) -> DeviceClass {
        unsafe { core::mem::transmute(self.device_class.load(Ordering::SeqCst)) }
    }

    fn initialize(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
}

pub trait DeviceManager {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, DeviceError>;
    fn unregister_device(&mut self, id: DeviceID) -> Result<(), DeviceError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
    fn list_devices(&self, device_class: DeviceClass) -> Vec<DeviceID>;
    fn scan_devices(&mut self) -> Vec<DeviceID>;
}

#[repr(C)]
pub struct SimpleDeviceManager {
    pub devices: Vec<Option<Box<dyn Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDeviceManager {
    pub fn new() -> Self {
        SimpleDeviceManager {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DeviceManager for SimpleDeviceManager {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, DeviceError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }

    fn unregister_device(&mut self, id: DeviceID) -> Result<(), DeviceError> {
        for device_option in self.devices.iter_mut() {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(DeviceError::NotFound)
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for device_option in self.devices.iter() {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Some(device.as_ref());
                }
            }
        }
        None
    }

    fn list_devices(&self, device_class: DeviceClass) -> Vec<DeviceID> {
        let mut ids = Vec::new();
        for device_option in self.devices.iter() {
            if let Some(ref device) = *device_option {
                if device.device_class() == device_class {
                    ids.push(device.id());
                }
            }
        }
        ids
    }

    fn scan_devices(&mut self) -> Vec<DeviceID> {
        let mut ids = Vec::new();
        for device_option in self.devices.iter() {
            if let Some(ref device) = *device_option {
                ids.push(device.id());
            }
        }
        ids
    }
}

pub trait DeviceDriver {
    fn device_id(&self) -> DeviceID;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError>;
    fn ioctl(&mut self, request: u32, arg: usize) -> Result<(), DeviceError>;
}

#[repr(C)]
pub struct SimpleDeviceDriver {
    pub device_id: DeviceID,
}

impl SimpleDeviceDriver {
    pub fn new(device_id: DeviceID) -> Self {
        SimpleDeviceDriver { device_id }
    }
}

impl DeviceDriver for SimpleDeviceDriver {
    fn device_id(&self) -> DeviceID {
        self.device_id
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        for i in 0..buffer.len() {
            buffer[i] = 0u8;
        }
        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError> {
        Ok(data.len())
    }

    fn ioctl(&mut self, _request: u32, _arg: usize) -> Result<(), DeviceError> {
        Ok(())
    }
}

pub trait DeviceHotplug {
    fn on_device_added(&mut self, device_id: DeviceID);
    fn on_device_removed(&mut self, device_id: DeviceID);
    fn enable_hotplug(&mut self, enabled: bool);
}

#[repr(C)]
pub struct SimpleDeviceHotplug {
    pub enabled: AtomicUsize,
    pub added_devices: Vec<DeviceID>,
    pub removed_devices: Vec<DeviceID>,
}

impl SimpleDeviceHotplug {
    pub fn new() -> Self {
        SimpleDeviceHotplug {
            enabled: AtomicUsize::new(1),
            added_devices: Vec::new(),
            removed_devices: Vec::new(),
        }
    }
}

impl DeviceHotplug for SimpleDeviceHotplug {
    fn on_device_added(&mut self, device_id: DeviceID) {
        if self.enabled.load(Ordering::SeqCst) == 1 {
            self.added_devices.push(device_id);
        }
    }

    fn on_device_removed(&mut self, device_id: DeviceID) {
        if self.enabled.load(Ordering::SeqCst) == 1 {
            self.removed_devices.push(device_id);
        }
    }

    fn enable_hotplug(&mut self, enabled: bool) {
        self.enabled
            .store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }
}

// ==========================================
// BSD-INSPIRED AUTOCONF SYSTEM
// ==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsdDeviceState {
    Unattached = 0,
    Probed = 1,
    Attached = 2,
    Error = 3,
}

#[repr(C)]
pub struct BsdDevice {
    pub id: DeviceID,
    pub name: &'static str,
    pub parent_bus_id: Option<DeviceID>,
    pub vendor_id: u16,
    pub device_id: u16,
    pub state: BsdDeviceState,
}

impl BsdDevice {
    pub fn new(id: DeviceID, name: &'static str, parent_bus_id: Option<DeviceID>, vendor_id: u16, device_id: u16) -> Self {
        BsdDevice {
            id,
            name,
            parent_bus_id,
            vendor_id,
            device_id,
            state: BsdDeviceState::Unattached,
        }
    }
}

pub trait BsdDriver {
    fn driver_name(&self) -> &'static str;
    fn probe(&self, device: &BsdDevice) -> i32; // >0 is matching score, <=0 is no match
    fn attach(&self, device: &mut BsdDevice) -> Result<(), DeviceError>;
    fn detach(&self, device: &mut BsdDevice) -> Result<(), DeviceError>;
    fn shutdown(&self, device: &mut BsdDevice) -> Result<(), DeviceError>;
}

pub struct BsdAutoconfEngine {
    pub drivers: Vec<Box<dyn BsdDriver>>,
    pub devices: Vec<BsdDevice>,
}

impl BsdAutoconfEngine {
    pub fn new() -> Self {
        BsdAutoconfEngine {
            drivers: Vec::new(),
            devices: Vec::new(),
        }
    }

    pub fn register_driver(&mut self, driver: Box<dyn BsdDriver>) {
        self.drivers.push(driver);
    }

    pub fn add_device(&mut self, device: BsdDevice) {
        self.devices.push(device);
    }

    pub fn probe_and_attach(&mut self, device_id: DeviceID) -> Result<(), DeviceError> {
        let mut device_idx = None;
        for i in 0..self.devices.len() {
            if self.devices[i].id == device_id {
                device_idx = Some(i);
                break;
            }
        }

        let idx = device_idx.ok_or(DeviceError::NotFound)?;

        let mut best_driver_idx = None;
        let mut best_score = 0;

        for d_i in 0..self.drivers.len() {
            let score = self.drivers[d_i].probe(&self.devices[idx]);
            if score > best_score {
                best_score = score;
                best_driver_idx = Some(d_i);
            }
        }

        if let Some(drv_idx) = best_driver_idx {
            let res = self.drivers[drv_idx].attach(&mut self.devices[idx]);
            if res.is_ok() {
                self.devices[idx].state = BsdDeviceState::Attached;
            } else {
                self.devices[idx].state = BsdDeviceState::Error;
            }
            res
        } else {
            Err(DeviceError::NotFound)
        }
    }

    pub fn detach_device(&mut self, device_id: DeviceID) -> Result<(), DeviceError> {
        for i in 0..self.devices.len() {
            if self.devices[i].id == device_id {
                if self.devices[i].state == BsdDeviceState::Attached {
                    for d_i in 0..self.drivers.len() {
                        if self.drivers[d_i].probe(&self.devices[i]) > 0 {
                            let res = self.drivers[d_i].detach(&mut self.devices[i]);
                            if res.is_ok() {
                                self.devices[i].state = BsdDeviceState::Unattached;
                            }
                            return res;
                        }
                    }
                }
                return Ok(());
            }
        }
        Err(DeviceError::NotFound)
    }

    pub fn shutdown_all(&mut self) -> Result<(), DeviceError> {
        for i in 0..self.devices.len() {
            if self.devices[i].state == BsdDeviceState::Attached {
                for d_i in 0..self.drivers.len() {
                    if self.drivers[d_i].probe(&self.devices[i]) > 0 {
                        let _ = self.drivers[d_i].shutdown(&mut self.devices[i]);
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

// ==========================================
// LINUX-INSPIRED UDEV & DEVFS SYSTEM
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MajorMinor {
    pub major: u32,
    pub minor: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct LinuxUdevRule {
    pub match_name_prefix: Option<&'static str>,
    pub match_class: Option<DeviceClass>,
    pub subsystem: Option<&'static str>,
    pub set_symlink: Option<&'static str>,
    pub set_permissions: Option<u32>,
}

impl LinuxUdevRule {
    pub fn new(
        match_name_prefix: Option<&'static str>,
        match_class: Option<DeviceClass>,
        subsystem: Option<&'static str>,
        set_symlink: Option<&'static str>,
        set_permissions: Option<u32>,
    ) -> Self {
        LinuxUdevRule {
            match_name_prefix,
            match_class,
            subsystem,
            set_symlink,
            set_permissions,
        }
    }
}

pub struct UdevNode {
    pub name: [u8; 64],
    pub major_minor: MajorMinor,
    pub symlink: [u8; 64],
    pub permissions: u32,
}

impl UdevNode {
    pub fn new(name: &str, major_minor: MajorMinor, symlink: &str, permissions: u32) -> Self {
        let mut name_arr = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_arr.as_mut_ptr(), name_len);
        }

        let mut sym_arr = [0u8; 64];
        let sym_len = symlink.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(symlink.as_ptr(), sym_arr.as_mut_ptr(), sym_len);
        }

        UdevNode {
            name: name_arr,
            major_minor,
            symlink: sym_arr,
            permissions,
        }
    }

    pub fn get_name(&self) -> &str {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        unsafe { core::str::from_utf8_unchecked(&self.name[..len]) }
    }

    pub fn get_symlink(&self) -> &str {
        let len = self.symlink.iter().position(|&b| b == 0).unwrap_or(64);
        unsafe { core::str::from_utf8_unchecked(&self.symlink[..len]) }
    }
}

pub struct LinuxUdevEngine {
    pub rules: Vec<LinuxUdevRule>,
    pub active_nodes: Vec<UdevNode>,
}

impl LinuxUdevEngine {
    pub fn new() -> Self {
        LinuxUdevEngine {
            rules: Vec::new(),
            active_nodes: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: LinuxUdevRule) {
        self.rules.push(rule);
    }

    pub fn handle_hotplug_event(
        &mut self,
        device_name: &str,
        device_class: DeviceClass,
        subsystem: &str,
        major_minor: MajorMinor,
    ) {
        let mut symlink = "";
        let mut permissions = 0o644;

        for r_i in 0..self.rules.len() {
            let rule = &self.rules[r_i];
            let mut matches = true;

            if let Some(prefix) = rule.match_name_prefix {
                if !device_name.starts_with(prefix) {
                    matches = false;
                }
            }

            if let Some(cls) = rule.match_class {
                if cls != device_class {
                    matches = false;
                }
            }

            if let Some(sub) = rule.subsystem {
                if sub != subsystem {
                    matches = false;
                }
            }

            if matches {
                if let Some(sym) = rule.set_symlink {
                    symlink = sym;
                }
                if let Some(perms) = rule.set_permissions {
                    permissions = perms;
                }
            }
        }

        self.active_nodes.push(UdevNode::new(
            device_name,
            major_minor,
            symlink,
            permissions,
        ));
    }
}

// ==========================================
// LINUX-INSPIRED POWER MANAGEMENT SYSTEM
// ==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
}

pub struct LinuxDevicePowerManager {
    pub device_states: Vec<(DeviceID, PowerState)>,
}

impl LinuxDevicePowerManager {
    pub fn new() -> Self {
        LinuxDevicePowerManager {
            device_states: Vec::new(),
        }
    }

    pub fn register_device_power(&mut self, id: DeviceID) {
        self.device_states.push((id, PowerState::D0));
    }

    pub fn get_power_state(&self, id: DeviceID) -> Option<PowerState> {
        for i in 0..self.device_states.len() {
            if self.device_states[i].0 == id {
                return Some(self.device_states[i].1);
            }
        }
        None
    }

    pub fn set_power_state(&mut self, id: DeviceID, state: PowerState) -> Result<(), DeviceError> {
        for i in 0..self.device_states.len() {
            if self.device_states[i].0 == id {
                self.device_states[i].1 = state;
                return Ok(());
            }
        }
        Err(DeviceError::NotFound)
    }

    pub fn suspend_system(&mut self) -> Result<(), DeviceError> {
        let len = self.device_states.len();
        if len == 0 {
            return Ok(());
        }
        for i in (0..len).rev() {
            self.device_states[i].1 = PowerState::D3;
        }
        Ok(())
    }

    pub fn resume_system(&mut self) -> Result<(), DeviceError> {
        for i in 0..self.device_states.len() {
            self.device_states[i].1 = PowerState::D0;
        }
        Ok(())
    }
}

// ==========================================
// LINUX-INSPIRED DEVRES SYSTEM
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceResourceType {
    Memory(usize),
    Irq(u32),
    IoPort(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceResource {
    pub res_type: DeviceResourceType,
    pub released: bool,
}

pub struct DeviceResourceManager {
    pub allocations: Vec<(DeviceID, DeviceResource)>,
}

impl DeviceResourceManager {
    pub fn new() -> Self {
        DeviceResourceManager {
            allocations: Vec::new(),
        }
    }

    pub fn alloc_resource(&mut self, device_id: DeviceID, res_type: DeviceResourceType) {
        self.allocations.push((
            device_id,
            DeviceResource {
                res_type,
                released: false,
            },
        ));
    }

    pub fn release_resources_for_device(&mut self, device_id: DeviceID) {
        for i in 0..self.allocations.len() {
            if self.allocations[i].0 == device_id {
                self.allocations[i].1.released = true;
            }
        }
    }

    pub fn get_active_resources(&self, device_id: DeviceID) -> Vec<DeviceResourceType> {
        let mut active = Vec::new();
        for i in 0..self.allocations.len() {
            if self.allocations[i].0 == device_id && !self.allocations[i].1.released {
                active.push(self.allocations[i].1.res_type);
            }
        }
        active
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod test_allocator {
    extern "C" {
        #[link_name = "malloc"]
        fn libc_malloc(size: usize) -> *mut u8;
    }

    #[no_mangle]
    pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
        libc_malloc(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyBsdEthDriver;
    impl BsdDriver for DummyBsdEthDriver {
        fn driver_name(&self) -> &'static str {
            "e1000"
        }
        fn probe(&self, device: &BsdDevice) -> i32 {
            if device.vendor_id == 0x8086 && device.device_id == 0x100e {
                100
            } else {
                0
            }
        }
        fn attach(&self, _device: &mut BsdDevice) -> Result<(), DeviceError> {
            Ok(())
        }
        fn detach(&self, _device: &mut BsdDevice) -> Result<(), DeviceError> {
            Ok(())
        }
        fn shutdown(&self, _device: &mut BsdDevice) -> Result<(), DeviceError> {
            Ok(())
        }
    }

    struct DummyBsdAudioDriver;
    impl BsdDriver for DummyBsdAudioDriver {
        fn driver_name(&self) -> &'static str {
            "snd_hda"
        }
        fn probe(&self, device: &BsdDevice) -> i32 {
            if device.vendor_id == 0x8086 && device.device_id == 0x24d5 {
                50
            } else {
                0
            }
        }
        fn attach(&self, _device: &mut BsdDevice) -> Result<(), DeviceError> {
            Ok(())
        }
        fn detach(&self, _device: &mut BsdDevice) -> Result<(), DeviceError> {
            Ok(())
        }
        fn shutdown(&self, _device: &mut BsdDevice) -> Result<(), DeviceError> {
            Ok(())
        }
    }

    #[test]
    fn test_bsd_autoconf_matching() {
        let mut engine = BsdAutoconfEngine::new();
        engine.register_driver(Box::new(DummyBsdEthDriver));
        engine.register_driver(Box::new(DummyBsdAudioDriver));

        let eth_dev = BsdDevice::new(1, "pci0:0:1", None, 0x8086, 0x100e);
        let snd_dev = BsdDevice::new(2, "pci0:0:2", None, 0x8086, 0x24d5);

        engine.add_device(eth_dev);
        engine.add_device(snd_dev);

        assert_eq!(engine.devices[0].state, BsdDeviceState::Unattached);
        assert_eq!(engine.devices[1].state, BsdDeviceState::Unattached);

        let attach_eth = engine.probe_and_attach(1);
        assert!(attach_eth.is_ok());
        assert_eq!(engine.devices[0].state, BsdDeviceState::Attached);

        let attach_snd = engine.probe_and_attach(2);
        assert!(attach_snd.is_ok());
        assert_eq!(engine.devices[1].state, BsdDeviceState::Attached);

        let detach_eth = engine.detach_device(1);
        assert!(detach_eth.is_ok());
        assert_eq!(engine.devices[0].state, BsdDeviceState::Unattached);

        let shutdown_res = engine.shutdown_all();
        assert!(shutdown_res.is_ok());
    }

    #[test]
    fn test_linux_udev_rules() {
        let mut engine = LinuxUdevEngine::new();

        let mouse_rule = LinuxUdevRule::new(
            Some("mouse"),
            Some(DeviceClass::Input),
            Some("input"),
            Some("/dev/input/mouse0"),
            Some(0o660),
        );
        let net_rule = LinuxUdevRule::new(
            Some("eth"),
            Some(DeviceClass::Network),
            Some("net"),
            Some("/dev/net/eth0"),
            Some(0o644),
        );

        engine.add_rule(mouse_rule);
        engine.add_rule(net_rule);

        engine.handle_hotplug_event(
            "mouse_usb",
            DeviceClass::Input,
            "input",
            MajorMinor { major: 13, minor: 32 },
        );

        engine.handle_hotplug_event(
            "eth0",
            DeviceClass::Network,
            "net",
            MajorMinor { major: 2, minor: 0 },
        );

        assert_eq!(engine.active_nodes.len(), 2);

        assert_eq!(engine.active_nodes[0].get_name(), "mouse_usb");
        assert_eq!(engine.active_nodes[0].major_minor.major, 13);
        assert_eq!(engine.active_nodes[0].major_minor.minor, 32);
        assert_eq!(engine.active_nodes[0].get_symlink(), "/dev/input/mouse0");
        assert_eq!(engine.active_nodes[0].permissions, 0o660);

        assert_eq!(engine.active_nodes[1].get_name(), "eth0");
        assert_eq!(engine.active_nodes[1].major_minor.major, 2);
        assert_eq!(engine.active_nodes[1].major_minor.minor, 0);
        assert_eq!(engine.active_nodes[1].get_symlink(), "/dev/net/eth0");
        assert_eq!(engine.active_nodes[1].permissions, 0o644);
    }

    #[test]
    fn test_power_state_transitions() {
        let mut power_mgr = LinuxDevicePowerManager::new();
        power_mgr.register_device_power(10);
        power_mgr.register_device_power(20);
        power_mgr.register_device_power(30);

        assert_eq!(power_mgr.get_power_state(10), Some(PowerState::D0));
        assert_eq!(power_mgr.get_power_state(20), Some(PowerState::D0));
        assert_eq!(power_mgr.get_power_state(30), Some(PowerState::D0));

        let set_res = power_mgr.set_power_state(20, PowerState::D2);
        assert!(set_res.is_ok());
        assert_eq!(power_mgr.get_power_state(20), Some(PowerState::D2));

        let suspend_res = power_mgr.suspend_system();
        assert!(suspend_res.is_ok());
        assert_eq!(power_mgr.get_power_state(10), Some(PowerState::D3));
        assert_eq!(power_mgr.get_power_state(20), Some(PowerState::D3));
        assert_eq!(power_mgr.get_power_state(30), Some(PowerState::D3));

        let resume_res = power_mgr.resume_system();
        assert!(resume_res.is_ok());
        assert_eq!(power_mgr.get_power_state(10), Some(PowerState::D0));
        assert_eq!(power_mgr.get_power_state(20), Some(PowerState::D0));
        assert_eq!(power_mgr.get_power_state(30), Some(PowerState::D0));
    }

    #[test]
    fn test_device_resource_cleanup() {
        let mut res_mgr = DeviceResourceManager::new();
        res_mgr.alloc_resource(42, DeviceResourceType::Memory(4096));
        res_mgr.alloc_resource(42, DeviceResourceType::Irq(15));
        res_mgr.alloc_resource(99, DeviceResourceType::IoPort(0x3f8));

        let active_42 = res_mgr.get_active_resources(42);
        assert_eq!(active_42.len(), 2);
        assert_eq!(active_42[0], DeviceResourceType::Memory(4096));
        assert_eq!(active_42[1], DeviceResourceType::Irq(15));

        res_mgr.release_resources_for_device(42);

        let active_42_after = res_mgr.get_active_resources(42);
        assert_eq!(active_42_after.len(), 0);

        let active_99 = res_mgr.get_active_resources(99);
        assert_eq!(active_99.len(), 1);
        assert_eq!(active_99[0], DeviceResourceType::IoPort(0x3f8));
    }
}
