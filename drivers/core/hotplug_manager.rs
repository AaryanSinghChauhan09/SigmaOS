// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Core Driver Hotplug Manager (Rust, no_std)
//! Replaces: drivers/core/hotplug_manager.cpp
//! =========================================================================

const MAX_DEVICES: usize = 128;

#[derive(Copy, Clone, PartialEq)]
pub enum BusType {
    PCI,
    USB,
    ACPI,
}

#[derive(Copy, Clone)]
pub struct DeviceNode {
    pub device_id: u32,
    pub vendor_id: u32,
    pub bus: BusType,
    pub active: bool,
}

impl DeviceNode {
    pub const fn new(dev: u32, vend: u32, bus: BusType) -> Self {
        Self { device_id: dev, vendor_id: vend, bus, active: false }
    }
}

pub struct HotplugManager {
    devices: [Option<DeviceNode>; MAX_DEVICES],
    count: usize,
}

impl HotplugManager {
    pub const fn new() -> Self {
        Self {
            devices: [None; MAX_DEVICES],
            count: 0,
        }
    }

    pub fn handle_device_insert(&mut self, dev_id: u32, vend_id: u32, bus: BusType) -> bool {
        if self.count >= MAX_DEVICES {
            return false;
        }
        let mut node = DeviceNode::new(dev_id, vend_id, bus);
        node.active = true;
        self.devices[self.count] = Some(node);
        self.count += 1;
        // Search and load driver shard asynchronously
        true
    }

    pub fn handle_device_remove(&mut self, dev_id: u32) -> bool {
        for i in 0..self.count {
            if let Some(ref mut dev) = self.devices[i] {
                if dev.device_id == dev_id {
                    dev.active = false;
                    // Unload driver shard safely
                    return true;
                }
            }
        }
        false
    }

    pub fn active_count(&self) -> usize {
        let mut n = 0;
        for i in 0..self.count {
            if let Some(ref dev) = self.devices[i] {
                if dev.active {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn class_name(&self) -> &'static str {
        "HotplugManager"
    }
}
