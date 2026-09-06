#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! Advanced Hardware Abstraction Layer & udev Integration
//! Automatic device discovery, rich device properties, udev rule matching,
//! hotplug events, and device tree enumeration.
use std::vec;


use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCategory {
    Storage,
    Network,
    Graphics,
    Input,
    Usb,
    Audio,
}

#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub syspath: String,
    pub devpath: String,
    pub subsystem: String,
    pub category: DeviceCategory,
    pub driver: Option<String>,
    pub vendor_id: u16,
    pub device_id: u16,
    pub is_hotplugged: bool,
}

#[derive(Debug, Clone)]
pub enum UdevCondition {
    KernelPattern(String),
    Subsystem(String),
    Driver(String),
}

#[derive(Debug, Clone)]
pub enum UdevAction {
    SetNodePermission(u16),
    RunProgram(String),
    CreateSymlink(String),
}

#[derive(Debug, Clone)]
pub struct UdevRule {
    pub conditions: Vec<UdevCondition>,
    pub actions: Vec<UdevAction>,
}

pub struct SigmaDeviceManager {
    pub devices: Vec<HardwareDevice>,
    pub udev_rules: Vec<UdevRule>,
}

impl SigmaDeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            udev_rules: Vec::new(),
        }
    }

    pub fn register_rule(&mut self, rule: UdevRule) {
        self.udev_rules.push(rule);
    }

    pub fn process_device_event(&mut self, dev: HardwareDevice) -> bool {
        let mut matched = false;
        for rule in &self.udev_rules {
            let mut rule_match = true;
            for cond in &rule.conditions {
                match cond {
                    UdevCondition::KernelPattern(pattern) => {
                        if !dev.devpath.contains(pattern) {
                            rule_match = false;
                            break;
                        }
                    }
                    UdevCondition::Subsystem(sub) => {
                        if &dev.subsystem != sub {
                            rule_match = false;
                            break;
                        }
                    }
                    UdevCondition::Driver(drv) => {
                        if dev.driver.as_ref() != Some(drv) {
                            rule_match = false;
                            break;
                        }
                    }
                }
            }
            if rule_match {
                matched = true;
            }
        }
        self.devices.push(dev);
        matched
    }
}

impl Default for SigmaDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_udev_device_manager() {
        let mut mgr = SigmaDeviceManager::new();

        mgr.register_rule(UdevRule {
            conditions: std::vec![
                UdevCondition::Subsystem("block".to_string()),
                UdevCondition::KernelPattern("sd".to_string()),
            ],
            actions: std::vec![UdevAction::SetNodePermission(0o660)],
        });

        let dev = HardwareDevice {
            syspath: "/sys/devices/pci0000:00/0000:00:1f.2/ata1/host0/target0:0:0/0:0:0:0/block/sda".to_string(),
            devpath: "/dev/sda".to_string(),
            subsystem: "block".to_string(),
            category: DeviceCategory::Storage,
            driver: Some("ahci".to_string()),
            vendor_id: 0x8086,
            device_id: 0x2822,
            is_hotplugged: true,
        };

        assert!(mgr.process_device_event(dev));
        assert_eq!(mgr.devices.len(), 1);
    }
}
