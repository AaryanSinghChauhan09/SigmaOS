//! Hardware and Firmware Subsystem for SigmaOS
//!
//! Inspired by Linux kernel subsystems (ACPI, cpufreq, PCIe AER/MSI-X, dm-crypt/LVM,
//! IOMMU DMA API, microcode loader) and BSD distros (OpenBSD pledge/unveil, FreeBSD GEOM/geli,
//! NetBSD devpubd).
//!
//! Authorizes and uses Rust standard library functions (std::sync, std::collections,
//! std::time) per SigmaOS updated architectural rules.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// ACPI System Sleep States (S0 - S5)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiSleepState {
    S0Working,
    S1PowerOnSuspend,
    S2CpuOff,
    S3SuspendToRam,
    S4Hibernate,
    S5SoftOff,
}

/// ACPI Thermal Zone Reading
#[derive(Debug, Clone)]
pub struct AcpiThermalZone {
    pub zone_name: String,
    pub current_temp_milli_celsius: u32,
    pub trip_point_passive_milli_celsius: u32,
    pub trip_point_critical_milli_celsius: u32,
    pub fan_active: bool,
}

/// ACPI Power Resource Parser & Governor
pub struct AcpiPowerGovernor {
    pub sleep_state: AcpiSleepState,
    pub thermal_zones: HashMap<String, AcpiThermalZone>,
    pub battery_percentage: u8,
    pub ac_connected: bool,
}

impl AcpiPowerGovernor {
    pub fn new() -> Self {
        let mut zones = HashMap::new();
        zones.insert(
            "TZ00".to_string(),
            AcpiThermalZone {
                zone_name: "TZ00".to_string(),
                current_temp_milli_celsius: 42_000,
                trip_point_passive_milli_celsius: 75_000,
                trip_point_critical_milli_celsius: 95_000,
                fan_active: false,
            },
        );
        Self {
            sleep_state: AcpiSleepState::S0Working,
            thermal_zones: zones,
            battery_percentage: 100,
            ac_connected: true,
        }
    }

    pub fn update_temperature(&mut self, zone_name: &str, temp_milli_c: u32) -> Result<bool, String> {
        if let Some(zone) = self.thermal_zones.get_mut(zone_name) {
            zone.current_temp_milli_celsius = temp_milli_c;
            if temp_milli_c >= zone.trip_point_critical_milli_celsius {
                return Err(format!("CRITICAL: Thermal runaway on {} at {}mC", zone_name, temp_milli_c));
            }
            zone.fan_active = temp_milli_c >= zone.trip_point_passive_milli_celsius;
            Ok(zone.fan_active)
        } else {
            Err(format!("Thermal zone {} not found", zone_name))
        }
    }

    pub fn request_sleep_state(&mut self, target: AcpiSleepState) -> Result<(), String> {
        if target == AcpiSleepState::S4Hibernate && self.battery_percentage < 5 && !self.ac_connected {
            return Err("Cannot hibernate: battery critically low with no AC adapter".to_string());
        }
        self.sleep_state = target;
        Ok(())
    }
}

/// CPU Frequency Scaling Governors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernorPolicy {
    Performance,
    Powersave,
    Schedutil,
    Ondemand,
    Conservative,
}

/// Core Frequency Profile
#[derive(Debug, Clone)]
pub struct CoreFrequencyProfile {
    pub core_id: u32,
    pub min_freq_khz: u32,
    pub max_freq_khz: u32,
    pub cur_freq_khz: u32,
    pub boost_enabled: bool,
    pub governor: CpuGovernorPolicy,
}

/// CPU Frequency Scaling Manager
pub struct CpuFreqGovernorManager {
    pub cores: BTreeMap<u32, CoreFrequencyProfile>,
}

impl CpuFreqGovernorManager {
    pub fn new(core_count: u32) -> Self {
        let mut cores = BTreeMap::new();
        for id in 0..core_count {
            cores.insert(
                id,
                CoreFrequencyProfile {
                    core_id: id,
                    min_freq_khz: 800_000,
                    max_freq_khz: 4_800_000,
                    cur_freq_khz: 2_400_000,
                    boost_enabled: true,
                    governor: CpuGovernorPolicy::Schedutil,
                },
            );
        }
        Self { cores }
    }

    pub fn set_governor_all(&mut self, policy: CpuGovernorPolicy) {
        for core in self.cores.values_mut() {
            core.governor = policy;
            match policy {
                CpuGovernorPolicy::Performance => core.cur_freq_khz = core.max_freq_khz,
                CpuGovernorPolicy::Powersave => core.cur_freq_khz = core.min_freq_khz,
                _ => core.cur_freq_khz = (core.min_freq_khz + core.max_freq_khz) / 2,
            }
        }
    }

    pub fn adjust_load(&mut self, core_id: u32, utilization_percent: u8) -> Result<u32, String> {
        if let Some(core) = self.cores.get_mut(&core_id) {
            if core.governor == CpuGovernorPolicy::Schedutil || core.governor == CpuGovernorPolicy::Ondemand {
                let range = core.max_freq_khz - core.min_freq_khz;
                let target = core.min_freq_khz + ((range as u64 * utilization_percent as u64) / 100) as u32;
                core.cur_freq_khz = target.clamp(core.min_freq_khz, core.max_freq_khz);
            }
            Ok(core.cur_freq_khz)
        } else {
            Err(format!("Core {} does not exist", core_id))
        }
    }
}

/// PCIe Device Capability and Address
#[derive(Debug, Clone)]
pub struct PciDeviceDescriptor {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub bar_addresses: [u64; 6],
    pub msi_x_vectors: u16,
    pub aer_error_count: u32,
}

/// PCIe Bus Enumeration and Error Handling Subsystem
pub struct PciExpressBusTree {
    pub devices: Vec<PciDeviceDescriptor>,
}

impl PciExpressBusTree {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn register_device(&mut self, dev: PciDeviceDescriptor) {
        self.devices.push(dev);
    }

    pub fn find_by_class(&self, class_code: u8) -> Vec<&PciDeviceDescriptor> {
        self.devices.iter().filter(|d| d.class_code == class_code).collect()
    }

    pub fn record_aer_error(&mut self, bus: u8, dev: u8, func: u8) -> Result<u32, String> {
        for d in self.devices.iter_mut() {
            if d.bus == bus && d.device == dev && d.function == func {
                d.aer_error_count += 1;
                return Ok(d.aer_error_count);
            }
        }
        Err(format!("PCIe device {:02x}:{:02x}.{:x} not found", bus, dev, func))
    }
}

/// Device Mapper / LVM Cryptsetup Volume
#[derive(Debug, Clone)]
pub struct DmCryptVolume {
    pub volume_name: String,
    pub uuid: String,
    pub cipher: String,
    pub key_size_bits: u16,
    pub is_unlocked: bool,
    pub sector_count: u64,
}

/// Device Mapper Block Subsystem
pub struct DeviceMapperSubsystem {
    pub volumes: HashMap<String, DmCryptVolume>,
}

impl DeviceMapperSubsystem {
    pub fn new() -> Self {
        Self {
            volumes: HashMap::new(),
        }
    }

    pub fn create_luks_target(&mut self, name: &str, sectors: u64, cipher: &str) {
        let vol = DmCryptVolume {
            volume_name: name.to_string(),
            uuid: format!("luks-{}", name),
            cipher: cipher.to_string(),
            key_size_bits: 256,
            is_unlocked: false,
            sector_count: sectors,
        };
        self.volumes.insert(name.to_string(), vol);
    }

    pub fn unlock_volume(&mut self, name: &str, passphrase: &str) -> Result<bool, String> {
        if passphrase.is_empty() {
            return Err("Empty LUKS passphrase rejected".to_string());
        }
        if let Some(vol) = self.volumes.get_mut(name) {
            vol.is_unlocked = true;
            Ok(true)
        } else {
            Err(format!("Volume {} does not exist", name))
        }
    }
}

/// IOMMU / DMA Scatter-Gather Entry
#[derive(Debug, Clone, Copy)]
pub struct DmaScatterGatherEntry {
    pub physical_address: u64,
    pub iova_address: u64,
    pub length_bytes: usize,
    pub readable: bool,
    pub writable: bool,
}

/// IOMMU Domain Manager
pub struct IommuDomainManager {
    pub domain_id: u32,
    pub translations: VecDeque<DmaScatterGatherEntry>,
}

impl IommuDomainManager {
    pub fn new(domain_id: u32) -> Self {
        Self {
            domain_id,
            translations: VecDeque::new(),
        }
    }

    pub fn map_pages(&mut self, phys: u64, iova: u64, len: usize, read: bool, write: bool) {
        self.translations.push_back(DmaScatterGatherEntry {
            physical_address: phys,
            iova_address: iova,
            length_bytes: len,
            readable: read,
            writable: write,
        });
    }

    pub fn translate_iova(&self, iova: u64) -> Option<u64> {
        for entry in &self.translations {
            if iova >= entry.iova_address && iova < entry.iova_address + (entry.length_bytes as u64) {
                let offset = iova - entry.iova_address;
                return Some(entry.physical_address + offset);
            }
        }
        None
    }
}

/// Firmware & CPU Microcode Revision
#[derive(Debug, Clone)]
pub struct MicrocodePatch {
    pub processor_signature: u32,
    pub update_revision: u32,
    pub date_code: u32,
    pub checksum: u32,
}

/// Firmware Microcode Updater Subsystem
pub struct FirmwareMicrocodeUpdater {
    pub patches: Vec<MicrocodePatch>,
    pub current_revision: u32,
}

impl FirmwareMicrocodeUpdater {
    pub fn new(initial_rev: u32) -> Self {
        Self {
            patches: Vec::new(),
            current_revision: initial_rev,
        }
    }

    pub fn register_patch(&mut self, patch: MicrocodePatch) {
        self.patches.push(patch);
    }

    pub fn apply_latest(&mut self, signature: u32) -> Result<u32, String> {
        let matching: Vec<&MicrocodePatch> = self
            .patches
            .iter()
            .filter(|p| p.processor_signature == signature && p.update_revision > self.current_revision)
            .collect();

        if let Some(best) = matching.into_iter().max_by_key(|p| p.update_revision) {
            self.current_revision = best.update_revision;
            Ok(self.current_revision)
        } else {
            Err("No newer microcode patch available for processor signature".to_string())
        }
    }
}

/// Unified Sovereign Hardware-Firmware Subsystem Engine
pub struct SovereignHardwareFirmwareEngine {
    pub acpi: Arc<Mutex<AcpiPowerGovernor>>,
    pub cpufreq: Arc<Mutex<CpuFreqGovernorManager>>,
    pub pci_tree: Arc<Mutex<PciExpressBusTree>>,
    pub dm_crypt: Arc<Mutex<DeviceMapperSubsystem>>,
    pub iommu: Arc<Mutex<IommuDomainManager>>,
    pub microcode: Arc<Mutex<FirmwareMicrocodeUpdater>>,
}

impl SovereignHardwareFirmwareEngine {
    pub fn new(core_count: u32, _processor_sig: u32) -> Self {
        let microcode_mgr = FirmwareMicrocodeUpdater::new(0x0001);
        Self {
            acpi: Arc::new(Mutex::new(AcpiPowerGovernor::new())),
            cpufreq: Arc::new(Mutex::new(CpuFreqGovernorManager::new(core_count))),
            pci_tree: Arc::new(Mutex::new(PciExpressBusTree::new())),
            dm_crypt: Arc::new(Mutex::new(DeviceMapperSubsystem::new())),
            iommu: Arc::new(Mutex::new(IommuDomainManager::new(1))),
            microcode: Arc::new(Mutex::new(microcode_mgr)),
        }
    }

    pub fn verify_integrity(&self) -> bool {
        let acpi_ok = self.acpi.lock().unwrap().sleep_state == AcpiSleepState::S0Working;
        let cores_ok = !self.cpufreq.lock().unwrap().cores.is_empty();
        acpi_ok && cores_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acpi_thermal_and_sleep() {
        let mut acpi = AcpiPowerGovernor::new();
        assert_eq!(acpi.sleep_state, AcpiSleepState::S0Working);

        let fan = acpi.update_temperature("TZ00", 76_000).expect("Update temp");
        assert!(fan, "Fan should activate at 76C");

        acpi.request_sleep_state(AcpiSleepState::S3SuspendToRam).expect("S3 sleep");
        assert_eq!(acpi.sleep_state, AcpiSleepState::S3SuspendToRam);
    }

    #[test]
    fn test_cpufreq_scaling() {
        let mut mgr = CpuFreqGovernorManager::new(8);
        mgr.set_governor_all(CpuGovernorPolicy::Performance);
        assert_eq!(mgr.cores.get(&0).unwrap().cur_freq_khz, 4_800_000);

        mgr.set_governor_all(CpuGovernorPolicy::Powersave);
        assert_eq!(mgr.cores.get(&0).unwrap().cur_freq_khz, 800_000);

        mgr.set_governor_all(CpuGovernorPolicy::Schedutil);
        let freq = mgr.adjust_load(0, 50).expect("Adjust load");
        assert!(freq > 800_000 && freq < 4_800_000);
    }

    #[test]
    fn test_pcie_bus_and_aer() {
        let mut bus = PciExpressBusTree::new();
        bus.register_device(PciDeviceDescriptor {
            bus: 0,
            device: 2,
            function: 0,
            vendor_id: 0x8086,
            device_id: 0x9a49,
            class_code: 0x03, // VGA Display Controller
            bar_addresses: [0xe0000000, 0, 0, 0, 0, 0],
            msi_x_vectors: 16,
            aer_error_count: 0,
        });

        let displays = bus.find_by_class(0x03);
        assert_eq!(displays.len(), 1);
        let errs = bus.record_aer_error(0, 2, 0).expect("Record AER");
        assert_eq!(errs, 1);
    }

    #[test]
    fn test_dm_crypt_and_iommu() {
        let mut dm = DeviceMapperSubsystem::new();
        dm.create_luks_target("root_crypt", 100_000_000, "aes-xts-plain64");
        let unlocked = dm.unlock_volume("root_crypt", "supersecret").expect("Unlock");
        assert!(unlocked);

        let mut iommu = IommuDomainManager::new(10);
        iommu.map_pages(0x1000_0000, 0x8000_0000, 4096, true, true);
        let phys = iommu.translate_iova(0x8000_0100).expect("Translate");
        assert_eq!(phys, 0x1000_0100);
    }

    #[test]
    fn test_microcode_patch_and_unified_engine() {
        let engine = SovereignHardwareFirmwareEngine::new(16, 0x000906ea);
        assert!(engine.verify_integrity());

        let mut ucode = engine.microcode.lock().unwrap();
        ucode.register_patch(MicrocodePatch {
            processor_signature: 0x000906ea,
            update_revision: 0x0045,
            date_code: 20261015,
            checksum: 0xcafe_babe,
        });

        let new_rev = ucode.apply_latest(0x000906ea).expect("Apply microcode");
        assert_eq!(new_rev, 0x0045);
    }
}
