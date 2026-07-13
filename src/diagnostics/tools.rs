#![no_std]
#![no_main]

/// OOP-based Low-Level Diagnostics Tools for SigmaOS
/// Implements diagnostics using OOP principles with traits and structs
/// No dependency on external diagnostics frameworks
/// Based on Roadmap Item 16: Low-level diagnostics tools

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Sensor ID
pub type SensorID = usize;

/// Sensor type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorType {
    CPU = 0,
    Memory = 1,
    Thermal = 2,
    Power = 3,
    Storage = 4,
    Network = 5,
}

/// Sensor trait (OOP interface)
pub trait Sensor {
    /// Get sensor ID
    fn id(&self) -> SensorID;
    /// Get sensor name
    fn name(&self) -> &[u8];
    /// Get sensor type
    fn sensor_type(&self) -> SensorType;
    /// Read sensor value
    fn read(&mut self) -> Result<f64, DiagnosticsError>;
    /// Get sensor info
    fn info(&self) -> SensorInfo;
}

/// Diagnostics error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticsError {
    Success = 0,
    SensorNotFound = 1,
    ReadFailed = 2,
    PermissionDenied = 3,
}

/// Sensor info
#[repr(C)]
pub struct SensorInfo {
    pub id: SensorID,
    pub name: [u8; 64],
    pub sensor_type: SensorType,
    pub value: f64,
    pub unit: [u8; 16],
    pub capability: SensorCapability,
}

impl SensorInfo {
    pub fn new(id: SensorID, sensor_type: SensorType) -> Self {
        SensorInfo {
            id,
            name: [0; 64],
            sensor_type,
            value: 0.0,
            unit: [0; 16],
            capability: SensorCapability::new(),
        }
    }
}

/// Sensor capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SensorCapability {
    pub can_read: bool,
    pub can_reset: bool,
}

impl SensorCapability {
    pub fn new() -> Self {
        SensorCapability {
            can_read: false,
            can_reset: false,
        }
    }

    pub fn full() -> Self {
        SensorCapability {
            can_read: true,
            can_reset: true,
        }
    }
}

/// Simple sensor (OOP: Concrete sensor class)
#[repr(C)]
pub struct SimpleSensor {
    pub id: SensorID,
    pub name: [u8; 64],
    pub sensor_type: SensorType,
    pub value: AtomicUsize, // Store as usize for atomic operations
    pub unit: [u8; 16],
    pub capability: SensorCapability,
}

impl SimpleSensor {
    pub fn new(id: SensorID, name: &[u8], sensor_type: SensorType, unit: &[u8], capability: SensorCapability) -> Self {
        let mut name_array = [0u8; 64];
        let mut unit_array = [0u8; 16];

        let name_len = name.len().min(63);
        let unit_len = unit.len().min(15);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(unit.as_ptr(), unit_array.as_mut_ptr(), unit_len);
        }

        SimpleSensor {
            id,
            name: name_array,
            sensor_type,
            value: AtomicUsize::new(0),
            unit: unit_array,
            capability,
        }
    }

    fn f64_to_usize(f: f64) -> usize {
        f as usize
    }

    fn usize_to_f64(u: usize) -> f64 {
        u as f64
    }
}

impl Sensor for SimpleSensor {
    fn id(&self) -> SensorID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn sensor_type(&self) -> SensorType {
        self.sensor_type
    }

    fn read(&mut self) -> Result<f64, DiagnosticsError> {
        if !self.capability.can_read {
            return Err(DiagnosticsError::PermissionDenied);
        }

        // In a real implementation, this would read from hardware
        // For now, simulate reading
        let simulated_value = match self.sensor_type {
            SensorType::CPU => 50.0,
            SensorType::Memory => 60.0,
            SensorType::Thermal => 45.0,
            SensorType::Power => 12.0,
            SensorType::Storage => 70.0,
            SensorType::Network => 100.0,
        };

        self.value.store(Self::f64_to_usize(simulated_value), Ordering::SeqCst);
        Ok(simulated_value)
    }

    fn info(&self) -> SensorInfo {
        SensorInfo {
            id: self.id,
            name: self.name,
            sensor_type: self.sensor_type,
            value: Self::usize_to_f64(self.value.load(Ordering::SeqCst)),
            unit: self.unit,
            capability: self.capability,
        }
    }
}

/// Diagnostics manager trait (OOP interface)
pub trait DiagnosticsManager {
    /// Register sensor
    fn register_sensor(&mut self, sensor: Box<dyn Sensor>) -> Result<SensorID, DiagnosticsError>;
    /// Unregister sensor
    fn unregister_sensor(&mut self, id: SensorID) -> Result<(), DiagnosticsError>;
    /// Read sensor
    fn read_sensor(&mut self, id: SensorID) -> Result<f64, DiagnosticsError>;
    /// Read all sensors
    fn read_all(&mut self) -> Result<Vec<(SensorID, f64)>, DiagnosticsError>;
    /// Get sensor
    fn get_sensor(&self, id: SensorID) -> Option<&dyn Sensor>;
    /// List sensors by type
    fn list_sensors(&self, sensor_type: SensorType) -> Vec<SensorID>;
    /// Get manager statistics
    fn stats(&self) -> DiagnosticsStats;
}

/// Diagnostics statistics
#[repr(C)]
pub struct DiagnosticsStats {
    pub total_sensors: usize,
    pub active_sensors: usize,
    pub by_type: [usize; 6],
}

impl DiagnosticsStats {
    pub fn new() -> Self {
        DiagnosticsStats {
            total_sensors: 0,
            active_sensors: 0,
            by_type: [0; 6],
        }
    }
}

/// Simple diagnostics manager (OOP: Concrete manager class)
pub struct SimpleDiagnosticsManager {
    sensors: Vec<Option<Box<dyn Sensor>>>,
    next_id: AtomicUsize,
    stats: DiagnosticsStats,
    capability: ManagerCapability,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ManagerCapability {
    pub can_register: bool,
    pub can_read: bool,
}

impl ManagerCapability {
    pub fn new() -> Self {
        ManagerCapability {
            can_register: false,
            can_read: false,
        }
    }

    pub fn full() -> Self {
        ManagerCapability {
            can_register: true,
            can_read: true,
        }
    }
}

impl SimpleDiagnosticsManager {
    pub fn new(capability: ManagerCapability) -> Self {
        SimpleDiagnosticsManager {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: DiagnosticsStats::new(),
            capability,
        }
    }
}

impl DiagnosticsManager for SimpleDiagnosticsManager {
    fn register_sensor(&mut self, sensor: Box<dyn Sensor>) -> Result<SensorID, DiagnosticsError> {
        if !self.capability.can_register {
            return Err(DiagnosticsError::PermissionDenied);
        }

        let id = sensor.id();
        let sensor_type = sensor.sensor_type();
        self.sensors.push(Some(sensor));
        self.stats.total_sensors += 1;
        self.stats.active_sensors += 1;
        self.stats.by_type[sensor_type as usize] += 1;
        Ok(id)
    }

    fn unregister_sensor(&mut self, id: SensorID) -> Result<(), DiagnosticsError> {
        if !self.capability.can_register {
            return Err(DiagnosticsError::PermissionDenied);
        }

        let mut index = None;
        let mut sensor_type = SensorType::CPU;

        for (i, sensor_option) in self.sensors.iter().enumerate() {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id {
                    index = Some(i);
                    sensor_type = sensor.sensor_type();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.sensors[i] = None;
            self.stats.total_sensors -= 1;
            self.stats.active_sensors -= 1;
            self.stats.by_type[sensor_type as usize] -= 1;
            Ok(())
        } else {
            Err(DiagnosticsError::SensorNotFound)
        }
    }

    fn read_sensor(&mut self, id: SensorID) -> Result<f64, DiagnosticsError> {
        if !self.capability.can_read {
            return Err(DiagnosticsError::PermissionDenied);
        }

        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == id {
                    return sensor.read();
                }
            }
        }
        Err(DiagnosticsError::SensorNotFound)
    }

    fn read_all(&mut self) -> Result<Vec<(SensorID, f64)>, DiagnosticsError> {
        if !self.capability.can_read {
            return Err(DiagnosticsError::PermissionDenied);
        }

        let mut readings = Vec::new();

        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if let Ok(value) = sensor.read() {
                    readings.push((sensor.id(), value));
                }
            }
        }

        Ok(readings)
    }

    fn get_sensor(&self, id: SensorID) -> Option<&dyn Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id {
                    return Some(sensor.as_ref());
                }
            }
        }
        None
    }

    fn list_sensors(&self, sensor_type: SensorType) -> Vec<SensorID> {
        let mut ids = Vec::new();

        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.sensor_type() == sensor_type {
                    ids.push(sensor.id());
                }
            }
        }

        ids
    }

    fn stats(&self) -> DiagnosticsStats {
        self.stats
    }
}

/// Simple Vec implementation for no_std
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

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
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

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
