#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;
use std::string::{String, ToString};
use std::format;

// Re-export std::vec::Vec for external use
pub use std::vec::Vec;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

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
    #[allow(clippy::new_without_default)]
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
#[derive(Debug, Clone, Copy)]
pub struct DiagnosticsStats {
    pub total_sensors: usize,
    pub active_sensors: usize,
    pub by_type: [usize; 6],
}

impl DiagnosticsStats {
    #[allow(clippy::new_without_default)]
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
    #[allow(clippy::new_without_default)]
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
struct CustomVec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> CustomVec<T> {
    fn new() -> Self {
        CustomVec {
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


impl<T> core::ops::Deref for CustomVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for CustomVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a CustomVec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut CustomVec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

// ============================================================================
// ⚡ BOLT PERFORMANCE ENGINE: KERNEL MEMORY & VFS CACHE PROFILERS
// ============================================================================

/// Analyzes kernel heap fragmentation and allocation latency percentiles.
pub struct KernelMemoryLayoutAnalyzer {
    pub total_heap_bytes: usize,
    pub allocated_bytes: usize,
    pub free_block_count: usize,
    pub largest_free_block_bytes: usize,
    pub latency_samples_ns: Vec<u64>,
}

impl KernelMemoryLayoutAnalyzer {
    pub fn new(total_heap_bytes: usize) -> Self {
        Self {
            total_heap_bytes,
            allocated_bytes: 0,
            free_block_count: 1,
            largest_free_block_bytes: total_heap_bytes,
            latency_samples_ns: Vec::new(),
        }
    }

    pub fn record_allocation_event(&mut self, size_bytes: usize, latency_ns: u64) {
        self.allocated_bytes = self.allocated_bytes.saturating_add(size_bytes);
        self.latency_samples_ns.push(latency_ns);
    }

    pub fn update_fragmentation_state(&mut self, free_blocks: usize, largest_free_bytes: usize) {
        self.free_block_count = free_blocks;
        self.largest_free_block_bytes = largest_free_bytes;
    }

    /// Computes heap fragmentation ratio as a percentage (0.0% = no fragmentation, 100.0% = severely fragmented).
    pub fn fragmentation_score(&self) -> f64 {
        let free_bytes = self.total_heap_bytes.saturating_sub(self.allocated_bytes);
        if free_bytes == 0 || self.largest_free_block_bytes >= free_bytes {
            0.0
        } else {
            (1.0 - (self.largest_free_block_bytes as f64 / free_bytes as f64)) * 100.0
        }
    }

    /// Computes latency percentiles (p50, p95, p99) in nanoseconds.
    pub fn latency_percentiles(&self) -> (u64, u64, u64) {
        if self.latency_samples_ns.is_empty() {
            return (0, 0, 0);
        }
        let mut sorted = self.latency_samples_ns.clone();
        sorted.sort();
        let len = sorted.len();
        let p50 = sorted[(len * 50 / 100).min(len - 1)];
        let p95 = sorted[(len * 95 / 100).min(len - 1)];
        let p99 = sorted[(len * 99 / 100).min(len - 1)];
        (p50, p95, p99)
    }
}

/// Profile entry for filesystem page cache warmth metrics.
#[derive(Debug, Clone)]
pub struct VfsFsCacheStats {
    pub mount_point: String,
    pub hits: u64,
    pub misses: u64,
}

/// Tracks page cache hit rates per filesystem and generates automated optimization suggestions.
pub struct VfsCacheWarmthProfiler {
    pub stats: Vec<VfsFsCacheStats>,
}

impl VfsCacheWarmthProfiler {
    pub fn new() -> Self {
        Self { stats: Vec::new() }
    }

    pub fn record_cache_access(&mut self, mount_point: &str, hit: bool) {
        if let Some(entry) = self.stats.iter_mut().find(|s| s.mount_point == mount_point) {
            if hit {
                entry.hits += 1;
            } else {
                entry.misses += 1;
            }
        } else {
            self.stats.push(VfsFsCacheStats {
                mount_point: mount_point.to_string(),
                hits: if hit { 1 } else { 0 },
                misses: if hit { 0 } else { 1 },
            });
        }
    }

    /// Calculates page cache hit rate for a given mount point (0.0 to 100.0%).
    pub fn hit_rate(&self, mount_point: &str) -> f64 {
        if let Some(entry) = self.stats.iter().find(|s| s.mount_point == mount_point) {
            let total = entry.hits + entry.misses;
            if total == 0 {
                100.0
            } else {
                (entry.hits as f64 / total as f64) * 100.0
            }
        } else {
            100.0
        }
    }

    /// Generates optimization suggestions for filesystems with cache hit rates below 80%.
    pub fn generate_optimization_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        for entry in &self.stats {
            let total = entry.hits + entry.misses;
            if total > 0 {
                let rate = (entry.hits as f64 / total as f64) * 100.0;
                if rate < 80.0 {
                    suggestions.push(format!(
                        "Low page cache hit rate ({:.1}%) on '{}': pre-warm read-ahead buffers",
                        rate, entry.mount_point
                    ));
                }
            }
        }
        suggestions
    }
}

impl Default for VfsCacheWarmthProfiler {
    fn default() -> Self {
        Self::new()
    }
}

// ── Unit Tests ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_memory_layout_analyzer() {
        let mut analyzer = KernelMemoryLayoutAnalyzer::new(1024 * 1024); // 1 MB heap
        analyzer.record_allocation_event(512 * 1024, 150);
        analyzer.record_allocation_event(256 * 1024, 220);
        analyzer.record_allocation_event(128 * 1024, 500);

        analyzer.update_fragmentation_state(4, 64 * 1024); // 4 blocks, largest is 64KB out of 128KB free
        let frag = analyzer.fragmentation_score();
        assert!(frag > 0.0);
        assert!(frag <= 100.0);

        let (p50, p95, p99) = analyzer.latency_percentiles();
        assert!(p50 > 0);
        assert!(p95 >= p50);
        assert!(p99 >= p95);
    }

    #[test]
    fn test_vfs_cache_warmth_profiler() {
        let mut profiler = VfsCacheWarmthProfiler::new();
        for _ in 0..90 {
            profiler.record_cache_access("/", true);
        }
        for _ in 0..10 {
            profiler.record_cache_access("/", false);
        }

        for _ in 0..30 {
            profiler.record_cache_access("/var", true);
        }
        for _ in 0..70 {
            profiler.record_cache_access("/var", false);
        }

        assert_eq!(profiler.hit_rate("/"), 90.0);
        assert_eq!(profiler.hit_rate("/var"), 30.0);

        let suggestions = profiler.generate_optimization_suggestions();
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].contains("/var"));
        assert!(suggestions[0].contains("30.0%"));
    }
}
