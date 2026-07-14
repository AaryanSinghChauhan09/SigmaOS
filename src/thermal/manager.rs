#![no_std]
#![no_main]

/// OOP-based Thermal Management for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 261
/// Implements temperature monitoring and thermal throttling

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThermalState { Normal = 0, Warning = 1, Critical = 2, Shutdown = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThermalError { Success = 0, NotFound = 1, SensorFailed = 2 }

pub trait ThermalSensor {
    fn id(&self) -> SensorID;
    fn name(&self) -> &[u8];
    fn temperature(&self) -> i32;
    fn max_temperature(&self) -> i32;
}

#[repr(C)]
pub struct SimpleThermalSensor {
    pub id: SensorID,
    pub name: [u8; 64],
    pub temperature: AtomicUsize,
    pub max_temperature: AtomicUsize,
}

impl SimpleThermalSensor {
    pub fn new(id: SensorID, name: &[u8], max_temperature: i32) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleThermalSensor {
            id,
            name: name_array,
            temperature: AtomicUsize::new(40),
            max_temperature: AtomicUsize::new(max_temperature as usize),
        }
    }
}

impl ThermalSensor for SimpleThermalSensor {
    fn id(&self) -> SensorID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn temperature(&self) -> i32 { self.temperature.load(Ordering::SeqCst) as i32 }
    fn max_temperature(&self) -> i32 { self.max_temperature.load(Ordering::SeqCst) as i32 }
}

pub trait ThermalManager {
    fn add_sensor(&mut self, sensor: Box<dyn ThermalSensor>) -> Result<SensorID, ThermalError>;
    fn remove_sensor(&mut self, id: SensorID) -> Result<(), ThermalError>;
    fn get_sensor(&self, id: SensorID) -> Option<&dyn ThermalSensor>;
    fn get_thermal_state(&self) -> ThermalState;
    fn update_temperature(&mut self, id: SensorID, temperature: i32) -> Result<(), ThermalError>;
}

#[repr(C)]
pub struct SimpleThermalManager {
    pub sensors: Vec<Option<Box<dyn ThermalSensor>>>,
    pub state: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleThermalManager {
    pub fn new() -> Self {
        SimpleThermalManager {
            sensors: Vec::new(),
            state: AtomicUsize::new(ThermalState::Normal as usize),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ThermalManager for SimpleThermalManager {
    fn add_sensor(&mut self, sensor: Box<dyn ThermalSensor>) -> Result<SensorID, ThermalError> {
        let id = sensor.id();
        self.sensors.push(Some(sensor));
        Ok(id)
    }
    
    fn remove_sensor(&mut self, id: SensorID) -> Result<(), ThermalError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id {
                    return Ok(());
                }
            }
        }
        Err(ThermalError::NotFound)
    }
    
    fn get_sensor(&self, id: SensorID) -> Option<&dyn ThermalSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
    
    fn get_thermal_state(&self) -> ThermalState {
        let mut max_temp = 0;
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                let temp = sensor.temperature();
                if temp > max_temp {
                    max_temp = temp;
                }
            }
        }
        
        if max_temp > 90 {
            ThermalState::Shutdown
        } else if max_temp > 80 {
            ThermalState::Critical
        } else if max_temp > 70 {
            ThermalState::Warning
        } else {
            ThermalState::Normal
        }
    }
    
    fn update_temperature(&mut self, id: SensorID, temperature: i32) -> Result<(), ThermalError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == id {
                    sensor.temperature.store(temperature as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ThermalError::NotFound)
    }
}

pub trait ThermalThrottling {
    fn enable_throttling(&mut self, enabled: bool);
    fn set_throttle_level(&mut self, level: u32);
    fn get_throttle_level(&self) -> u32;
}

#[repr(C)]
pub struct SimpleThermalThrottling {
    pub enabled: AtomicUsize,
    pub throttle_level: AtomicUsize,
}

impl SimpleThermalThrottling {
    pub fn new() -> Self {
        SimpleThermalThrottling {
            enabled: AtomicUsize::new(1),
            throttle_level: AtomicUsize::new(0),
        }
    }
}

impl ThermalThrottling for SimpleThermalThrottling {
    fn enable_throttling(&mut self, enabled: bool) {
        self.enabled.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }
    
    fn set_throttle_level(&mut self, level: u32) {
        self.throttle_level.store(level as usize, Ordering::SeqCst);
    }
    
    fn get_throttle_level(&self) -> u32 { self.throttle_level.load(Ordering::SeqCst) as u32 }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

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

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
