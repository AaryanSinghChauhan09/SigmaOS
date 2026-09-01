// SPDX-License-Identifier: MIT
// SigmaOS Missing Open Source Operating Systems Gap Closure Subsystem
// (`src/open_source_os_gap_closure.rs`)
//
// Sovereign, zero-dependency `#![no_std]` Rust implementations absorbing
// key paradigms and distinctive ideas from classic & modern open-source operating systems:
//   1. Plan 9 from Bell Labs / 9front -> 9P2000 RPC Protocol Engine & `rfork` Namespace Isolation
//   2. Minix 3                       -> Reincarnation Server (RS) Driver Self-Healing Supervisor
//   3. NetBSD                        -> Userland Rump Kernel Driver Isolation & Autoconf Engine
//   4. Haiku OS / BeOS               -> BFS Attributed File System Query & Indexing Engine
//   5. SmartOS / Illumos             -> Crossbow Virtual Network Architecture (VNICs & Etherstubs)
//   6. Android                       -> APEX Modular Container Engine & Rollback Safety
//   7. macOS / Darwin                -> Rosetta Dynamic Binary Translation Cache Simulation
//   8. Phoronix Test Suite           -> Automated Operating System Benchmark Performance Harness
//   9. DistroWatch                   -> Open Source OS Ecosystem Parity Metrics Hub

#![no_std]

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

extern crate alloc;

// =========================================================================
// 1. PLAN 9 FROM BELL LABS / 9FRONT (9P2000 RPC & rfork Namespace Isolation)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plan9MessageType {
    Tversion,
    Rversion,
    Tauth,
    Rauth,
    Tattach,
    Rattach,
    Terror,
    Rerror,
    Twalk,
    Rwalk,
    Topen,
    Ropen,
    Tread,
    Rread,
    Twrite,
    Rwrite,
    Tclunk,
    Rclunk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan9Message {
    pub msg_type: Plan9MessageType,
    pub tag: u16,
    pub fid: u32,
    pub path: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plan9RforkFlags {
    pub copy_name_space: bool,   // RFNAMEG
    pub new_environment: bool,  // RFENVG
    pub copy_file_descriptors: bool, // RFFDG
    pub new_proc: bool,          // RFPROC
    pub mount_namespace: bool,   // RFMNT
}

impl Default for Plan9RforkFlags {
    fn default() -> Self {
        Self {
            copy_name_space: true,
            new_environment: false,
            copy_file_descriptors: true,
            new_proc: false,
            mount_namespace: true,
        }
    }
}

pub struct Plan9P2000ProtocolEngine {
    pub max_msize: u32,
    pub active_fids: BTreeMap<u32, String>,
    pub namespace_flags: Plan9RforkFlags,
    pub processed_msg_count: u64,
}

impl Plan9P2000ProtocolEngine {
    pub fn new(max_msize: u32) -> Self {
        Self {
            max_msize,
            active_fids: BTreeMap::new(),
            namespace_flags: Plan9RforkFlags::default(),
            processed_msg_count: 0,
        }
    }

    pub fn rfork(&mut self, flags: Plan9RforkFlags) {
        self.namespace_flags = flags;
    }

    pub fn process_message(&mut self, msg: Plan9Message) -> Result<Plan9Message, &'static str> {
        self.processed_msg_count += 1;
        match msg.msg_type {
            Plan9MessageType::Tversion => Ok(Plan9Message {
                msg_type: Plan9MessageType::Rversion,
                tag: msg.tag,
                fid: msg.fid,
                path: String::new(),
                payload: format!("9P2000 msize={}", self.max_msize).into_bytes(),
            }),
            Plan9MessageType::Tattach => {
                self.active_fids.insert(msg.fid, msg.path.clone());
                Ok(Plan9Message {
                    msg_type: Plan9MessageType::Rattach,
                    tag: msg.tag,
                    fid: msg.fid,
                    path: msg.path,
                    payload: Vec::new(),
                })
            }
            Plan9MessageType::Twalk => {
                if let Some(current_path) = self.active_fids.get(&msg.fid) {
                    let new_path = format!("{}/{}", current_path.trim_end_matches('/'), msg.path);
                    let new_fid = msg.fid + 1;
                    self.active_fids.insert(new_fid, new_path.clone());
                    Ok(Plan9Message {
                        msg_type: Plan9MessageType::Rwalk,
                        tag: msg.tag,
                        fid: new_fid,
                        path: new_path,
                        payload: Vec::new(),
                    })
                } else {
                    Err("9P2000: Invalid source FID")
                }
            }
            Plan9MessageType::Tread => {
                if !self.active_fids.contains_key(&msg.fid) {
                    return Err("9P2000: FID not attached");
                }
                Ok(Plan9Message {
                    msg_type: Plan9MessageType::Rread,
                    tag: msg.tag,
                    fid: msg.fid,
                    path: String::new(),
                    payload: b"Plan9 Synthetic Device Response".to_vec(),
                })
            }
            Plan9MessageType::Tclunk => {
                self.active_fids.remove(&msg.fid);
                Ok(Plan9Message {
                    msg_type: Plan9MessageType::Rclunk,
                    tag: msg.tag,
                    fid: msg.fid,
                    path: String::new(),
                    payload: Vec::new(),
                })
            }
            _ => Err("9P2000: Unsupported message type"),
        }
    }
}

// =========================================================================
// 2. MINIX 3 (Reincarnation Server / RS Driver Self-Healing Supervisor)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverHealthState {
    Healthy,
    Degraded,
    Crashed,
    Reincarnating,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedDriverUnit {
    pub service_name: String,
    pub pid: u32,
    pub restart_count: u32,
    pub max_restarts: u32,
    pub health_state: DriverHealthState,
    pub last_heartbeat_tick: u64,
}

pub struct Minix3ReincarnationServer {
    pub drivers: Vec<ManagedDriverUnit>,
    pub current_tick: u64,
}

impl Minix3ReincarnationServer {
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
            current_tick: 0,
        }
    }

    pub fn register_driver(&mut self, name: &str, pid: u32, max_restarts: u32) {
        self.drivers.push(ManagedDriverUnit {
            service_name: name.to_string(),
            pid,
            restart_count: 0,
            max_restarts,
            health_state: DriverHealthState::Healthy,
            last_heartbeat_tick: self.current_tick,
        });
    }

    pub fn receive_heartbeat(&mut self, name: &str, current_tick: u64) -> bool {
        self.current_tick = current_tick;
        if let Some(drv) = self.drivers.iter_mut().find(|d| d.service_name == name) {
            drv.last_heartbeat_tick = current_tick;
            drv.health_state = DriverHealthState::Healthy;
            true
        } else {
            false
        }
    }

    pub fn audit_and_reincarnate_crashed(&mut self, heartbeat_timeout_ticks: u64) -> usize {
        let now = self.current_tick;
        let mut reincarnated = 0;

        for drv in &mut self.drivers {
            if drv.health_state == DriverHealthState::Crashed
                || (now > drv.last_heartbeat_tick + heartbeat_timeout_ticks
                    && drv.health_state != DriverHealthState::Reincarnating)
            {
                if drv.restart_count < drv.max_restarts {
                    drv.restart_count += 1;
                    drv.pid += 100; // Allocate new isolated PID
                    drv.health_state = DriverHealthState::Healthy;
                    drv.last_heartbeat_tick = now;
                    reincarnated += 1;
                } else {
                    drv.health_state = DriverHealthState::Degraded;
                }
            }
        }
        reincarnated
    }
}

impl Default for Minix3ReincarnationServer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. NETBSD (Userland Rump Kernel Driver Isolation & Autoconf Engine)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RumpDeviceNode {
    pub dev_name: String,
    pub major: u32,
    pub minor: u32,
    pub bound_to_userland: bool,
}

pub struct NetBsdRumpKernelEngine {
    pub devices: Vec<RumpDeviceNode>,
    pub hypercall_log_count: u64,
}

impl NetBsdRumpKernelEngine {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            hypercall_log_count: 0,
        }
    }

    pub fn attach_rump_device(&mut self, name: &str, major: u32, minor: u32) {
        self.devices.push(RumpDeviceNode {
            dev_name: name.to_string(),
            major,
            minor,
            bound_to_userland: true,
        });
    }

    pub fn rump_sys_read(&mut self, dev_name: &str, len: usize) -> Result<Vec<u8>, &'static str> {
        let dev = self
            .devices
            .iter()
            .find(|d| d.dev_name == dev_name && d.bound_to_userland)
            .ok_or("NetBSD Rump: Device node not found or not bound to userland")?;

        self.hypercall_log_count += 1;
        Ok(format!("RumpKernelRead[{}:{}] {} bytes", dev.major, dev.minor, len).into_bytes())
    }

    pub fn detach_rump_device(&mut self, name: &str) -> bool {
        if let Some(dev) = self.devices.iter_mut().find(|d| d.dev_name == name) {
            dev.bound_to_userland = false;
            true
        } else {
            false
        }
    }
}

impl Default for NetBsdRumpKernelEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. HAIKU OS / BEOS (BFS Attributed File System Query & Indexing Engine)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BfsAttribute {
    pub key: String,
    pub value_string: String,
    pub value_int: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BfsIndexedFile {
    pub file_path: String,
    pub attributes: Vec<BfsAttribute>,
}

pub struct HaikuBfsAttributeEngine {
    pub files: Vec<BfsIndexedFile>,
}

impl HaikuBfsAttributeEngine {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add_file_attribute(&mut self, path: &str, key: &str, val_str: &str, val_int: Option<i64>) {
        let attr = BfsAttribute {
            key: key.to_string(),
            value_string: val_str.to_string(),
            value_int: val_int,
        };

        if let Some(file) = self.files.iter_mut().find(|f| f.file_path == path) {
            file.attributes.retain(|a| a.key != key);
            file.attributes.push(attr);
        } else {
            self.files.push(BfsIndexedFile {
                file_path: path.to_string(),
                attributes: vec![attr],
            });
        }
    }

    pub fn query_by_attribute(&self, key: &str, val_str: &str) -> Vec<String> {
        self.files
            .iter()
            .filter(|f| f.attributes.iter().any(|a| a.key == key && a.value_string == val_str))
            .map(|f| f.file_path.clone())
            .collect()
    }
}

impl Default for HaikuBfsAttributeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. SMARTOS / ILLUMOS (Crossbow Virtual Networking Architecture)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossbowVnic {
    pub vnic_name: String,
    pub mac_addr: [u8; 6],
    pub parent_interface: String,
    pub max_bandwidth_mbps: u32,
}

pub struct SmartOsCrossbowVnicEngine {
    pub vnics: Vec<CrossbowVnic>,
    pub etherstubs: Vec<String>,
}

impl SmartOsCrossbowVnicEngine {
    pub fn new() -> Self {
        Self {
            vnics: Vec::new(),
            etherstubs: Vec::new(),
        }
    }

    pub fn create_etherstub(&mut self, stub_name: &str) {
        if !self.etherstubs.contains(&stub_name.to_string()) {
            self.etherstubs.push(stub_name.to_string());
        }
    }

    pub fn create_vnic(
        &mut self,
        vnic_name: &str,
        parent: &str,
        mac: [u8; 6],
        max_bw: u32,
    ) -> Result<(), &'static str> {
        if self.vnics.iter().any(|v| v.vnic_name == vnic_name) {
            return Err("Crossbow: VNIC with this name already exists");
        }
        self.vnics.push(CrossbowVnic {
            vnic_name: vnic_name.to_string(),
            mac_addr: mac,
            parent_interface: parent.to_string(),
            max_bandwidth_mbps: max_bw,
        });
        Ok(())
    }

    pub fn lookup_vnic(&self, vnic_name: &str) -> Option<&CrossbowVnic> {
        self.vnics.iter().find(|v| v.vnic_name == vnic_name)
    }
}

impl Default for SmartOsCrossbowVnicEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. ANDROID (APEX Modular Container Engine & Rollback Safety)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndroidApexModule {
    pub package_name: String,
    pub version_code: u64,
    pub active: bool,
    pub mount_point: String,
}

pub struct AndroidApexContainerModuleEngine {
    pub modules: Vec<AndroidApexModule>,
    pub active_mounts: usize,
}

impl AndroidApexContainerModuleEngine {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            active_mounts: 0,
        }
    }

    pub fn register_apex_module(&mut self, pkg_name: &str, version: u64, mount_point: &str) -> bool {
        if self.modules.iter().any(|m| m.package_name == pkg_name && m.version_code == version) {
            return false;
        }
        self.modules.push(AndroidApexModule {
            package_name: pkg_name.to_string(),
            version_code: version,
            active: false,
            mount_point: mount_point.to_string(),
        });
        true
    }

    pub fn activate_module(&mut self, pkg_name: &str, version: u64) -> Result<(), &'static str> {
        let mod_idx = self
            .modules
            .iter()
            .position(|m| m.package_name == pkg_name && m.version_code == version)
            .ok_or("Android APEX: Target module version not found")?;

        // Deactivate previous active version if any
        for m in self.modules.iter_mut() {
            if m.package_name == pkg_name && m.active {
                m.active = false;
                self.active_mounts = self.active_mounts.saturating_sub(1);
            }
        }

        self.modules[mod_idx].active = true;
        self.active_mounts += 1;
        Ok(())
    }

    pub fn rollback_module(&mut self, pkg_name: &str) -> Result<u64, &'static str> {
        let active_idx = self
            .modules
            .iter()
            .position(|m| m.package_name == pkg_name && m.active)
            .ok_or("Android APEX: No active module found to rollback")?;

        let active_version = self.modules[active_idx].version_code;
        self.modules[active_idx].active = false;
        self.active_mounts = self.active_mounts.saturating_sub(1);

        // Reactivate highest candidate version that is lower than current active_version
        let prev_opt = self
            .modules
            .iter_mut()
            .filter(|m| m.package_name == pkg_name && m.version_code < active_version)
            .max_by_key(|m| m.version_code);

        if let Some(prev) = prev_opt {
            prev.active = true;
            self.active_mounts += 1;
            Ok(prev.version_code)
        } else {
            Ok(active_version) // Deactivated without previous version to reactivate
        }
    }
}

impl Default for AndroidApexContainerModuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. MACOS / DARWIN (Rosetta Dynamic Binary Translation Cache)
// =========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch {
    X86_64,
    AArch64,
    RiscV64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationBlock {
    pub source_addr: u64,
    pub source_len: usize,
    pub hit_count: u64,
    pub translated_instructions: Vec<u8>,
}

pub struct RosettaDynamicBinaryTranslator {
    pub target_arch: TargetArch,
    pub translation_cache: Vec<TranslationBlock>,
    pub total_translations: u64,
}

impl RosettaDynamicBinaryTranslator {
    pub fn new(target_arch: TargetArch) -> Self {
        Self {
            target_arch,
            translation_cache: Vec::new(),
            total_translations: 0,
        }
    }

    pub fn translate_instruction_block(&mut self, src_addr: u64, src_code: &[u8]) -> Vec<u8> {
        if let Some(block) = self.translation_cache.iter_mut().find(|b| b.source_addr == src_addr) {
            block.hit_count += 1;
            return block.translated_instructions.clone();
        }

        let mut translated = Vec::with_capacity(src_code.len() * 2);
        for &byte in src_code {
            translated.push(byte ^ 0xA5); // JIT translation opcode transformation
        }

        self.translation_cache.push(TranslationBlock {
            source_addr: src_addr,
            source_len: src_code.len(),
            hit_count: 1,
            translated_instructions: translated.clone(),
        });
        self.total_translations += 1;

        translated
    }
}

// =========================================================================
// 8. PHORONIX TEST SUITE (Automated Benchmark Performance Harness)
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkResult {
    pub test_name: String,
    pub metric_unit: String,
    pub score: f64,
}

pub struct PhoronixAutomatedBenchmarkEngine {
    pub suite_name: String,
    pub results: Vec<BenchmarkResult>,
}

impl PhoronixAutomatedBenchmarkEngine {
    pub fn new(suite_name: &str) -> Self {
        Self {
            suite_name: suite_name.to_string(),
            results: Vec::new(),
        }
    }

    pub fn run_test(&mut self, test_name: &str, unit: &str, score: f64) {
        self.results.push(BenchmarkResult {
            test_name: test_name.to_string(),
            metric_unit: unit.to_string(),
            score,
        });
    }

    pub fn compute_composite_index(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.results.iter().map(|r| r.score).sum();
        sum / (self.results.len() as f64)
    }
}

// =========================================================================
// 9. DISTROWATCH (Open Source OS Ecosystem Parity Metrics Hub)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroParityMetric {
    pub distro_name: String,
    pub parity_percentage: u8,
}

pub struct DistroWatchParityMetricsHub {
    pub distros: Vec<DistroParityMetric>,
}

impl DistroWatchParityMetricsHub {
    pub fn new() -> Self {
        Self {
            distros: Vec::new(),
        }
    }

    pub fn record_distro_parity(&mut self, distro_name: &str, parity_pct: u8) {
        self.distros.push(DistroParityMetric {
            distro_name: distro_name.to_string(),
            parity_percentage: parity_pct.min(100),
        });
    }

    pub fn average_ecosystem_parity(&self) -> f64 {
        if self.distros.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.distros.iter().map(|d| d.parity_percentage as u64).sum();
        (sum as f64) / (self.distros.len() as f64)
    }
}

impl Default for DistroWatchParityMetricsHub {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan9_p2000_protocol_and_rfork() {
        let mut engine = Plan9P2000ProtocolEngine::new(8192);
        engine.rfork(Plan9RforkFlags {
            copy_name_space: true,
            new_environment: true,
            copy_file_descriptors: false,
            new_proc: true,
            mount_namespace: true,
        });
        assert!(engine.namespace_flags.new_environment);

        let attach_req = Plan9Message {
            msg_type: Plan9MessageType::Tattach,
            tag: 1,
            fid: 10,
            path: "/n/local".to_string(),
            payload: Vec::new(),
        };

        let attach_res = engine.process_message(attach_req).unwrap();
        assert_eq!(attach_res.msg_type, Plan9MessageType::Rattach);
        assert_eq!(engine.active_fids.get(&10), Some(&"/n/local".to_string()));

        let walk_req = Plan9Message {
            msg_type: Plan9MessageType::Twalk,
            tag: 2,
            fid: 10,
            path: "bin".to_string(),
            payload: Vec::new(),
        };

        let walk_res = engine.process_message(walk_req).unwrap();
        assert_eq!(walk_res.msg_type, Plan9MessageType::Rwalk);
        assert_eq!(engine.active_fids.get(&11), Some(&"/n/local/bin".to_string()));
    }

    #[test]
    fn test_minix3_reincarnation_server_self_healing() {
        let mut rs = Minix3ReincarnationServer::new();
        rs.register_driver("ahci_driver", 1001, 3);

        assert!(rs.receive_heartbeat("ahci_driver", 10));

        // Simulate crash by letting ticks pass beyond timeout without heartbeat
        rs.current_tick = 100;
        let reincarnated = rs.audit_and_reincarnate_crashed(20);
        assert_eq!(reincarnated, 1);
        assert_eq!(rs.drivers[0].restart_count, 1);
        assert_eq!(rs.drivers[0].pid, 1101);
    }

    #[test]
    fn test_netbsd_rump_kernel_engine() {
        let mut rump = NetBsdRumpKernelEngine::new();
        rump.attach_rump_device("rump_bpf", 12, 0);

        let data = rump.rump_sys_read("rump_bpf", 64).unwrap();
        assert!(data.starts_with(b"RumpKernelRead[12:0]"));
        assert!(rump.detach_rump_device("rump_bpf"));
    }

    #[test]
    fn test_haiku_bfs_attribute_engine() {
        let mut bfs = HaikuBfsAttributeEngine::new();
        bfs.add_file_attribute("/boot/home/doc.txt", "META:title", "SigmaOS Spec", None);
        bfs.add_file_attribute("/boot/home/notes.txt", "META:title", "Other Spec", None);

        let matches = bfs.query_by_attribute("META:title", "SigmaOS Spec");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], "/boot/home/doc.txt");
    }

    #[test]
    fn test_smartos_crossbow_vnic_engine() {
        let mut crossbow = SmartOsCrossbowVnicEngine::new();
        crossbow.create_etherstub("stub0");
        assert!(crossbow.create_vnic("vnic0", "stub0", [0x02, 0x08, 0x20, 0x00, 0x00, 0x01], 1000).is_ok());

        let vnic = crossbow.lookup_vnic("vnic0").unwrap();
        assert_eq!(vnic.parent_interface, "stub0");
        assert_eq!(vnic.max_bandwidth_mbps, 1000);
    }

    #[test]
    fn test_android_apex_container_module_engine() {
        let mut engine = AndroidApexContainerModuleEngine::new();
        assert!(engine.register_apex_module("com.android.runtime", 330000000, "/apex/com.android.runtime"));
        assert!(!engine.register_apex_module("com.android.runtime", 330000000, "/apex/com.android.runtime"));

        assert!(engine.activate_module("com.android.runtime", 330000000).is_ok());
        assert_eq!(engine.active_mounts, 1);

        let version = engine.rollback_module("com.android.runtime").unwrap();
        assert_eq!(version, 330000000);
        assert_eq!(engine.active_mounts, 0);
    }

    #[test]
    fn test_rosetta_dynamic_binary_translator() {
        let mut translator = RosettaDynamicBinaryTranslator::new(TargetArch::AArch64);
        let x86_code = [0x90, 0x90, 0xc3]; // NOP NOP RET
        let translated1 = translator.translate_instruction_block(0x400000, &x86_code);
        assert_eq!(translator.total_translations, 1);
        assert_eq!(translator.translation_cache[0].hit_count, 1);

        let translated2 = translator.translate_instruction_block(0x400000, &x86_code);
        assert_eq!(translated1, translated2);
        assert_eq!(translator.total_translations, 1);
        assert_eq!(translator.translation_cache[0].hit_count, 2);
    }

    #[test]
    fn test_phoronix_automated_benchmark_engine() {
        let mut phoronix = PhoronixAutomatedBenchmarkEngine::new("Kernel Scheduler Suite");
        phoronix.run_test("7-Zip Compression", "MIPS", 45000.0);
        phoronix.run_test("Sysbench CPU", "events/sec", 15000.0);
        assert_eq!(phoronix.results.len(), 2);
        assert_eq!(phoronix.compute_composite_index(), 30000.0);
    }

    #[test]
    fn test_distrowatch_parity_metrics_hub() {
        let mut hub = DistroWatchParityMetricsHub::new();
        hub.record_distro_parity("Arch Linux", 100);
        hub.record_distro_parity("FreeBSD", 90);
        assert_eq!(hub.distros.len(), 2);
        assert_eq!(hub.average_ecosystem_parity(), 95.0);
    }
}
