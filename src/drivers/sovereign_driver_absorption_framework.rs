//! Sovereign Driver Absorption Framework Master Suite (`src/drivers/sovereign_driver_absorption_framework.rs`)
//!
//! Master coordinator for absorbing, translating, sandboxing, and hot-swapping drivers from Linux, FreeBSD, and OpenBSD:
//! - Foreign driver C symbol bridge (`LinuxCSymbolBridge`)
//! - Absorbed Linux drivers (`IntelXeGpuDriver`, `IntelWifiDriver`)
//! - Absorbed FreeBSD drivers (`FreeBsdGeliDiskDriver`)
//! - Absorbed OpenBSD drivers (`OpenBsdWsMouseDriver`)
//! - Declarative firmware blob registry (`FirmwareBlobRegistry`)
//! - Hot-swappable driver shards (`DriverShard`)

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};

#[cfg(test)]
#[path = "adapters/linux_c_shim.rs"]
mod linux_c_shim;

#[cfg(test)]
#[path = "linux_absorption/gpu/intel_xe.rs"]
mod intel_xe;

#[cfg(test)]
#[path = "linux_absorption/net/iwlwifi.rs"]
mod iwlwifi;

#[cfg(test)]
#[path = "freebsd_absorption/encryption/geli_disk.rs"]
mod geli_disk;

#[cfg(test)]
#[path = "openbsd_absorption/input/wsmouse.rs"]
mod wsmouse;

#[cfg(test)]
#[path = "firmware/registry.rs"]
mod registry;

#[cfg(test)]
use self::geli_disk::FreeBsdGeliDiskDriver;
#[cfg(test)]
use self::intel_xe::IntelXeGpuDriver;
#[cfg(test)]
use self::iwlwifi::IntelWifiDriver;
#[cfg(test)]
use self::linux_c_shim::LinuxCSymbolBridge;
#[cfg(test)]
use self::registry::FirmwareBlobRegistry;
#[cfg(test)]
use self::wsmouse::OpenBsdWsMouseDriver;

#[cfg(not(test))]
use super::adapters::linux_c_shim::LinuxCSymbolBridge;
#[cfg(not(test))]
use super::firmware::registry::FirmwareBlobRegistry;
#[cfg(not(test))]
use super::freebsd_absorption::encryption::geli_disk::FreeBsdGeliDiskDriver;
#[cfg(not(test))]
use super::linux_absorption::gpu::intel_xe::IntelXeGpuDriver;
#[cfg(not(test))]
use super::linux_absorption::net::iwlwifi::IntelWifiDriver;
#[cfg(not(test))]
use super::openbsd_absorption::input::wsmouse::OpenBsdWsMouseDriver;

/// Hot-Swappable Driver Shard Record
#[derive(Debug, Clone)]
pub struct DriverShard {
    pub shard_id: usize,
    pub name: String,
    pub device_type: String,
    pub is_hot_swappable: bool,
    pub is_active: bool,
    pub revision: u32,
}

/// Sovereign Driver Absorption Framework Master Suite
pub struct SovereignDriverAbsorptionFrameworkMasterSuite {
    pub symbol_bridge: LinuxCSymbolBridge,
    pub intel_xe_gpu: IntelXeGpuDriver,
    pub intel_wifi: IntelWifiDriver,
    pub freebsd_geli: FreeBsdGeliDiskDriver,
    pub openbsd_wsmouse: OpenBsdWsMouseDriver,
    pub firmware_registry: FirmwareBlobRegistry,
    pub driver_shards: BTreeMap<usize, DriverShard>,
}

impl SovereignDriverAbsorptionFrameworkMasterSuite {
    pub fn new() -> Self {
        let mut suite = Self {
            symbol_bridge: LinuxCSymbolBridge::new(),
            intel_xe_gpu: IntelXeGpuDriver::new(0x56A0),
            intel_wifi: IntelWifiDriver::new(),
            freebsd_geli: FreeBsdGeliDiskDriver::new("ada0p2"),
            openbsd_wsmouse: OpenBsdWsMouseDriver::new(),
            firmware_registry: FirmwareBlobRegistry::new(),
            driver_shards: BTreeMap::new(),
        };

        // Register default driver shards
        suite.register_shard("intel-xe-gpu", "gpu", true);
        suite.register_shard("rtw89-wifi", "wireless", true);
        suite.register_shard("nvme-storage", "storage", false);

        suite
    }

    pub fn register_shard(&mut self, name: &str, device_type: &str, hot_swappable: bool) -> usize {
        let shard_id = self.driver_shards.len() + 1;
        self.driver_shards.insert(
            shard_id,
            DriverShard {
                shard_id,
                name: name.to_string(),
                device_type: device_type.to_string(),
                is_hot_swappable: hot_swappable,
                is_active: true,
                revision: 1,
            },
        );
        shard_id
    }

    pub fn hot_swap_unload_shard(&mut self, name: &str) -> Result<String, &'static str> {
        if let Some((_, shard)) = self.driver_shards.iter_mut().find(|(_, s)| s.name == name) {
            if !shard.is_hot_swappable {
                return Err("Driver shard is not marked as hot-swappable");
            }
            shard.is_active = false;
            Ok(format!("Unloaded driver shard '{}'. Device moved to fallback mode.", name))
        } else {
            Err("Driver shard name not found")
        }
    }

    pub fn run_absorption_suite_diagnostics(&mut self) -> BTreeMap<String, bool> {
        let mut report = BTreeMap::new();

        report.insert("symbol_bridge".to_string(), self.symbol_bridge.resolve("printk").is_some());

        let guc_ok = self.intel_xe_gpu.load_guc_firmware(b"ucode").is_ok() && self.intel_xe_gpu.initialize().is_ok();
        report.insert("linux_gpu_intel_xe".to_string(), guc_ok);

        let wifi_ok = self.intel_wifi.load_ucode_firmware(b"ucode").is_ok();
        report.insert("linux_net_iwlwifi".to_string(), wifi_ok);

        let geli_ok = self.freebsd_geli.attach_and_decrypt("pass").is_ok();
        report.insert("freebsd_geli_disk".to_string(), geli_ok);

        self.openbsd_wsmouse.inject_relative_motion(5, 5, 0);
        report.insert("openbsd_wsmouse".to_string(), self.openbsd_wsmouse.read_event().is_some());

        report.insert("firmware_registry".to_string(), !self.firmware_registry.blobs.is_empty());

        report
    }
}

impl Default for SovereignDriverAbsorptionFrameworkMasterSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_absorption_framework_master_suite() {
        let mut master = SovereignDriverAbsorptionFrameworkMasterSuite::new();
        let diagnostics = master.run_absorption_suite_diagnostics();

        assert_eq!(diagnostics.get("symbol_bridge"), Some(&true));
        assert_eq!(diagnostics.get("linux_gpu_intel_xe"), Some(&true));
        assert_eq!(diagnostics.get("linux_net_iwlwifi"), Some(&true));
        assert_eq!(diagnostics.get("freebsd_geli_disk"), Some(&true));
        assert_eq!(diagnostics.get("openbsd_wsmouse"), Some(&true));
        assert_eq!(diagnostics.get("firmware_registry"), Some(&true));

        let res = master.hot_swap_unload_shard("intel-xe-gpu").unwrap();
        assert!(res.contains("Unloaded driver shard 'intel-xe-gpu'"));
    }
}
