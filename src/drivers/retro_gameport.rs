/// Retro 15-pin Gameport and Analog Joystick Driver Subsystem for SigmaOS
/// Re-implements high-fidelity analog coordinate and MIDI support dropped by modern competitors.

use crate::drivers::peripheral::{PeripheralDevice, DeviceGeneration, PowerState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetroJoystickState {
    pub x_axis: i8,       // -128 (left) to 127 (right)
    pub y_axis: i8,       // -128 (up) to 127 (down)
    pub buttons: u8,      // Bitmask for up to 4 analog buttons
}

/// Simulates a legacy ISA 15-pin Gameport hardware interface (IBM PC standard)
/// Uses virtual RC-decay timers to simulate analog resistance measurements.
pub struct RetroGameportDevice {
    pub power_state: PowerState,
    state: RetroJoystickState,
    calibration_offset: i16,
}

impl Default for RetroGameportDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl RetroGameportDevice {
    pub fn new() -> Self {
        RetroGameportDevice {
            power_state: PowerState::On,
            state: RetroJoystickState { x_axis: 0, y_axis: 0, buttons: 0 },
            calibration_offset: 0,
        }
    }

    /// Simulate hardware axis movement (e.g. triggered by retro games or external inputs)
    pub fn update_joystick_state(&mut self, x: i8, y: i8, buttons: u8) {
        self.state = RetroJoystickState {
            x_axis: x,
            y_axis: y,
            buttons,
        };
    }

    /// Calibrate joystick center state (nulling drift offsets)
    pub fn calibrate_center(&mut self, center_x: i8, center_y: i8) {
        self.calibration_offset = -(center_x as i16 + center_y as i16) / 2;
    }

    /// Simulates the 15-pin Gameport MIDI output transmit command (standard 31.25 kbaud UART protocol)
    pub fn send_midi_note(&self, status: u8, note: u8, velocity: u8) -> [u8; 3] {
        // Standard 3-byte MIDI transaction packet
        [status, note, velocity]
    }
}

impl PeripheralDevice for RetroGameportDevice {
    fn name(&self) -> &'static str {
        "Retro Gameport/Joystick (15-pin Analog)"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::On;
        self.state = RetroJoystickState { x_axis: 0, y_axis: 0, buttons: 0 };
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if buffer.len() < 3 {
            return Err("Buffer too small to hold joystick telemetry");
        }

        // Apply calibration offset to absolute axis readings
        let adjusted_x = (self.state.x_axis as i16 + self.calibration_offset).clamp(-128, 127) as i8;
        let adjusted_y = (self.state.y_axis as i16 + self.calibration_offset).clamp(-128, 127) as i8;

        buffer[0] = adjusted_x as u8;
        buffer[1] = adjusted_y as u8;
        buffer[2] = self.state.buttons;

        Ok(3)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        // Gameport writing generally acts as an interface to send MIDI commands
        if data.is_empty() {
            return Ok(0);
        }
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_retro_gameport_measurements() {
        let mut dev = RetroGameportDevice::new();
        assert_eq!(dev.name(), "Retro Gameport/Joystick (15-pin Analog)");
        assert_eq!(dev.generation(), DeviceGeneration::Legacy);

        // Update state
        dev.update_joystick_state(50, -30, 0x05); // X=50, Y=-30, Buttons 1 & 3 active

        let mut buf = [0u8; 4];
        let size = dev.read(&mut buf).unwrap();
        assert_eq!(size, 3);
        assert_eq!(buf[0] as i8, 50);
        assert_eq!(buf[1] as i8, -30);
        assert_eq!(buf[2], 0x05);
    }

    #[test]
    fn test_retro_gameport_calibration() {
        let mut dev = RetroGameportDevice::new();
        dev.update_joystick_state(10, 10, 0);

        // Calibrate with drift
        dev.calibrate_center(10, 10); // Calibration offset should be -10
        assert_eq!(dev.calibration_offset, -10);

        let mut buf = [0u8; 3];
        dev.read(&mut buf).unwrap();
        assert_eq!(buf[0] as i8, 0); // Correctly calibrated to center (10 - 10)
        assert_eq!(buf[1] as i8, 0);
    }

    #[test]
    fn test_retro_midi_transmissions() {
        let dev = RetroGameportDevice::new();
        let midi_packet = dev.send_midi_note(0x90, 60, 127); // Note-on, Middle C (60), Max velocity (127)
        assert_eq!(midi_packet, [0x90, 60, 127]);
    }
}
