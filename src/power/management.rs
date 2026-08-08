#![no_std]
#![no_main]

extern crate alloc;

/// OOP-based Power Management Stack for SigmaOS
/// Based on 100-Improvement-Ideas.md #15: Battery saver mode
/// Implements advanced power profiles, CPU governor tuning, thermal management,
/// and adaptive power saving for extended battery life

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PowerProfileID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfileType { Performance = 0, Balanced = 1, PowerSaver = 2, Custom = 3 }

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPUGovernorType { Performance = 0, Ondemand = 1, Conservative = 2, Powersave = 3, Userspace = 4, Balanced = 5 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PowerError { Success = 0, InvalidProfile = 1, GovernorFailed = 2, ThermalCritical = 3 }

pub trait PowerProfile {
    fn id(&self) -> PowerProfileID;
    fn name(&self) -> &[u8];
    fn profile_type(&self) -> PowerProfileType;
    fn cpu_governor(&self) -> CPUGovernorType;
    fn max_cpu_freq(&self) -> usize;
    fn min_cpu_freq(&self) -> usize;
}

#[repr(C)]
pub struct SimplePowerProfile {
    pub id: PowerProfileID,
    pub name: [u8; 32],
    pub profile_type: AtomicUsize,
    pub cpu_governor: AtomicUsize,
    pub max_cpu_freq: AtomicUsize,
    pub min_cpu_freq: AtomicUsize,
}

impl SimplePowerProfile {
    pub fn new(id: PowerProfileID, name: &[u8], profile_type: PowerProfileType, governor: CPUGovernorType) -> Self {
        let mut name_array = [0u8; 32];
        let name_len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimplePowerProfile {
            id,
            name: name_array,
            profile_type: AtomicUsize::new(profile_type as usize),
            cpu_governor: AtomicUsize::new(governor as usize),
            max_cpu_freq: AtomicUsize::new(3500000),
            min_cpu_freq: AtomicUsize::new(800000),
        }
    }
}

impl PowerProfile for SimplePowerProfile {
    fn id(&self) -> PowerProfileID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        &self.name[..len]
    }
    fn profile_type(&self) -> PowerProfileType { unsafe { core::mem::transmute(self.profile_type.load(Ordering::SeqCst)) } }
    fn cpu_governor(&self) -> CPUGovernorType { unsafe { core::mem::transmute(self.cpu_governor.load(Ordering::SeqCst)) } }
    fn max_cpu_freq(&self) -> usize { self.max_cpu_freq.load(Ordering::SeqCst) }
    fn min_cpu_freq(&self) -> usize { self.min_cpu_freq.load(Ordering::SeqCst) }
}

pub trait CPUGovernor {
    fn set_governor(&mut self, governor: CPUGovernorType) -> Result<(), PowerError>;
    fn get_governor(&self) -> CPUGovernorType;
    fn set_frequency(&mut self, freq_khz: usize) -> Result<(), PowerError>;
    fn get_frequency(&self) -> usize;
}

#[repr(C)]
pub struct SimpleCPUGovernor {
    pub current_governor: AtomicUsize,
    pub current_freq: AtomicUsize,
    pub max_freq: AtomicUsize,
    pub min_freq: AtomicUsize,
}

impl SimpleCPUGovernor {
    pub fn new() -> Self {
        SimpleCPUGovernor {
            current_governor: AtomicUsize::new(CPUGovernorType::Balanced as usize),
            current_freq: AtomicUsize::new(2000000),
            max_freq: AtomicUsize::new(3500000),
            min_freq: AtomicUsize::new(800000),
        }
    }
}

impl CPUGovernor for SimpleCPUGovernor {
    fn set_governor(&mut self, governor: CPUGovernorType) -> Result<(), PowerError> {
        self.current_governor.store(governor as usize, Ordering::SeqCst);
        match governor {
            CPUGovernorType::Performance => self.current_freq.store(self.max_freq.load(Ordering::SeqCst), Ordering::SeqCst),
            CPUGovernorType::Powersave => self.current_freq.store(self.min_freq.load(Ordering::SeqCst), Ordering::SeqCst),
            CPUGovernorType::Balanced => self.current_freq.store(2000000, Ordering::SeqCst),
            _ => self.current_freq.store(1500000, Ordering::SeqCst),
        }
        Ok(())
    }

    fn get_governor(&self) -> CPUGovernorType { unsafe { core::mem::transmute(self.current_governor.load(Ordering::SeqCst)) } }

    fn set_frequency(&mut self, freq_khz: usize) -> Result<(), PowerError> {
        let max = self.max_freq.load(Ordering::SeqCst);
        let min = self.min_freq.load(Ordering::SeqCst);
        if freq_khz < min || freq_khz > max {
            return Err(PowerError::InvalidProfile);
        }
        self.current_freq.store(freq_khz, Ordering::SeqCst);
        Ok(())
    }

    fn get_frequency(&self) -> usize { self.current_freq.load(Ordering::SeqCst) }
}

pub trait ThermalManager {
    fn get_temperature(&self) -> i32;
    fn set_threshold(&mut self, temp_celsius: i32);
    fn get_threshold(&self) -> i32;
    fn is_critical(&self) -> bool;
}

#[repr(C)]
pub struct SimpleThermalManager {
    pub current_temp: AtomicUsize,
    pub critical_threshold: AtomicUsize,
    pub warning_threshold: AtomicUsize,
}

impl SimpleThermalManager {
    pub fn new() -> Self {
        SimpleThermalManager {
            current_temp: AtomicUsize::new(45),
            critical_threshold: AtomicUsize::new(90),
            warning_threshold: AtomicUsize::new(75),
        }
    }
}

impl ThermalManager for SimpleThermalManager {
    fn get_temperature(&self) -> i32 { self.current_temp.load(Ordering::SeqCst) as i32 }

    fn set_threshold(&mut self, temp_celsius: i32) {
        self.critical_threshold.store(temp_celsius as usize, Ordering::SeqCst);
    }

    fn get_threshold(&self) -> i32 { self.critical_threshold.load(Ordering::SeqCst) as i32 }

    fn is_critical(&self) -> bool {
        self.current_temp.load(Ordering::SeqCst) >= self.critical_threshold.load(Ordering::SeqCst)
    }
}

pub trait PowerManager {
    fn add_profile(&mut self, profile: Box<dyn PowerProfile>) -> Result<PowerProfileID, PowerError>;
    fn set_profile(&mut self, id: PowerProfileID) -> Result<(), PowerError>;
    fn get_profile(&self, id: PowerProfileID) -> Option<&dyn PowerProfile>;
    fn get_current_profile(&self) -> Option<PowerProfileID>;
}

#[repr(C)]
pub struct SimplePowerManager {
    pub profiles: Vec<Option<Box<dyn PowerProfile>>>,
    pub current_profile: AtomicUsize,
    pub governor: SimpleCPUGovernor,
    pub thermal: SimpleThermalManager,
    pub next_id: AtomicUsize,
}

impl SimplePowerManager {
    pub fn new() -> Self {
        SimplePowerManager {
            profiles: Vec::new(),
            current_profile: AtomicUsize::new(0),
            governor: SimpleCPUGovernor::new(),
            thermal: SimpleThermalManager::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn create_default_profiles(&mut self) {
        let perf_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let perf_profile = SimplePowerProfile::new(perf_id, b"performance", PowerProfileType::Performance, CPUGovernorType::Performance);
        self.profiles.push(Some(Box::new(perf_profile)));

        let balanced_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let balanced_profile = SimplePowerProfile::new(balanced_id, b"balanced", PowerProfileType::Balanced, CPUGovernorType::Ondemand);
        self.profiles.push(Some(Box::new(balanced_profile)));

        let powersave_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let powersave_profile = SimplePowerProfile::new(powersave_id, b"powersave", PowerProfileType::PowerSaver, CPUGovernorType::Powersave);
        self.profiles.push(Some(Box::new(powersave_profile)));
    }
}

impl PowerManager for SimplePowerManager {
    fn add_profile(&mut self, profile: Box<dyn PowerProfile>) -> Result<PowerProfileID, PowerError> {
        let id = profile.id();
        self.profiles.push(Some(profile));
        Ok(id)
    }

    fn set_profile(&mut self, id: PowerProfileID) -> Result<(), PowerError> {
        for profile_option in &self.profiles {
            if let Some(ref profile) = *profile_option {
                if profile.id() == id {
                    self.current_profile.store(id, Ordering::SeqCst);
                    self.governor.set_governor(profile.cpu_governor())?;
                    return Ok(());
                }
            }
        }
        Err(PowerError::InvalidProfile)
    }

    fn get_profile(&self, id: PowerProfileID) -> Option<&dyn PowerProfile> {
        for profile_option in &self.profiles {
            if let Some(ref profile) = *profile_option {
                if profile.id() == id { return Some(profile.as_ref()); }
            }
        }
        None
    }

    fn get_current_profile(&self) -> Option<PowerProfileID> {
        let id = self.current_profile.load(Ordering::SeqCst);
        if id == 0 { None } else { Some(id) }
    }
}

pub trait BatteryManager {
    fn get_capacity(&self) -> i32;
    fn get_status(&self) -> BatteryStatus;
    fn is_charging(&self) -> bool;
    fn get_time_remaining(&self) -> i32;
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus { Unknown = 0, Charging = 1, Discharging = 2, Full = 3 }

#[repr(C)]
pub struct SimpleBatteryManager {
    pub capacity: AtomicUsize,
    pub status: AtomicUsize,
    pub is_charging_flag: AtomicUsize,
}

impl SimpleBatteryManager {
    pub fn new() -> Self {
        SimpleBatteryManager {
            capacity: AtomicUsize::new(100),
            status: AtomicUsize::new(BatteryStatus::Full as usize),
            is_charging_flag: AtomicUsize::new(0),
        }
    }
}

impl BatteryManager for SimpleBatteryManager {
    fn get_capacity(&self) -> i32 { self.capacity.load(Ordering::SeqCst) as i32 }

    fn get_status(&self) -> BatteryStatus { unsafe { core::mem::transmute(self.status.load(Ordering::SeqCst)) } }

    fn is_charging(&self) -> bool { self.is_charging_flag.load(Ordering::SeqCst) == 1 }

    fn get_time_remaining(&self) -> i32 {
        let capacity = self.capacity.load(Ordering::SeqCst) as i32;
        if capacity <= 0 { return 0; }
        capacity * 5
    }
}
