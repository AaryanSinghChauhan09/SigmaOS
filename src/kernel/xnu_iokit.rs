// SPDX-License-Identifier: GPL-2.0-or-later
// Principle: OBJECT-ORIENTED DRIVER MODEL, PLUG-AND-PLAY, POWER MANAGEMENT, WORKLOOP CONCURRENCY

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

/// Lifecycle state of an IOService instance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOServiceState {
    Uninitialized,
    Probed,
    Attached,
    Started,
    Stopping,
    Terminated,
}

/// Describes a contiguous range of physical memory mapped for I/O
#[derive(Debug, Clone)]
pub struct IOMemoryDescriptor {
    pub phys_addr: u64,
    pub length: usize,
    pub direction: IOMemoryDirection,
    pub mapped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOMemoryDirection {
    In,
    Out,
    InOut,
}

impl IOMemoryDescriptor {
    pub fn new(phys_addr: u64, length: usize, direction: IOMemoryDirection) -> Self {
        Self {
            phys_addr,
            length,
            direction,
            mapped: false,
        }
    }

    pub fn map(&mut self) -> u64 {
        self.mapped = true;
        // In a real system, this would map the physical pages
        self.phys_addr // return virtual address (stub: same as physical)
    }

    pub fn unmap(&mut self) {
        self.mapped = false;
    }
}

/// Work loop — serialises work items for a driver
#[derive(Debug)]
pub struct IOWorkLoop {
    pub name: String,
    pub running: bool,
    pending_work: u32,
}

impl IOWorkLoop {
    pub fn new(name: String) -> Self {
        Self {
            name,
            running: false,
            pending_work: 0,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn schedule_work(&mut self) {
        if self.running {
            self.pending_work += 1;
        }
    }

    pub fn drain(&mut self) -> u32 {
        let w = self.pending_work;
        self.pending_work = 0;
        w
    }
}

/// Command gate — serialises access to a driver's command queue
#[derive(Debug)]
pub struct IOCommandGate {
    pub work_loop: IOWorkLoop,
    commands: Vec<String>,
}

impl IOCommandGate {
    pub fn new(work_loop: IOWorkLoop) -> Self {
        Self {
            work_loop,
            commands: Vec::new(),
        }
    }

    pub fn run_action(&mut self, action: String) -> Result<(), &'static str> {
        if !self.work_loop.running {
            return Err("work loop not running");
        }
        self.commands.push(action);
        self.work_loop.schedule_work();
        Ok(())
    }

    pub fn pending_commands(&self) -> &[String] {
        &self.commands
    }
}

/// Matching table — key/value properties used during driver probing
#[derive(Debug, Clone)]
pub struct IOMatchingTable {
    pub properties: BTreeMap<String, String>,
}

impl IOMatchingTable {
    pub fn new() -> Self {
        Self {
            properties: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, val: &str) {
        self.properties.insert(String::from(key), String::from(val));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    pub fn matches(&self, other: &IOMatchingTable) -> bool {
        for (k, v) in &self.properties {
            match other.properties.get(k) {
                Some(ov) if ov == v => {}
                _ => return false,
            }
        }
        true
    }
}

/// Trait for XNU I/O Kit drivers
pub trait IOService {
    fn name(&self) -> &str;
    fn matching_table(&self) -> IOMatchingTable;
    fn probe(&self, provider_table: &IOMatchingTable) -> bool;
    fn start(&mut self) -> Result<(), &'static str>;
    fn stop(&mut self);
    fn state(&self) -> IOServiceState;
}

/// A registered driver entry in the registry
pub struct IODriverEntry {
    pub driver: Box<dyn IOService>,
    pub state: IOServiceState,
}

/// XNU IOKit driver registry
pub struct IODriverRegistry {
    drivers: Vec<IODriverEntry>,
}

impl IODriverRegistry {
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    pub fn register_driver(&mut self, driver: Box<dyn IOService>) {
        self.drivers.push(IODriverEntry {
            driver,
            state: IOServiceState::Uninitialized,
        });
    }

    /// Probe all registered drivers against a hardware matching table
    pub fn probe_all(&mut self, hw_table: &IOMatchingTable) -> Vec<usize> {
        let mut matched = Vec::new();
        for (i, entry) in self.drivers.iter_mut().enumerate() {
            let driver_table = entry.driver.matching_table();
            if entry.driver.probe(hw_table) {
                entry.state = IOServiceState::Probed;
                matched.push(i);
            }
        }
        matched
    }

    pub fn start_service(&mut self, index: usize) -> Result<(), &'static str> {
        let entry = self
            .drivers
            .get_mut(index)
            .ok_or("driver index out of range")?;
        if entry.state != IOServiceState::Probed {
            return Err("driver not in probed state");
        }
        entry.driver.start()?;
        entry.state = IOServiceState::Started;
        Ok(())
    }

    pub fn stop_service(&mut self, index: usize) -> Result<(), &'static str> {
        let entry = self
            .drivers
            .get_mut(index)
            .ok_or("driver index out of range")?;
        entry.driver.stop();
        entry.state = IOServiceState::Terminated;
        Ok(())
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    pub fn driver_state(&self, index: usize) -> Option<IOServiceState> {
        self.drivers.get(index).map(|e| e.state)
    }
}

// ── Example stub drivers ──────────────────────────────────────────────────────

pub struct NullBlockDriver {
    state: IOServiceState,
}

impl NullBlockDriver {
    pub fn new() -> Self {
        Self {
            state: IOServiceState::Uninitialized,
        }
    }
}

impl IOService for NullBlockDriver {
    fn name(&self) -> &str {
        "NullBlockDriver"
    }

    fn matching_table(&self) -> IOMatchingTable {
        let mut t = IOMatchingTable::new();
        t.set("IOClass", "NullBlock");
        t
    }

    fn probe(&self, provider: &IOMatchingTable) -> bool {
        provider.get("IOClass") == Some("NullBlock")
    }

    fn start(&mut self) -> Result<(), &'static str> {
        self.state = IOServiceState::Started;
        Ok(())
    }

    fn stop(&mut self) {
        self.state = IOServiceState::Terminated;
    }

    fn state(&self) -> IOServiceState {
        self.state
    }
}

pub struct PCINetworkDriver {
    state: IOServiceState,
    pub vendor_id: u16,
    pub device_id: u16,
}

impl PCINetworkDriver {
    pub fn new(vendor_id: u16, device_id: u16) -> Self {
        Self {
            state: IOServiceState::Uninitialized,
            vendor_id,
            device_id,
        }
    }
}

impl IOService for PCINetworkDriver {
    fn name(&self) -> &str {
        "PCINetworkDriver"
    }

    fn matching_table(&self) -> IOMatchingTable {
        let mut t = IOMatchingTable::new();
        t.set("IOPCIVendorID", "8086");
        t.set("IOPCIDeviceID", "100e");
        t
    }

    fn probe(&self, provider: &IOMatchingTable) -> bool {
        provider.get("IOPCIVendorID") == Some("8086")
    }

    fn start(&mut self) -> Result<(), &'static str> {
        self.state = IOServiceState::Started;
        Ok(())
    }

    fn stop(&mut self) {
        self.state = IOServiceState::Terminated;
    }
    fn state(&self) -> IOServiceState {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_probe() {
        let mut reg = IODriverRegistry::new();
        reg.register_driver(Box::new(NullBlockDriver::new()));

        let mut hw = IOMatchingTable::new();
        hw.set("IOClass", "NullBlock");
        let matched = reg.probe_all(&hw);
        assert_eq!(matched, alloc::vec![0]);
        assert_eq!(reg.driver_state(0), Some(IOServiceState::Probed));
    }

    #[test]
    fn test_start_service() {
        let mut reg = IODriverRegistry::new();
        reg.register_driver(Box::new(NullBlockDriver::new()));
        let mut hw = IOMatchingTable::new();
        hw.set("IOClass", "NullBlock");
        reg.probe_all(&hw);
        reg.start_service(0).unwrap();
        assert_eq!(reg.driver_state(0), Some(IOServiceState::Started));
    }

    #[test]
    fn test_start_without_probe_fails() {
        let mut reg = IODriverRegistry::new();
        reg.register_driver(Box::new(NullBlockDriver::new()));
        let result = reg.start_service(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_workloop_drain() {
        let mut wl = IOWorkLoop::new(String::from("disk-wl"));
        wl.start();
        wl.schedule_work();
        wl.schedule_work();
        assert_eq!(wl.drain(), 2);
        assert_eq!(wl.drain(), 0);
    }

    #[test]
    fn test_memory_descriptor_map() {
        let mut desc = IOMemoryDescriptor::new(0x8000_0000, 4096, IOMemoryDirection::InOut);
        assert!(!desc.mapped);
        let va = desc.map();
        assert!(desc.mapped);
        assert_eq!(va, 0x8000_0000);
        desc.unmap();
        assert!(!desc.mapped);
    }

    #[test]
    fn test_matching_table() {
        let mut t1 = IOMatchingTable::new();
        t1.set("vendor", "8086");
        let mut t2 = IOMatchingTable::new();
        t2.set("vendor", "8086");
        t2.set("class", "net");
        assert!(t1.matches(&t2)); // t1's keys ⊆ t2
        assert!(!t2.matches(&t1)); // t2 has extra keys
    }
}
