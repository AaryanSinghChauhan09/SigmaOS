#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

use core::mem;
/// OOP-based Low-Level Diagnostics Tools for SigmaOS
/// Implements diagnostics using OOP principles with traits and structs
/// No dependency on external diagnostics frameworks
/// Based on Roadmap Item 16: Low-level diagnostics tools
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(target_os = "none"))]
use std::boxed::Box;
#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]
pub struct Box<T: ?Sized>(*mut T);

#[cfg(target_os = "none")]
impl<T: ?Sized> Box<T> {
    pub fn new(val: T) -> Self
    where
        T: Sized,
    {
        let ptr = unsafe { alloc(mem::size_of::<T>()) as *mut T };
        if !ptr.is_null() {
            unsafe {
                core::ptr::write(ptr, val);
            }
        }
        Box(ptr)
    }
}

#[cfg(target_os = "none")]
impl<T: ?Sized> core::ops::Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

#[cfg(target_os = "none")]
impl<T: ?Sized> core::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

#[cfg(target_os = "none")]
impl<T: ?Sized> core::convert::AsRef<T> for Box<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

#[cfg(target_os = "none")]
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

#[cfg(target_os = "none")]
impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
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

    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.len == 0 {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
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

#[cfg(target_os = "none")]
pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

#[cfg(target_os = "none")]
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

#[cfg(target_os = "none")]
pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

#[cfg(target_os = "none")]
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

#[cfg(target_os = "none")]
impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(target_os = "none")]
impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(target_os = "none")]
impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

#[cfg(target_os = "none")]
impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Sensor ID
pub type SensorID = usize;

/// Sensor type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn new(
        id: SensorID,
        name: &[u8],
        sensor_type: SensorType,
        unit: &[u8],
        capability: SensorCapability,
    ) -> Self {
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

        self.value
            .store(Self::f64_to_usize(simulated_value), Ordering::SeqCst);
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

/// Memory Leak Sensor (OOP: Professional diagnostics concrete class)
#[repr(C)]
pub struct MemoryLeakSensor {
    pub id: SensorID,
    pub name: [u8; 64],
    pub sensor_type: SensorType,
    pub used_memory_history: [usize; 8], // Store history of allocations
    pub history_index: usize,
    pub unit: [u8; 16],
    pub capability: SensorCapability,
}

impl MemoryLeakSensor {
    pub fn new(id: SensorID, name: &[u8], capability: SensorCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        MemoryLeakSensor {
            id,
            name: name_array,
            sensor_type: SensorType::Memory,
            used_memory_history: [0; 8],
            history_index: 0,
            unit: *b"Bytes\0\0\0\0\0\0\0\0\0\0\0",
            capability,
        }
    }
}

impl Sensor for MemoryLeakSensor {
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
        // Simulated used memory with upward leak trend
        let base_val = 1024 * 1024;
        let leaked_increment = self.history_index * 256 * 1024;
        let value = (base_val + leaked_increment) as f64;

        self.used_memory_history[self.history_index % 8] = value as usize;
        self.history_index += 1;

        Ok(value)
    }
    fn info(&self) -> SensorInfo {
        SensorInfo {
            id: self.id,
            name: self.name,
            sensor_type: self.sensor_type,
            value: if self.history_index > 0 {
                self.used_memory_history[(self.history_index - 1) % 8] as f64
            } else {
                0.0
            },
            unit: self.unit,
            capability: self.capability,
        }
    }
}

/// CPU Cache Profiler Sensor (OOP: Professional diagnostics concrete class)
#[repr(C)]
pub struct CpuCacheProfilerSensor {
    pub id: SensorID,
    pub name: [u8; 64],
    pub sensor_type: SensorType,
    pub cache_misses: AtomicUsize,
    pub cache_accesses: AtomicUsize,
    pub unit: [u8; 16],
    pub capability: SensorCapability,
}

impl CpuCacheProfilerSensor {
    pub fn new(id: SensorID, name: &[u8], capability: SensorCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        CpuCacheProfilerSensor {
            id,
            name: name_array,
            sensor_type: SensorType::CPU,
            cache_misses: AtomicUsize::new(0),
            cache_accesses: AtomicUsize::new(0),
            unit: *b"Ratio\0\0\0\0\0\0\0\0\0\0\0",
            capability,
        }
    }
}

impl Sensor for CpuCacheProfilerSensor {
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

        let misses = self.cache_misses.fetch_add(45, Ordering::SeqCst) + 45;
        let accesses = self.cache_accesses.fetch_add(1000, Ordering::SeqCst) + 1000;

        // Cache miss ratio
        let ratio = (misses as f64) / (accesses as f64);
        Ok(ratio)
    }
    fn info(&self) -> SensorInfo {
        let misses = self.cache_misses.load(Ordering::SeqCst);
        let accesses = self.cache_accesses.load(Ordering::SeqCst);
        let ratio = if accesses > 0 {
            (misses as f64) / (accesses as f64)
        } else {
            0.0
        };
        SensorInfo {
            id: self.id,
            name: self.name,
            sensor_type: self.sensor_type,
            value: ratio,
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
#[derive(Debug, Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_leak_sensor() {
        let capability = SensorCapability::full();
        let mut sensor = MemoryLeakSensor::new(201, b"MemLeakSensor", capability);
        assert_eq!(sensor.id(), 201);
        assert_eq!(sensor.name(), b"MemLeakSensor");
        assert_eq!(sensor.sensor_type(), SensorType::Memory);

        // Read sensor several times and check trend upward
        let r1 = sensor.read().unwrap();
        let r2 = sensor.read().unwrap();
        assert!(r2 > r1);

        let info = sensor.info();
        assert_eq!(info.value, r2);
    }

    #[test]
    fn test_cpu_cache_profiler_sensor() {
        let capability = SensorCapability::full();
        let mut sensor = CpuCacheProfilerSensor::new(202, b"CpuCacheSensor", capability);
        assert_eq!(sensor.id(), 202);
        assert_eq!(sensor.name(), b"CpuCacheSensor");
        assert_eq!(sensor.sensor_type(), SensorType::CPU);

        let ratio = sensor.read().unwrap();
        assert!(ratio > 0.0);

        let info = sensor.info();
        assert_eq!(info.value, ratio);
    }

    #[test]
    fn test_diagnostics_manager_registration() {
        let mut manager = SimpleDiagnosticsManager::new(ManagerCapability::full());
        let cap = SensorCapability::full();
        let leak_sensor = Box::new(MemoryLeakSensor::new(301, b"Leak1", cap));
        let cache_sensor = Box::new(CpuCacheProfilerSensor::new(302, b"Cache1", cap));

        assert!(manager.register_sensor(leak_sensor).is_ok());
        assert!(manager.register_sensor(cache_sensor).is_ok());

        assert_eq!(manager.stats().total_sensors, 2);

        // Read sensors through manager
        let r1 = manager.read_sensor(301).unwrap();
        assert!(r1 > 0.0);

        let r2 = manager.read_sensor(302).unwrap();
        assert!(r2 > 0.0);
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
