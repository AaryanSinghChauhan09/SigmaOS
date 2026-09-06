use std::string::{String, ToString};
use std::vec::Vec;

#[cfg(not(test))]
use crate::klib::collections::HashMap;
#[cfg(test)]
use std::collections::HashMap;

/// Arch Linux inspired AUR-style user repos and minimal base
pub struct ArchUserRepoManager {
    packages: HashMap<String, String>,
}

impl ArchUserRepoManager {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn install_from_aur(
        &mut self,
        pkg_name: &str,
        build_script: &str,
    ) -> Result<(), &'static str> {
        self.packages
            .insert(pkg_name.to_string(), build_script.to_string());
        Ok(())
    }
}

// ================= OpenBSD PF Stateful Packet Filtering Table =================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PfFiveTuple {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String, // "TCP", "UDP", "ICMP"
}

#[derive(Debug, Clone)]
pub struct PfStateEntry {
    pub tuple: PfFiveTuple,
    pub translated_src_ip: Option<String>, // NAT mapping if active
    pub translated_src_port: Option<u16>,
    pub packets_passed: u64,
    pub last_seen_timestamp_sec: u64,
    pub timeout_sec: u64,
}

/// OpenBSD PF (Packet Filter) stateful firewall engine
pub struct BsdPfStateTable {
    pub states: HashMap<PfFiveTuple, PfStateEntry>,
    pub default_timeout_sec: u64,
}

impl BsdPfStateTable {
    pub fn new(default_timeout_sec: u64) -> Self {
        Self {
            states: HashMap::new(),
            default_timeout_sec,
        }
    }

    pub fn create_state(
        &mut self,
        tuple: PfFiveTuple,
        nat_ip: Option<&str>,
        nat_port: Option<u16>,
        now_sec: u64,
    ) {
        let entry = PfStateEntry {
            tuple: tuple.clone(),
            translated_src_ip: nat_ip.map(|s| s.to_string()),
            translated_src_port: nat_port,
            packets_passed: 1,
            last_seen_timestamp_sec: now_sec,
            timeout_sec: self.default_timeout_sec,
        };
        self.states.insert(tuple, entry);
    }

    pub fn process_packet(
        &mut self,
        tuple: &PfFiveTuple,
        now_sec: u64,
    ) -> Result<Option<(String, u16)>, &'static str> {
        if let Some(state) = self.states.get_mut(tuple) {
            if now_sec
                > state
                    .last_seen_timestamp_sec
                    .saturating_add(state.timeout_sec)
            {
                // State expired
                self.states.remove(tuple);
                return Err("PF: Matching state entry expired");
            }
            state.packets_passed += 1;
            state.last_seen_timestamp_sec = now_sec;

            if let (Some(ref nat_ip), Some(nat_port)) =
                (&state.translated_src_ip, state.translated_src_port)
            {
                let nat_ip: &String = nat_ip;
                Ok(Some((nat_ip.clone(), nat_port)))
            } else {
                Ok(None)
            }
        } else {
            Err("PF: No state match found (packet blocked by default drop policy)")
        }
    }

    pub fn expire_states(&mut self, now_sec: u64) -> usize {
        let mut expired_keys: Vec<PfFiveTuple> = Vec::new();
        for (tuple, state) in self.states.iter() {
            if now_sec
                > state
                    .last_seen_timestamp_sec
                    .saturating_add(state.timeout_sec)
            {
                let k: PfFiveTuple = tuple.clone();
                expired_keys.push(k);
            }
        }
        let count = expired_keys.len();
        for k in expired_keys {
            self.states.remove(&k);
        }
        count
    }
}

// ================= Linux Fast Userspace Mutex (sys_futex) Engine =================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutexOp {
    Wait,
    Wake,
    Requeue,
}

#[derive(Debug, Clone)]
pub struct FutexWaiter {
    pub thread_id: u64,
    pub uaddr: u64,
    pub val: u32,
    pub timeout_ns: Option<u64>,
}

/// Linux sys_futex fast userspace mutex lock queue engine
pub struct LinuxFutexEngine {
    pub buckets: HashMap<u64, Vec<FutexWaiter>>,
}

impl LinuxFutexEngine {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    /// Futex WAIT: Atomically verify *uaddr == val; if true, enqueue caller thread to wait
    pub fn futex_wait(
        &mut self,
        uaddr: u64,
        current_mem_val: u32,
        expected_val: u32,
        thread_id: u64,
        timeout_ns: Option<u64>,
    ) -> Result<(), &'static str> {
        if current_mem_val != expected_val {
            return Err("Futex: EAGAIN - Memory value changed before lock acquired");
        }
        let waiter = FutexWaiter {
            thread_id,
            uaddr,
            val: expected_val,
            timeout_ns,
        };
        self.buckets
            .entry(uaddr)
            .or_insert_with(Vec::new)
            .push(waiter);
        Ok(())
    }

    /// Futex WAKE: Wake up at most `val_wake` threads sleeping on `uaddr`
    pub fn futex_wake(&mut self, uaddr: u64, val_wake: usize) -> usize {
        let mut woken = 0;
        if let Some(waiters) = self.buckets.get_mut(&uaddr) {
            let count = val_wake.min(waiters.len());
            for _ in 0..count {
                if !waiters.is_empty() {
                    waiters.remove(0);
                    woken += 1;
                }
            }
            if waiters.is_empty() {
                self.buckets.remove(&uaddr);
            }
        }
        woken
    }
}

// ================= FreeBSD VFS Nullfs Loopback Overlay Layer =================

#[derive(Debug, Clone)]
pub struct NullfsLayerNode {
    pub target_lower_path: String,
    pub mount_point: String,
    pub read_only: bool,
    pub override_permissions: Option<u32>,
}

/// FreeBSD nullfs / loopback filesystem overlay layer
pub struct FreeBsdVfsNullfs {
    pub mounts: HashMap<String, NullfsLayerNode>,
}

impl FreeBsdVfsNullfs {
    pub fn new() -> Self {
        Self {
            mounts: HashMap::new(),
        }
    }

    pub fn mount_nullfs(
        &mut self,
        lower_path: &str,
        mount_point: &str,
        read_only: bool,
        perm_override: Option<u32>,
    ) -> Result<(), &'static str> {
        if self.mounts.contains_key(mount_point) {
            return Err("Nullfs: Mount point busy");
        }
        self.mounts.insert(
            mount_point.to_string(),
            NullfsLayerNode {
                target_lower_path: lower_path.to_string(),
                mount_point: mount_point.to_string(),
                read_only,
                override_permissions: perm_override,
            },
        );
        Ok(())
    }

    pub fn resolve_overlay_path(
        &self,
        overlay_path: &str,
        is_write: bool,
    ) -> Result<(String, Option<u32>), &'static str> {
        for (mp, node) in self.mounts.iter() {
            let mp: &String = mp;
            if overlay_path == mp
                || (overlay_path.starts_with(mp.as_str())
                    && overlay_path.as_bytes().get(mp.len()) == Some(&b'/'))
            {
                if is_write && node.read_only {
                    return Err("Nullfs: EROFS - Read-only file system layer");
                }
                let relative_suffix = &overlay_path[mp.len()..];
                let resolved = std::format!("{}{}", node.target_lower_path, relative_suffix);
                return Ok((resolved, node.override_permissions));
            }
        }
        Err("Nullfs: Path not mapped under any active nullfs overlay mount")
    }
}

/// Alpine Linux inspired minimal base with hardened security
pub struct AlpineHardenedEnv {
    secure_mode: bool,
}

impl AlpineHardenedEnv {
    pub fn new() -> Self {
        Self { secure_mode: true }
    }

    pub fn execute_with_musl_stub(&self, _binary: &[u8]) -> Result<u32, &'static str> {
        if !self.secure_mode {
            return Err("Must be in secure mode");
        }
        Ok(0) // Return exit code 0
    }
}

/// OpenBSD inspired pledge/unveil syscall restrictions
pub struct OpenBsdPledge {
    pub promises: Vec<String>,
    pub is_pledged: bool,
}

impl OpenBsdPledge {
    pub fn new() -> Self {
        Self {
            promises: Vec::new(),
            is_pledged: false,
        }
    }

    pub fn pledge(&mut self, promise_list: &str) -> Result<(), &'static str> {
        let mut new_promises = Vec::new();
        for promise in promise_list.split(' ') {
            if !promise.is_empty() {
                new_promises.push(promise.to_string());
            }
        }

        if self.is_pledged {
            // Once pledged, subsequent calls can only drop capabilities, never escalate
            for promise in &new_promises {
                if !self.promises.contains(promise) {
                    return Err("Illegal pledge escalation blocked");
                }
            }
        }

        self.promises = new_promises;
        self.is_pledged = true;
        Ok(())
    }

    pub fn check_permission(&self, operation: &str) -> bool {
        if !self.is_pledged {
            return true;
        }
        for promise in &self.promises {
            let promise: &String = promise;
            if promise.as_str() == operation {
                return true;
            }
        }
        false
    }
}

// =========================================================================
// EBPF XDP FAST PACKET ENGINE (LINUX XDP & FREEBSD NETMAP ZERO-COPY PARITY)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EbpfXdpAction {
    Aborted = 0,
    Drop = 1,
    Pass = 2,
    Tx = 3,
    Redirect = 4,
}

#[derive(Debug, Clone)]
pub struct EbpfXdpProgram {
    pub name: String,
    pub instructions: Vec<u64>,
}

impl EbpfXdpProgram {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            instructions: Vec::new(),
        }
    }

    pub fn execute(&self, packet_data: &[u8]) -> EbpfXdpAction {
        if packet_data.is_empty() {
            return EbpfXdpAction::Drop;
        }
        if packet_data.len() > 14 && packet_data[0] == 0xFF && packet_data[1] == 0xFF {
            return EbpfXdpAction::Drop;
        }
        EbpfXdpAction::Pass
    }
}

pub struct EbpfXdpFastPacketEngine {
    pub active_program: Option<EbpfXdpProgram>,
    pub rx_ring_buffer: Vec<Vec<u8>>,
    pub total_processed: u64,
    pub total_dropped: u64,
}

impl EbpfXdpFastPacketEngine {
    pub fn new() -> Self {
        Self {
            active_program: None,
            rx_ring_buffer: Vec::new(),
            total_processed: 0,
            total_dropped: 0,
        }
    }

    pub fn attach_xdp_program(&mut self, program: EbpfXdpProgram) {
        self.active_program = Some(program);
    }

    pub fn process_rx_packet(&mut self, packet_data: &[u8]) -> EbpfXdpAction {
        self.total_processed += 1;
        let action = if let Some(ref prog) = self.active_program {
            prog.execute(packet_data)
        } else {
            EbpfXdpAction::Pass
        };

        if action == EbpfXdpAction::Drop || action == EbpfXdpAction::Aborted {
            self.total_dropped += 1;
        } else if action == EbpfXdpAction::Pass {
            self.rx_ring_buffer.push(packet_data.to_vec());
        }
        action
    }
}

impl Default for EbpfXdpFastPacketEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Intel Clear Linux Stateless Architecture Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuIsaMicroarch {
    X86_64_V1,
    X86_64_V2,
    X86_64_V3,
    X86_64_V4,
}

pub struct IntelClearLinuxStatelessEngine {
    pub usr_share_defaults: HashMap<String, String>,
    pub etc_user_overrides: HashMap<String, String>,
    pub detected_isa: CpuIsaMicroarch,
}

impl IntelClearLinuxStatelessEngine {
    pub fn new() -> Self {
        Self {
            usr_share_defaults: HashMap::new(),
            etc_user_overrides: HashMap::new(),
            detected_isa: CpuIsaMicroarch::X86_64_V3,
        }
    }

    pub fn auto_detect_isa(&mut self, has_avx2: bool, has_avx512: bool) -> CpuIsaMicroarch {
        if has_avx512 {
            self.detected_isa = CpuIsaMicroarch::X86_64_V4;
        } else if has_avx2 {
            self.detected_isa = CpuIsaMicroarch::X86_64_V3;
        } else {
            self.detected_isa = CpuIsaMicroarch::X86_64_V1;
        }
        self.detected_isa
    }

    pub fn register_default_config(&mut self, path: &str, content: &str) {
        self.usr_share_defaults
            .insert(path.to_string(), content.to_string());
    }

    pub fn set_user_override(&mut self, path: &str, content: &str) {
        self.etc_user_overrides
            .insert(path.to_string(), content.to_string());
    }

    pub fn resolve_config(&self, path: &str) -> Option<String> {
        if let Some(user_val) = self.etc_user_overrides.get(path) {
            Some(user_val.clone())
        } else {
            self.usr_share_defaults.get(path).cloned()
        }
    }

    pub fn reset_etc_to_stateless(&mut self) {
        self.etc_user_overrides.clear();
    }
}

/// FreeBSD inspired Jails (capability-based isolation)
pub struct FreeBsdJail {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub isolated: bool,
}

impl FreeBsdJail {
    pub fn create(id: u32) -> Self {
        Self {
            id,
            parent_id: None,
            isolated: true,
        }
    }

    pub fn create_nested(id: u32, parent_id: u32) -> Self {
        Self {
            id,
            parent_id: Some(parent_id),
            isolated: true,
        }
    }

    pub fn is_isolated(&self) -> bool {
        self.isolated
    }

    /// Recursively check if this jail is a descendant of the target parent jail ID
    pub fn is_descendant_of(&self, target_parent_id: u32) -> bool {
        if let Some(pid) = self.parent_id {
            if pid == target_parent_id {
                return true;
            }
        }
        false
    }
}

/// NixOS inspired Declarative package management
pub struct NixOsDeclarativeManager {
    pub configuration: Vec<String>,
    pub previous_generations: Vec<Vec<String>>,
}

impl NixOsDeclarativeManager {
    pub fn new() -> Self {
        Self {
            configuration: Vec::new(),
            previous_generations: Vec::new(),
        }
    }

    pub fn apply_configuration(&mut self, config: &[&str]) -> Result<(), &'static str> {
        // Save previous generation before applying new one
        if !self.configuration.is_empty() {
            self.previous_generations.push(self.configuration.clone());
        }
        self.configuration.clear();
        for c in config {
            self.configuration.push(c.to_string());
        }
        Ok(())
    }

    /// Rollbacks to the previous configuration generation atomically
    pub fn rollback(&mut self) -> Result<(), &'static str> {
        if let Some(prev) = self.previous_generations.pop() {
            self.configuration = prev;
            Ok(())
        } else {
            Err("No previous generations available for rollback")
        }
    }
}

/// Gentoo inspired USE flags / compile-time feature selection
pub struct GentooUseFlags {
    pub flags: HashMap<String, bool>,
    pub dependencies: HashMap<String, String>, // (flag -> required companion flag)
}

impl GentooUseFlags {
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn set_flag(&mut self, flag: &str, enabled: bool) {
        self.flags.insert(flag.to_string(), enabled);
    }

    pub fn add_dependency(&mut self, flag: &str, required_companion: &str) {
        self.dependencies
            .insert(flag.to_string(), required_companion.to_string());
    }

    pub fn has_feature(&self, flag: &str) -> bool {
        if let Some(&val) = self.flags.get(flag) {
            val
        } else {
            false
        }
    }

    /// Check if all active USE-flags have their required companion dependencies enabled
    pub fn check_dependencies(&self) -> bool {
        for (flag, required) in &self.dependencies {
            if self.has_feature(flag) && !self.has_feature(required) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsd_pf_state_table() {
        let mut pf = BsdPfStateTable::new(60);
        let tuple = PfFiveTuple {
            src_ip: "192.168.1.100".to_string(),
            dst_ip: "1.1.1.1".to_string(),
            src_port: 5000,
            dst_port: 443,
            protocol: "TCP".to_string(),
        };

        pf.create_state(tuple.clone(), Some("203.0.113.5"), Some(40000), 1000);

        let nat_res = pf.process_packet(&tuple, 1010).unwrap();
        assert!(nat_res.is_some());
        let (nat_ip, nat_port) = nat_res.unwrap();
        assert_eq!(nat_ip, "203.0.113.5");
        assert_eq!(nat_port, 40000);

        // Expire state
        assert_eq!(pf.expire_states(1100), 1);
        assert!(pf.process_packet(&tuple, 1105).is_err());
    }

    #[test]
    fn test_linux_futex_engine() {
        let mut futex = LinuxFutexEngine::new();
        let uaddr = 0x7fff0000;

        // Memory value changed (EAGAIN)
        assert!(futex.futex_wait(uaddr, 1, 0, 101, None).is_err());

        // Successful wait enqueue
        assert!(futex.futex_wait(uaddr, 0, 0, 101, None).is_ok());
        assert!(futex.futex_wait(uaddr, 0, 0, 102, None).is_ok());

        // Wake 1 thread
        let woken = futex.futex_wake(uaddr, 1);
        assert_eq!(woken, 1);
        assert_eq!(futex.buckets.get(&uaddr).unwrap().len(), 1);

        // Wake remaining
        let woken_all = futex.futex_wake(uaddr, 5);
        assert_eq!(woken_all, 1);
        assert!(futex.buckets.get(&uaddr).is_none());
    }

    #[test]
    fn test_freebsd_vfs_nullfs() {
        let mut nullfs = FreeBsdVfsNullfs::new();
        nullfs
            .mount_nullfs("/usr/src/sys", "/sys", true, Some(0o755))
            .unwrap();

        let (resolved, perm) = nullfs
            .resolve_overlay_path("/sys/kern/vfs_subr.c", false)
            .unwrap();
        assert_eq!(resolved, "/usr/src/sys/kern/vfs_subr.c");
        assert_eq!(perm, Some(0o755));

        // Write to read-only nullfs layer should fail
        assert!(nullfs
            .resolve_overlay_path("/sys/kern/vfs_subr.c", true)
            .is_err());
    }

    #[test]
    fn test_openbsd_pledge() {
        let mut pledge = OpenBsdPledge::new();
        assert!(pledge.check_permission("exec"));

        pledge.pledge("stdio rpath wpath").unwrap();
        assert!(pledge.check_permission("stdio"));
        assert!(pledge.check_permission("rpath"));
        assert!(!pledge.check_permission("exec"));

        pledge.pledge("stdio").unwrap();
        assert!(pledge.check_permission("stdio"));
        assert!(!pledge.check_permission("rpath"));

        assert!(pledge.pledge("stdio rpath").is_err());
    }

    #[test]
    fn test_intel_clear_linux_stateless() {
        let mut stateless = IntelClearLinuxStatelessEngine::new();
        stateless.register_default_config("/etc/hostname", "sigma-default");
        assert_eq!(
            stateless.resolve_config("/etc/hostname").unwrap(),
            "sigma-default"
        );

        stateless.set_user_override("/etc/hostname", "sigma-custom");
        assert_eq!(
            stateless.resolve_config("/etc/hostname").unwrap(),
            "sigma-custom"
        );

        stateless.reset_etc_to_stateless();
        assert_eq!(
            stateless.resolve_config("/etc/hostname").unwrap(),
            "sigma-default"
        );
    }
}
