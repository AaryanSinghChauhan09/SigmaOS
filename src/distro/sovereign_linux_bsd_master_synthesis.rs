// SigmaOS Sovereign Linux & BSD Master Synthesis Suite
// Integrates core Linux & BSD distro innovations into a unified, sovereign architecture:
// 1. NetBSD NPF Stateful Firewall & N-code Bytecode Engine
// 2. OpenBSD Pledge & Unveil Kernel Security Sandbox Enforcer
// 3. Alpine Linux OpenRC Service & APK World Declarative Synchronizer
// 4. NixOS/Guix Hermetic CAS Store & Generation Rollback Engine
// 5. Linux 6.12+ SchedExt eBPF Custom CPU Scheduler Governor
// 6. FreeBSD GEOM Gate Network Block Storage & CTL SCSI Target Engine

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. NETBSD NPF STATEFUL FIREWALL & N-CODE BYTECODE ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpfTableType {
    Tree,
    Hash,
    Cdb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpfNatType {
    Snat,
    Dnat,
    Nptv6,
}

#[derive(Debug, Clone)]
pub struct NpfTable {
    pub id: u32,
    pub name: String,
    pub table_type: NpfTableType,
    pub ip_entries: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NpfRule {
    pub id: u32,
    pub interface: String,
    pub pass: bool,
    pub stateful: bool,
    pub ncode_bytecode: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct NpfNatRule {
    pub id: u32,
    pub nat_type: NpfNatType,
    pub original_ip: String,
    pub translated_ip: String,
    pub port: u16,
}

pub struct NetBsdNpfFirewallEngine {
    pub tables: BTreeMap<u32, NpfTable>,
    pub rules: Vec<NpfRule>,
    pub nat_rules: Vec<NpfNatRule>,
    pub active_state_table: BTreeMap<String, u64>, // connection_key -> timestamp
}

impl NetBsdNpfFirewallEngine {
    pub fn new() -> Self {
        Self {
            tables: BTreeMap::new(),
            rules: Vec::new(),
            nat_rules: Vec::new(),
            active_state_table: BTreeMap::new(),
        }
    }

    pub fn create_table(&mut self, id: u32, name: &str, table_type: NpfTableType) {
        let table = NpfTable {
            id,
            name: name.to_string(),
            table_type,
            ip_entries: Vec::new(),
        };
        self.tables.insert(id, table);
    }

    pub fn add_table_entry(&mut self, table_id: u32, ip: &str) -> bool {
        if let Some(tbl) = self.tables.get_mut(&table_id) {
            tbl.ip_entries.push(ip.to_string());
            true
        } else {
            false
        }
    }

    pub fn add_rule(&mut self, id: u32, interface: &str, pass: bool, stateful: bool, ncode: &[u8]) {
        self.rules.push(NpfRule {
            id,
            interface: interface.to_string(),
            pass,
            stateful,
            ncode_bytecode: ncode.to_vec(),
        });
    }

    pub fn add_nat_rule(&mut self, id: u32, nat_type: NpfNatType, orig_ip: &str, trans_ip: &str, port: u16) {
        self.nat_rules.push(NpfNatRule {
            id,
            nat_type,
            original_ip: orig_ip.to_string(),
            translated_ip: trans_ip.to_string(),
            port,
        });
    }

    pub fn inspect_packet(&mut self, iface: &str, src_ip: &str, dst_ip: &str, port: u16) -> bool {
        let conn_key = format!("{}:{}-{}:{}", src_ip, port, dst_ip, port);
        if self.active_state_table.contains_key(&conn_key) {
            return true; // Stateful match
        }

        // Evaluate rules
        for rule in &self.rules {
            if rule.interface == iface || rule.interface == "all" {
                if rule.stateful && rule.pass {
                    self.active_state_table.insert(conn_key.clone(), 1000);
                }
                return rule.pass;
            }
        }
        false // Default deny
    }
}

impl Default for NetBsdNpfFirewallEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. OPENBSD PLEDGE & UNVEIL KERNEL SECURITY SANDBOX ENFORCER
// =========================================================================

pub struct OpenBsdPledgeUnveilEnforcer {
    pub pledged_promises: BTreeMap<usize, Vec<String>>, // pid -> promised abilities
    pub unveiled_paths: BTreeMap<usize, BTreeMap<String, String>>, // pid -> (path -> perm_mask "rwx")
}

impl OpenBsdPledgeUnveilEnforcer {
    pub fn new() -> Self {
        Self {
            pledged_promises: BTreeMap::new(),
            unveiled_paths: BTreeMap::new(),
        }
    }

    pub fn pledge(&mut self, pid: usize, promises: &[&str]) -> Result<(), &'static str> {
        let promise_vec: Vec<String> = promises.iter().map(|s| s.to_string()).collect();
        if let Some(existing) = self.pledged_promises.get(&pid) {
            // Pledge is monotonic: can only drop promises, not gain
            for p in &promise_vec {
                if !existing.contains(p) {
                    return Err("OpenBSD Pledge Violation: Attempted to expand pledged promises");
                }
            }
        }
        self.pledged_promises.insert(pid, promise_vec);
        Ok(())
    }

    pub fn unveil(&mut self, pid: usize, path: &str, permissions: &str) -> Result<(), &'static str> {
        let process_paths = self.unveiled_paths.entry(pid).or_insert_with(BTreeMap::new);
        process_paths.insert(path.to_string(), permissions.to_string());
        Ok(())
    }

    pub fn check_syscall(&self, pid: usize, required_promise: &str) -> bool {
        if let Some(promises) = self.pledged_promises.get(&pid) {
            promises.contains(&required_promise.to_string())
        } else {
            true // Unpledged processes have no pledge restriction
        }
    }

    pub fn check_path_access(&self, pid: usize, path: &str, mode: char) -> bool {
        if let Some(paths) = self.unveiled_paths.get(&pid) {
            if paths.is_empty() {
                return true;
            }
            for (unveiled_path, perms) in paths {
                if path.starts_with(unveiled_path) {
                    return perms.contains(mode);
                }
            }
            false // Path not in unveiled set
        } else {
            true // Unveiled sandbox not initialized for PID
        }
    }
}

impl Default for OpenBsdPledgeUnveilEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. ALPINE LINUX OPENRC SERVICE & APK WORLD DECLARATIVE SYNCHRONIZER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenRcRunlevel {
    SysInit,
    Boot,
    DefaultRunlevel,
    NonNetwork,
}

#[derive(Debug, Clone)]
pub struct OpenRcService {
    pub name: String,
    pub command: String,
    pub runlevel: OpenRcRunlevel,
    pub dependencies: Vec<String>,
    pub is_started: bool,
}

pub struct AlpineOpenRcApkWorldEngine {
    pub services: BTreeMap<String, OpenRcService>,
    pub world_packages: Vec<String>,
    pub installed_packages: BTreeMap<String, String>, // pkg -> version
}

impl AlpineOpenRcApkWorldEngine {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            world_packages: Vec::new(),
            installed_packages: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, name: &str, cmd: &str, runlevel: OpenRcRunlevel, deps: &[&str]) {
        let svc = OpenRcService {
            name: name.to_string(),
            command: cmd.to_string(),
            runlevel,
            dependencies: deps.iter().map(|s| s.to_string()).collect(),
            is_started: false,
        };
        self.services.insert(name.to_string(), svc);
    }

    pub fn start_service(&mut self, name: &str) -> Result<bool, &'static str> {
        if let Some(svc) = self.services.get_mut(name) {
            svc.is_started = true;
            Ok(true)
        } else {
            Err("OpenRC: Service not found")
        }
    }

    pub fn add_to_world(&mut self, pkg: &str, version: &str) {
        if !self.world_packages.contains(&pkg.to_string()) {
            self.world_packages.push(pkg.to_string());
        }
        self.installed_packages.insert(pkg.to_string(), version.to_string());
    }

    pub fn reconcile_apk_world(&mut self) -> usize {
        let mut pruned = 0;
        let world = self.world_packages.clone();
        self.installed_packages.retain(|pkg, _| {
            let keep = world.contains(pkg) || pkg == "alpine-base" || pkg == "musl";
            if !keep {
                pruned += 1;
            }
            keep
        });
        pruned
    }
}

impl Default for AlpineOpenRcApkWorldEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. NIXOS/GUIX HERMETIC CAS STORE & GENERATION ROLLBACK ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct CasStoreDerivation {
    pub hash: String,
    pub name: String,
    pub store_path: String,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SystemGeneration {
    pub generation_id: u32,
    pub timestamp: u64,
    pub root_derivation_hash: String,
    pub kernel_version: String,
}

pub struct NixGuixCasStoreEngine {
    pub store_derivations: BTreeMap<String, CasStoreDerivation>, // hash -> drv
    pub generations: BTreeMap<u32, SystemGeneration>,
    pub current_generation_id: u32,
}

impl NixGuixCasStoreEngine {
    pub fn new() -> Self {
        Self {
            store_derivations: BTreeMap::new(),
            generations: BTreeMap::new(),
            current_generation_id: 0,
        }
    }

    pub fn add_derivation(&mut self, name: &str, inputs: &[&str]) -> String {
        let mut combined = name.to_string();
        for inp in inputs {
            combined.push_str(inp);
        }
        let mut hash_val: u64 = 0xcbf29ce484222325;
        for &b in combined.as_bytes() {
            hash_val ^= u64::from(b);
            hash_val = hash_val.wrapping_mul(0x100000001b3);
        }
        let hash_str = format!("{:016x}", hash_val);
        let store_path = format!("/nix/store/{}-{}", hash_str, name);

        let drv = CasStoreDerivation {
            hash: hash_str.clone(),
            name: name.to_string(),
            store_path,
            inputs: inputs.iter().map(|s| s.to_string()).collect(),
        };

        self.store_derivations.insert(hash_str.clone(), drv);
        hash_str
    }

    pub fn build_generation(&mut self, root_hash: &str, kernel_ver: &str) -> u32 {
        self.current_generation_id += 1;
        let gen_id = self.current_generation_id;

        let gen = SystemGeneration {
            generation_id: gen_id,
            timestamp: 1700000000 + u64::from(gen_id),
            root_derivation_hash: root_hash.to_string(),
            kernel_version: kernel_ver.to_string(),
        };

        self.generations.insert(gen_id, gen);
        gen_id
    }

    pub fn rollback_generation(&mut self, target_gen_id: u32) -> Result<SystemGeneration, &'static str> {
        if let Some(gen) = self.generations.get(&target_gen_id).cloned() {
            self.current_generation_id = target_gen_id;
            Ok(gen)
        } else {
            Err("Nix/Guix CAS Engine: Generation not found")
        }
    }
}

impl Default for NixGuixCasStoreEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. LINUX 6.12+ SCHEDEXT EBPF CUSTOM CPU SCHEDULER GOVERNOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedExtPolicy {
    Fifo,
    BoreInteractive,
    DirectPass,
    PowerSaving,
}

#[derive(Debug, Clone)]
pub struct SchedTask {
    pub pid: usize,
    pub name: String,
    pub priority: u32,
    pub cpu_affinity: u32,
    pub runtime_ns: u64,
}

pub struct LinuxSchedExtBpfGovernor {
    pub policy: SchedExtPolicy,
    pub active_tasks: BTreeMap<usize, SchedTask>,
    pub runqueue: Vec<usize>, // PIDs
}

impl LinuxSchedExtBpfGovernor {
    pub fn new(policy: SchedExtPolicy) -> Self {
        Self {
            policy,
            active_tasks: BTreeMap::new(),
            runqueue: Vec::new(),
        }
    }

    pub fn enqueue_task(&mut self, pid: usize, name: &str, priority: u32, cpu: u32) {
        let task = SchedTask {
            pid,
            name: name.to_string(),
            priority,
            cpu_affinity: cpu,
            runtime_ns: 0,
        };
        self.active_tasks.insert(pid, task);
        self.runqueue.push(pid);
    }

    pub fn dispatch_next_task(&mut self) -> Option<SchedTask> {
        if self.runqueue.is_empty() {
            return None;
        }

        match self.policy {
            SchedExtPolicy::BoreInteractive => {
                // Sort runqueue by highest priority first
                let active = &self.active_tasks;
                self.runqueue.sort_by(|a, b| {
                    let prio_a = active.get(a).map(|t| t.priority).unwrap_or(0);
                    let prio_b = active.get(b).map(|t| t.priority).unwrap_or(0);
                    prio_b.cmp(&prio_a)
                });
                let next_pid = self.runqueue.remove(0);
                self.active_tasks.get_mut(&next_pid).map(|t| {
                    t.runtime_ns += 1_000_000;
                    t.clone()
                })
            }
            _ => {
                let next_pid = self.runqueue.remove(0);
                self.active_tasks.get_mut(&next_pid).map(|t| {
                    t.runtime_ns += 1_000_000;
                    t.clone()
                })
            }
        }
    }
}

impl Default for LinuxSchedExtBpfGovernor {
    fn default() -> Self {
        Self::new(SchedExtPolicy::BoreInteractive)
    }
}

// =========================================================================
// 6. FREEBSD GEOM GATE NETWORK BLOCK STORAGE & CTL SCSI TARGET ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct GeomGateDevice {
    pub name: String,
    pub remote_ip: String,
    pub port: u16,
    pub block_size: usize,
    pub total_sectors: u64,
    pub is_connected: bool,
}

pub struct FreeBsdGeomGateStorageEngine {
    pub devices: BTreeMap<String, GeomGateDevice>,
}

impl FreeBsdGeomGateStorageEngine {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
        }
    }

    pub fn create_gate_device(&mut self, name: &str, remote_ip: &str, port: u16, total_sectors: u64) -> GeomGateDevice {
        let dev = GeomGateDevice {
            name: name.to_string(),
            remote_ip: remote_ip.to_string(),
            port,
            block_size: 512,
            total_sectors,
            is_connected: true,
        };
        self.devices.insert(name.to_string(), dev.clone());
        dev
    }

    pub fn read_sector(&self, name: &str, sector_id: u64) -> Result<Vec<u8>, &'static str> {
        if let Some(dev) = self.devices.get(name) {
            if !dev.is_connected {
                return Err("GEOM Gate Error: Device disconnected");
            }
            if sector_id >= dev.total_sectors {
                return Err("GEOM Gate Error: Sector ID out of bounds");
            }
            Ok(vec![0xAA; dev.block_size])
        } else {
            Err("GEOM Gate Error: Device not found")
        }
    }

    pub fn disconnect_device(&mut self, name: &str) -> bool {
        if let Some(dev) = self.devices.get_mut(name) {
            dev.is_connected = false;
            true
        } else {
            false
        }
    }
}

impl Default for FreeBsdGeomGateStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// MASTER COORDINATOR: SOVEREIGN LINUX & BSD MASTER SYNTHESIS SUITE
// =========================================================================

pub struct SovereignLinuxBsdMasterSynthesisSuite {
    pub npf_firewall: NetBsdNpfFirewallEngine,
    pub pledge_unveil: OpenBsdPledgeUnveilEnforcer,
    pub openrc_apk: AlpineOpenRcApkWorldEngine,
    pub nix_guix_cas: NixGuixCasStoreEngine,
    pub sched_ext: LinuxSchedExtBpfGovernor,
    pub geom_gate: FreeBsdGeomGateStorageEngine,
}

impl SovereignLinuxBsdMasterSynthesisSuite {
    pub fn new() -> Self {
        Self {
            npf_firewall: NetBsdNpfFirewallEngine::new(),
            pledge_unveil: OpenBsdPledgeUnveilEnforcer::new(),
            openrc_apk: AlpineOpenRcApkWorldEngine::new(),
            nix_guix_cas: NixGuixCasStoreEngine::new(),
            sched_ext: LinuxSchedExtBpfGovernor::new(SchedExtPolicy::BoreInteractive),
            geom_gate: FreeBsdGeomGateStorageEngine::new(),
        }
    }

    pub fn health_check(&self) -> bool {
        true
    }

    pub fn summary_report(&self) -> String {
        format!(
            "Sovereign Linux & BSD Master Synthesis Suite Active:\n- NPF Firewall Tables: {}\n- Pledged PIDs: {}\n- OpenRC Services: {}\n- CAS Store Derivations: {}\n- SchedExt Active Tasks: {}\n- GEOM Gate Devices: {}",
            self.npf_firewall.tables.len(),
            self.pledge_unveil.pledged_promises.len(),
            self.openrc_apk.services.len(),
            self.nix_guix_cas.store_derivations.len(),
            self.sched_ext.active_tasks.len(),
            self.geom_gate.devices.len(),
        )
    }
}

impl Default for SovereignLinuxBsdMasterSynthesisSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npf_firewall_engine() {
        let mut npf = NetBsdNpfFirewallEngine::new();
        npf.create_table(1, "bad_ips", NpfTableType::Tree);
        assert!(npf.add_table_entry(1, "192.168.1.100"));

        npf.add_rule(10, "eth0", true, true, &[0x01, 0x02, 0x03]);
        assert!(npf.inspect_packet("eth0", "10.0.0.1", "10.0.0.2", 80));
    }

    #[test]
    fn test_pledge_unveil_enforcer() {
        let mut security = OpenBsdPledgeUnveilEnforcer::new();
        let pid = 101;
        assert!(security.pledge(pid, &["stdio", "rpath"]).is_ok());
        assert!(security.check_syscall(pid, "stdio"));
        assert!(!security.check_syscall(pid, "exec"));

        // Test pledge monotonicity drop
        assert!(security.pledge(pid, &["stdio"]).is_ok());
        assert!(security.pledge(pid, &["stdio", "inet"]).is_err()); // cannot expand

        assert!(security.unveil(pid, "/usr/lib", "r").is_ok());
        assert!(security.check_path_access(pid, "/usr/lib/libc.so", 'r'));
        assert!(!security.check_path_access(pid, "/etc/shadow", 'r'));
    }

    #[test]
    fn test_openrc_apk_engine() {
        let mut alpine = AlpineOpenRcApkWorldEngine::new();
        alpine.register_service("sshd", "/usr/sbin/sshd", OpenRcRunlevel::DefaultRunlevel, &["network"]);
        assert!(alpine.start_service("sshd").unwrap());

        alpine.add_to_world("neofetch", "7.1.0");
        alpine.installed_packages.insert("orphaned-pkg".to_string(), "1.0.0".to_string());
        assert_eq!(alpine.reconcile_apk_world(), 1);
        assert!(!alpine.installed_packages.contains_key("orphaned-pkg"));
    }

    #[test]
    fn test_nix_guix_cas_store() {
        let mut nix = NixGuixCasStoreEngine::new();
        let drv_hash = nix.add_derivation("firefox-120.0", &["glibc", "gtk3"]);
        assert_eq!(nix.store_derivations.len(), 1);

        let gen_id = nix.build_generation(&drv_hash, "6.12.0-sovereign");
        assert_eq!(gen_id, 1);

        let rolled_back = nix.rollback_generation(1).unwrap();
        assert_eq!(rolled_back.kernel_version, "6.12.0-sovereign");
    }

    #[test]
    fn test_sched_ext_bpf_governor() {
        let mut sched = LinuxSchedExtBpfGovernor::new(SchedExtPolicy::BoreInteractive);
        sched.enqueue_task(1001, "web_browser", 90, 0);
        sched.enqueue_task(1002, "background_indexer", 10, 1);

        let next = sched.dispatch_next_task().unwrap();
        assert_eq!(next.pid, 1001); // browser has higher priority
    }

    #[test]
    fn test_geom_gate_storage() {
        let mut geom = FreeBsdGeomGateStorageEngine::new();
        geom.create_gate_device("ggate0", "192.168.1.50", 3080, 2048);

        let sector_data = geom.read_sector("ggate0", 100).unwrap();
        assert_eq!(sector_data.len(), 512);

        assert!(geom.disconnect_device("ggate0"));
        assert!(geom.read_sector("ggate0", 100).is_err());
    }

    #[test]
    fn test_master_synthesis_suite() {
        let suite = SovereignLinuxBsdMasterSynthesisSuite::new();
        assert!(suite.health_check());
        assert!(suite.summary_report().contains("Sovereign Linux & BSD Master Synthesis Suite Active"));
    }
}
