#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

pub struct AdLibSynthDriver {
    pub power: PowerState,
}
impl AdLibSynthDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for AdLibSynthDriver {
    fn name(&self) -> &'static str {
        "AdLibSynthDriver"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct PciIdeBridge {
    pub power: PowerState,
}
impl PciIdeBridge {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for PciIdeBridge {
    fn name(&self) -> &'static str {
        "PciIdeBridge"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct Ps2MouseDriver {
    pub power: PowerState,
}
impl Ps2MouseDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for Ps2MouseDriver {
    fn name(&self) -> &'static str {
        "Ps2MouseDriver"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct VgaTextModeDriver {
    pub power: PowerState,
}
impl VgaTextModeDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for VgaTextModeDriver {
    fn name(&self) -> &'static str {
        "VgaTextModeDriver"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct SerialMouseDriver {
    pub power: PowerState,
}
impl SerialMouseDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for SerialMouseDriver {
    fn name(&self) -> &'static str {
        "SerialMouseDriver"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct Ne2000NetworkDriver {
    pub power: PowerState,
}
impl Ne2000NetworkDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for Ne2000NetworkDriver {
    fn name(&self) -> &'static str {
        "Ne2000NetworkDriver"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct Usb4HostController {
    pub power: PowerState,
}
impl Usb4HostController {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for Usb4HostController {
    fn name(&self) -> &'static str {
        "Usb4HostController"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct NvlinkBusDriver {
    pub power: PowerState,
}
impl NvlinkBusDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for NvlinkBusDriver {
    fn name(&self) -> &'static str {
        "NvlinkBusDriver"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct Bluetooth5_4Adapter {
    pub power: PowerState,
}
impl Bluetooth5_4Adapter {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for Bluetooth5_4Adapter {
    fn name(&self) -> &'static str {
        "Bluetooth5_4Adapter"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct PcieGen6Bridge {
    pub power: PowerState,
}
impl PcieGen6Bridge {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for PcieGen6Bridge {
    fn name(&self) -> &'static str {
        "PcieGen6Bridge"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct Sata3Controller {
    pub power: PowerState,
}
impl Sata3Controller {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for Sata3Controller {
    fn name(&self) -> &'static str {
        "Sata3Controller"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}

pub struct Ufs4StorageDriver {
    pub power: PowerState,
}
impl Ufs4StorageDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
        }
    }
}
impl PeripheralDevice for Ufs4StorageDriver {
    fn name(&self) -> &'static str {
        "Ufs4StorageDriver"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::On;
        Ok(())
    }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(0)
    }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power = state;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power = PowerState::Off;
        Ok(())
    }
}
