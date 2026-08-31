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
//   6. DragonFly BSD                 -> HAMMER2 File System CoW & Multi-Master Clustering Engine
//   7. OpenBSD                       -> Pledge & Unveil Capability & Path Security Engine
//   8. Firecracker / Cloud Hypervisor -> MicroVM Zero-Overhead Virtual Machine Hypervisor

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
// 6. DRAGONFLY BSD (HAMMER2 File System CoW & Multi-Master Clustering Engine)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hammer2BlockRef {
    pub block_offset: u64,
    pub data_len: u32,
    pub checksum_crc32: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hammer2Snapshot {
    pub snapshot_id: u32,
    pub name: String,
    pub root_block_offset: u64,
    pub timestamp_sec: u64,
}

pub struct Hammer2StorageEngine {
    pub cluster_name: String,
    pub allocated_bytes: u64,
    pub block_refs: BTreeMap<u64, Hammer2BlockRef>,
    pub snapshots: Vec<Hammer2Snapshot>,
    pub deduplicated_blocks_count: u64,
}

impl Hammer2StorageEngine {
    pub fn new(cluster_name: &str) -> Self {
        Self {
            cluster_name: cluster_name.to_string(),
            allocated_bytes: 0,
            block_refs: BTreeMap::new(),
            snapshots: Vec::new(),
            deduplicated_blocks_count: 0,
        }
    }

    pub fn write_cow_block(&mut self, payload: &[u8]) -> u64 {
        let mut crc = 0u32;
        for &b in payload {
            crc = crc.wrapping_add(b as u32).wrapping_mul(31);
        }

        // Deduplication check
        if let Some((&offset, _)) = self.block_refs.iter().find(|(_, r)| r.checksum_crc32 == crc) {
            self.deduplicated_blocks_count += 1;
            return offset;
        }

        let new_offset = self.allocated_bytes + 4096;
        self.allocated_bytes = new_offset;

        self.block_refs.insert(
            new_offset,
            Hammer2BlockRef {
                block_offset: new_offset,
                data_len: payload.len() as u32,
                checksum_crc32: crc,
            },
        );

        new_offset
    }

    pub fn create_instant_snapshot(&mut self, name: &str, timestamp: u64) -> u32 {
        let snap_id = (self.snapshots.len() + 1) as u32;
        let root_offset = self.block_refs.keys().last().copied().unwrap_or(0);

        self.snapshots.push(Hammer2Snapshot {
            snapshot_id: snap_id,
            name: name.to_string(),
            root_block_offset: root_offset,
            timestamp_sec: timestamp,
        });

        snap_id
    }
}

// =========================================================================
// 7. OPENBSD (Pledge & Unveil Capability & Path Security Engine)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenBsdPledgePromises {
    Stdio,
    Rpath,
    Wpath,
    Cpath,
    Inet,
    Dns,
    Exec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBsdUnveilRule {
    pub path: String,
    pub permissions: String, // "r", "w", "x", "c", "rwxc"
}

pub struct OpenBsdPledgeUnveilEngine {
    pub active_promises: Vec<OpenBsdPledgePromises>,
    pub pledge_enforced: bool,
    pub unveil_rules: Vec<OpenBsdUnveilRule>,
    pub unveil_locked: bool,
}

impl OpenBsdPledgeUnveilEngine {
    pub fn new() -> Self {
        Self {
            active_promises: Vec::new(),
            pledge_enforced: false,
            unveil_rules: Vec::new(),
            unveil_locked: false,
        }
    }

    pub fn pledge(&mut self, promises: &[OpenBsdPledgePromises]) -> Result<(), &'static str> {
        if self.pledge_enforced {
            // Pledge can only restrict promises further, never expand
            for p in promises {
                if !self.active_promises.contains(p) {
                    return Err("Pledge: Cannot elevate syscall promises once enforced");
                }
            }
        }
        self.active_promises = promises.to_vec();
        self.pledge_enforced = true;
        Ok(())
    }

    pub fn unveil(&mut self, path: &str, permissions: &str) -> Result<(), &'static str> {
        if self.unveil_locked {
            return Err("Unveil: Unveil restrictions locked by unveil(NULL, NULL)");
        }
        if path.is_empty() && permissions.is_empty() {
            self.unveil_locked = true;
            return Ok(());
        }

        self.unveil_rules.retain(|r| r.path != path);
        self.unveil_rules.push(OpenBsdUnveilRule {
            path: path.to_string(),
            permissions: permissions.to_string(),
        });

        Ok(())
    }

    pub fn check_syscall(&self, promise: OpenBsdPledgePromises) -> bool {
        if !self.pledge_enforced {
            return true;
        }
        self.active_promises.contains(&promise)
    }

    pub fn check_path_access(&self, path: &str, required_perm: char) -> bool {
        if self.unveil_rules.is_empty() {
            return true;
        }
        for rule in &self.unveil_rules {
            if path.starts_with(&rule.path) {
                return rule.permissions.contains(required_perm);
            }
        }
        false
    }
}

impl Default for OpenBsdPledgeUnveilEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. FIRECRACKER / CLOUD HYPERVISOR (MicroVM Hypervisor Engine)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Created,
    Running,
    Paused,
    Terminated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicroVmInstance {
    pub vm_id: String,
    pub vcpu_count: u32,
    pub memory_mb: u64,
    pub kernel_path: String,
    pub state: VmState,
    pub virtio_devices: Vec<String>,
}

pub struct MicroVmHypervisorOrchestrator {
    pub instances: Vec<MicroVmInstance>,
}

impl MicroVmHypervisorOrchestrator {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    pub fn create_microvm(
        &mut self,
        vm_id: &str,
        vcpus: u32,
        memory_mb: u64,
        kernel: &str,
    ) -> Result<(), &'static str> {
        if self.instances.iter().any(|i| i.vm_id == vm_id) {
            return Err("MicroVM: Instance ID already exists");
        }
        self.instances.push(MicroVmInstance {
            vm_id: vm_id.to_string(),
            vcpu_count: vcpus,
            memory_mb,
            kernel_path: kernel.to_string(),
            state: VmState::Created,
            virtio_devices: Vec::from(["virtio-net".to_string(), "virtio-blk".to_string()]),
        });
        Ok(())
    }

    pub fn boot_microvm(&mut self, vm_id: &str) -> Result<(), &'static str> {
        let instance = self
            .instances
            .iter_mut()
            .find(|i| i.vm_id == vm_id)
            .ok_or("MicroVM: Instance not found")?;

        instance.state = VmState::Running;
        Ok(())
    }

    pub fn balloon_memory(&mut self, vm_id: &str, new_memory_mb: u64) -> Result<(), &'static str> {
        let instance = self
            .instances
            .iter_mut()
            .find(|i| i.vm_id == vm_id)
            .ok_or("MicroVM: Instance not found")?;

        instance.memory_mb = new_memory_mb;
        Ok(())
    }
}

impl Default for MicroVmHypervisorOrchestrator {
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
    fn test_dragonfly_hammer2_storage_engine() {
        let mut hammer2 = Hammer2StorageEngine::new("dragon_cluster");
        let offset1 = hammer2.write_cow_block(b"hammer2_block_data");
        assert!(offset1 > 0);

        // Deduplicated write
        let offset2 = hammer2.write_cow_block(b"hammer2_block_data");
        assert_eq!(offset1, offset2);
        assert_eq!(hammer2.deduplicated_blocks_count, 1);

        let snap_id = hammer2.create_instant_snapshot("root_v1", 1700000000);
        assert_eq!(snap_id, 1);
    }

    #[test]
    fn test_openbsd_pledge_unveil_engine() {
        let mut openbsd = OpenBsdPledgeUnveilEngine::new();
        openbsd
            .unveil("/var/log", "rw")
            .expect("Unveil failed");
        assert!(openbsd.check_path_access("/var/log/syslog", 'r'));
        assert!(!openbsd.check_path_access("/var/log/syslog", 'x'));
        assert!(!openbsd.check_path_access("/etc/shadow", 'r'));

        openbsd
            .pledge(&[OpenBsdPledgePromises::Stdio, OpenBsdPledgePromises::Rpath])
            .expect("Pledge failed");
        assert!(openbsd.check_syscall(OpenBsdPledgePromises::Stdio));
        assert!(!openbsd.check_syscall(OpenBsdPledgePromises::Inet));

        // Elevating promises should fail
        assert!(openbsd
            .pledge(&[
                OpenBsdPledgePromises::Stdio,
                OpenBsdPledgePromises::Rpath,
                OpenBsdPledgePromises::Inet
            ])
            .is_err());
    }

    #[test]
    fn test_microvm_hypervisor_orchestrator() {
        let mut hypervisor = MicroVmHypervisorOrchestrator::new();
        hypervisor
            .create_microvm("vm-alpha", 2, 512, "/boot/vmlinux-6.6")
            .unwrap();

        assert!(hypervisor.boot_microvm("vm-alpha").is_ok());
        assert_eq!(hypervisor.instances[0].state, VmState::Running);

        assert!(hypervisor.balloon_memory("vm-alpha", 1024).is_ok());
        assert_eq!(hypervisor.instances[0].memory_mb, 1024);
    }
}
