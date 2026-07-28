#![no_std]
#![no_main]

use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DriverID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
    Storage = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

// Concrete Driver Classes (OOP Implementation)

pub struct SimpleStorageDriver {
    pub id: DriverID,
    pub driver_type: DriverType,
    pub state: AtomicUsize,
}

impl SimpleStorageDriver {
    pub fn new(id: DriverID) -> Self {
        SimpleStorageDriver {
            id,
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
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst) as u32) }
    }
    fn set_state(&self, state: DriverState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
    fn init(&mut self) -> Result<(), DriverError> {
        Ok(())
    }
    fn probe(&mut self) -> Result<bool, DriverError> {
        Ok(true)
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.set_state(DriverState::Active);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.set_state(DriverState::Unloaded);
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

impl SimpleDriverFramework {
    pub fn new() -> Self {
        SimpleDriverFramework {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    fn verify_dependencies(&self, driver: &dyn Driver) -> bool {
        for &dep in driver.dependencies() {
            let mut dep_found = false;
            for option_d in self.drivers.iter() {
                if let Some(ref d) = *option_d {
                    // Check if matching driver is loaded
                    let dt = d.driver_type() as usize;
                    let dep_t = dep as usize;
                    if dt == dep_t && d.state() == DriverState::Active {
                        dep_found = true;
                        break;
                    }
                }
            }
            if !dep_found {
                return false;
            }
        }
        true
    }
}

impl DriverFramework for SimpleDriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError> {
        let id = driver.id();
        self.drivers.push(Some(driver));
        Ok(id)
    }
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in &mut self.drivers {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.load();
                }
            }
        }
        Err(DriverError::LoadFailed)
    }
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in &mut self.drivers {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.unload();
                }
            }
        }
        Err(DriverError::UnloadFailed)
    }
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id {
                    return Some(driver.as_ref());
                }
            }
        }
        None
    }
}

struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
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

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
