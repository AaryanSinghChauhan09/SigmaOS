//! # ACPI Power Management
//!
//! Advanced Configuration and Power Interface for system power control.
//! Inspired by Linux drivers/acpi/ and FreeBSD sys/contrib/dev/acpica/.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

/// ACPI sleep states (S-states)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AcpiSleepState {
    S0 = 0,  // Working (fully on)
    S1 = 1,  // CPU stopped, RAM powered
    S2 = 2,  // CPU off, dirty cache, RAM powered
    S3 = 3,  // Suspend to RAM
    S4 = 4,  // Suspend to disk (hibernate)
    S5 = 5,  // Soft off (power button can wake)
}

/// ACPI performance states (P-states) for CPU frequency scaling
#[derive(Debug, Clone, Copy)]
pub struct AcpiPState {
    pub frequency: u32,    // MHz
    pub power: u32,        // mW
    pub latency: u32,      // μs
}

/// ACPI CPU throttling states (T-states)
#[derive(Debug, Clone, Copy)]
pub struct AcpiTState {
    pub percent: u8,       // Throttle percentage (100 = full speed)
    pub latency: u32,      // μs
    pub power: u32,        // mW
}

/// ACPI device power states (D-states)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AcpiDevicePowerState {
    D0 = 0,  // Fully on
    D1 = 1,  // Intermediate state
    D2 = 2,  // Intermediate state
    D3Hot = 3,  // Off, but responsive to software
    D3Cold = 4, // Off, requires full reinitialization
}

/// CPU frequency governor types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuGovernor {
    Performance = 0,   // Maximum frequency always
    Powersave = 1,     // Minimum frequency always
    Ondemand = 2,      // Dynamic based on load
    Conservative = 3,  // Gradual frequency changes
    Schedutil = 4,     // Scheduler-integrated
    Userspace = 5,     // User-controlled
}

/// Battery information
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub present: bool,
    pub charging: bool,
    pub discharging: bool,
    pub capacity_percent: u8,      // 0-100%
    pub capacity_remaining: u32,   // mWh
    pub capacity_full: u32,        // mWh
    pub capacity_design: u32,      // mWh
    pub voltage: u32,              // mV
    pub current: i32,              // mA (positive = charging)
    pub temperature: i16,          // 0.1°K
    pub cycle_count: u32,
}

/// AC adapter information
#[derive(Debug, Clone, Copy)]
pub struct AcAdapterInfo {
    pub present: bool,
    pub online: bool,
}

/// Thermal zone information
#[derive(Debug, Clone)]
pub struct ThermalZone {
    pub temperature: i32,          // 0.1°K
    pub critical: Option<i32>,     // Critical temp
    pub hot: Option<i32>,          // Hot temp
    pub passive: Option<i32>,      // Passive cooling
    pub active: Vec<i32>,          // Active cooling thresholds
    pub polling_freq: u32,         // ms
}

/// Cooling device
#[derive(Debug, Clone)]
pub struct CoolingDevice {
    pub device_type: CoolingDeviceType,
    pub max_state: u32,
    pub current_state: AtomicU32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoolingDeviceType {
    Fan,
    ProcessorThrottle,
    LiquidCooling,
}

impl CoolingDevice {
    pub fn new(device_type: CoolingDeviceType, max_state: u32) -> Self {
        Self {
            device_type,
            max_state,
            current_state: AtomicU32::new(0),
        }
    }
    
    pub fn set_state(&self, state: u32) -> Result<(), AcpiError> {
        if state > self.max_state {
            return Err(AcpiError::InvalidState);
        }
        
        self.current_state.store(state, Ordering::Release);
        // In production: program hardware registers
        Ok(())
    }
    
    pub fn get_state(&self) -> u32 {
        self.current_state.load(Ordering::Acquire)
    }
}

/// ACPI power manager
pub struct AcpiPowerManager {
    current_sleep_state: AtomicU32,
    cpu_governor: AtomicU32,
    available_p_states: Vec<AcpiPState>,
    current_p_state: AtomicU32,
    available_t_states: Vec<AcpiTState>,
    battery: Option<BatteryInfo>,
    ac_adapter: AcAdapterInfo,
    thermal_zones: Vec<ThermalZone>,
    cooling_devices: Vec<CoolingDevice>,
    lid_open: AtomicBool,
    power_button_pressed: AtomicBool,
}

impl AcpiPowerManager {
    pub fn new() -> Self {
        Self {
            current_sleep_state: AtomicU32::new(AcpiSleepState::S0 as u32),
            cpu_governor: AtomicU32::new(CpuGovernor::Ondemand as u32),
            available_p_states: Vec::new(),
            current_p_state: AtomicU32::new(0),
            available_t_states: Vec::new(),
            battery: None,
            ac_adapter: AcAdapterInfo {
                present: false,
                online: false,
            },
            thermal_zones: Vec::new(),
            cooling_devices: Vec::new(),
            lid_open: AtomicBool::new(true),
            power_button_pressed: AtomicBool::new(false),
        }
    }
    
    /// Enter sleep state
    pub fn enter_sleep_state(&self, state: AcpiSleepState) -> Result<(), AcpiError> {
        match state {
            AcpiSleepState::S0 => Ok(()), // Already awake
            AcpiSleepState::S3 => {
                // Suspend to RAM
                // 1. Freeze processes
                // 2. Suspend devices
                // 3. Disable non-boot CPUs
                // 4. Enter low-power state
                self.current_sleep_state.store(state as u32, Ordering::Release);
                Ok(())
            }
            AcpiSleepState::S4 => {
                // Hibernate (suspend to disk)
                // 1. Create hibernation image
                // 2. Write to swap
                // 3. Power off
                Err(AcpiError::NotImplemented)
            }
            AcpiSleepState::S5 => {
                // Shutdown
                self.current_sleep_state.store(state as u32, Ordering::Release);
                Ok(())
            }
            _ => Err(AcpiError::UnsupportedState)
        }
    }
    
    /// Wake from sleep state
    pub fn wake(&self) -> Result<(), AcpiError> {
        let current = self.current_sleep_state.load(Ordering::Acquire);
        
        if current == AcpiSleepState::S0 as u32 {
            return Ok(()); // Already awake
        }
        
        // 1. Enable CPUs
        // 2. Resume devices
        // 3. Thaw processes
        
        self.current_sleep_state.store(AcpiSleepState::S0 as u32, Ordering::Release);
        Ok(())
    }
    
    /// Set CPU frequency governor
    pub fn set_governor(&self, governor: CpuGovernor) -> Result<(), AcpiError> {
        self.cpu_governor.store(governor as u32, Ordering::Release);
        
        match governor {
            CpuGovernor::Performance => {
                // Set to maximum P-state
                if !self.available_p_states.is_empty() {
                    self.current_p_state.store(0, Ordering::Release);
                }
            }
            CpuGovernor::Powersave => {
                // Set to minimum P-state
                let min_state = self.available_p_states.len().saturating_sub(1);
                self.current_p_state.store(min_state as u32, Ordering::Release);
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Get current CPU frequency
    pub fn get_cpu_frequency(&self) -> Option<u32> {
        let p_state = self.current_p_state.load(Ordering::Acquire) as usize;
        self.available_p_states.get(p_state).map(|p| p.frequency)
    }
    
    /// Set CPU P-state
    pub fn set_p_state(&self, state: u32) -> Result<(), AcpiError> {
        if state as usize >= self.available_p_states.len() {
            return Err(AcpiError::InvalidState);
        }
        
        self.current_p_state.store(state, Ordering::Release);
        // In production: write to MSR or ACPI registers
        Ok(())
    }
    
    /// Register P-states
    pub fn register_p_states(&mut self, states: Vec<AcpiPState>) {
        self.available_p_states = states;
    }
    
    /// Get battery status
    pub fn get_battery_info(&self) -> Option<&BatteryInfo> {
        self.battery.as_ref()
    }
    
    /// Update battery information
    pub fn update_battery(&mut self, info: BatteryInfo) {
        self.battery = Some(info);
    }
    
    /// Get AC adapter status
    pub fn get_ac_adapter_info(&self) -> AcAdapterInfo {
        self.ac_adapter
    }
    
    /// Set AC adapter status
    pub fn set_ac_adapter(&mut self, present: bool, online: bool) {
        self.ac_adapter = AcAdapterInfo { present, online };
    }
    
    /// Get thermal zone temperature
    pub fn get_temperature(&self, zone: usize) -> Option<i32> {
        self.thermal_zones.get(zone).map(|tz| tz.temperature)
    }
    
    /// Add thermal zone
    pub fn add_thermal_zone(&mut self, zone: ThermalZone) {
        self.thermal_zones.push(zone);
    }
    
    /// Add cooling device
    pub fn add_cooling_device(&mut self, device: CoolingDevice) {
        self.cooling_devices.push(device);
    }
    
    /// Handle thermal event
    pub fn handle_thermal_event(&mut self, zone: usize) -> Result<(), AcpiError> {
        let tz = self.thermal_zones.get(zone).ok_or(AcpiError::InvalidZone)?;
        let temp = tz.temperature;
        
        // Check thresholds
        if let Some(critical) = tz.critical {
            if temp >= critical {
                // Critical temperature - emergency shutdown
                return self.enter_sleep_state(AcpiSleepState::S5);
            }
        }
        
        if let Some(hot) = tz.hot {
            if temp >= hot {
                // Hot temperature - aggressive cooling
                for device in &self.cooling_devices {
                    device.set_state(device.max_state)?;
                }
            }
        }
        
        // Active cooling based on active thresholds
        for (i, &threshold) in tz.active.iter().enumerate() {
            if temp >= threshold {
                if let Some(device) = self.cooling_devices.get(i) {
                    let state = ((temp - threshold) as u32 * device.max_state) / 1000;
                    device.set_state(state.min(device.max_state))?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle lid event
    pub fn on_lid_event(&self, open: bool) {
        self.lid_open.store(open, Ordering::Release);
        
        if !open {
            // Lid closed - optionally suspend
            // Policy decision: suspend on lid close if on battery
            if self.battery.as_ref().map(|b| b.discharging).unwrap_or(false) {
                let _ = self.enter_sleep_state(AcpiSleepState::S3);
            }
        }
    }
    
    /// Handle power button event
    pub fn on_power_button(&self) {
        self.power_button_pressed.store(true, Ordering::Release);
        // In production: signal userspace or initiate shutdown
    }
    
    /// Get estimated battery time remaining
    pub fn get_battery_time_remaining(&self) -> Option<u32> {
        let battery = self.battery.as_ref()?;
        
        if battery.current == 0 {
            return None;
        }
        
        if battery.discharging {
            // Time in minutes
            let time = (battery.capacity_remaining as f32 / battery.current.abs() as f32) * 60.0;
            Some(time as u32)
        } else if battery.charging {
            let remaining = battery.capacity_full - battery.capacity_remaining;
            let time = (remaining as f32 / battery.current as f32) * 60.0;
            Some(time as u32)
        } else {
            None
        }
    }
}

/// ACPI errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiError {
    UnsupportedState,
    InvalidState,
    InvalidZone,
    NotImplemented,
    HardwareError,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_acpi_power_manager() {
        let pm = AcpiPowerManager::new();
        assert_eq!(pm.current_sleep_state.load(Ordering::Acquire), AcpiSleepState::S0 as u32);
    }
    
    #[test]
    fn test_cooling_device() {
        let device = CoolingDevice::new(CoolingDeviceType::Fan, 10);
        assert_eq!(device.get_state(), 0);
        
        assert!(device.set_state(5).is_ok());
        assert_eq!(device.get_state(), 5);
        
        assert!(device.set_state(11).is_err());
    }
    
    #[test]
    fn test_battery_time_calculation() {
        let mut pm = AcpiPowerManager::new();
        
        let battery = BatteryInfo {
            present: true,
            charging: false,
            discharging: true,
            capacity_percent: 50,
            capacity_remaining: 5000, // 5000 mWh
            capacity_full: 10000,
            capacity_design: 10000,
            voltage: 11100, // 11.1V
            current: -1000, // -1000 mA (discharging)
            temperature: 3000, // 300K
            cycle_count: 100,
        };
        
        pm.update_battery(battery);
        
        // 5000 mWh / 1000 mA = 5 hours = 300 minutes
        let time = pm.get_battery_time_remaining();
        assert_eq!(time, Some(300));
    }
}
