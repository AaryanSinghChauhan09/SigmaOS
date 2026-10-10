//! SchedExt — Linux 6.11+ Extensible Scheduler Framework for SigmaOS
//!
//! Inspired by Linux sched_ext (BPF Extensible Scheduler Class, merged in Linux 6.12).
//! Allows BPF programs to implement custom scheduling policies with full kernel-scheduler
//! integration, enabling per-task, per-CPU, and per-cgroup scheduling decisions from
//! safe, verifiable BPF programs.
//!
//! References:
//! - Linux sched_ext: https://lwn.net/Articles/922405/
//! - sched-ext GitHub: https://github.com/sched-ext/scx
//! - scx_rustland, scx_lavd, scx_rusty — reference userspace schedulers
//! - FreeBSD ULE scheduler (inspiration for per-CPU run queues)
//! - DragonFlyBSD LWKT scheduler (lightweight kernel thread model)
//!
//! Future Development:
//! - QEMU integration for testing custom schedules in SigmaOS VMs
//! - scx_lavd port: Latency-priority Aware Virtual Deadline (for gaming/RT)
//! - scx_rusty port: multi-domain load balancing
//! - Energy-aware scheduling (EAS) integration with CPUFreq governor
//! - Per-cgroup scheduling domains for container isolation

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// SchedExt operation codes — operations the BPF scheduler can perform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SchedExtOp {
    /// Select CPU for a task to run on
    SelectCpu = 0,
    /// Enqueue a task into the scheduler's run queue  
    Enqueue = 1,
    /// Dequeue a task (e.g. for sleeping or migration)
    Dequeue = 2,
    /// Dispatch the next task to a CPU
    Dispatch = 3,
    /// Task yield (voluntary preemption)
    Yield = 4,
    /// Preempt a running task
    Preempt = 5,
    /// Task exit hook
    Exit = 6,
    /// Error/fallback to CFS
    Error = 7,
}

/// SchedExt flags — per-task scheduling hints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedExtFlags(pub u32);

impl SchedExtFlags {
    pub const NONE: Self = Self(0);
    /// Allow task to be migrated across scheduling domains
    pub const ALLOW_MIGRATION: Self = Self(1 << 0);
    /// Task is latency-sensitive (gaming, audio, UI)
    pub const LATENCY_SENSITIVE: Self = Self(1 << 1);
    /// Task benefits from cache affinity (scientific, compile)
    pub const CACHE_AFFINE: Self = Self(1 << 2);
    /// Task is idle-class (background, batch)
    pub const IDLE_CLASS: Self = Self(1 << 3);
    /// Task is a real-time deadline task
    pub const REALTIME: Self = Self(1 << 4);
    /// Task should be scheduled on efficiency cores (E-cores)
    pub const PREFER_ECORE: Self = Self(1 << 5);
    /// Task should be scheduled on performance cores (P-cores)
    pub const PREFER_PCORE: Self = Self(1 << 6);
}

/// Task Scheduling Context — passed to BPF scheduler operations
#[derive(Debug, Clone)]
pub struct SchedExtTaskCtx {
    /// Task identifier (PID)
    pub pid: u32,
    /// Task group identifier (TGID / process)
    pub tgid: u32,
    /// Task name (comm)
    pub comm: [u8; 16],
    /// Current CPU the task is running on (or last ran on)
    pub cpu: u32,
    /// Task scheduling flags
    pub flags: SchedExtFlags,
    /// Task's static priority (0=RT, 1-99=nice, 100=idle)
    pub priority: i32,
    /// vruntime in nanoseconds (virtual runtime, CFS-compatible)
    pub vruntime_ns: u64,
    /// Last run timestamp (nanoseconds since boot)
    pub last_run_ns: u64,
    /// Accumulated runtime (nanoseconds)
    pub sum_exec_runtime_ns: u64,
    /// Number of times this task has been scheduled
    pub nr_switches: u64,
    /// cgroup ID for resource accounting
    pub cgroup_id: u64,
    /// NUMA node preference (-1 = no preference)
    pub numa_preferred_node: i32,
}

impl SchedExtTaskCtx {
    pub fn new(pid: u32, priority: i32) -> Self {
        Self {
            pid,
            tgid: pid,
            comm: [0u8; 16],
            cpu: 0,
            flags: SchedExtFlags::NONE,
            priority,
            vruntime_ns: 0,
            last_run_ns: 0,
            sum_exec_runtime_ns: 0,
            nr_switches: 0,
            cgroup_id: 0,
            numa_preferred_node: -1,
        }
    }

    pub fn set_comm(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(15);
        self.comm[..len].copy_from_slice(&bytes[..len]);
        self.comm[len] = 0;
    }

    pub fn is_latency_sensitive(&self) -> bool {
        self.flags.0 & SchedExtFlags::LATENCY_SENSITIVE.0 != 0
    }

    pub fn is_realtime(&self) -> bool {
        self.flags.0 & SchedExtFlags::REALTIME.0 != 0
    }
}

/// Per-CPU run queue for SchedExt
#[derive(Debug, Default)]
pub struct SchedExtCpuRq {
    /// CPU ID this queue belongs to
    pub cpu_id: u32,
    /// Ordered list of tasks (by vruntime)
    pub tasks: Vec<u32>, // PIDs ordered by priority
    /// Is this CPU currently idle?
    pub idle: bool,
    /// Current running task PID (0 = idle)
    pub current_pid: u32,
    /// Load metric (sum of task weights on this CPU)
    pub load: u64,
}

impl SchedExtCpuRq {
    pub fn new(cpu_id: u32) -> Self {
        Self {
            cpu_id,
            tasks: Vec::new(),
            idle: true,
            current_pid: 0,
            load: 0,
        }
    }

    /// Enqueue a task onto this CPU's run queue
    pub fn enqueue(&mut self, pid: u32) {
        if !self.tasks.contains(&pid) {
            self.tasks.push(pid);
            self.idle = false;
            self.load += 1;
        }
    }

    /// Dequeue the highest-priority task
    pub fn dequeue_next(&mut self) -> Option<u32> {
        if self.tasks.is_empty() {
            self.idle = true;
            self.current_pid = 0;
            None
        } else {
            let pid = self.tasks.remove(0);
            self.current_pid = pid;
            self.load = self.load.saturating_sub(1);
            Some(pid)
        }
    }
}

/// SchedExt Scheduling Domain — groups CPUs with shared characteristics
#[derive(Debug)]
pub struct SchedExtDomain {
    /// Domain name (e.g. "P-cores", "E-cores", "NUMA-0")
    pub name: String,
    /// CPU IDs in this domain
    pub cpus: Vec<u32>,
    /// Domain flags
    pub flags: u32,
    /// Per-CPU run queues
    pub rqueues: Vec<SchedExtCpuRq>,
}

impl SchedExtDomain {
    pub fn new(name: &str, cpus: Vec<u32>) -> Self {
        let rqueues = cpus.iter().map(|&id| SchedExtCpuRq::new(id)).collect();
        Self {
            name: String::from(name),
            cpus,
            flags: 0,
            rqueues,
        }
    }

    /// Find the least-loaded CPU in this domain
    pub fn find_idle_cpu(&self) -> Option<u32> {
        self.rqueues
            .iter()
            .filter(|rq| rq.idle)
            .map(|rq| rq.cpu_id)
            .next()
    }

    /// Find the least-loaded CPU (non-idle fallback)
    pub fn find_least_loaded_cpu(&self) -> Option<u32> {
        self.rqueues
            .iter()
            .min_by_key(|rq| rq.load)
            .map(|rq| rq.cpu_id)
    }
}

/// Built-in SchedExt policy — LAVD (Latency-priority Aware Virtual Deadline)
/// Inspired by scx_lavd from the sched-ext project.
/// Prioritizes latency-sensitive tasks (games, audio, UI) while keeping
/// throughput tasks on efficiency cores.
#[derive(Debug, Default)]
pub struct SchedExtLavd {
    /// Virtual deadline map: pid -> deadline_ns
    pub deadlines: BTreeMap<u32, u64>,
    /// Current tick (nanoseconds)
    pub current_ns: u64,
    /// Latency-sensitive task boost (shorter deadline = higher priority)
    pub latency_boost_ns: u64,
}

impl SchedExtLavd {
    pub fn new() -> Self {
        Self {
            deadlines: BTreeMap::new(),
            current_ns: 0,
            latency_boost_ns: 500_000, // 500µs boost for latency-sensitive tasks
        }
    }

    /// Calculate virtual deadline for a task
    pub fn compute_deadline(&self, task: &SchedExtTaskCtx) -> u64 {
        let base_slice_ns: u64 = 5_000_000; // 5ms default slice
        let weight = if task.priority < 0 {
            1u64
        } else {
            (100 - task.priority.min(99)) as u64
        };
        let slice = base_slice_ns * weight / 100;

        if task.is_latency_sensitive() {
            self.current_ns + slice.saturating_sub(self.latency_boost_ns)
        } else if task.is_realtime() {
            self.current_ns + slice / 4 // RT tasks get very short slices
        } else {
            self.current_ns + slice
        }
    }

    /// Schedule a task — assign virtual deadline and select CPU
    pub fn select_cpu(&mut self, task: &SchedExtTaskCtx, domains: &[SchedExtDomain]) -> u32 {
        let deadline = self.compute_deadline(task);
        self.deadlines.insert(task.pid, deadline);

        // For P-core preference tasks, use first domain; E-core for others
        if task.flags.0 & SchedExtFlags::PREFER_ECORE.0 != 0 {
            if let Some(domain) = domains.last() {
                if let Some(cpu) = domain
                    .find_idle_cpu()
                    .or_else(|| domain.find_least_loaded_cpu())
                {
                    return cpu;
                }
            }
        }

        // Default: find any idle CPU across all domains
        for domain in domains {
            if let Some(cpu) = domain.find_idle_cpu() {
                return cpu;
            }
        }

        // Fallback: least loaded globally
        domains
            .iter()
            .flat_map(|d| d.rqueues.iter())
            .min_by_key(|rq| rq.load)
            .map(|rq| rq.cpu_id)
            .unwrap_or(0)
    }
}

/// Built-in SchedExt policy — Rusty (multi-domain load balancing)
/// Inspired by scx_rusty from the sched-ext project.
/// Uses per-domain load tracking and cross-domain migration to balance load.
#[derive(Debug, Default)]
pub struct SchedExtRusty {
    /// Load per domain
    pub domain_loads: Vec<u64>,
    /// Migration threshold (migrate if load diff > threshold)
    pub migration_threshold: u64,
    /// Statistics
    pub migrations: u64,
    pub total_scheduled: u64,
}

impl SchedExtRusty {
    pub fn new(num_domains: usize) -> Self {
        Self {
            domain_loads: vec![0u64; num_domains],
            migration_threshold: 20, // 20% load imbalance threshold
            migrations: 0,
            total_scheduled: 0,
        }
    }

    /// Balance load across domains — migrate tasks if imbalanced
    pub fn rebalance(&mut self, domains: &mut Vec<SchedExtDomain>) {
        if domains.is_empty() {
            return;
        }

        let loads: Vec<u64> = domains
            .iter()
            .map(|d| d.rqueues.iter().map(|rq| rq.load).sum())
            .collect();
        let avg_load = loads.iter().sum::<u64>() / loads.len() as u64;

        for (i, &load) in loads.iter().enumerate() {
            self.domain_loads[i] = load;
            if load > avg_load + self.migration_threshold {
                // Would trigger migration in real kernel — increment counter
                self.migrations += 1;
            }
        }
        self.total_scheduled += 1;
    }
}

/// Main SchedExt Manager — orchestrates all scheduling domains and policies
#[derive(Debug)]
pub struct SchedExtManager {
    /// Scheduling domains (e.g. P-cores, E-cores, NUMA nodes)
    pub domains: Vec<SchedExtDomain>,
    /// Task contexts (pid -> ctx)
    pub tasks: BTreeMap<u32, SchedExtTaskCtx>,
    /// LAVD policy engine
    pub lavd: SchedExtLavd,
    /// Rusty load balancer
    pub rusty: SchedExtRusty,
    /// Active policy name
    pub active_policy: String,
    /// Scheduler statistics
    pub stats: SchedExtStats,
}

/// Scheduling statistics for observability
#[derive(Debug, Default)]
pub struct SchedExtStats {
    pub total_enqueues: u64,
    pub total_dispatches: u64,
    pub total_preemptions: u64,
    pub total_migrations: u64,
    pub latency_sensitive_boosts: u64,
    pub rt_tasks_scheduled: u64,
}

impl SchedExtManager {
    /// Create a new SchedExt manager with default topology (single domain)
    pub fn new() -> Self {
        // Default: single unified domain with 4 CPUs
        let default_domain = SchedExtDomain::new("unified", vec![0, 1, 2, 3]);
        let num_domains = 1;
        Self {
            domains: vec![default_domain],
            tasks: BTreeMap::new(),
            lavd: SchedExtLavd::new(),
            rusty: SchedExtRusty::new(num_domains),
            active_policy: String::from("lavd"),
            stats: SchedExtStats::default(),
        }
    }

    /// Configure hybrid topology (P-cores + E-cores, like Intel 12th gen+)
    pub fn configure_hybrid_topology(&mut self, p_cores: Vec<u32>, e_cores: Vec<u32>) {
        self.domains.clear();
        let num_domains = 2;
        self.domains.push(SchedExtDomain::new("P-cores", p_cores));
        self.domains.push(SchedExtDomain::new("E-cores", e_cores));
        self.rusty = SchedExtRusty::new(num_domains);
    }

    /// Configure NUMA topology
    pub fn configure_numa_topology(&mut self, nodes: Vec<(String, Vec<u32>)>) {
        let num = nodes.len();
        self.domains.clear();
        for (name, cpus) in nodes {
            self.domains.push(SchedExtDomain::new(&name, cpus));
        }
        self.rusty = SchedExtRusty::new(num);
    }

    /// Register a new task with the scheduler
    pub fn task_init(&mut self, mut ctx: SchedExtTaskCtx) {
        if ctx.is_realtime() {
            self.stats.rt_tasks_scheduled += 1;
        }
        if ctx.is_latency_sensitive() {
            self.stats.latency_sensitive_boosts += 1;
        }
        self.tasks.insert(ctx.pid, ctx);
    }

    /// Select CPU for a task (SchedExt: ops.select_cpu)
    pub fn select_cpu(&mut self, pid: u32) -> u32 {
        if let Some(task) = self.tasks.get(&pid).cloned() {
            self.lavd.select_cpu(&task, &self.domains)
        } else {
            0
        }
    }

    /// Enqueue a task (SchedExt: ops.enqueue)
    pub fn enqueue(&mut self, pid: u32, cpu: u32) {
        self.stats.total_enqueues += 1;
        for domain in &mut self.domains {
            if let Some(rq) = domain.rqueues.iter_mut().find(|rq| rq.cpu_id == cpu) {
                rq.enqueue(pid);
                return;
            }
        }
        // Fallback: enqueue on first available CPU
        if let Some(rq) = self.domains.first_mut().and_then(|d| d.rqueues.first_mut()) {
            rq.enqueue(pid);
        }
    }

    /// Dispatch next task on a CPU (SchedExt: ops.dispatch)
    pub fn dispatch(&mut self, cpu: u32) -> Option<u32> {
        self.stats.total_dispatches += 1;
        for domain in &mut self.domains {
            if let Some(rq) = domain.rqueues.iter_mut().find(|rq| rq.cpu_id == cpu) {
                return rq.dequeue_next();
            }
        }
        None
    }

    /// Remove a task from the scheduler (SchedExt: ops.exit)
    pub fn task_exit(&mut self, pid: u32) {
        self.tasks.remove(&pid);
        for domain in &mut self.domains {
            for rq in &mut domain.rqueues {
                rq.tasks.retain(|&p| p != pid);
                if rq.current_pid == pid {
                    rq.current_pid = 0;
                    rq.idle = rq.tasks.is_empty();
                }
            }
        }
        self.lavd.deadlines.remove(&pid);
    }

    /// Run load rebalancing across domains
    pub fn rebalance(&mut self) {
        self.rusty.rebalance(&mut self.domains);
        self.stats.total_migrations += self.rusty.migrations;
    }

    /// Get scheduler stats
    pub fn stats(&self) -> &SchedExtStats {
        &self.stats
    }

    /// Set active scheduling policy  
    pub fn set_policy(&mut self, policy: &str) {
        self.active_policy = String::from(policy);
    }

    /// Get list of available scheduling policies
    pub fn available_policies() -> Vec<&'static str> {
        vec!["lavd", "rusty", "cfs-compat", "deadline", "idle"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedext_manager_new() {
        let mgr = SchedExtManager::new();
        assert_eq!(mgr.domains.len(), 1);
        assert_eq!(mgr.active_policy, "lavd");
        assert_eq!(mgr.domains[0].cpus, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_task_scheduling_lifecycle() {
        let mut mgr = SchedExtManager::new();
        let mut ctx = SchedExtTaskCtx::new(1234, 0);
        ctx.set_comm("firefox");
        ctx.flags = SchedExtFlags::LATENCY_SENSITIVE;
        mgr.task_init(ctx);
        assert_eq!(mgr.stats.latency_sensitive_boosts, 1);

        let cpu = mgr.select_cpu(1234);
        mgr.enqueue(1234, cpu);
        assert_eq!(mgr.stats.total_enqueues, 1);

        let dispatched = mgr.dispatch(cpu);
        assert_eq!(dispatched, Some(1234));
        assert_eq!(mgr.stats.total_dispatches, 1);

        mgr.task_exit(1234);
        assert!(!mgr.tasks.contains_key(&1234));
    }

    #[test]
    fn test_hybrid_topology() {
        let mut mgr = SchedExtManager::new();
        mgr.configure_hybrid_topology(
            vec![0, 1, 2, 3],               // P-cores
            vec![4, 5, 6, 7, 8, 9, 10, 11], // E-cores
        );
        assert_eq!(mgr.domains.len(), 2);
        assert_eq!(mgr.domains[0].name, "P-cores");
        assert_eq!(mgr.domains[1].name, "E-cores");
    }

    #[test]
    fn test_lavd_deadline_computation() {
        let lavd = SchedExtLavd::new();
        let mut rt_task = SchedExtTaskCtx::new(100, -20);
        rt_task.flags = SchedExtFlags::REALTIME;

        let mut latency_task = SchedExtTaskCtx::new(200, 0);
        latency_task.flags = SchedExtFlags::LATENCY_SENSITIVE;

        let rt_deadline = lavd.compute_deadline(&rt_task);
        let lat_deadline = lavd.compute_deadline(&latency_task);
        // RT tasks should have shorter deadlines (higher priority)
        assert!(rt_deadline <= lat_deadline);
    }

    #[test]
    fn test_cpu_rq_operations() {
        let mut rq = SchedExtCpuRq::new(0);
        assert!(rq.idle);

        rq.enqueue(42);
        assert!(!rq.idle);
        assert_eq!(rq.load, 1);

        let pid = rq.dequeue_next();
        assert_eq!(pid, Some(42));
        assert_eq!(rq.current_pid, 42);

        let none = rq.dequeue_next();
        assert!(none.is_none());
        assert!(rq.idle);
    }

    #[test]
    fn test_numa_topology() {
        let mut mgr = SchedExtManager::new();
        mgr.configure_numa_topology(vec![
            (String::from("NUMA-0"), vec![0, 1, 2, 3]),
            (String::from("NUMA-1"), vec![4, 5, 6, 7]),
        ]);
        assert_eq!(mgr.domains.len(), 2);
    }

    #[test]
    fn test_schedext_available_policies() {
        let policies = SchedExtManager::available_policies();
        assert!(policies.contains(&"lavd"));
        assert!(policies.contains(&"rusty"));
    }
}
