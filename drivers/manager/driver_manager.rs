// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/manager/driver_manager.rs — Driver Warehouse Manager

use std::fs;
use std::path::Path;
use crate::drivers::catalogue::driver_catalogue::{
    DriverCatalogue, CatalogueQuery, HardwareId, DriverStatus, CompatStatus
};

pub struct DriverManager {
    pub catalogue: DriverCatalogue,
    pub installed_cache: Vec<String>,
}

impl DriverManager {
    pub fn new(catalogue_path: &str) -> Result<Self, String> {
        let catalogue = DriverCatalogue::load_from_json(catalogue_path)?;
        Ok(Self {
            catalogue,
            installed_cache: Vec::new(),
        })
    }

    /// Simulates scanning PCI/USB/ACPI system hardware
    pub fn scan_system_hardware(&self) -> Vec<HardwareId> {
        // Stub/mock: return typical target hardware (Intel e1000, nvme, virtio-net, legacy 3c59x)
        vec![
            HardwareId::Pci { vendor: 0x8086, device: 0x100E, subvendor: None, subdevice: None, class: None },
            HardwareId::Pci { vendor: 0x144D, device: 0xA808, subvendor: None, subdevice: None, class: None },
            HardwareId::Virtio { device_id: 1, vendor_id: 0x1AF4 },
            HardwareId::Pci { vendor: 0x10B7, device: 0x5900, subvendor: None, subdevice: None, class: None },
        ]
    }

    /// Matches scanned hardware against catalogue
    pub fn match_drivers(&self) -> Vec<(HardwareId, String)> {
        let hardware = self.scan_system_hardware();
        let mut matches = Vec::new();
        for hw in hardware {
            let matched_drivers = self.catalogue.find_by_hardware(&hw);
            for driver in matched_drivers {
                matches.push((hw.clone(), driver.id.clone()));
            }
        }
        matches
    }

    /// Installs a driver via download (mock/stub)
    pub fn install_driver(&mut self, driver_id: &str) -> Result<(), String> {
        if self.catalogue.get(driver_id).is_some() {
            if !self.installed_cache.contains(&driver_id.to_string()) {
                self.installed_cache.push(driver_id.to_string());
            }
            Ok(())
        } else {
            Err(format!("Driver '{}' not found in catalogue", driver_id))
        }
    }
}
