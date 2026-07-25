use core::mem;
/// OOP-based Driver Framework for SigmaOS
/// Based on Roadmap Item 1: Driver framework
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DriverID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Unloaded = 0,
    Loaded = 1,
    Active = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
    ProbeFailed = 3,
    InitFailed = 4,
    DependencyMissing = 5,
}

/// Abstract Base Interface (Driver) - Core OOP abstraction
pub trait Driver {
    fn id(&self) -> DriverID;
    fn driver_type(&self) -> DriverType;
    fn state(&self) -> DriverState;
    fn set_state(&self, state: DriverState);

    /// Initialises the driver, configuring standard structures
    fn init(&mut self) -> Result<(), DriverError>;

    /// Probes hardware to verify device presence
    fn probe(&mut self) -> Result<bool, DriverError>;

    /// Loads driver into memory and transitions state to Active
    fn load(&mut self) -> Result<(), DriverError>;

    /// Unloads driver from memory and transitions state to Unloaded
    fn unload(&mut self) -> Result<(), DriverError>;

    /// Gracefully powers down hardware resources
    fn shutdown(&mut self) -> Result<(), DriverError>;

    /// Returns driver dependencies
    fn dependencies(&self) -> &'static [DriverType];
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
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
}

impl Driver for SimpleDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn driver_type(&self) -> DriverType {
        self.driver_type
    }
    fn state(&self) -> DriverState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state
            .store(DriverState::Loaded as usize, Ordering::SeqCst);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state
            .store(DriverState::Unloaded as usize, Ordering::SeqCst);
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), DriverError> {
        Ok(())
    }
    fn dependencies(&self) -> &'static [DriverType] {
        &[]
    }
}

impl StorageDriver for SimpleStorageDriver {
    fn read_blocks(&mut self, _block_idx: u64, buf: &mut [u8]) -> Result<usize, DriverError> {
        if self.state() != DriverState::Active {
            return Err(DriverError::LoadFailed);
        }
        for byte in buf.iter_mut() {
            *byte = 0xAA;
        }
        Ok(buf.len())
    }
    fn write_blocks(&mut self, _block_idx: u64, buf: &[u8]) -> Result<usize, DriverError> {
        if self.state() != DriverState::Active {
            return Err(DriverError::LoadFailed);
        }
        Ok(buf.len())
    }
}

pub struct SimpleNetworkDriver {
    pub id: DriverID,
    pub state: AtomicUsize,
    pub deps: &'static [DriverType],
}

impl SimpleNetworkDriver {
    pub fn new(id: DriverID, deps: &'static [DriverType]) -> Self {
        Self {
            id,
            state: AtomicUsize::new(DriverState::Unloaded as usize),
            deps,
        }
    }
}

impl Driver for SimpleNetworkDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn driver_type(&self) -> DriverType {
        DriverType::Network
    }
    fn state(&self) -> DriverState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
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
    fn shutdown(&mut self) -> Result<(), DriverError> {
        Ok(())
    }
    fn dependencies(&self) -> &'static [DriverType] {
        self.deps
    }
}

impl NetworkDriver for SimpleNetworkDriver {
    fn send_packet(&mut self, _packet: &[u8]) -> Result<(), DriverError> {
        if self.state() != DriverState::Active {
            return Err(DriverError::LoadFailed);
        }
        Ok(())
    }
    fn receive_packet(&mut self, buf: &mut [u8]) -> Result<usize, DriverError> {
        if self.state() != DriverState::Active {
            return Err(DriverError::LoadFailed);
        }
        if !buf.is_empty() {
            buf[0] = 0xFF;
            Ok(1)
        } else {
            Ok(0)
        }
    }
}

// Hardware Abstraction (Bus Abstraction Classes)

pub trait Bus {
    fn name(&self) -> &'static str;
    fn discover_devices(&self) -> Vec<u32>;
}

pub struct PciBus;
impl Bus for PciBus {
    fn name(&self) -> &'static str {
        "PCI Bus"
    }
    fn discover_devices(&self) -> Vec<u32> {
        let mut dev = Vec::new();
        dev.push(0x10DE); // Simulated GPU Vendor ID
        dev.push(0x1AF4); // Simulated VirtIO Storage
        dev
    }
}

pub struct UsbBus;
impl Bus for UsbBus {
    fn name(&self) -> &'static str {
        "USB Bus"
    }
    fn discover_devices(&self) -> Vec<u32> {
        let mut dev = Vec::new();
        dev.push(0x046D); // Simulated Logitech Mouse
        dev
    }
}

// Factory Pattern (Dynamic Driver Factory Instantiation)

pub struct DriverFactory;
impl DriverFactory {
    pub fn create_driver(id: DriverID, driver_type: DriverType) -> Box<dyn Driver> {
        match driver_type {
            DriverType::Storage => Box::new(SimpleStorageDriver::new(id)),
            DriverType::Network => Box::new(SimpleNetworkDriver::new(id, &[])),
            _ => Box::new(SimpleStorageDriver::new(id)), // Fallback
        }
    }
}

// Core OOP Driver Framework with Dependency Resolution & Hot-Swapping

pub trait DriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError>;
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver>;
    fn query_by_type(&self, driver_type: DriverType) -> Vec<DriverID>;
}

#[allow(dead_code)]
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
        // Find if any driver matches ID and check dependencies first
        let mut dep_ok = false;
        for i in 0..self.drivers.len() {
            if let Some(ref d) = self.drivers[i] {
                if d.id() == id {
                    dep_ok = self.verify_dependencies(d.as_ref());
                    break;
                }
            }
        }

        if !dep_ok {
            return Err(DriverError::DependencyMissing);
        }

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

    fn query_by_type(&self, driver_type: DriverType) -> Vec<DriverID> {
        let mut ids = Vec::new();
        for driver_option in self.drivers.iter() {
            if let Some(ref d) = *driver_option {
                if d.driver_type() == driver_type {
                    ids.push(d.id());
                }
            }
        }
        ids
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
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
    pub fn is_empty(&self) -> bool {
        self.len == 0
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
