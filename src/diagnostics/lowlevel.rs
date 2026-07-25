#![no_std]
#![no_main]

/// OOP-based Low-level Diagnostics Tools for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 16
/// Implements hardware health, SMART, thermal, and power telemetry

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorType { Temperature = 0, Voltage = 1, Current = 2, Power = 3, Fan = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HealthStatus { Healthy = 0, Warning = 1, Critical = 2, Unknown = 3 }

pub trait Sensor {
    fn id(&self) -> SensorID;
    fn sensor_type(&self) -> SensorType;
    fn name(&self) -> &[u8];
    fn read_value(&self) -> i32;
    fn get_unit(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleSensor {
    pub id: SensorID,
    pub sensor_type: AtomicUsize,
    pub name: [u8; 64],
    pub value: AtomicUsize,
    pub unit: [u8; 16],
}

impl SimpleSensor {
    pub fn new(id: SensorID, sensor_type: SensorType, name: &[u8], unit: &[u8]) -> Self {
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
            sensor_type: AtomicUsize::new(sensor_type as usize),
            name: name_array,
            value: AtomicUsize::new(0),
            unit: unit_array,
        }
    }
}

impl Sensor for SimpleSensor {
    fn id(&self) -> SensorID { self.id }
    fn sensor_type(&self) -> SensorType { unsafe { core::mem::transmute(self.sensor_type.load(Ordering::SeqCst)) } }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn read_value(&self) -> i32 { self.value.load(Ordering::SeqCst) as i32 }
    fn get_unit(&self) -> &[u8] {
        let len = self.unit.iter().position(|&b| b == 0).unwrap_or(16);
        &self.unit[..len]
    }
}

pub trait ThermalMonitor {
    fn add_sensor(&mut self, sensor: Box<dyn Sensor>) -> Result<SensorID, ()>;
    fn get_temperature(&self, sensor_id: SensorID) -> Option<i32>;
    fn get_max_temperature(&self) -> i32;
    fn check_thresholds(&self) -> Vec<(SensorID, HealthStatus)>;
}

#[repr(C)]
pub struct SimpleThermalMonitor {
    pub sensors: Vec<Option<Box<dyn Sensor>>>,
    pub next_id: AtomicUsize,
    pub warning_threshold: AtomicUsize,
    pub critical_threshold: AtomicUsize,
}

impl SimpleThermalMonitor {
    pub fn new() -> Self {
        SimpleThermalMonitor {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
            warning_threshold: AtomicUsize::new(75),
            critical_threshold: AtomicUsize::new(90),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let cpu_temp = SimpleSensor::new(self.next_id.fetch_add(1, Ordering::SeqCst), SensorType::Temperature, b"CPU Core 0", b"C");
        cpu_temp.value.store(45, Ordering::SeqCst);
        self.sensors.push(Some(Box::new(cpu_temp)));

        let gpu_temp = SimpleSensor::new(self.next_id.fetch_add(1, Ordering::SeqCst), SensorType::Temperature, b"GPU Core", b"C");
        gpu_temp.value.store(55, Ordering::SeqCst);
        self.sensors.push(Some(Box::new(gpu_temp)));
    }
}

impl ThermalMonitor for SimpleThermalMonitor {
    fn add_sensor(&mut self, sensor: Box<dyn Sensor>) -> Result<SensorID, ()> {
        let id = sensor.id();
        self.sensors.push(Some(sensor));
        Ok(id)
    }

    fn get_temperature(&self, sensor_id: SensorID) -> Option<i32> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id && sensor.sensor_type() == SensorType::Temperature {
                    return Some(sensor.read_value());
                }
            }
        }
        None
    }

    fn get_max_temperature(&self) -> i32 {
        let mut max = 0;
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.sensor_type() == SensorType::Temperature {
                    let val = sensor.read_value();
                    if val > max { max = val; }
                }
            }
        }
        max
    }

    fn check_thresholds(&self) -> Vec<(SensorID, HealthStatus)> {
        let mut results = Vec::new();
        let warning = self.warning_threshold.load(Ordering::SeqCst) as i32;
        let critical = self.critical_threshold.load(Ordering::SeqCst) as i32;

        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.sensor_type() == SensorType::Temperature {
                    let temp = sensor.read_value();
                    let status = if temp >= critical {
                        HealthStatus::Critical
                    } else if temp >= warning {
                        HealthStatus::Warning
                    } else {
                        HealthStatus::Healthy
                    };
                    results.push((sensor.id(), status));
                }
            }
        }
        results
    }
}

pub trait SMARTMonitor {
    fn get_smart_data(&self, device_id: usize) -> Option<SMARTData>;
    fn predict_failure(&self, device_id: usize) -> HealthStatus;
    fn get_attribute(&self, device_id: usize, attribute_id: u8) -> Option<u8>;
}

#[repr(C)]
pub struct SMARTData {
    pub temperature: u8,
    pub reallocated_sectors: u16,
    pub pending_sectors: u16,
    pub power_on_hours: u32,
    pub health_percentage: u8,
}

#[repr(C)]
pub struct SimpleSMARTMonitor {
    pub devices: Vec<(usize, SMARTData)>,
}

impl SimpleSMARTMonitor {
    pub fn new() -> Self {
        SimpleSMARTMonitor {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_id: usize, data: SMARTData) {
        self.devices.push((device_id, data));
    }
}

impl SMARTMonitor for SimpleSMARTMonitor {
    fn get_smart_data(&self, device_id: usize) -> Option<SMARTData> {
        for &(id, ref data) in &self.devices {
            if id == device_id {
                return Some(*data);
            }
        }
        None
    }

    fn predict_failure(&self, device_id: usize) -> HealthStatus {
        if let Some(data) = self.get_smart_data(device_id) {
            if data.reallocated_sectors > 10 || data.pending_sectors > 5 {
                return HealthStatus::Critical;
            } else if data.health_percentage < 80 {
                return HealthStatus::Warning;
            }
        }
        HealthStatus::Healthy
    }

    fn get_attribute(&self, device_id: usize, _attribute_id: u8) -> Option<u8> {
        if let Some(data) = self.get_smart_data(device_id) {
            return Some(data.health_percentage);
        }
        None
    }
}

pub trait PowerTelemetry {
    fn get_power_consumption(&self) -> u32;
    fn get_voltage(&self, rail: &[u8]) -> Option<u32>;
    fn get_current(&self, rail: &[u8]) -> Option<u32>;
    fn calculate_efficiency(&self) -> u32;
}

#[repr(C)]
pub struct SimplePowerTelemetry {
    pub total_power: AtomicUsize,
    pub rails: Vec<([u8; 16], (AtomicUsize, AtomicUsize))>,
}

impl SimplePowerTelemetry {
    pub fn new() -> Self {
        SimplePowerTelemetry {
            total_power: AtomicUsize::new(0),
            rails: Vec::new(),
        }
    }

    pub fn add_rail(&mut self, name: &[u8], voltage: u32, current: u32) {
        let mut name_array = [0u8; 16];
        let name_len = name.len().min(15);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        self.rails.push((name_array, (AtomicUsize::new(voltage as usize), AtomicUsize::new(current as usize))));
    }
}

impl PowerTelemetry for SimplePowerTelemetry {
    fn get_power_consumption(&self) -> u32 {
        self.total_power.load(Ordering::SeqCst) as u32
    }

    fn get_voltage(&self, rail: &[u8]) -> Option<u32> {
        for &(name, (ref voltage, _)) in &self.rails {
            let len = name.iter().position(|&b| b == 0).unwrap_or(16);
            if &name[..len] == rail {
                return Some(voltage.load(Ordering::SeqCst) as u32);
            }
        }
        None
    }

    fn get_current(&self, rail: &[u8]) -> Option<u32> {
        for &(name, (_, ref current)) in &self.rails {
            let len = name.iter().position(|&b| b == 0).unwrap_or(16);
            if &name[..len] == rail {
                return Some(current.load(Ordering::SeqCst) as u32);
            }
        }
        None
    }

    fn calculate_efficiency(&self) -> u32 {
        let total_power = self.total_power.load(Ordering::SeqCst) as u32;
        if total_power == 0 { return 0; }
        let input_power = total_power * 110 / 100;
        if input_power == 0 { return 0; }
        (total_power * 100) / input_power
    }
}

pub trait DiagnosticsReport {
    fn generate_report(&self) -> Vec<u8>;
    fn get_health_summary(&self) -> HealthStatus;
}

#[repr(C)]
pub struct SimpleDiagnosticsReport {
    pub thermal: SimpleThermalMonitor,
    pub smart: SimpleSMARTMonitor,
    pub power: SimplePowerTelemetry,
}

impl SimpleDiagnosticsReport {
    pub fn new(thermal: SimpleThermalMonitor, smart: SimpleSMARTMonitor, power: SimplePowerTelemetry) -> Self {
        SimpleDiagnosticsReport { thermal, smart, power }
    }
}

impl DiagnosticsReport for SimpleDiagnosticsReport {
    fn generate_report(&self) -> Vec<u8> {
        let mut report = Vec::new();

        let header = b"=== SigmaOS Diagnostics Report ===\n";
        for &byte in header { report.push(byte); }

        let thermal_header = b"\nThermal Status:\n";
        for &byte in thermal_header { report.push(byte); }
        let max_temp = self.thermal.get_max_temperature();
        let temp_str = [b'0' + (max_temp / 10) as u8, b'0' + (max_temp % 10) as u8];
        report.push(b' ');
        report.push(b'M');
        report.push(b'a');
        report.push(b'x');
        report.push(b':');
        report.push(b' ');
        report.push(temp_str[0]);
        report.push(temp_str[1]);
        report.push(b'C');
        report.push(b'\n');

        let power_header = b"\nPower Consumption:\n";
        for &byte in power_header { report.push(byte); }
        let power = self.power.get_power_consumption();
        report.push(b' ');
        report.push(b'T');
        report.push(b'o');
        report.push(b't');
        report.push(b'a');
        report.push(b'l');
        report.push(b':');
        report.push(b' ');
        report.push(b'0' + (power / 100) as u8);
        report.push(b'W');
        report.push(b'\n');

        report
    }

    fn get_health_summary(&self) -> HealthStatus {
        let thermal_status = self.thermal.check_thresholds();
        for &(_, status) in &thermal_status {
            if status == HealthStatus::Critical {
                return HealthStatus::Critical;
            }
        }
        HealthStatus::Healthy
    }
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
