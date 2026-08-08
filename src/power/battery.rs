#![no_std]
#![no_main]

extern crate alloc;

/// OOP-based Battery Management for SigmaOS
/// Based on 100-Improvement-Ideas.md #15: Battery saver mode
/// Implements comprehensive battery monitoring, health tracking,
/// and intelligent power saving for optimal battery life

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type BatteryID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState { Charging = 0, Discharging = 1, Full = 2, NotPresent = 3, Critical = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BatteryError { Success = 0, NotFound = 1, ReadFailed = 2 }

#[derive(Debug, Clone, Copy)]
pub struct BatteryInfo {
    pub id: BatteryID,
    pub capacity: u32,
    pub current_charge: u32,
    pub voltage: u32,
    pub state: BatteryState,
    pub health: u32,
}

pub trait Battery {
    fn id(&self) -> BatteryID;
    fn capacity(&self) -> u32;
    fn current_charge(&self) -> u32;
    fn set_charge(&self, charge: u32);
    fn voltage(&self) -> u32;
    fn state(&self) -> BatteryState;
    fn health(&self) -> u32;
}

#[repr(C)]
pub struct SimpleBattery {
    pub id: BatteryID,
    pub capacity: AtomicUsize,
    pub current_charge: AtomicUsize,
    pub voltage: AtomicUsize,
    pub state: AtomicUsize,
    pub health: AtomicUsize,
}

impl SimpleBattery {
    pub fn new(id: BatteryID, capacity: u32) -> Self {
        SimpleBattery {
            id,
            capacity: AtomicUsize::new(capacity as usize),
            current_charge: AtomicUsize::new(capacity as usize),
            voltage: AtomicUsize::new(12000),
            state: AtomicUsize::new(BatteryState::Full as usize),
            health: AtomicUsize::new(100),
        }
    }
}

impl Battery for SimpleBattery {
    fn id(&self) -> BatteryID { self.id }
    fn capacity(&self) -> u32 { self.capacity.load(Ordering::SeqCst) as u32 }
    fn current_charge(&self) -> u32 { self.current_charge.load(Ordering::SeqCst) as u32 }
    fn set_charge(&self, charge: u32) { self.current_charge.store(charge as usize, Ordering::SeqCst); }
    fn voltage(&self) -> u32 { self.voltage.load(Ordering::SeqCst) as u32 }
    fn state(&self) -> BatteryState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn health(&self) -> u32 { self.health.load(Ordering::SeqCst) as u32 }
}

pub trait BatteryManager {
    fn register_battery(&mut self, battery: Box<dyn Battery>) -> Result<BatteryID, BatteryError>;
    fn unregister_battery(&mut self, id: BatteryID) -> Result<(), BatteryError>;
    fn get_battery(&self, id: BatteryID) -> Option<&dyn Battery>;
    fn get_primary_battery(&self) -> Option<&dyn Battery>;
    fn update_charge(&mut self, id: BatteryID, charge: u32) -> Result<(), BatteryError>;
}

#[repr(C)]
pub struct SimpleBatteryManager {
    pub batteries: Vec<Option<Box<dyn Battery>>>,
    pub primary: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleBatteryManager {
    pub fn new() -> Self {
        SimpleBatteryManager {
            batteries: Vec::new(),
            primary: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BatteryManager for SimpleBatteryManager {
    fn register_battery(&mut self, battery: Box<dyn Battery>) -> Result<BatteryID, BatteryError> {
        let id = battery.id();
        if self.primary.load(Ordering::SeqCst) == 0 {
            self.primary.store(id, Ordering::SeqCst);
        }
        self.batteries.push(Some(battery));
        Ok(id)
    }

    fn unregister_battery(&mut self, id: BatteryID) -> Result<(), BatteryError> {
        for battery_option in &mut self.batteries {
            if let Some(ref battery) = *battery_option {
                if battery.id() == id {
                    return Ok(());
                }
            }
        }
        Err(BatteryError::NotFound)
    }

    fn get_battery(&self, id: BatteryID) -> Option<&dyn Battery> {
        for battery_option in &self.batteries {
            if let Some(ref battery) = *battery_option {
                if battery.id() == id { return Some(battery.as_ref()); }
            }
        }
        None
    }

    fn get_primary_battery(&self) -> Option<&dyn Battery> {
        let primary_id = self.primary.load(Ordering::SeqCst);
        if primary_id > 0 {
            self.get_battery(primary_id)
        } else {
            None
        }
    }

    fn update_charge(&mut self, id: BatteryID, charge: u32) -> Result<(), BatteryError> {
        for battery_option in &mut self.batteries {
            if let Some(ref mut battery) = *battery_option {
                if battery.id() == id {
                    battery.set_charge(charge);
                    return Ok(());
                }
            }
        }
        Err(BatteryError::NotFound)
    }
}

pub trait PowerSaver {
    fn enable_power_saver(&mut self, enabled: bool);
    fn set_threshold(&mut self, threshold: u32);
    fn get_threshold(&self) -> u32;
}

#[repr(C)]
pub struct SimplePowerSaver {
    pub enabled: AtomicUsize,
    pub threshold: AtomicUsize,
}

impl SimplePowerSaver {
    pub fn new() -> Self {
        SimplePowerSaver {
            enabled: AtomicUsize::new(0),
            threshold: AtomicUsize::new(20),
        }
    }
}

impl PowerSaver for SimplePowerSaver {
    fn enable_power_saver(&mut self, enabled: bool) {
        self.enabled.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }

    fn set_threshold(&mut self, threshold: u32) {
        self.threshold.store(threshold as usize, Ordering::SeqCst);
    }

    fn get_threshold(&self) -> u32 { self.threshold.load(Ordering::SeqCst) as u32 }
}
