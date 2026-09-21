//! Sovereign Modular Kernel System Suite for SigmaOS
//!
//! Inspired by Linux (LKMs, udev, cgroups v2, eBPF, netfilter, vfs, io_uring) and
//! BSD (FreeBSD kldload, devd, UMA, Capsicum, Jails; OpenBSD PF, pledge, unveil; NetBSD Rump Kernels).
//!
//! Provides a complete, modular kernel subsystem bridging:
//! 1. Dynamic Kernel Module Management (`SovereignModularKernelEngine`)
//! 2. Universal Hardware Driver Engine (`SovereignDriverManager`)
//! 3. Process Control & Resource Management (`SovereignProcessControlManager`)
//! 4. Sovereign Network Stack & Firewall (`SovereignNetworkStackManager`)
//! 5. Peripheral Access & Event Controller (`SovereignPeripheralAccessManager`)
//! 6. Virtual File System & Storage Framework (`SovereignVfsStorageManager`)

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. Dynamic Kernel Module Loader & Symbol Resolver
// (Linux LKM / insmod / rmmod & FreeBSD kldload / kldunload & NetBSD Rump)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleStatus {
    Unloaded,
    Initializing,
    Active,
    Unloading,
    Faulted,
}

#[derive(Debug, Clone)]
pub struct KernelModuleManifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub exported_symbols: Vec<String>,
    pub is_pqc_signed: bool,
    pub params: BTreeMap<String, String>,
}

pub struct LoadedKernelModule {
    pub manifest: KernelModuleManifest,
    pub status: ModuleStatus,
    pub ref_count: usize,
    pub memory_base_addr: u64,
    pub memory_size_bytes: usize,
}

pub struct SovereignModularKernelEngine {
    pub modules: BTreeMap<String, LoadedKernelModule>,
    pub symbol_table: BTreeMap<String, String>, // symbol_name -> module_name
    pub enforce_pqc_signatures: bool,
    pub next_alloc_addr: u64,
}

impl SovereignModularKernelEngine {
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            symbol_table: BTreeMap::new(),
            enforce_pqc_signatures: true,
            next_alloc_addr: 0xFFFF_8000_0000_0000,
        }
    }

    /// Load kernel module (Linux `insmod`, FreeBSD `kldload`)
    pub fn kldload_insmod(
        &mut self,
        manifest: KernelModuleManifest,
        size_bytes: usize,
    ) -> Result<u64, &'static str> {
        if self.modules.contains_key(&manifest.name) {
            return Err("ModuleAlreadyLoaded");
        }

        if self.enforce_pqc_signatures && !manifest.is_pqc_signed {
            return Err("PqcSignatureVerificationFailed");
        }

        // Verify dependencies
        for dep in &manifest.dependencies {
            if let Some(loaded_dep) = self.modules.get_mut(dep) {
                if loaded_dep.status != ModuleStatus::Active {
                    return Err("DependencyNotActive");
                }
                loaded_dep.ref_count += 1;
            } else {
                return Err("MissingDependency");
            }
        }

        let base_addr = self.next_alloc_addr;
        self.next_alloc_addr += ((size_bytes + 0xFFF) & !0xFFF) as u64; // Align to 4KB page boundary

        // Register exported symbols
        for sym in &manifest.exported_symbols {
            self.symbol_table.insert(sym.clone(), manifest.name.clone());
        }

        let module = LoadedKernelModule {
            manifest: manifest.clone(),
            status: ModuleStatus::Active,
            ref_count: 0,
            memory_base_addr: base_addr,
            memory_size_bytes: size_bytes,
        };

        self.modules.insert(manifest.name, module);
        Ok(base_addr)
    }

    /// Unload kernel module (Linux `rmmod`, FreeBSD `kldunload`)
    pub fn kldunload_rmmod(&mut self, name: &str) -> Result<(), &'static str> {
        let module = self.modules.get(name).ok_or("ModuleNotFound")?;

        if module.ref_count > 0 {
            return Err("ModuleInUse");
        }

        let deps = module.manifest.dependencies.clone();
        let syms = module.manifest.exported_symbols.clone();

        self.modules.remove(name);

        // Clean up symbol table
        for sym in syms {
            self.symbol_table.remove(&sym);
        }

        // Decrement dependency ref counts
        for dep in deps {
            if let Some(dep_mod) = self.modules.get_mut(&dep) {
                if dep_mod.ref_count > 0 {
                    dep_mod.ref_count -= 1;
                }
            }
        }

        Ok(())
    }

    pub fn set_module_param(
        &mut self,
        module_name: &str,
        key: &str,
        value: &str,
    ) -> Result<(), &'static str> {
        let module = self.modules.get_mut(module_name).ok_or("ModuleNotFound")?;
        module.manifest.params.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn get_module_param(&self, module_name: &str, key: &str) -> Option<String> {
        self.modules
            .get(module_name)
            .and_then(|m| m.manifest.params.get(key).cloned())
    }

    pub fn resolve_symbol(&self, symbol: &str) -> Option<String> {
        self.symbol_table.get(symbol).cloned()
    }
}

impl Default for SovereignModularKernelEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Universal Hardware Driver Engine & Bus Controller
// (Linux udev/sysfs & FreeBSD devd/devfs & PCI/USB/NVMe/VirtIO/ACPI)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusType {
    Pci,
    Usb,
    Nvme,
    VirtIo,
    Acpi,
    I2c,
}

#[derive(Debug, Clone)]
pub struct HardwareDeviceNode {
    pub device_id: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub bus_type: BusType,
    pub path: String, // e.g. "/sys/bus/pci/devices/0000:01:00.0"
    pub bound_driver: Option<String>,
    pub irq_vector: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct HardwareDriver {
    pub name: String,
    pub supported_buses: Vec<BusType>,
    pub vendor_mask: Option<u16>,
    pub device_mask: Option<u16>,
    pub active_instances: usize,
}

pub struct SovereignDriverManager {
    pub devices: BTreeMap<String, HardwareDeviceNode>,
    pub registered_drivers: BTreeMap<String, HardwareDriver>,
    pub devfs_nodes: Vec<String>,
}

impl SovereignDriverManager {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            registered_drivers: BTreeMap::new(),
            devfs_nodes: Vec::new(),
        }
    }

    pub fn register_driver(&mut self, driver: HardwareDriver) {
        self.registered_drivers.insert(driver.name.clone(), driver);
    }

    /// Hotplug device discovery (Linux `udev`, FreeBSD `devd`)
    pub fn device_hotplug_add(&mut self, dev: HardwareDeviceNode) -> Option<String> {
        let dev_id = dev.device_id.clone();
        let dev_path = dev.path.clone();

        // Match against registered drivers
        let mut matched_driver = None;
        for (drv_name, drv) in &self.registered_drivers {
            if drv.supported_buses.contains(&dev.bus_type) {
                let vendor_match = drv.vendor_mask.map_or(true, |v| v == dev.vendor_id);
                let device_match = drv.device_mask.map_or(true, |d| d == dev.product_id);
                if vendor_match && device_match {
                    matched_driver = Some(drv_name.clone());
                    break;
                }
            }
        }

        let mut node = dev;
        if let Some(ref drv_name) = matched_driver {
            node.bound_driver = Some(drv_name.clone());
            if let Some(drv) = self.registered_drivers.get_mut(drv_name) {
                drv.active_instances += 1;
            }
        }

        self.devfs_nodes.push(dev_path);
        self.devices.insert(dev_id, node);
        matched_driver
    }

    pub fn device_hotplug_remove(&mut self, device_id: &str) -> Result<(), &'static str> {
        let node = self.devices.remove(device_id).ok_or("DeviceNotFound")?;

        if let Some(drv_name) = node.bound_driver {
            if let Some(drv) = self.registered_drivers.get_mut(&drv_name) {
                if drv.active_instances > 0 {
                    drv.active_instances -= 1;
                }
            }
        }

        self.devfs_nodes.retain(|p| p != &node.path);
        Ok(())
    }
}

impl Default for SovereignDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Process Control & Scheduling Subsystem
// (Linux cgroups v2, EEVDF/BORE, Futexes, FreeBSD Jails / Capsicum, kqueue)
// =========================================================================

#[derive(Debug, Clone)]
pub struct ResourceQuotaCgroup {
    pub group_id: String,
    pub cpu_quota_pct: u32,
    pub memory_max_bytes: u64,
    pub max_pids: u32,
    pub active_pids: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct ProcessControlEntry {
    pub pid: u64,
    pub name: String,
    pub cgroup_id: String,
    pub priority_weight: u32,
    pub in_capsicum_sandbox: bool,
    pub allowed_rights_mask: u64,
    pub state: ProcessState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    FutexBlocked(u64),
    Suspended,
    Exited(i32),
}

pub struct SovereignProcessControlManager {
    pub processes: BTreeMap<u64, ProcessControlEntry>,
    pub cgroups: BTreeMap<String, ResourceQuotaCgroup>,
    pub futex_waiters: BTreeMap<u64, Vec<u64>>, // futex_addr -> vec of pids
    pub kqueue_events: Vec<(u64, String)>,       // (pid, event_type)
}

impl SovereignProcessControlManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            processes: BTreeMap::new(),
            cgroups: BTreeMap::new(),
            futex_waiters: BTreeMap::new(),
            kqueue_events: Vec::new(),
        };

        // Root cgroup
        mgr.cgroups.insert(
            "root".to_string(),
            ResourceQuotaCgroup {
                group_id: "root".to_string(),
                cpu_quota_pct: 100,
                memory_max_bytes: u64::MAX,
                max_pids: 32768,
                active_pids: Vec::new(),
            },
        );

        mgr
    }

    pub fn create_cgroup(&mut self, id: &str, cpu_pct: u32, mem_max: u64, max_pids: u32) {
        self.cgroups.insert(
            id.to_string(),
            ResourceQuotaCgroup {
                group_id: id.to_string(),
                cpu_quota_pct: cpu_pct,
                memory_max_bytes: mem_max,
                max_pids,
                active_pids: Vec::new(),
            },
        );
    }

    pub fn spawn_process(
        &mut self,
        pid: u64,
        name: &str,
        cgroup_id: &str,
        weight: u32,
    ) -> Result<(), &'static str> {
        let cg = self.cgroups.get_mut(cgroup_id).ok_or("CgroupNotFound")?;

        if cg.active_pids.len() >= cg.max_pids as usize {
            return Err("CgroupPidLimitExceeded");
        }

        cg.active_pids.push(pid);

        self.processes.insert(
            pid,
            ProcessControlEntry {
                pid,
                name: name.to_string(),
                cgroup_id: cgroup_id.to_string(),
                priority_weight: weight,
                in_capsicum_sandbox: false,
                allowed_rights_mask: u64::MAX,
                state: ProcessState::Ready,
            },
        );

        self.kqueue_events.push((pid, "EVFILT_PROC_CREATE".to_string()));
        Ok(())
    }

    /// FreeBSD Capsicum capability mode enter
    pub fn enter_capsicum_sandbox(&mut self, pid: u64, rights_mask: u64) -> Result<(), &'static str> {
        let proc_entry = self.processes.get_mut(&pid).ok_or("ProcessNotFound")?;
        proc_entry.in_capsicum_sandbox = true;
        proc_entry.allowed_rights_mask = rights_mask;
        Ok(())
    }

    /// Futex wait operation
    pub fn futex_wait(&mut self, pid: u64, addr: u64) -> Result<(), &'static str> {
        let proc_entry = self.processes.get_mut(&pid).ok_or("ProcessNotFound")?;
        proc_entry.state = ProcessState::FutexBlocked(addr);
        self.futex_waiters.entry(addr).or_default().push(pid);
        Ok(())
    }

    /// Futex wake operation
    pub fn futex_wake(&mut self, addr: u64, wake_count: usize) -> usize {
        let mut woken = 0;
        if let Some(waiters) = self.futex_waiters.get_mut(&addr) {
            while woken < wake_count && !waiters.is_empty() {
                let pid = waiters.remove(0);
                if let Some(proc_entry) = self.processes.get_mut(&pid) {
                    proc_entry.state = ProcessState::Ready;
                    woken += 1;
                }
            }
        }
        woken
    }
}

impl Default for SovereignProcessControlManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Sovereign Network Stack & Firewall Subsystem
// (OpenBSD PF packet filter, Linux eBPF/XDP engine, VNET stack, IPv4/IPv6)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketAction {
    Pass,
    Block,
    Nat,
}

#[derive(Debug, Clone)]
pub struct PfFirewallRule {
    pub rule_id: u32,
    pub action: PacketAction,
    pub protocol: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub dst_port: u16,
}

#[derive(Debug, Clone)]
pub struct SocketBuffer {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub payload: Vec<u8>,
}

pub struct SovereignNetworkStackManager {
    pub pf_rules: Vec<PfFirewallRule>,
    pub xdp_programs_attached: usize,
    pub vnet_namespaces: Vec<String>,
    pub packet_ring_buffer: Vec<SocketBuffer>,
}

impl SovereignNetworkStackManager {
    pub fn new() -> Self {
        Self {
            pf_rules: Vec::new(),
            xdp_programs_attached: 0,
            vnet_namespaces: vec!["default_vnet".to_string()],
            packet_ring_buffer: Vec::new(),
        }
    }

    pub fn add_pf_rule(
        &mut self,
        action: PacketAction,
        proto: &str,
        src: &str,
        dst: &str,
        port: u16,
    ) {
        let id = self.pf_rules.len() as u32 + 1;
        self.pf_rules.push(PfFirewallRule {
            rule_id: id,
            action,
            protocol: proto.to_string(),
            src_ip: src.to_string(),
            dst_ip: dst.to_string(),
            dst_port: port,
        });
    }

    /// Fast-path packet evaluation (OpenBSD PF + eBPF/XDP)
    pub fn evaluate_and_route_packet(&mut self, pkt: SocketBuffer) -> PacketAction {
        let mut final_action = PacketAction::Pass;

        for rule in &self.pf_rules {
            if (rule.src_ip == "*" || rule.src_ip == pkt.src_ip)
                && (rule.dst_ip == "*" || rule.dst_ip == pkt.dst_ip)
                && (rule.dst_port == 0 || rule.dst_port == pkt.dst_port)
            {
                final_action = rule.action;
            }
        }

        if final_action == PacketAction::Pass {
            self.packet_ring_buffer.push(pkt);
        }

        final_action
    }

    pub fn attach_xdp_filter(&mut self) {
        self.xdp_programs_attached += 1;
    }

    pub fn create_vnet_namespace(&mut self, name: &str) {
        self.vnet_namespaces.push(name.to_string());
    }
}

impl Default for SovereignNetworkStackManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Peripheral Access & Event Subsystem
// (USB xHCI, HID input mapping, Audio HDA, Bluetooth, V4L2 Camera, TPM 2.0)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeripheralCategory {
    UsbXhci,
    HidKeyboardMouse,
    AudioHda,
    Bluetooth,
    V4l2Camera,
    Tpm2Security,
    EnvironmentalSensor,
}

#[derive(Debug, Clone)]
pub struct PeripheralEvent {
    pub device_id: String,
    pub category: PeripheralCategory,
    pub timestamp_ns: u64,
    pub event_code: u32,
    pub data: Vec<u8>,
}

pub struct SovereignPeripheralAccessManager {
    pub registered_peripherals: BTreeMap<String, PeripheralCategory>,
    pub event_ring_buffer: Vec<PeripheralEvent>,
    pub tpm2_active: bool,
}

impl SovereignPeripheralAccessManager {
    pub fn new() -> Self {
        Self {
            registered_peripherals: BTreeMap::new(),
            event_ring_buffer: Vec::new(),
            tpm2_active: true,
        }
    }

    pub fn register_peripheral(&mut self, id: &str, cat: PeripheralCategory) {
        self.registered_peripherals.insert(id.to_string(), cat);
    }

    pub fn push_event(&mut self, event: PeripheralEvent) {
        self.event_ring_buffer.push(event);
    }

    pub fn poll_events(&mut self) -> Vec<PeripheralEvent> {
        let events = self.event_ring_buffer.clone();
        self.event_ring_buffer.clear();
        events
    }
}

impl Default for SovereignPeripheralAccessManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. Virtual File System (VFS) & Storage Framework
// (Ext4, ZFS, Btrfs snapshots, Soft Updates FFS, DevFS, ProcFS, io_uring)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemKind {
    Ext4,
    Zfs,
    Btrfs,
    FfsSoftUpdates,
    DevFs,
    ProcFs,
    SysFs,
    SigmaFsPlusPlus,
}

#[derive(Debug, Clone)]
pub struct VfsMountPoint {
    pub path: String,
    pub kind: FilesystemKind,
    pub device_source: String,
    pub is_read_only: bool,
}

#[derive(Debug, Clone)]
pub struct IoUringSubmissionEntry {
    pub sqe_id: u64,
    pub opcode: u8, // 0 = Read, 1 = Write, 2 = Fsync
    pub fd: u32,
    pub offset: u64,
    pub buffer_len: usize,
}

pub struct SovereignVfsStorageManager {
    pub mount_table: Vec<VfsMountPoint>,
    pub io_uring_sq: Vec<IoUringSubmissionEntry>,
    pub io_uring_cq: Vec<(u64, i32)>, // (sqe_id, result_bytes)
    pub btrfs_snapshots: Vec<String>,
}

impl SovereignVfsStorageManager {
    pub fn new() -> Self {
        Self {
            mount_table: vec![
                VfsMountPoint {
                    path: "/".to_string(),
                    kind: FilesystemKind::SigmaFsPlusPlus,
                    device_source: "/dev/nvme0n1p2".to_string(),
                    is_read_only: false,
                },
                VfsMountPoint {
                    path: "/dev".to_string(),
                    kind: FilesystemKind::DevFs,
                    device_source: "devfs".to_string(),
                    is_read_only: false,
                },
                VfsMountPoint {
                    path: "/proc".to_string(),
                    kind: FilesystemKind::ProcFs,
                    device_source: "procfs".to_string(),
                    is_read_only: true,
                },
            ],
            io_uring_sq: Vec::new(),
            io_uring_cq: Vec::new(),
            btrfs_snapshots: Vec::new(),
        }
    }

    pub fn mount_fs(&mut self, path: &str, kind: FilesystemKind, src: &str, ro: bool) {
        self.mount_table.push(VfsMountPoint {
            path: path.to_string(),
            kind,
            device_source: src.to_string(),
            is_read_only: ro,
        });
    }

    pub fn submit_io_uring(&mut self, entry: IoUringSubmissionEntry) {
        let sqe_id = entry.sqe_id;
        let len = entry.buffer_len as i32;
        self.io_uring_sq.push(entry);
        // Simulate async completion queue processing
        self.io_uring_cq.push((sqe_id, len));
    }

    pub fn create_snapshot(&mut self, snap_name: &str) {
        self.btrfs_snapshots.push(snap_name.to_string());
    }
}

impl Default for SovereignVfsStorageManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Sovereign Kernel Subsystem Orchestrator Engine
// =========================================================================

pub struct SovereignKernelSubsystemOrchestrator {
    pub module_engine: SovereignModularKernelEngine,
    pub driver_manager: SovereignDriverManager,
    pub process_manager: SovereignProcessControlManager,
    pub network_manager: SovereignNetworkStackManager,
    pub peripheral_manager: SovereignPeripheralAccessManager,
    pub vfs_manager: SovereignVfsStorageManager,
    pub is_kernel_synchronized: bool,
}

impl SovereignKernelSubsystemOrchestrator {
    pub fn new() -> Self {
        Self {
            module_engine: SovereignModularKernelEngine::new(),
            driver_manager: SovereignDriverManager::new(),
            process_manager: SovereignProcessControlManager::new(),
            network_manager: SovereignNetworkStackManager::new(),
            peripheral_manager: SovereignPeripheralAccessManager::new(),
            vfs_manager: SovereignVfsStorageManager::new(),
            is_kernel_synchronized: true,
        }
    }

    pub fn synchronize_kernel_subsystems(&mut self) -> bool {
        self.is_kernel_synchronized = true;
        self.is_kernel_synchronized
    }
}

impl Default for SovereignKernelSubsystemOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Unit Tests for Step 1, Step 2 & Step 3
// =========================================================================

#[cfg(test)]
mod step1_tests {
    use super::*;

    #[test]
    fn test_modular_kernel_loader_lifecycle() {
        let mut engine = SovereignModularKernelEngine::new();

        let base_manifest = KernelModuleManifest {
            name: "snd_hda_core".to_string(),
            version: "1.0.0".to_string(),
            author: "SigmaOS".to_string(),
            license: "GPL-2.0".to_string(),
            dependencies: Vec::new(),
            exported_symbols: vec!["azx_sound_init".to_string()],
            is_pqc_signed: true,
            params: BTreeMap::new(),
        };

        let driver_manifest = KernelModuleManifest {
            name: "snd_hda_intel".to_string(),
            version: "1.0.0".to_string(),
            author: "SigmaOS".to_string(),
            license: "GPL-2.0".to_string(),
            dependencies: vec!["snd_hda_core".to_string()],
            exported_symbols: vec!["azx_probe_pci".to_string()],
            is_pqc_signed: true,
            params: BTreeMap::new(),
        };

        let base_addr = engine.kldload_insmod(base_manifest, 0x2000).unwrap();
        assert!(base_addr > 0);
        assert_eq!(engine.resolve_symbol("azx_sound_init"), Some("snd_hda_core".to_string()));

        let drv_addr = engine.kldload_insmod(driver_manifest, 0x3000).unwrap();
        assert!(drv_addr > base_addr);

        // Cannot unload base while driver depends on it
        assert!(engine.kldunload_rmmod("snd_hda_core").is_err());

        // Unload driver first
        assert!(engine.kldunload_rmmod("snd_hda_intel").is_ok());
        // Now base unloads
        assert!(engine.kldunload_rmmod("snd_hda_core").is_ok());
    }

    #[test]
    fn test_driver_manager_hotplug() {
        let mut mgr = SovereignDriverManager::new();

        let nvme_drv = HardwareDriver {
            name: "nvme_driver".to_string(),
            supported_buses: vec![BusType::Nvme, BusType::Pci],
            vendor_mask: Some(0x144D), // Samsung
            device_mask: None,
            active_instances: 0,
        };
        mgr.register_driver(nvme_drv);

        let dev_node = HardwareDeviceNode {
            device_id: "nvme0n1".to_string(),
            vendor_id: 0x144D,
            product_id: 0xA808,
            bus_type: BusType::Nvme,
            path: "/dev/nvme0n1".to_string(),
            bound_driver: None,
            irq_vector: Some(34),
        };

        let matched = mgr.device_hotplug_add(dev_node).unwrap();
        assert_eq!(matched, "nvme_driver");
        assert_eq!(mgr.registered_drivers.get("nvme_driver").unwrap().active_instances, 1);

        assert!(mgr.device_hotplug_remove("nvme0n1").is_ok());
        assert_eq!(mgr.registered_drivers.get("nvme_driver").unwrap().active_instances, 0);
    }
}

#[cfg(test)]
mod step2_tests {
    use super::*;

    #[test]
    fn test_process_control_and_futexes() {
        let mut proc_mgr = SovereignProcessControlManager::new();

        proc_mgr.create_cgroup("sandboxed_app", 50, 1024 * 1024 * 512, 10);
        proc_mgr.spawn_process(101, "zenith_app", "sandboxed_app", 100).unwrap();

        assert!(proc_mgr.enter_capsicum_sandbox(101, 0x05).is_ok());
        let proc_entry = proc_mgr.processes.get(&101).unwrap();
        assert!(proc_entry.in_capsicum_sandbox);
        assert_eq!(proc_entry.allowed_rights_mask, 0x05);

        // Futex blocking & waking
        assert!(proc_mgr.futex_wait(101, 0x7FFF_0000).is_ok());
        assert_eq!(proc_mgr.processes.get(&101).unwrap().state, ProcessState::FutexBlocked(0x7FFF_0000));

        let woken = proc_mgr.futex_wake(0x7FFF_0000, 1);
        assert_eq!(woken, 1);
        assert_eq!(proc_mgr.processes.get(&101).unwrap().state, ProcessState::Ready);
    }

    #[test]
    fn test_network_pf_firewall() {
        let mut net_mgr = SovereignNetworkStackManager::new();
        net_mgr.add_pf_rule(PacketAction::Block, "TCP", "*", "10.0.0.5", 80);

        let valid_pkt = SocketBuffer {
            src_ip: "192.168.1.50".to_string(),
            dst_ip: "10.0.0.6".to_string(),
            src_port: 54321,
            dst_port: 80,
            payload: vec![1, 2, 3],
        };

        let blocked_pkt = SocketBuffer {
            src_ip: "192.168.1.50".to_string(),
            dst_ip: "10.0.0.5".to_string(),
            src_port: 54321,
            dst_port: 80,
            payload: vec![1, 2, 3],
        };

        assert_eq!(net_mgr.evaluate_and_route_packet(valid_pkt), PacketAction::Pass);
        assert_eq!(net_mgr.evaluate_and_route_packet(blocked_pkt), PacketAction::Block);
        assert_eq!(net_mgr.packet_ring_buffer.len(), 1);
    }
}

#[cfg(test)]
mod step3_tests {
    use super::*;

    #[test]
    fn test_peripheral_access_manager() {
        let mut periph_mgr = SovereignPeripheralAccessManager::new();
        periph_mgr.register_peripheral("usb_mouse_1", PeripheralCategory::HidKeyboardMouse);

        periph_mgr.push_event(PeripheralEvent {
            device_id: "usb_mouse_1".to_string(),
            category: PeripheralCategory::HidKeyboardMouse,
            timestamp_ns: 1_000_000_000,
            event_code: 0x01, // Click
            data: vec![1, 0, 0],
        });

        let events = periph_mgr.poll_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].device_id, "usb_mouse_1");
        assert!(periph_mgr.event_ring_buffer.is_empty());
    }

    #[test]
    fn test_vfs_and_io_uring_manager() {
        let mut vfs_mgr = SovereignVfsStorageManager::new();
        assert_eq!(vfs_mgr.mount_table.len(), 3);

        vfs_mgr.mount_fs("/mnt/data", FilesystemKind::Zfs, "pool/data", false);
        assert_eq!(vfs_mgr.mount_table.len(), 4);

        vfs_mgr.submit_io_uring(IoUringSubmissionEntry {
            sqe_id: 42,
            opcode: 0, // Read
            fd: 3,
            offset: 0,
            buffer_len: 4096,
        });

        assert_eq!(vfs_mgr.io_uring_cq.len(), 1);
        assert_eq!(vfs_mgr.io_uring_cq[0], (42, 4096));

        vfs_mgr.create_snapshot("root_2026_03_28");
        assert_eq!(vfs_mgr.btrfs_snapshots[0], "root_2026_03_28");
    }

    #[test]
    fn test_sovereign_kernel_subsystem_orchestrator() {
        let mut orchestrator = SovereignKernelSubsystemOrchestrator::new();
        assert!(orchestrator.is_kernel_synchronized);
        assert!(orchestrator.synchronize_kernel_subsystems());
    }
}
