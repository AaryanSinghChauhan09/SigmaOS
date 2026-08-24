use crate::driver::framework::{Driver, DriverID, DriverType, DriverState, DriverError, SdfResult};
use crate::PowerState;

pub struct PciDeviceId { pub vendor: u16, pub device: u16 }
pub type DeviceId = PciDeviceId;

pub struct E1000Driver {
    pub power_state: PowerState,
}

impl E1000Driver {
    pub fn new() -> Self {
        Self { power_state: PowerState::Off }
    }
    pub fn probe(dev: &DeviceId) -> bool {
        dev.vendor == 0x8086 && dev.device == 0x100E
    }
    pub fn init(&mut self) -> SdfResult<()> {
        self.power_state = PowerState::On;
        Ok(())
    }
    pub fn shutdown(&mut self) {
        self.power_state = PowerState::Off;
    }
}

impl Driver for E1000Driver {
    fn id(&self) -> DriverID { 1000 }
    fn driver_type(&self) -> DriverType { DriverType::Network }
    fn state(&self) -> DriverState { DriverState::Unloaded }
    fn load(&mut self) -> Result<(), DriverError> { Ok(()) }
    fn unload(&mut self) -> Result<(), DriverError> { Ok(()) }
}
