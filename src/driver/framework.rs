extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DriverID = usize;
pub type SdfResult<T> = Result<T, DriverError>;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
    InvalidDevice = 3,
    PqcAttestationFailed = 4,
    PermissionDenied = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceId {
    pub vendor: u16,
    pub device: u16,
    pub subvendor: u16,
    pub subdevice: u16,
}

impl DeviceId {
    pub fn new(vendor: u16, device: u16) -> Self {
        Self {
            vendor,
            device,
            subvendor: 0,
            subdevice: 0,
        }
    }
}

/// Sovereign Driver Framework (SDF) Core Interface
pub trait SdfDriver {
    fn probe(dev: &DeviceId) -> bool where Self: Sized;
    fn init(&mut self) -> SdfResult<()>;
    fn shutdown(&mut self);
    fn verify_pqc_attestation(&self, token: &[u8]) -> bool {
        token.len() >= 16 && token[0..4] == [0x50, 0x51, 0x43, 0x31]
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
    Storage = 3,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Unloaded = 0,
    Loaded = 1,
    Active = 2,
}

pub trait Driver {
    fn id(&self) -> DriverID;
    fn driver_type(&self) -> DriverType;
    fn state(&self) -> DriverState;
    fn load(&mut self) -> Result<(), DriverError>;
    fn unload(&mut self) -> Result<(), DriverError>;
}

pub struct SimpleDriver {
    pub id: DriverID,
    pub driver_type: DriverType,
    pub state: AtomicUsize,
}

impl SimpleDriver {
    pub fn new(id: DriverID, driver_type: DriverType) -> Self {
        SimpleDriver {
            id,
            driver_type,
            state: AtomicUsize::new(DriverState::Unloaded as usize),
        }
    }

    pub fn init(&self) -> Result<(), DriverError> {
        Ok(())
    }

    pub fn probe(&self) -> Result<bool, DriverError> {
        Ok(true)
    }

    pub fn shutdown(&self) -> Result<(), DriverError> {
        Ok(())
    }
}

impl Driver for SimpleDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn driver_type(&self) -> DriverType {
        self.driver_type
    }
    fn state(&self) -> DriverState {
        match self.state.load(Ordering::SeqCst) {
            1 => DriverState::Loaded,
            2 => DriverState::Active,
            _ => DriverState::Unloaded,
        }
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state.store(DriverState::Active as usize, Ordering::SeqCst);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state.store(DriverState::Unloaded as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait StorageDriver: Driver {
    fn read_blocks(&mut self, block_idx: u64, buf: &mut [u8]) -> Result<usize, DriverError>;
    fn write_blocks(&mut self, block_idx: u64, buf: &[u8]) -> Result<usize, DriverError>;
}

pub trait NetworkDriver: Driver {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), DriverError>;
    fn receive_packet(&mut self, buf: &mut [u8]) -> Result<usize, DriverError>;
}

pub trait GraphicsDriver: Driver {
    fn set_resolution(&mut self, width: u32, height: u32) -> Result<(), DriverError>;
    fn flip_buffers(&mut self) -> Result<(), DriverError>;
}

pub trait InputDriver: Driver {
    fn poll_events(&mut self) -> Result<usize, DriverError>;
}

pub struct SimpleStorageDriver {
    pub id: DriverID,
    pub driver_type: DriverType,
    pub state: AtomicUsize,
}

impl SimpleStorageDriver {
    pub fn new(id: DriverID) -> Self {
        SimpleStorageDriver {
            id,
            driver_type: DriverType::Storage,
            state: AtomicUsize::new(DriverState::Unloaded as usize),
        }
    }
}

impl Driver for SimpleStorageDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn driver_type(&self) -> DriverType {
        DriverType::Storage
    }
    fn state(&self) -> DriverState {
        match self.state.load(Ordering::SeqCst) {
            1 => DriverState::Loaded,
            2 => DriverState::Active,
            _ => DriverState::Unloaded,
        }
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state.store(DriverState::Active as usize, Ordering::SeqCst);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state.store(DriverState::Unloaded as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait DriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError>;
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver>;
}

pub struct SimpleDriverFramework {
    drivers: Vec<Option<Box<dyn Driver>>>,
    next_id: AtomicUsize,
}

impl Default for SimpleDriverFramework {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleDriverFramework {
    pub fn new() -> Self {
        SimpleDriverFramework {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DriverFramework for SimpleDriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError> {
        let id = driver.id();
        self.drivers.push(Some(driver));
        Ok(id)
    }
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in self.drivers.iter_mut() {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.load();
                }
            }
        }
        Err(DriverError::LoadFailed)
    }
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in self.drivers.iter_mut() {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.unload();
                }
            }
        }
        Err(DriverError::UnloadFailed)
    }
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver> {
        for driver_option in self.drivers.iter() {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id {
                    return Some(driver.as_ref());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestPqcDriver {
        pub base: usize,
    }

    impl SdfDriver for TestPqcDriver {
        fn probe(dev: &DeviceId) -> bool {
            dev.vendor == 0x8086 && dev.device == 0x100E
        }
        fn init(&mut self) -> SdfResult<()> {
            self.base = 0xF000_0000;
            Ok(())
        }
        fn shutdown(&mut self) {
            self.base = 0;
        }
    }

    #[test]
    fn test_sdf_driver_lifecycle_and_pqc() {
        let dev = DeviceId::new(0x8086, 0x100E);
        assert!(TestPqcDriver::probe(&dev));

        let mut driver = TestPqcDriver { base: 0 };
        assert!(driver.init().is_ok());
        assert_eq!(driver.base, 0xF000_0000);

        let token = b"PQC1_VALID_TOKEN_123";
        assert!(driver.verify_pqc_attestation(token));

        let invalid_token = b"INVALID_TOKEN_123";
        assert!(!driver.verify_pqc_attestation(invalid_token));

        driver.shutdown();
        assert_eq!(driver.base, 0);
    }

    #[test]
    fn test_simple_driver_framework() {
        let mut framework = SimpleDriverFramework::new();
        let driver = Box::new(SimpleStorageDriver::new(100));

        assert!(framework.register_driver(driver).is_ok());
        assert!(framework.load_driver(100).is_ok());

        let loaded = framework.get_driver(100).unwrap();
        assert_eq!(loaded.state(), DriverState::Active);

        assert!(framework.unload_driver(100).is_ok());
        let unloaded = framework.get_driver(100).unwrap();
        assert_eq!(unloaded.state(), DriverState::Unloaded);
    }
}
