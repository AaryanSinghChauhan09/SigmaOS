// Dynamic Kernel Module Support (DKMS) & Hardware Autoloader Subsystem for SigmaOS
// Inspired by Linux DKMS, udev hardware autoprobing, and Arch Linux / Gentoo module builders.

use std::collections::HashMap;

/// PCI Hardware Identification Match Rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PciIdMatch {
    pub vendor_id: u16,
    pub device_id: u16,
}

/// USB Hardware Identification Match Rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsbIdMatch {
    pub vendor_id: u16,
    pub product_id: u16,
}

/// DKMS Module Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DkmsModuleStatus {
    Registered,
    Building,
    Installed,
    Loaded,
    Error,
}

/// DKMS Kernel Module Descriptor
#[derive(Debug, Clone)]
pub struct DkmsModule {
    pub name: String,
    pub version: String,
    pub source_dir: String,
    pub target_kernel_version: String,
    pub status: DkmsModuleStatus,
    pub is_signed: bool,
    pub pci_matches: Vec<PciIdMatch>,
    pub usb_matches: Vec<UsbIdMatch>,
}

impl DkmsModule {
    pub fn new(name: &str, version: &str, source_dir: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            source_dir: source_dir.to_string(),
            target_kernel_version: "6.12.0-sigma".to_string(),
            status: DkmsModuleStatus::Registered,
            is_signed: true,
            pci_matches: Vec::new(),
            usb_matches: Vec::new(),
        }
    }
}

/// DKMS Engine & Hardware Autoloader Manager
pub struct DkmsEngine {
    pub modules: HashMap<String, DkmsModule>,
    pub pci_lookup_map: HashMap<PciIdMatch, String>,
    pub usb_lookup_map: HashMap<UsbIdMatch, String>,
    pub loaded_modules: Vec<String>,
}

impl DkmsEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            modules: HashMap::new(),
            pci_lookup_map: HashMap::new(),
            usb_lookup_map: HashMap::new(),
            loaded_modules: Vec::new(),
        };
        engine.seed_default_dkms_modules();
        engine
    }

    pub fn seed_default_dkms_modules(&mut self) {
        let mut nvidia = DkmsModule::new("nvidia-driver", "550.54.14", "/usr/src/nvidia-550.54.14");
        nvidia.pci_matches.push(PciIdMatch {
            vendor_id: 0x10DE,
            device_id: 0x1C02,
        }); // RTX 3060
        self.register_dkms_module(nvidia);

        let mut realtek_wifi =
            DkmsModule::new("rtl8852ae-driver", "1.15.0", "/usr/src/rtl8852ae-1.15.0");
        realtek_wifi.pci_matches.push(PciIdMatch {
            vendor_id: 0x10EC,
            device_id: 0x8852,
        }); // RTL8852AE
        self.register_dkms_module(realtek_wifi);

        let mut usb_serial = DkmsModule::new("ch341-usb-serial", "2.0.0", "/usr/src/ch341-2.0.0");
        usb_serial.usb_matches.push(UsbIdMatch {
            vendor_id: 0x1A86,
            product_id: 0x7523,
        }); // CH340/CH341
        self.register_dkms_module(usb_serial);
    }

    pub fn register_dkms_module(&mut self, module: DkmsModule) {
        for pci in &module.pci_matches {
            self.pci_lookup_map.insert(*pci, module.name.clone());
        }
        for usb in &module.usb_matches {
            self.usb_lookup_map.insert(*usb, module.name.clone());
        }
        self.modules.insert(module.name.clone(), module);
    }

    /// Build and install DKMS module for a target kernel
    pub fn build_and_install(
        &mut self,
        name: &str,
        kernel_version: &str,
    ) -> Result<(), &'static str> {
        let module = self.modules.get_mut(name).ok_or("DKMS module not found")?;
        module.status = DkmsModuleStatus::Building;
        module.target_kernel_version = kernel_version.to_string();

        // Simulate build and DKMS module signature verification
        if !module.is_signed {
            module.status = DkmsModuleStatus::Error;
            return Err("Module signature verification failed");
        }

        module.status = DkmsModuleStatus::Installed;
        Ok(())
    }

    /// Linux udev-inspired hardware autoloader for PCI events
    pub fn autoprobe_pci(&mut self, vendor_id: u16, device_id: u16) -> Option<String> {
        let pci = PciIdMatch {
            vendor_id,
            device_id,
        };
        if let Some(module_name) = self.pci_lookup_map.get(&pci).cloned() {
            if !self.loaded_modules.contains(&module_name) {
                if let Some(module) = self.modules.get_mut(&module_name) {
                    module.status = DkmsModuleStatus::Loaded;
                    self.loaded_modules.push(module_name.clone());
                    return Some(module_name);
                }
            }
        }
        None
    }

    /// Linux udev-inspired hardware autoloader for USB events
    pub fn autoprobe_usb(&mut self, vendor_id: u16, product_id: u16) -> Option<String> {
        let usb = UsbIdMatch {
            vendor_id,
            product_id,
        };
        if let Some(module_name) = self.usb_lookup_map.get(&usb).cloned() {
            if !self.loaded_modules.contains(&module_name) {
                if let Some(module) = self.modules.get_mut(&module_name) {
                    module.status = DkmsModuleStatus::Loaded;
                    self.loaded_modules.push(module_name.clone());
                    return Some(module_name);
                }
            }
        }
        None
    }
}

impl Default for DkmsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dkms_build_and_hardware_autoloader() {
        let mut dkms = DkmsEngine::new();

        // Build and install nvidia module for 6.12 kernel
        assert!(dkms
            .build_and_install("nvidia-driver", "6.12.0-sigma")
            .is_ok());

        // Autoprobe PCI RTX 3060 (0x10DE:0x1C02)
        let loaded_pci = dkms.autoprobe_pci(0x10DE, 0x1C02).unwrap();
        assert_eq!(loaded_pci, "nvidia-driver");
        assert!(dkms.loaded_modules.contains(&"nvidia-driver".to_string()));

        // Autoprobe USB CH340 Serial Converter (0x1A86:0x7523)
        let loaded_usb = dkms.autoprobe_usb(0x1A86, 0x7523).unwrap();
        assert_eq!(loaded_usb, "ch341-usb-serial");
        assert!(dkms
            .loaded_modules
            .contains(&"ch341-usb-serial".to_string()));
    }
}
