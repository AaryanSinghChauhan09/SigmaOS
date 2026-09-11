// SigmaOS Linux Kernel Component Parity Suite
// Implements native zero-dependency Rust implementations of core Linux kernel subsystems:
// 1. BPF_MAP_TYPE_RINGBUF lock-free event streaming engine
// 2. VirtIO memory ballooning driver (inflation, deflation, free page reporting)
// 3. Userfaultfd virtual memory demand paging & page fault trapping
// 4. Linux kernel audit logging subsystem (AUDIT_SYSCALL, AUDIT_AVC)

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. Linux BPF_MAP_TYPE_RINGBUF Event Ring Buffer Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BpfRingSample {
    pub sample_id: u64,
    pub producer_index: usize,
    pub data: Vec<u8>,
    pub is_discarded: bool,
}

pub struct BpfRingBufferStreamEngine {
    pub capacity: usize,
    pub samples: Vec<BpfRingSample>,
    pub next_sample_id: u64,
}

impl BpfRingBufferStreamEngine {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(4096).next_power_of_two(),
            samples: Vec::new(),
            next_sample_id: 1,
        }
    }

    pub fn reserve(&mut self, payload_len: usize) -> Result<u64, &'static str> {
        if payload_len == 0 {
            return Err("BpfRingBufferStreamEngine: Payload length cannot be zero");
        }

        let current_allocated: usize = self.samples.iter().map(|s| s.data.len() + 8).sum();
        if current_allocated + payload_len + 8 > self.capacity {
            return Err("BpfRingBufferStreamEngine: Buffer overflow");
        }

        let sample_id = self.next_sample_id;
        self.next_sample_id += 1;

        self.samples.push(BpfRingSample {
            sample_id,
            producer_index: self.samples.len(),
            data: vec![0u8; payload_len],
            is_discarded: false,
        });

        Ok(sample_id)
    }

    pub fn submit(&mut self, sample_id: u64, payload: &[u8]) -> Result<(), &'static str> {
        let sample = self
            .samples
            .iter_mut()
            .find(|s| s.sample_id == sample_id)
            .ok_or("BpfRingBufferStreamEngine: Sample ID not found")?;

        if sample.data.len() != payload.len() {
            return Err("BpfRingBufferStreamEngine: Payload length mismatch");
        }

        sample.data.copy_from_slice(payload);
        sample.is_discarded = false;
        Ok(())
    }

    pub fn discard(&mut self, sample_id: u64) -> Result<(), &'static str> {
        let sample = self
            .samples
            .iter_mut()
            .find(|s| s.sample_id == sample_id)
            .ok_or("BpfRingBufferStreamEngine: Sample ID not found")?;

        sample.is_discarded = true;
        Ok(())
    }

    pub fn consume(&mut self) -> Option<BpfRingSample> {
        while !self.samples.is_empty() {
            let sample = self.samples.remove(0);
            if !sample.is_discarded {
                return Some(sample);
            }
        }
        None
    }
}

impl Default for BpfRingBufferStreamEngine {
    fn default() -> Self {
        Self::new(4096)
    }
}

// ============================================================================
// 2. VirtIO Memory Balloon Driver Engine
// ============================================================================

pub const VIRTIO_BALLOON_F_MUST_TELL_HOST: u64 = 1 << 0;
pub const VIRTIO_BALLOON_F_STATS_VQ: u64 = 1 << 1;
pub const VIRTIO_BALLOON_F_FREE_PAGE_HINT: u64 = 1 << 2;

pub struct VirtioBalloonDriverEngine {
    pub actual_pages: u32,
    pub num_pages_requested: u32,
    pub inflated_page_pfns: Vec<u64>,
    pub features: u64,
}

impl VirtioBalloonDriverEngine {
    pub fn new(features: u64) -> Self {
        Self {
            actual_pages: 0,
            num_pages_requested: 0,
            inflated_page_pfns: Vec::new(),
            features,
        }
    }

    pub fn request_balloon_target(&mut self, target_pages: u32) {
        self.num_pages_requested = target_pages;
    }

    pub fn inflate(&mut self, pfns: &[u64]) -> Result<u32, &'static str> {
        if pfns.is_empty() {
            return Err("VirtioBalloon: PFN array cannot be empty");
        }

        for &pfn in pfns {
            if !self.inflated_page_pfns.contains(&pfn) {
                self.inflated_page_pfns.push(pfn);
                self.actual_pages += 1;
            }
        }

        Ok(self.actual_pages)
    }

    pub fn deflate(&mut self, count: usize) -> Result<u32, &'static str> {
        if count > self.inflated_page_pfns.len() {
            return Err("VirtioBalloon: Deflate count exceeds inflated pages");
        }

        for _ in 0..count {
            self.inflated_page_pfns.pop();
            self.actual_pages -= 1;
        }

        Ok(self.actual_pages)
    }
}

impl Default for VirtioBalloonDriverEngine {
    fn default() -> Self {
        Self::new(VIRTIO_BALLOON_F_MUST_TELL_HOST | VIRTIO_BALLOON_F_STATS_VQ)
    }
}

// ============================================================================
// 3. Userfaultfd Virtual Memory Page Fault Trapping Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UffdMode {
    Missing,
    Minor,
    WriteProtect,
}

#[derive(Debug, Clone)]
pub struct UffdRegisteredRange {
    pub start_addr: usize,
    pub len: usize,
    pub mode: UffdMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UffdFaultEvent {
    pub fault_addr: usize,
    pub mode: UffdMode,
    pub pid: u32,
}

pub struct UserfaultfdSubsystemEngine {
    pub registered_ranges: Vec<UffdRegisteredRange>,
    pub pending_faults: Vec<UffdFaultEvent>,
}

impl UserfaultfdSubsystemEngine {
    pub fn new() -> Self {
        Self {
            registered_ranges: Vec::new(),
            pending_faults: Vec::new(),
        }
    }

    pub fn register_range(&mut self, start_addr: usize, len: usize, mode: UffdMode) -> Result<(), &'static str> {
        if len == 0 || start_addr % 4096 != 0 {
            return Err("Userfaultfd: Address and length must be page-aligned (4096)");
        }

        self.registered_ranges.push(UffdRegisteredRange {
            start_addr,
            len,
            mode,
        });

        Ok(())
    }

    pub fn trigger_page_fault(&mut self, fault_addr: usize, mode: UffdMode, pid: u32) -> bool {
        let is_registered = self.registered_ranges.iter().any(|r| {
            fault_addr >= r.start_addr && fault_addr < r.start_addr + r.len && r.mode == mode
        });

        if is_registered {
            self.pending_faults.push(UffdFaultEvent {
                fault_addr,
                mode,
                pid,
            });
            true
        } else {
            false
        }
    }

    pub fn resolve_page_fault(&mut self, fault_addr: usize) -> bool {
        if let Some(pos) = self.pending_faults.iter().position(|f| f.fault_addr == fault_addr) {
            self.pending_faults.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for UserfaultfdSubsystemEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Linux Kernel Audit Subsystem Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelAuditRecordType {
    Syscall = 1000,
    AvcDenial = 1400,
    UserAuth = 1100,
    ConfigChange = 1300,
}

#[derive(Debug, Clone)]
pub struct KernelAuditRecord {
    pub audit_id: u64,
    pub record_type: KernelAuditRecordType,
    pub pid: u32,
    pub uid: u32,
    pub message: String,
    pub timestamp_epoch: u64,
}

pub struct LinuxKernelAuditSubsystemEngine {
    pub audit_enabled: bool,
    pub records: Vec<KernelAuditRecord>,
    pub next_audit_id: u64,
}

impl LinuxKernelAuditSubsystemEngine {
    pub fn new() -> Self {
        Self {
            audit_enabled: true,
            records: Vec::new(),
            next_audit_id: 1,
        }
    }

    pub fn log_audit_event(
        &mut self,
        record_type: KernelAuditRecordType,
        pid: u32,
        uid: u32,
        msg: &str,
        timestamp: u64,
    ) -> Result<u64, &'static str> {
        if !self.audit_enabled {
            return Err("KernelAudit: Auditing disabled");
        }

        let audit_id = self.next_audit_id;
        self.next_audit_id += 1;

        self.records.push(KernelAuditRecord {
            audit_id,
            record_type,
            pid,
            uid,
            message: msg.to_string(),
            timestamp_epoch: timestamp,
        });

        Ok(audit_id)
    }

    pub fn query_records_by_pid(&self, pid: u32) -> Vec<&KernelAuditRecord> {
        self.records.iter().filter(|r| r.pid == pid).collect()
    }
}

impl Default for LinuxKernelAuditSubsystemEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpf_ring_buffer_stream() {
        let mut engine = BpfRingBufferStreamEngine::new(4096);
        let id1 = engine.reserve(16).unwrap();
        let id2 = engine.reserve(16).unwrap();

        assert!(engine.submit(id1, b"0123456789abcdef").is_ok());
        assert!(engine.discard(id2).is_ok());

        let sample = engine.consume().unwrap();
        assert_eq!(sample.sample_id, id1);
        assert_eq!(sample.data, b"0123456789abcdef");
        assert!(engine.consume().is_none()); // id2 was discarded
    }

    #[test]
    fn test_virtio_balloon_driver() {
        let mut balloon = VirtioBalloonDriverEngine::default();
        balloon.request_balloon_target(256);

        let pfns = vec![100, 101, 102, 103];
        assert_eq!(balloon.inflate(&pfns).unwrap(), 4);
        assert_eq!(balloon.deflate(2).unwrap(), 2);
    }

    #[test]
    fn test_userfaultfd_subsystem() {
        let mut uffd = UserfaultfdSubsystemEngine::new();
        assert!(uffd.register_range(0x7fff_0000_0000, 8192, UffdMode::Missing).is_ok());

        assert!(uffd.trigger_page_fault(0x7fff_0000_1000, UffdMode::Missing, 4201));
        assert!(!uffd.trigger_page_fault(0x1000, UffdMode::Missing, 4201)); // Unregistered address

        assert_eq!(uffd.pending_faults.len(), 1);
        assert!(uffd.resolve_page_fault(0x7fff_0000_1000));
        assert_eq!(uffd.pending_faults.len(), 0);
    }

    #[test]
    fn test_linux_kernel_audit_subsystem() {
        let mut audit = LinuxKernelAuditSubsystemEngine::new();
        let id = audit
            .log_audit_event(
                KernelAuditRecordType::Syscall,
                1234,
                1000,
                "syscall=openat path=/etc/passwd",
                1700000000,
            )
            .unwrap();

        assert_eq!(id, 1);
        let records = audit.query_records_by_pid(1234);
        assert_eq!(records.len(), 1);
        assert!(records[0].message.contains("openat"));
    }
}

// ============================================================================
// 5. MEMCG V2 OOM KILLER ENGINE
// ============================================================================

#[derive(Debug, Clone)]
pub struct MemcgProcessEntry {
    pub pid: u32,
    pub oom_score_adj: i32, // -1000 to 1000
    pub memory_bytes_used: u64,
}

pub struct LinuxMemoryCgroupV2OomKillerEngine {
    pub cgroup_path: String,
    pub memory_limit_bytes: u64,
    pub processes: Vec<MemcgProcessEntry>,
}

impl LinuxMemoryCgroupV2OomKillerEngine {
    pub fn new(path: &str, limit_bytes: u64) -> Self {
        Self {
            cgroup_path: path.to_string(),
            memory_limit_bytes: limit_bytes,
            processes: Vec::new(),
        }
    }

    pub fn register_process(&mut self, entry: MemcgProcessEntry) {
        self.processes.push(entry);
    }

    /// Selects the OOM kill candidate process using memcg v2 heuristics
    pub fn select_oom_kill_candidate(&self) -> Option<u32> {
        if self.processes.is_empty() {
            return None;
        }

        let mut best_pid = None;
        let mut max_score = i64::MIN;

        for proc in &self.processes {
            if proc.oom_score_adj <= -1000 {
                continue; // Unkillable
            }

            let base_score = (proc.memory_bytes_used / 1024) as i64;
            let final_score = base_score + (proc.oom_score_adj as i64 * 10);

            if final_score > max_score {
                max_score = final_score;
                best_pid = Some(proc.pid);
            }
        }

        best_pid
    }
}

// ============================================================================
// 6. LINUX EPOLL EVENT POLL ENGINE (epoll_create/ctl/wait)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpollCtlOp {
    Add,
    Mod,
    Del,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpollEvent {
    pub fd: i32,
    pub events: u32, // EPOLLIN (1), EPOLLOUT (4)
}

pub struct LinuxEpollEventPollEngine {
    pub registered_fds: Vec<EpollEvent>,
}

impl LinuxEpollEventPollEngine {
    pub fn new() -> Self {
        Self {
            registered_fds: Vec::new(),
        }
    }

    pub fn epoll_ctl(&mut self, op: EpollCtlOp, event: EpollEvent) -> Result<(), &'static str> {
        match op {
            EpollCtlOp::Add => {
                if self.registered_fds.iter().any(|e| e.fd == event.fd) {
                    return Err("EPOLL_CTL_ADD: FD already registered");
                }
                self.registered_fds.push(event);
            }
            EpollCtlOp::Mod => {
                let entry = self
                    .registered_fds
                    .iter_mut()
                    .find(|e| e.fd == event.fd)
                    .ok_or("EPOLL_CTL_MOD: FD not found")?;
                entry.events = event.events;
            }
            EpollCtlOp::Del => {
                let pos = self
                    .registered_fds
                    .iter()
                    .position(|e| e.fd == event.fd)
                    .ok_or("EPOLL_CTL_DEL: FD not found")?;
                self.registered_fds.remove(pos);
            }
        }
        Ok(())
    }

    pub fn epoll_wait(&self, ready_fd: i32) -> Vec<EpollEvent> {
        self.registered_fds
            .iter()
            .filter(|e| e.fd == ready_fd)
            .cloned()
            .collect()
    }
}

impl Default for LinuxEpollEventPollEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. LINUX KPROBES TRACEPOINT ENGINE
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KprobeEntry {
    pub symbol_name: String,
    pub offset: usize,
    pub is_retprobe: bool,
}

pub struct LinuxKprobesTracepointEngine {
    pub active_probes: Vec<KprobeEntry>,
}

impl LinuxKprobesTracepointEngine {
    pub fn new() -> Self {
        Self {
            active_probes: Vec::new(),
        }
    }

    pub fn register_kprobe(&mut self, symbol: &str, offset: usize, is_retprobe: bool) -> Result<(), &'static str> {
        if symbol.is_empty() {
            return Err("Kprobes: Symbol name cannot be empty");
        }
        self.active_probes.push(KprobeEntry {
            symbol_name: symbol.to_string(),
            offset,
            is_retprobe,
        });
        Ok(())
    }
}

impl Default for LinuxKprobesTracepointEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 8. LINUX SECCOMP BPF SYSCALL FILTER ENGINE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    Allow,
    KillProcess,
    Errno(u16),
}

pub struct LinuxSeccompBpfSyscallFilterEngine {
    pub allowed_syscalls: Vec<u32>,
    pub default_action: SeccompAction,
}

impl LinuxSeccompBpfSyscallFilterEngine {
    pub fn new(default_act: SeccompAction) -> Self {
        Self {
            allowed_syscalls: Vec::new(),
            default_action: default_act,
        }
    }

    pub fn allow_syscall(&mut self, syscall_number: u32) {
        if !self.allowed_syscalls.contains(&syscall_number) {
            self.allowed_syscalls.push(syscall_number);
        }
    }

    pub fn evaluate_syscall(&self, syscall_number: u32) -> SeccompAction {
        if self.allowed_syscalls.contains(&syscall_number) {
            SeccompAction::Allow
        } else {
            self.default_action
        }
    }
}

#[cfg(test)]
mod extended_kernel_tests {
    use super::*;

    #[test]
    fn test_memcg_v2_oom_killer() {
        let mut oom = LinuxMemoryCgroupV2OomKillerEngine::new("/sys/fs/cgroup/user.slice", 1024 * 1024 * 1024);
        oom.register_process(MemcgProcessEntry {
            pid: 100,
            oom_score_adj: -1000, // Unkillable
            memory_bytes_used: 500 * 1024 * 1024,
        });
        oom.register_process(MemcgProcessEntry {
            pid: 200,
            oom_score_adj: 0,
            memory_bytes_used: 200 * 1024 * 1024,
        });

        assert_eq!(oom.select_oom_kill_candidate(), Some(200));
    }

    #[test]
    fn test_linux_epoll_engine() {
        let mut epoll = LinuxEpollEventPollEngine::new();
        let ev = EpollEvent { fd: 5, events: 1 };

        assert!(epoll.epoll_ctl(EpollCtlOp::Add, ev.clone()).is_ok());
        assert_eq!(epoll.epoll_wait(5).len(), 1);

        assert!(epoll.epoll_ctl(EpollCtlOp::Del, ev).is_ok());
        assert_eq!(epoll.epoll_wait(5).len(), 0);
    }

    #[test]
    fn test_linux_kprobes_engine() {
        let mut kprobes = LinuxKprobesTracepointEngine::new();
        assert!(kprobes.register_kprobe("sys_openat", 0, false).is_ok());
        assert_eq!(kprobes.active_probes.len(), 1);
    }

    #[test]
    fn test_seccomp_bpf_filter() {
        let mut seccomp = LinuxSeccompBpfSyscallFilterEngine::new(SeccompAction::KillProcess);
        seccomp.allow_syscall(1); // sys_write

        assert_eq!(seccomp.evaluate_syscall(1), SeccompAction::Allow);
        assert_eq!(seccomp.evaluate_syscall(2), SeccompAction::KillProcess);
    }
}
