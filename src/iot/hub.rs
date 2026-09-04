#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based IoT Hub for SigmaOS
/// Based on Ideas-999-Structured: IoT & Smart Home Item 976
/// Implements IoT device management

pub use alloc::string::String;
pub use alloc::boxed::Box;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceType { Sensor = 0, Actuator = 1, Controller = 2, Gateway = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IoTError { Success = 0, NotFound = 1, ConnectionFailed = 2 }

pub trait IoTDevice {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn device_type(&self) -> DeviceType;
    fn is_online(&self) -> bool;
}

#[repr(C)]
pub struct SimpleIoTDevice {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub device_type: AtomicUsize,
    pub online: AtomicUsize,
}

impl SimpleIoTDevice {
    pub fn new(id: DeviceID, name: &[u8], device_type: DeviceType) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleIoTDevice {
            id,
            name: name_array,
            device_type: AtomicUsize::new(device_type as usize),
            online: AtomicUsize::new(0),
        }
    }
}

impl IoTDevice for SimpleIoTDevice {
    fn id(&self) -> DeviceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn device_type(&self) -> DeviceType {
        match self.device_type.load(Ordering::SeqCst) {
            0 => DeviceType::Sensor,
            1 => DeviceType::Actuator,
            2 => DeviceType::Controller,
            _ => DeviceType::Gateway,
        }
    }
    fn is_online(&self) -> bool { self.online.load(Ordering::SeqCst) == 1 }
}

pub trait IoTHub {
    fn add_device(&mut self, device: Box<dyn IoTDevice>) -> Result<DeviceID, IoTError>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), IoTError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn IoTDevice>;
    fn send_command(&self, id: DeviceID, command: &[u8]) -> Result<(), IoTError>;
}

#[repr(C)]
pub struct SimpleIoTHub {
    pub devices: Vec<Option<Box<dyn IoTDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleIoTHub {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleIoTHub {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl IoTHub for SimpleIoTHub {
    fn add_device(&mut self, device: Box<dyn IoTDevice>) -> Result<DeviceID, IoTError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }
    
    fn remove_device(&mut self, id: DeviceID) -> Result<(), IoTError> {
        for device_option in &mut self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(IoTError::NotFound)
    }
    
    fn get_device(&self, id: DeviceID) -> Option<&dyn IoTDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
    
    fn send_command(&self, id: DeviceID, _command: &[u8]) -> Result<(), IoTError> {
        if self.get_device(id).is_some() {
            Ok(())
        } else {
            Err(IoTError::NotFound)
        }
    }
}

pub trait AutomationRule {
    fn add_rule(&mut self, trigger: &[u8], action: &[u8]);
    fn execute_rules(&self, event: &[u8]) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleAutomationRule {
    pub rules: Vec<([u8; 64], [u8; 64])>,
}

impl SimpleAutomationRule {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleAutomationRule {
            rules: Vec::new(),
        }
    }
}

impl AutomationRule for SimpleAutomationRule {
    fn add_rule(&mut self, trigger: &[u8], action: &[u8]) {
        let mut trigger_array = [0u8; 64];
        let mut action_array = [0u8; 64];
        let trigger_len = trigger.len().min(63);
        let action_len = action.len().min(63);
        for i in 0..trigger_len { trigger_array[i] = trigger[i]; }
        for i in 0..action_len { action_array[i] = action[i]; }
        self.rules.push((trigger_array, action_array));
    }
    
    fn execute_rules(&self, event: &[u8]) -> Vec<&[u8]> {
        let mut actions = Vec::new();
        for &(ref trigger, ref action) in &self.rules {
            let trigger_len = trigger.iter().position(|&b| b == 0).unwrap_or(64);
            if &trigger[..trigger_len] == event {
                let action_len = action.iter().position(|&b| b == 0).unwrap_or(64);
                actions.push(&action[..action_len]);
            }
        }
        actions
    }
}

/// SmartScene models a grouped set of actuator commands (inspired by Google Home)
pub struct SmartScene {
    pub name: String,
    pub commands: Vec<(DeviceID, [u8; 64])>,
}

impl SmartScene {
    pub fn new(name: &str) -> Self {
        let mut s = String::new();
        for &b in name.as_bytes() { s.push(b as char); }
        Self {
            name: s,
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, device_id: DeviceID, cmd: &[u8]) {
        let mut cmd_array = [0u8; 64];
        let cmd_len = cmd.len().min(63);
        for i in 0..cmd_len { cmd_array[i] = cmd[i]; }
        self.commands.push((device_id, cmd_array));
    }

    pub fn execute(&self, hub: &dyn IoTHub) -> Result<(), IoTError> {
        for &(device_id, ref cmd) in &self.commands {
            let cmd_len = cmd.iter().position(|&b| b == 0).unwrap_or(64);
            hub.send_command(device_id, &cmd[..cmd_len])?;
        }
        Ok(())
    }
}

/// DeviceState models properties of an active device (inspired by Home Assistant)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceState {
    pub device_id: DeviceID,
    pub battery_level: u8,
    pub temperature_c: i8,
    pub is_active: bool,
}

/// DeviceStateStore caches state information of all smart devices
pub struct DeviceStateStore {
    pub states: Vec<DeviceState>,
}

impl DeviceStateStore {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
        }
    }

    pub fn update_state(&mut self, state: DeviceState) {
        for i in 0..self.states.len() {
            if self.states[i].device_id == state.device_id {
                self.states[i] = state;
                return;
            }
        }
        self.states.push(state);
    }

    pub fn get_state(&self, device_id: DeviceID) -> Option<&DeviceState> {
        for s in &self.states {
            if s.device_id == device_id {
                return Some(s);
            }
        }
        None
    }
}

/// VoiceAssistantMock parses plain-text phrase patterns (Alexa/Google style) to trigger actions
pub struct VoiceAssistantMock {
    pub scenes: Vec<(String, SmartScene)>,
}

impl VoiceAssistantMock {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
        }
    }

    pub fn register_scene(&mut self, phrase: &str, scene: SmartScene) {
        let mut p = String::new();
        for &b in phrase.as_bytes() { p.push(b as char); }
        self.scenes.push((p, scene));
    }

    /// Process a natural language voice command phrase (e.g. "turn on lights" or "trigger movie night")
    pub fn handle_voice_phrase(&self, phrase: &str, hub: &dyn IoTHub) -> Result<String, IoTError> {
        let clean_phrase = phrase.trim().to_lowercase();

        // Match registered scene phrases first
        for &(ref trigger_phrase, ref scene) in &self.scenes {
            if trigger_phrase.to_lowercase() == clean_phrase {
                scene.execute(hub)?;
                let mut response = String::new();
                response.push_str("triggered scene: ");
                response.push_str(&scene.name);
                return Ok(response);
            }
        }

        // Direct device command parsing: "turn on device X" or "activate device X"
        if clean_phrase.contains("turn on device") || clean_phrase.contains("activate device") {
            for ch in clean_phrase.chars() {
                if ch.is_ascii_digit() {
                    let device_id = (ch as u8 - b'0') as DeviceID;
                    hub.send_command(device_id, b"ON")?;
                    let mut response = String::new();
                    response.push_str("activated device ID ");
                    response.push(ch);
                    return Ok(response);
                }
            }
        }

        // Direct device query parsing: "query status of device X"
        if clean_phrase.contains("status of device") || clean_phrase.contains("query device") {
            for ch in clean_phrase.chars() {
                if ch.is_ascii_digit() {
                    let device_id = (ch as u8 - b'0') as DeviceID;
                    if let Some(dev) = hub.get_device(device_id) {
                        let mut response = String::new();
                        response.push_str("device ");
                        for &b in dev.name() { response.push(b as char); }
                        response.push_str(" is ");
                        response.push_str(if dev.is_online() { "online" } else { "offline" });
                        return Ok(response);
                    }
                }
            }
        }

        let mut fallback = String::new();
        fallback.push_str("sorry, I didn't catch that command");
        Ok(fallback)
    }
}

pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

#[cfg(not(test))]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe { malloc(size) }
}


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iot_device_properties() {
        let dev = SimpleIoTDevice::new(1, b"Living Room Light", DeviceType::Actuator);
        assert_eq!(dev.id(), 1);
        assert_eq!(dev.name(), b"Living Room Light");
        assert_eq!(dev.device_type() as usize, DeviceType::Actuator as usize);
        assert!(!dev.is_online());
    }

    #[test]
    fn test_iot_hub_management() {
        let mut hub = SimpleIoTHub::new();
        let dev = SimpleIoTDevice::new(1, b"Living Room Light", DeviceType::Actuator);
        assert_eq!(hub.add_device(Box::new(dev)).unwrap(), 1);

        assert!(hub.get_device(1).is_some());
        assert_eq!(hub.get_device(1).unwrap().name(), b"Living Room Light");

        // Verify command execution
        assert!(hub.send_command(1, b"ON").is_ok());

        // Device not found scenarios
        assert!(hub.send_command(99, b"ON").is_err());
    }

    #[test]
    fn test_smart_scene_execution() {
        let mut hub = SimpleIoTHub::new();
        let dev = SimpleIoTDevice::new(5, b"Smart Plug", DeviceType::Actuator);
        hub.add_device(Box::new(dev)).unwrap();

        let mut scene = SmartScene::new("Movie Night");
        scene.add_command(5, b"TURN_ON");
        assert_eq!(scene.commands.len(), 1);

        assert!(scene.execute(&hub).is_ok());
    }

    #[test]
    fn test_device_state_store() {
        let mut store = DeviceStateStore::new();
        let s = DeviceState {
            device_id: 10,
            battery_level: 85,
            temperature_c: 22,
            is_active: true,
        };
        store.update_state(s);

        let retrieved = store.get_state(10).unwrap();
        assert_eq!(retrieved.battery_level, 85);
        assert_eq!(retrieved.temperature_c, 22);
        assert!(retrieved.is_active);

        // Update existing state
        let mut updated_s = s;
        updated_s.battery_level = 80;
        store.update_state(updated_s);
        assert_eq!(store.get_state(10).unwrap().battery_level, 80);
    }

    #[test]
    fn test_voice_assistant_parsing() {
        let mut hub = SimpleIoTHub::new();
        let dev = SimpleIoTDevice::new(2, b"Thermostat", DeviceType::Sensor);
        hub.add_device(Box::new(dev)).unwrap();

        let mut assistant = VoiceAssistantMock::new();

        let mut scene = SmartScene::new("Good Morning");
        scene.add_command(2, b"SET_TEMP_21");
        assistant.register_scene("good morning", scene);

        // 1. Trigger registered scene phrase
        let res1 = assistant.handle_voice_phrase("Good Morning", &hub).unwrap();
        assert_eq!(res1, "triggered scene: Good Morning");

        // 2. Direct device command parsing "turn on device X"
        let res2 = assistant.handle_voice_phrase("Alexa, turn on device 2", &hub).unwrap();
        assert_eq!(res2, "activated device ID 2");

        // 3. Direct device query status parsing "status of device X"
        let res3 = assistant.handle_voice_phrase("Google, query status of device 2", &hub).unwrap();
        assert_eq!(res3, "device Thermostat is offline");

        // 4. Fallback message
        let res4 = assistant.handle_voice_phrase("Open the pod bay doors", &hub).unwrap();
        assert_eq!(res4, "sorry, I didn't catch that command");
    }
}
