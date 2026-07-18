#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

#[derive(Debug, Clone, Copy)]
pub struct KernelReleaseInfo {
    pub version: &'static str,
    pub active_stream: &'static str,
}

pub trait LinuxReleaseDriver: PeripheralDevice {
    fn release_info(&self) -> KernelReleaseInfo;
}

pub struct MainlineReleaseDriver {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl MainlineReleaseDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.10",
                active_stream: "Mainline",
            },
        }
    }
}
impl PeripheralDevice for MainlineReleaseDriver {
    fn name(&self) -> &'static str {
        "MainlineReleaseDriver"
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
impl LinuxReleaseDriver for MainlineReleaseDriver {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct StableReleaseDriver {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl StableReleaseDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.9.9",
                active_stream: "Stable",
            },
        }
    }
}
impl PeripheralDevice for StableReleaseDriver {
    fn name(&self) -> &'static str {
        "StableReleaseDriver"
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
impl LinuxReleaseDriver for StableReleaseDriver {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct LongtermReleaseDriver {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl LongtermReleaseDriver {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.6.35",
                active_stream: "Longterm",
            },
        }
    }
}
impl PeripheralDevice for LongtermReleaseDriver {
    fn name(&self) -> &'static str {
        "LongtermReleaseDriver"
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
impl LinuxReleaseDriver for LongtermReleaseDriver {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct PrepatchRcDriver1 {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl PrepatchRcDriver1 {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.11-rc1",
                active_stream: "Prepatch",
            },
        }
    }
}
impl PeripheralDevice for PrepatchRcDriver1 {
    fn name(&self) -> &'static str {
        "PrepatchRcDriver1"
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
impl LinuxReleaseDriver for PrepatchRcDriver1 {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct PrepatchRcDriver2 {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl PrepatchRcDriver2 {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.11-rc2",
                active_stream: "Prepatch",
            },
        }
    }
}
impl PeripheralDevice for PrepatchRcDriver2 {
    fn name(&self) -> &'static str {
        "PrepatchRcDriver2"
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
impl LinuxReleaseDriver for PrepatchRcDriver2 {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct PrepatchRcDriver3 {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl PrepatchRcDriver3 {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.11-rc3",
                active_stream: "Prepatch",
            },
        }
    }
}
impl PeripheralDevice for PrepatchRcDriver3 {
    fn name(&self) -> &'static str {
        "PrepatchRcDriver3"
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
impl LinuxReleaseDriver for PrepatchRcDriver3 {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct PrepatchRcDriver4 {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl PrepatchRcDriver4 {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.11-rc4",
                active_stream: "Prepatch",
            },
        }
    }
}
impl PeripheralDevice for PrepatchRcDriver4 {
    fn name(&self) -> &'static str {
        "PrepatchRcDriver4"
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
impl LinuxReleaseDriver for PrepatchRcDriver4 {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct PrepatchRcDriver5 {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl PrepatchRcDriver5 {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.11-rc5",
                active_stream: "Prepatch",
            },
        }
    }
}
impl PeripheralDevice for PrepatchRcDriver5 {
    fn name(&self) -> &'static str {
        "PrepatchRcDriver5"
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
impl LinuxReleaseDriver for PrepatchRcDriver5 {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}

pub struct PrepatchRcDriver6 {
    pub power: PowerState,
    pub info: KernelReleaseInfo,
}
impl PrepatchRcDriver6 {
    pub fn new() -> Self {
        Self {
            power: PowerState::Off,
            info: KernelReleaseInfo {
                version: "6.11-rc6",
                active_stream: "Prepatch",
            },
        }
    }
}
impl PeripheralDevice for PrepatchRcDriver6 {
    fn name(&self) -> &'static str {
        "PrepatchRcDriver6"
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
impl LinuxReleaseDriver for PrepatchRcDriver6 {
    fn release_info(&self) -> KernelReleaseInfo {
        self.info
    }
}
