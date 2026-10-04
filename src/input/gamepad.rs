// Gamepad Input Driver
// Inspired by Linux evdev and Xbox controller support

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Gamepad button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    Back,
    Start,
    LeftStick,
    RightStick,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
}

/// Gamepad axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

/// Gamepad event
#[derive(Debug, Clone)]
pub struct GamepadEvent {
    pub device_id: u64,
    pub timestamp: u64,
    pub event_type: GamepadEventType,
}

#[derive(Debug, Clone)]
pub enum GamepadEventType {
    ButtonPressed(GamepadButton),
    ButtonReleased(GamepadButton),
    AxisMoved(GamepadAxis, i16),
}

/// Gamepad device
#[derive(Debug, Clone)]
pub struct GamepadDevice {
    pub id: u64,
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub connected: bool,
    pub axis_values: HashMap<GamepadAxis, i16>,
    pub button_states: HashMap<GamepadButton, bool>,
}

/// Gamepad manager
pub struct GamepadManager {
    next_device_id: AtomicU64,
    devices: HashMap<u64, GamepadDevice>,
    event_queue: Vec<GamepadEvent>,
    next_event_id: AtomicU64,
}

impl GamepadManager {
    pub fn new() -> Self {
        Self {
            next_device_id: AtomicU64::new(1),
            devices: HashMap::new(),
            event_queue: Vec::new(),
            next_event_id: AtomicU64::new(0),
        }
    }

    /// Register a gamepad device
    pub fn register_device(
        &mut self,
        name: String,
        vendor_id: u16,
        product_id: u16,
    ) -> GamepadDevice {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);

        let device = GamepadDevice {
            id,
            name,
            vendor_id,
            product_id,
            connected: true,
            axis_values: HashMap::new(),
            button_states: HashMap::new(),
        };

        self.devices.insert(id, device.clone());
        device
    }

    /// Unregister a gamepad device
    pub fn unregister_device(&mut self, id: u64) -> Result<(), &'static str> {
        if self.devices.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Device not found")
        }
    }

    /// Handle button press
    pub fn handle_button_press(
        &mut self,
        device_id: u64,
        button: GamepadButton,
    ) -> Result<(), &'static str> {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.button_states.insert(button, true);

            let event = GamepadEvent {
                device_id,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64,
                event_type: GamepadEventType::ButtonPressed(button),
            };

            self.event_queue.push(event);
            Ok(())
        } else {
            Err("Device not found")
        }
    }

    /// Handle button release
    pub fn handle_button_release(
        &mut self,
        device_id: u64,
        button: GamepadButton,
    ) -> Result<(), &'static str> {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.button_states.insert(button, false);

            let event = GamepadEvent {
                device_id,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64,
                event_type: GamepadEventType::ButtonReleased(button),
            };

            self.event_queue.push(event);
            Ok(())
        } else {
            Err("Device not found")
        }
    }

    /// Handle axis movement
    pub fn handle_axis_move(
        &mut self,
        device_id: u64,
        axis: GamepadAxis,
        value: i16,
    ) -> Result<(), &'static str> {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.axis_values.insert(axis, value);

            let event = GamepadEvent {
                device_id,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64,
                event_type: GamepadEventType::AxisMoved(axis, value),
            };

            self.event_queue.push(event);
            Ok(())
        } else {
            Err("Device not found")
        }
    }

    /// Get next event
    pub fn get_next_event(&mut self) -> Option<GamepadEvent> {
        if self.event_queue.is_empty() {
            None
        } else {
            Some(self.event_queue.remove(0))
        }
    }

    /// Get device by ID
    pub fn get_device(&self, id: u64) -> Option<&GamepadDevice> {
        self.devices.get(&id)
    }

    /// Get button state
    pub fn get_button_state(&self, device_id: u64, button: GamepadButton) -> Option<bool> {
        self.devices
            .get(&device_id)
            .and_then(|d| d.button_states.get(&button).copied())
    }

    /// Get axis value
    pub fn get_axis_value(&self, device_id: u64, axis: GamepadAxis) -> Option<i16> {
        self.devices
            .get(&device_id)
            .and_then(|d| d.axis_values.get(&axis).copied())
    }

    /// Get all devices
    pub fn get_all_devices(&self) -> Vec<&GamepadDevice> {
        self.devices.values().collect()
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.event_queue.len()
    }

    /// Clear event queue
    pub fn clear_events(&mut self) {
        self.event_queue.clear();
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_register_device() {
        let mut manager = GamepadManager::new();

        let device = manager.register_device("Xbox Controller".to_string(), 0x045e, 0x028e);
        assert_eq!(device.id, 1);
        assert_eq!(manager.device_count(), 1);
    }

    #[test]
    fn test_button_press() {
        let mut manager = GamepadManager::new();

        let device = manager.register_device("Xbox Controller".to_string(), 0x045e, 0x028e);
        assert!(manager
            .handle_button_press(device.id, GamepadButton::A)
            .is_ok());

        let state = manager.get_button_state(device.id, GamepadButton::A);
        assert_eq!(state, Some(true));
    }

    #[test]
    fn test_button_release() {
        let mut manager = GamepadManager::new();

        let device = manager.register_device("Xbox Controller".to_string(), 0x045e, 0x028e);
        manager
            .handle_button_press(device.id, GamepadButton::A)
            .unwrap();
        assert!(manager
            .handle_button_release(device.id, GamepadButton::A)
            .is_ok());

        let state = manager.get_button_state(device.id, GamepadButton::A);
        assert_eq!(state, Some(false));
    }

    #[test]
    fn test_axis_move() {
        let mut manager = GamepadManager::new();

        let device = manager.register_device("Xbox Controller".to_string(), 0x045e, 0x028e);
        assert!(manager
            .handle_axis_move(device.id, GamepadAxis::LeftStickX, 100)
            .is_ok());

        let value = manager.get_axis_value(device.id, GamepadAxis::LeftStickX);
        assert_eq!(value, Some(100));
    }

    #[test]
    fn test_event_queue() {
        let mut manager = GamepadManager::new();

        let device = manager.register_device("Xbox Controller".to_string(), 0x045e, 0x028e);
        manager
            .handle_button_press(device.id, GamepadButton::A)
            .unwrap();

        assert_eq!(manager.event_count(), 1);

        let event = manager.get_next_event();
        assert!(event.is_some());
    }
}
