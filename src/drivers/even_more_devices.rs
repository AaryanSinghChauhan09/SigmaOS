//! # Even More Devices - OOP-based Plug-and-Play drivers
//!
//! Concrete polymorphic implementations of `PeripheralDevice` with explicit OOP state hierarchies.
//! This fulfills the requirements of the master plan INTEGRATED_ABSORPTION_AND_IMPLEMENTATION_PLAN.md:
//!   - PS2MouseDriver (InputDriver class; State: Uninitialized -> StreamMode -> Error)
//!   - AmdRadeonGpuDriver (GpuDriver class; State: Off -> VgaFallback -> HardwareAccelerated -> Panic)
//!   - IntelProEthernetDriver (NetworkDriver class; State: Down -> LinkUp -> Transmitting -> Resetting)
//!   - BroadcomBluetoothDriver (BluetoothDriver class; State: Disabled -> InquiryMode -> Connected -> LowPower)

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

// =========================================================================
// 1. PS/2 Mouse Driver (InputDriver Class)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseState {
    Uninitialized,
    StreamMode,
    Error,
}

pub struct PS2MouseDriver {
    state: MouseState,
    power: PowerState,
    buffer: [u8; 64],
    len: usize,
}

impl PS2MouseDriver {
    pub fn new() -> Self {
        Self {
            state: MouseState::Uninitialized,
            power: PowerState::Off,
            buffer: [0u8; 64],
            len: 0,
        }
    }

    pub fn get_mouse_state(&self) -> MouseState {
        self.state
    }
}

impl PeripheralDevice for PS2MouseDriver {
    fn name(&self) -> &'static str {
        "PS/2 Mouse"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.state = MouseState::StreamMode;
        self.power = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.state != MouseState::StreamMode {
            return Err("Mouse is not in stream mode");
        }
        let read_len = self.len.min(buffer.len());
        buffer[..read_len].copy_from_slice(&self.buffer[..read_len]);
        self.len = 0;
        Ok(read_len)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if data.is_empty() {
            return Err("No data to write");
        }
        // Write to FM Synthesizer register pairs [reg, value]
        let mut idx = 0;
        while idx + 1 < data.len() {
            let reg = data[idx] as usize;
            let val = data[idx + 1];
            if reg < self.registers.len() {
                self.registers[reg] = val;
            }
            idx += 2;
        }
        Ok(idx)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        if state == PowerState::Off {
            self.state = MouseState::Uninitialized;
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.state = MouseState::Uninitialized;
        self.power = PowerState::Off;
        Ok(())
    }
}

// =========================================================================
// 2. AMD Radeon GPU Driver (GpuDriver Class)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuState {
    Off,
    VgaFallback,
    HardwareAccelerated,
    Panic,
}

pub struct AmdRadeonGpuDriver {
    state: GpuState,
    power: PowerState,
    memory_mapped_io: u64,
}

impl AmdRadeonGpuDriver {
    pub fn new(mmio: u64) -> Self {
        Self {
            state: GpuState::Off,
            power: PowerState::Off,
            memory_mapped_io: mmio,
        }
    }

    pub fn get_gpu_state(&self) -> GpuState {
        self.state
    }
}

impl PeripheralDevice for AmdRadeonGpuDriver {
    fn name(&self) -> &'static str {
        "AMD Radeon GPU"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.state = GpuState::VgaFallback;
        self.power = PowerState::On;
        Ok(())
    }

    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0) // Render output only
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state == GpuState::Panic {
            return Err("GPU is in panic state");
        }
        // Simulate uploading rendering vertex shader arrays / registers
        self.state = GpuState::HardwareAccelerated;
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        if state == PowerState::Sleep {
            self.state = GpuState::VgaFallback;
        } else if state == PowerState::Off {
            self.state = GpuState::Off;
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.state = GpuState::Off;
        self.power = PowerState::Off;
        Ok(())
    }
}

// =========================================================================
// 3. Intel PRO/1000 Ethernet Driver (NetworkDriver Class)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetState {
    Down,
    LinkUp,
    Transmitting,
    Resetting,
}

pub struct IntelProEthernetDriver {
    state: NetState,
    power: PowerState,
    mac_address: [u8; 6],
}

impl IntelProEthernetDriver {
    pub fn new(mac: [u8; 6]) -> Self {
        Self {
            state: NetState::Down,
            power: PowerState::Off,
            mac_address: mac,
        }
    }

    pub fn get_net_state(&self) -> NetState {
        self.state
    }
}

impl PeripheralDevice for IntelProEthernetDriver {
    fn name(&self) -> &'static str {
        "Intel PRO/1000 Ethernet"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.state = NetState::LinkUp;
        self.power = PowerState::On;
        Ok(())
    }

    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state == NetState::Down {
            return Err("Link is down");
        }
        self.state = NetState::Transmitting;
        // Transmit packet simulate
        self.state = NetState::LinkUp;
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        if state == PowerState::Off {
            self.state = NetState::Down;
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.state = NetState::Down;
        self.power = PowerState::Off;
        Ok(())
    }
}

// =========================================================================
// 4. Broadcom Bluetooth Driver (BluetoothDriver Class)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BtState {
    Disabled,
    InquiryMode,
    Connected,
    LowPower,
}

pub struct BroadcomBluetoothDriver {
    state: BtState,
    power: PowerState,
}

impl BroadcomBluetoothDriver {
    pub fn new() -> Self {
        Self {
            state: BtState::Disabled,
            power: PowerState::Off,
        }
    }

    pub fn get_bt_state(&self) -> BtState {
        self.state
    }
}

impl PeripheralDevice for BroadcomBluetoothDriver {
    fn name(&self) -> &'static str {
        "Broadcom Bluetooth Controller"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.state = BtState::InquiryMode;
        self.power = PowerState::On;
        Ok(())
    }

    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state == BtState::Disabled {
            return Err("Bluetooth is disabled");
        }
        self.state = BtState::Connected;
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        if state == PowerState::Sleep {
            self.state = BtState::LowPower;
        } else if state == PowerState::Off {
            self.state = BtState::Disabled;
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.state = BtState::Disabled;
        self.power = PowerState::Off;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ps2_mouse_lifecycle() {
        let mut mouse = PS2MouseDriver::new();
        assert_eq!(mouse.get_mouse_state(), MouseState::Uninitialized);
        mouse.initialize().unwrap();
        assert_eq!(mouse.get_mouse_state(), MouseState::StreamMode);

        let write_data = [1u8, 2u8, 3u8];
        assert_eq!(mouse.write(&write_data).unwrap(), 3);

        let mut read_buf = [0u8; 10];
        assert_eq!(mouse.read(&mut read_buf).unwrap(), 3);
        assert_eq!(&read_buf[..3], &write_data);
    }

    #[test]
    fn test_amd_radeon_lifecycle() {
        let mut gpu = AmdRadeonGpuDriver::new(0xE000_0000);
        assert_eq!(gpu.get_gpu_state(), GpuState::Off);
        gpu.initialize().unwrap();
        assert_eq!(gpu.get_gpu_state(), GpuState::VgaFallback);

        gpu.write(&[0u8; 128]).unwrap();
        assert_eq!(gpu.get_gpu_state(), GpuState::HardwareAccelerated);
    }

    #[test]
    fn test_intel_pro_lifecycle() {
        let mut eth = IntelProEthernetDriver::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(eth.get_net_state(), NetState::Down);
        eth.initialize().unwrap();
        assert_eq!(eth.get_net_state(), NetState::LinkUp);

        eth.write(&[0xAA; 64]).unwrap();
    }

    #[test]
    fn test_broadcom_bluetooth_lifecycle() {
        let mut bt = BroadcomBluetoothDriver::new();
        assert_eq!(bt.get_bt_state(), BtState::Disabled);
        bt.initialize().unwrap();
        assert_eq!(bt.get_bt_state(), BtState::InquiryMode);

        bt.write(&[1]).unwrap();
        assert_eq!(bt.get_bt_state(), BtState::Connected);
    }
}
