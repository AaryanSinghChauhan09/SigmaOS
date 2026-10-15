//! SigmaMonitor - Hardware Monitoring Daemon
//!
//! Inspired by lm-sensors, this module provides comprehensive hardware monitoring
//! including temperature sensors, voltage sensors, fan speed monitoring, and
//! hardware health tracking.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Sensor type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorType {
    Temperature,
    Voltage,
    Fan,
    Power,
    Current,
    Unknown,
}

/// Sensor state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorState {
    Normal,
    Warning,
    Critical,
    Unknown,
}

/// Hardware sensor data structure
#[derive(Debug, Clone)]
pub struct HardwareSensor {
    pub sensor_id: String,
    pub sensor_name: String,
    pub sensor_type: SensorType,
    pub current_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub unit: String,
    pub state: SensorState,
}

impl HardwareSensor {
    /// Create a new hardware sensor
    pub fn new(
        sensor_id: String,
        sensor_name: String,
        sensor_type: SensorType,
        current_value: f64,
        min_value: f64,
        max_value: f64,
        unit: String,
    ) -> Self {
        let state = Self::calculate_state(current_value, min_value, max_value);
        Self {
            sensor_id,
            sensor_name,
            sensor_type,
            current_value,
            min_value,
            max_value,
            unit,
            state,
        }
    }

    /// Calculate sensor state based on current value
    fn calculate_state(current: f64, min: f64, max: f64) -> SensorState {
        if current < min * 0.9 || current > max * 1.1 {
            SensorState::Critical
        } else if current < min * 0.95 || current > max * 1.05 {
            SensorState::Warning
        } else {
            SensorState::Normal
        }
    }

    /// Update sensor value and recalculate state
    pub fn update_value(&mut self, new_value: f64) {
        self.current_value = new_value;
        self.state = Self::calculate_state(new_value, self.min_value, self.max_value);
    }

    /// Check if sensor is in warning or critical state
    pub fn is_alert(&self) -> bool {
        self.state == SensorState::Warning || self.state == SensorState::Critical
    }
}

/// Hardware monitor manager
pub struct SigmaHwMon {
    sensors: BTreeMap<String, HardwareSensor>,
    running: AtomicBool,
}

impl SigmaHwMon {
    /// Create a new hardware monitor
    pub fn new() -> Self {
        Self {
            sensors: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the hardware monitor
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the hardware monitor
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the monitor is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a hardware sensor
    pub fn add_sensor(&mut self, sensor: HardwareSensor) {
        self.sensors.insert(sensor.sensor_id.clone(), sensor);
    }

    /// Remove a hardware sensor
    pub fn remove_sensor(&mut self, sensor_id: &str) -> Option<HardwareSensor> {
        self.sensors.remove(sensor_id)
    }

    /// Get a sensor by ID
    pub fn get_sensor(&self, sensor_id: &str) -> Option<&HardwareSensor> {
        self.sensors.get(sensor_id)
    }

    /// Update a sensor value
    pub fn update_sensor(&mut self, sensor_id: &str, new_value: f64) -> bool {
        if let Some(sensor) = self.sensors.get_mut(sensor_id) {
            sensor.update_value(new_value);
            true
        } else {
            false
        }
    }

    /// List all sensors
    pub fn list_sensors(&self) -> Vec<&HardwareSensor> {
        self.sensors.values().collect()
    }

    /// List sensors by type
    pub fn list_sensors_by_type(&self, sensor_type: SensorType) -> Vec<&HardwareSensor> {
        self.sensors
            .values()
            .filter(|s| s.sensor_type == sensor_type)
            .collect()
    }

    /// List sensors by state
    pub fn list_sensors_by_state(&self, state: SensorState) -> Vec<&HardwareSensor> {
        self.sensors
            .values()
            .filter(|s| s.state == state)
            .collect()
    }

    /// List sensors in alert state (warning or critical)
    pub fn list_alert_sensors(&self) -> Vec<&HardwareSensor> {
        self.sensors.values().filter(|s| s.is_alert()).collect()
    }

    /// Get sensor count
    pub fn sensor_count(&self) -> usize {
        self.sensors.len()
    }

    /// Get average temperature from all temperature sensors
    pub fn average_temperature(&self) -> Option<f64> {
        let temp_sensors: Vec<&HardwareSensor> = self.list_sensors_by_type(SensorType::Temperature);
        if temp_sensors.is_empty() {
            return None;
        }
        let sum: f64 = temp_sensors.iter().map(|s| s.current_value).sum();
        Some(sum / temp_sensors.len() as f64)
    }

    /// Get maximum temperature from all temperature sensors
    pub fn max_temperature(&self) -> Option<f64> {
        let temp_sensors: Vec<&HardwareSensor> = self.list_sensors_by_type(SensorType::Temperature);
        if temp_sensors.is_empty() {
            return None;
        }
        temp_sensors.iter().map(|s| s.current_value).fold(
            f64::NEG_INFINITY,
            |acc, &val| if val > acc { val } else { acc },
        )
    }
}

impl Default for SigmaHwMon {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaHwMon instance (placeholder for actual global state)
static mut SIGMAHWMON_INSTANCE: Option<SigmaHwMon> = None;

/// Initialize the global SigmaHwMon instance
pub fn init_sigmahwmon() {
    unsafe {
        SIGMAHWMON_INSTANCE = Some(SigmaHwMon::new());
    }
}

/// Get the global SigmaHwMon instance
pub fn get_sigmahwmon() -> Option<&'static SigmaHwMon> {
    unsafe {
        SIGMAHWMON_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaHwMon instance
pub fn get_sigmahwmon_mut() -> Option<&'static mut SigmaHwMon> {
    unsafe {
        SIGMAHWMON_INSTANCE.as_mut()
    }
}

/// Convenience function to add a sensor
pub fn add_hw_sensor(sensor: HardwareSensor) {
    if let Some(hwmon) = get_sigmahwmon_mut() {
        hwmon.add_sensor(sensor);
    }
}

/// Convenience function to update a sensor value
pub fn update_hw_sensor(sensor_id: &str, new_value: f64) -> bool {
    if let Some(hwmon) = get_sigmahwmon_mut() {
        hwmon.update_sensor(sensor_id, new_value)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_creation() {
        let sensor = HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        );

        assert_eq!(sensor.sensor_id, "cpu_temp");
        assert_eq!(sensor.sensor_name, "CPU Temperature");
        assert_eq!(sensor.sensor_type, SensorType::Temperature);
        assert_eq!(sensor.current_value, 45.0);
        assert_eq!(sensor.state, SensorState::Normal);
    }

    #[test]
    fn test_sensor_state_calculation() {
        let normal_sensor = HardwareSensor::new(
            "temp1".to_string(),
            "Temperature 1".to_string(),
            SensorType::Temperature,
            50.0,
            0.0,
            100.0,
            "°C".to_string(),
        );
        assert_eq!(normal_sensor.state, SensorState::Normal);

        let warning_sensor = HardwareSensor::new(
            "temp2".to_string(),
            "Temperature 2".to_string(),
            SensorType::Temperature,
            95.0,
            0.0,
            100.0,
            "°C".to_string(),
        );
        assert_eq!(warning_sensor.state, SensorState::Warning);

        let critical_sensor = HardwareSensor::new(
            "temp3".to_string(),
            "Temperature 3".to_string(),
            SensorType::Temperature,
            110.0,
            0.0,
            100.0,
            "°C".to_string(),
        );
        assert_eq!(critical_sensor.state, SensorState::Critical);
    }

    #[test]
    fn test_sensor_value_update() {
        let mut sensor = HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        );

        sensor.update_value(55.0);
        assert_eq!(sensor.current_value, 55.0);
        assert_eq!(sensor.state, SensorState::Normal);

        sensor.update_value(95.0);
        assert_eq!(sensor.current_value, 95.0);
        assert_eq!(sensor.state, SensorState::Warning);
    }

    #[test]
    fn test_sensor_alert_check() {
        let normal_sensor = HardwareSensor::new(
            "temp1".to_string(),
            "Temperature 1".to_string(),
            SensorType::Temperature,
            50.0,
            0.0,
            100.0,
            "°C".to_string(),
        );
        assert!(!normal_sensor.is_alert());

        let warning_sensor = HardwareSensor::new(
            "temp2".to_string(),
            "Temperature 2".to_string(),
            SensorType::Temperature,
            95.0,
            0.0,
            100.0,
            "°C".to_string(),
        );
        assert!(warning_sensor.is_alert());
    }

    #[test]
    fn test_hwmon_creation() {
        let hwmon = SigmaHwMon::new();
        assert!(!hwmon.is_running());
        assert_eq!(hwmon.sensor_count(), 0);
    }

    #[test]
    fn test_hwmon_start_stop() {
        let mut hwmon = SigmaHwMon::new();
        hwmon.start();
        assert!(hwmon.is_running());
        hwmon.stop();
        assert!(!hwmon.is_running());
    }

    #[test]
    fn test_hwmon_add_sensor() {
        let mut hwmon = SigmaHwMon::new();
        let sensor = HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        );

        hwmon.add_sensor(sensor);
        assert_eq!(hwmon.sensor_count(), 1);
        assert!(hwmon.get_sensor("cpu_temp").is_some());
    }

    #[test]
    fn test_hwmon_remove_sensor() {
        let mut hwmon = SigmaHwMon::new();
        let sensor = HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        );

        hwmon.add_sensor(sensor);
        assert_eq!(hwmon.sensor_count(), 1);

        let removed = hwmon.remove_sensor("cpu_temp");
        assert!(removed.is_some());
        assert_eq!(hwmon.sensor_count(), 0);
    }

    #[test]
    fn test_hwmon_update_sensor() {
        let mut hwmon = SigmaHwMon::new();
        let sensor = HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        );

        hwmon.add_sensor(sensor);
        let updated = hwmon.update_sensor("cpu_temp", 55.0);
        assert!(updated);

        let sensor = hwmon.get_sensor("cpu_temp").unwrap();
        assert_eq!(sensor.current_value, 55.0);
    }

    #[test]
    fn test_hwmon_list_sensors() {
        let mut hwmon = SigmaHwMon::new();

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_fan".to_string(),
            "CPU Fan".to_string(),
            SensorType::Fan,
            1200.0,
            0.0,
            3000.0,
            "RPM".to_string(),
        ));

        let sensors = hwmon.list_sensors();
        assert_eq!(sensors.len(), 2);
    }

    #[test]
    fn test_hwmon_list_sensors_by_type() {
        let mut hwmon = SigmaHwMon::new();

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        hwmon.add_sensor(HardwareSensor::new(
            "gpu_temp".to_string(),
            "GPU Temperature".to_string(),
            SensorType::Temperature,
            60.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_fan".to_string(),
            "CPU Fan".to_string(),
            SensorType::Fan,
            1200.0,
            0.0,
            3000.0,
            "RPM".to_string(),
        ));

        let temp_sensors = hwmon.list_sensors_by_type(SensorType::Temperature);
        assert_eq!(temp_sensors.len(), 2);

        let fan_sensors = hwmon.list_sensors_by_type(SensorType::Fan);
        assert_eq!(fan_sensors.len(), 1);
    }

    #[test]
    fn test_hwmon_list_sensors_by_state() {
        let mut hwmon = SigmaHwMon::new();

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        hwmon.add_sensor(HardwareSensor::new(
            "gpu_temp".to_string(),
            "GPU Temperature".to_string(),
            SensorType::Temperature,
            95.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        let normal_sensors = hwmon.list_sensors_by_state(SensorState::Normal);
        assert_eq!(normal_sensors.len(), 1);

        let warning_sensors = hwmon.list_sensors_by_state(SensorState::Warning);
        assert_eq!(warning_sensors.len(), 1);
    }

    #[test]
    fn test_hwmon_list_alert_sensors() {
        let mut hwmon = SigmaHwMon::new();

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        hwmon.add_sensor(HardwareSensor::new(
            "gpu_temp".to_string(),
            "GPU Temperature".to_string(),
            SensorType::Temperature,
            95.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        let alert_sensors = hwmon.list_alert_sensors();
        assert_eq!(alert_sensors.len(), 1);
    }

    #[test]
    fn test_hwmon_average_temperature() {
        let mut hwmon = SigmaHwMon::new();

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        hwmon.add_sensor(HardwareSensor::new(
            "gpu_temp".to_string(),
            "GPU Temperature".to_string(),
            SensorType::Temperature,
            60.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        let avg = hwmon.average_temperature();
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), 52.5);
    }

    #[test]
    fn test_hwmon_max_temperature() {
        let mut hwmon = SigmaHwMon::new();

        hwmon.add_sensor(HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        hwmon.add_sensor(HardwareSensor::new(
            "gpu_temp".to_string(),
            "GPU Temperature".to_string(),
            SensorType::Temperature,
            60.0,
            0.0,
            100.0,
            "°C".to_string(),
        ));

        let max = hwmon.max_temperature();
        assert!(max.is_some());
        assert_eq!(max.unwrap(), 60.0);
    }

    #[test]
    fn test_global_instance() {
        init_sigmahwmon();

        let hwmon = get_sigmahwmon();
        assert!(hwmon.is_some());

        let mut hwmon_mut = get_sigmahwmon_mut();
        assert!(hwmon_mut.is_some());

        if let Some(hwmon) = hwmon_mut {
            hwmon.start();
            assert!(hwmon.is_running());
        }
    }

    #[test]
    fn test_convenience_add_sensor() {
        init_sigmahwmon();

        let sensor = HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        );

        add_hw_sensor(sensor);

        let hwmon = get_sigmahwmon().unwrap();
        assert_eq!(hwmon.sensor_count(), 1);
    }

    #[test]
    fn test_convenience_update_sensor() {
        init_sigmahwmon();

        let sensor = HardwareSensor::new(
            "cpu_temp".to_string(),
            "CPU Temperature".to_string(),
            SensorType::Temperature,
            45.0,
            0.0,
            100.0,
            "°C".to_string(),
        );

        add_hw_sensor(sensor);

        let updated = update_hw_sensor("cpu_temp", 55.0);
        assert!(updated);

        let hwmon = get_sigmahwmon().unwrap();
        let sensor = hwmon.get_sensor("cpu_temp").unwrap();
        assert_eq!(sensor.current_value, 55.0);
    }
}
