// SigmaOS SMP/NUMA Load Balancer
// Inspired by Linux kernel CFS load balancer, NUMA topology, and IPI-based task migration.
// Provides: per-CPU run queues, domain hierarchy, IPI dispatch, and topology-aware migration.

use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// ─────────────────────────────────────────────────────────────────────────────
// CPU Topology
// ─────────────────────────────────────────────────────────────────────────────

/// NUMA node descriptor
#[derive(Debug, Clone)]
pub struct NumaNode {
    pub node_id: u32,
    pub cpu_ids: Vec<u32>,
    pub memory_bytes: u64,
    pub distance_to: BTreeMap<u32, u32>, // node_id → NUMA distance
}

/// SMT (Hyper-Threading) sibling pair
#[derive(Debug, Clone)]
pub struct SmtPair {
    pub logical_cpu_a: u32,
    pub logical_cpu_b: u32,
    pub physical_core: u32,
}

/// Full SMP topology discovered at boot
#[derive(Debug, Clone)]
pub struct SmpTopology {
    pub total_cpus: u32,
    pub numa_nodes: Vec<NumaNode>,
    pub smt_pairs: Vec<SmtPair>,
    pub cache_domains: Vec<CacheDomain>,
}

#[derive(Debug, Clone)]
pub struct CacheDomain {
    pub level: u8, // L1/L2/L3
    pub shared_cpus: Vec<u32>,
    pub size_kb: u32,
}

impl SmpTopology {
    /// Build a synthetic 4-core/2-NUMA topology for simulation
    pub fn detect() -> Self {
        SmpTopology {
            total_cpus: 4,
            numa_nodes: vec![
                NumaNode {
                    node_id: 0,
                    cpu_ids: vec![0, 1],
                    memory_bytes: 8 * 1024 * 1024 * 1024,
                    distance_to: BTreeMap::from([(0, 10), (1, 20)]),
                },
                NumaNode {
                    node_id: 1,
                    cpu_ids: vec![2, 3],
                    memory_bytes: 8 * 1024 * 1024 * 1024,
                    distance_to: BTreeMap::from([(0, 20), (1, 10)]),
                },
            ],
            smt_pairs: vec![
                SmtPair {
                    logical_cpu_a: 0,
                    logical_cpu_b: 1,
                    physical_core: 0,
                },
                SmtPair {
                    logical_cpu_a: 2,
                    logical_cpu_b: 3,
                    physical_core: 1,
                },
            ],
            cache_domains: vec![
                CacheDomain {
                    level: 3,
                    shared_cpus: vec![0, 1],
                    size_kb: 8192,
                },
                CacheDomain {
                    level: 3,
                    shared_cpus: vec![2, 3],
                    size_kb: 8192,
                },
            ],
        }
    }

    pub fn numa_node_for_cpu(&self, cpu: u32) -> Option<u32> {
        self.numa_nodes
            .iter()
            .find(|n| n.cpu_ids.contains(&cpu))
            .map(|n| n.node_id)
    }

    pub fn numa_distance(&self, cpu_a: u32, cpu_b: u32) -> u32 {
        let node_a = self.numa_node_for_cpu(cpu_a).unwrap_or(0);
        let node_b = self.numa_node_for_cpu(cpu_b).unwrap_or(0);
        if node_a == node_b {
            return 10;
        }
        self.numa_nodes
            .iter()
            .find(|n| n.node_id == node_a)
            .and_then(|n| n.distance_to.get(&node_b))
            .copied()
            .unwrap_or(40)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-CPU Run Queue
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Task {
    pub tid: u64,
    pub priority: i32,    // -20..+19 like Linux nice values
    pub vruntime_ns: u64, // CFS virtual runtime
    pub numa_home: u32,   // preferred NUMA node
    pub cpu_affinity: Option<Vec<u32>>,
    pub load_weight: u64,
}

impl Task {
    pub fn new(tid: u64, priority: i32, numa_home: u32) -> Self {
        Task {
            tid,
            priority,
            vruntime_ns: 0,
            numa_home,
            cpu_affinity: None,
            load_weight: Self::nice_to_weight(priority),
        }
    }

    /// Linux-compatible nice-to-weight conversion (prio_to_weight[])
    pub fn nice_to_weight(nice: i32) -> u64 {
        const WEIGHT_TABLE: [u64; 40] = [
            88761, 71755, 56483, 46273, 36291, 29154, 23254, 18705, 14949, 11916, 9548, 7620, 6100,
            4904, 3906, 3121, 2501, 1991, 1586, 1277, 1024, 820, 655, 526, 423, 335, 272, 215, 172,
            137, 110, 87, 70, 56, 45, 36, 29, 23, 18, 15,
        ];
        let idx = ((nice + 20).clamp(0, 39)) as usize;
        WEIGHT_TABLE[idx]
    }
}

#[derive(Debug)]
pub struct PerCpuRunQueue {
    pub cpu_id: u32,
    pub tasks: VecDeque<Task>,
    pub load: AtomicU64,
    pub nr_running: AtomicUsize,
    pub min_vruntime_ns: AtomicU64,
}

impl PerCpuRunQueue {
    pub fn new(cpu_id: u32) -> Self {
        PerCpuRunQueue {
            cpu_id,
            tasks: VecDeque::new(),
            load: AtomicU64::new(0),
            nr_running: AtomicUsize::new(0),
            min_vruntime_ns: AtomicU64::new(0),
        }
    }

    pub fn enqueue(&mut self, mut task: Task) {
        // Place at min_vruntime to avoid starvation
        let min_vr = self.min_vruntime_ns.load(Ordering::Relaxed);
        if task.vruntime_ns < min_vr {
            task.vruntime_ns = min_vr;
        }
        self.load.fetch_add(task.load_weight, Ordering::Relaxed);
        self.nr_running.fetch_add(1, Ordering::Relaxed);
        self.tasks.push_back(task);
        self.sort_by_vruntime();
    }

    pub fn dequeue(&mut self) -> Option<Task> {
        if let Some(task) = self.tasks.pop_front() {
            self.load.fetch_sub(
                task.load_weight.min(self.load.load(Ordering::Relaxed)),
                Ordering::Relaxed,
            );
            self.nr_running.fetch_sub(1, Ordering::Relaxed);
            Some(task)
        } else {
            None
        }
    }

    pub fn pick_next(&self) -> Option<&Task> {
        self.tasks.front()
    }

    fn sort_by_vruntime(&mut self) {
        let mut v: Vec<Task> = self.tasks.drain(..).collect();
        v.sort_by_key(|t| t.vruntime_ns);
        self.tasks = v.into();
    }

    pub fn current_load(&self) -> u64 {
        self.load.load(Ordering::Relaxed)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IPI (Inter-Processor Interrupt) Subsystem
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum IpiMessage {
    /// Request remote CPU to reschedule (TLB shootdown in real HW)
    Reschedule,
    /// Request remote CPU to migrate task `tid` to `dst_cpu`
    MigrateTask { tid: u64, dst_cpu: u32 },
    /// Flush TLB on remote CPU (INVLPG broadcast)
    TlbFlush { address: u64 },
    /// NMI (non-maskable) watchdog ping
    WatchdogPing,
    /// CPU hotplug notification
    CpuOffline { cpu_id: u32 },
}

#[derive(Debug)]
pub struct IpiChannel {
    /// Per-CPU IPI message queues
    queues: BTreeMap<u32, Mutex<VecDeque<IpiMessage>>>,
}

impl IpiChannel {
    pub fn new(cpu_ids: &[u32]) -> Self {
        let mut queues = BTreeMap::new();
        for &cpu in cpu_ids {
            queues.insert(cpu, Mutex::new(VecDeque::new()));
        }
        IpiChannel { queues }
    }

    /// Simulate sending an IPI to target CPU
    pub fn send(&self, target_cpu: u32, msg: IpiMessage) -> Result<(), &'static str> {
        match self.queues.get(&target_cpu) {
            Some(q) => {
                q.lock().unwrap().push_back(msg);
                Ok(())
            }
            None => Err("Target CPU not found"),
        }
    }

    /// Drain all pending IPIs for a CPU (called on each timer tick)
    pub fn drain(&self, cpu_id: u32) -> Vec<IpiMessage> {
        match self.queues.get(&cpu_id) {
            Some(q) => q.lock().unwrap().drain(..).collect(),
            None => vec![],
        }
    }

    /// Broadcast IPI to all CPUs except sender
    pub fn broadcast(&self, sender_cpu: u32, msg: IpiMessage) {
        for (&cpu, q) in &self.queues {
            if cpu != sender_cpu {
                q.lock().unwrap().push_back(msg.clone());
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Load Balancer
// ─────────────────────────────────────────────────────────────────────────────

pub struct SmpLoadBalancer {
    pub topology: SmpTopology,
    pub run_queues: BTreeMap<u32, PerCpuRunQueue>,
    pub ipi: Arc<IpiChannel>,
    pub balance_interval_ticks: u64,
    pub tick_counter: AtomicU64,
}

impl SmpLoadBalancer {
    pub fn new(topology: SmpTopology) -> Self {
        let cpu_ids: Vec<u32> = (0..topology.total_cpus).collect();
        let ipi = Arc::new(IpiChannel::new(&cpu_ids));
        let mut run_queues = BTreeMap::new();
        for id in &cpu_ids {
            run_queues.insert(*id, PerCpuRunQueue::new(*id));
        }
        SmpLoadBalancer {
            topology,
            run_queues,
            ipi,
            balance_interval_ticks: 10,
            tick_counter: AtomicU64::new(0),
        }
    }

    /// Called by the per-CPU scheduler tick (simulate APIC timer ISR)
    pub fn tick(&mut self, cpu_id: u32) {
        let tick = self.tick_counter.fetch_add(1, Ordering::Relaxed);

        // Process incoming IPIs for this CPU
        let ipis = self.ipi.drain(cpu_id);
        for ipi in ipis {
            self.handle_ipi(cpu_id, ipi);
        }

        // Periodic load balancing
        if tick % self.balance_interval_ticks == 0 {
            self.rebalance(cpu_id);
        }
    }

    fn handle_ipi(&mut self, cpu_id: u32, msg: IpiMessage) {
        match msg {
            IpiMessage::Reschedule => {
                // trigger context switch on this CPU (simulated)
                if let Some(rq) = self.run_queues.get_mut(&cpu_id) {
                    rq.sort_by_vruntime();
                }
            }
            IpiMessage::MigrateTask { tid, dst_cpu } => {
                self.migrate_task_to(tid, cpu_id, dst_cpu);
            }
            IpiMessage::TlbFlush { address: _ } => {
                // In hardware: INVLPG instruction would be issued here
            }
            IpiMessage::WatchdogPing => {}
            IpiMessage::CpuOffline { cpu_id: dead_cpu } => {
                self.evacuate_cpu(dead_cpu);
            }
        }
    }

    /// Pull task migration: find the busiest CPU in the same NUMA domain and pull tasks
    pub fn rebalance(&mut self, this_cpu: u32) {
        let this_node = self.topology.numa_node_for_cpu(this_cpu).unwrap_or(0);
        let this_load = self.run_queues[&this_cpu].current_load();

        // Find all CPUs in same NUMA node
        let same_node_cpus: Vec<u32> = self
            .topology
            .numa_nodes
            .iter()
            .find(|n| n.node_id == this_node)
            .map(|n| n.cpu_ids.clone())
            .unwrap_or_default();

        // Find the busiest CPU
        let busiest_cpu = same_node_cpus
            .iter()
            .filter(|&&c| c != this_cpu)
            .max_by_key(|&&c| {
                self.run_queues
                    .get(&c)
                    .map(|rq| rq.current_load())
                    .unwrap_or(0)
            });

        if let Some(&src_cpu) = busiest_cpu {
            let src_load = self.run_queues[&src_cpu].current_load();

            // Only migrate if imbalance > 25%
            if src_load > this_load + src_load / 4 {
                self.pull_task(src_cpu, this_cpu);
            }
        }

        // Cross-NUMA balancing (less aggressive)
        if this_load == 0 {
            let remote_busiest = self
                .run_queues
                .iter()
                .filter(|(&c, _)| c != this_cpu)
                .max_by_key(|(_, rq)| rq.current_load())
                .map(|(&c, rq)| (c, rq.current_load()));

            if let Some((src, src_load)) = remote_busiest {
                let dist = self.topology.numa_distance(this_cpu, src);
                // Only cross-NUMA if distance is small and load very imbalanced
                if dist <= 20 && src_load > 2048 {
                    self.pull_task(src, this_cpu);
                }
            }
        }
    }

    /// Pull one task from src_cpu to dst_cpu
    fn pull_task(&mut self, src_cpu: u32, dst_cpu: u32) {
        let task = self
            .run_queues
            .get_mut(&src_cpu)
            .and_then(|rq| rq.dequeue());
        if let Some(task) = task {
            let _ = self.ipi.send(dst_cpu, IpiMessage::Reschedule);
            self.run_queues.get_mut(&dst_cpu).map(|rq| rq.enqueue(task));
        }
    }

    /// Move a specific task between CPUs
    fn migrate_task_to(&mut self, tid: u64, src_cpu: u32, dst_cpu: u32) {
        let src_rq = self.run_queues.get_mut(&src_cpu);
        if let Some(rq) = src_rq {
            if let Some(pos) = rq.tasks.iter().position(|t| t.tid == tid) {
                if let Some(task) = rq.tasks.remove(pos) {
                    rq.load.fetch_sub(
                        task.load_weight.min(rq.load.load(Ordering::Relaxed)),
                        Ordering::Relaxed,
                    );
                    rq.nr_running.fetch_sub(1, Ordering::Relaxed);
                    if let Some(dst_rq) = self.run_queues.get_mut(&dst_cpu) {
                        dst_rq.enqueue(task);
                    }
                }
            }
        }
    }

    /// Evacuate all tasks from a CPU that's going offline
    pub fn evacuate_cpu(&mut self, dead_cpu: u32) {
        let tasks: Vec<Task> = {
            let rq = self.run_queues.get_mut(&dead_cpu);
            if let Some(rq) = rq {
                rq.tasks.drain(..).collect()
            } else {
                return;
            }
        };

        for task in tasks {
            // Find the least loaded online CPU
            let target = self
                .run_queues
                .iter()
                .filter(|(&c, _)| c != dead_cpu)
                .min_by_key(|(_, rq)| rq.current_load())
                .map(|(&c, _)| c);

            if let Some(cpu) = target {
                self.run_queues.get_mut(&cpu).map(|rq| rq.enqueue(task));
            }
        }
    }

    /// Enqueue a new task, choosing the best CPU (NUMA-aware)
    pub fn enqueue_task(&mut self, task: Task) {
        let preferred_cpus: Vec<u32> = self
            .topology
            .numa_nodes
            .iter()
            .find(|n| n.node_id == task.numa_home)
            .map(|n| n.cpu_ids.clone())
            .unwrap_or_else(|| (0..self.topology.total_cpus).collect());

        let target = preferred_cpus
            .iter()
            .min_by_key(|&&c| {
                self.run_queues
                    .get(&c)
                    .map(|rq| rq.current_load())
                    .unwrap_or(u64::MAX)
            })
            .copied()
            .unwrap_or(0);

        self.run_queues.get_mut(&target).map(|rq| rq.enqueue(task));
    }

    pub fn stats(&self) -> SmpStats {
        let loads: Vec<(u32, u64, usize)> = self
            .run_queues
            .iter()
            .map(|(&c, rq)| (c, rq.current_load(), rq.nr_running.load(Ordering::Relaxed)))
            .collect();
        let total_load: u64 = loads.iter().map(|(_, l, _)| l).sum();
        let total_tasks: usize = loads.iter().map(|(_, _, n)| n).sum();
        SmpStats {
            per_cpu_loads: loads,
            total_load,
            total_tasks,
        }
    }
}

#[derive(Debug)]
pub struct SmpStats {
    pub per_cpu_loads: Vec<(u32, u64, usize)>, // (cpu_id, load, nr_running)
    pub total_load: u64,
    pub total_tasks: usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// APIC Timer Simulation
// ─────────────────────────────────────────────────────────────────────────────

/// Models the Local APIC periodic timer that drives scheduler ticks.
/// On real hardware: LAPIC_TIMER_ICR register is programmed at boot.
pub struct ApicTimer {
    pub cpu_id: u32,
    pub frequency_hz: u64,
    pub ticks_elapsed: AtomicU64,
    pub calibrated_ns_per_tick: u64,
}

impl ApicTimer {
    pub fn new(cpu_id: u32, frequency_hz: u64) -> Self {
        ApicTimer {
            cpu_id,
            frequency_hz,
            ticks_elapsed: AtomicU64::new(0),
            calibrated_ns_per_tick: 1_000_000_000 / frequency_hz,
        }
    }

    /// Simulate a hardware timer interrupt (would be ISR vector in real kernel)
    pub fn interrupt(&self) -> u64 {
        let tick = self.ticks_elapsed.fetch_add(1, Ordering::Relaxed);
        tick
    }

    pub fn elapsed_ns(&self) -> u64 {
        self.ticks_elapsed.load(Ordering::Relaxed) * self.calibrated_ns_per_tick
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smp_topology_detect() {
        let topo = SmpTopology::detect();
        assert_eq!(topo.total_cpus, 4);
        assert_eq!(topo.numa_nodes.len(), 2);
        assert_eq!(topo.numa_node_for_cpu(0), Some(0));
        assert_eq!(topo.numa_node_for_cpu(2), Some(1));
        assert_eq!(topo.numa_distance(0, 2), 20);
        assert_eq!(topo.numa_distance(0, 1), 10);
    }

    #[test]
    fn test_per_cpu_run_queue() {
        let mut rq = PerCpuRunQueue::new(0);
        rq.enqueue(Task::new(1, 0, 0));
        rq.enqueue(Task::new(2, 5, 0));
        rq.enqueue(Task::new(3, -5, 0));
        assert_eq!(rq.nr_running.load(Ordering::Relaxed), 3);
        let t = rq.dequeue().unwrap();
        assert_eq!(t.tid, 1); // vruntime 0 should be first
    }

    #[test]
    fn test_ipi_channel() {
        let ipi = IpiChannel::new(&[0, 1, 2, 3]);
        ipi.send(1, IpiMessage::Reschedule).unwrap();
        ipi.send(
            1,
            IpiMessage::TlbFlush {
                address: 0xDEAD_BEEF,
            },
        )
        .unwrap();
        let msgs = ipi.drain(1);
        assert_eq!(msgs.len(), 2);
    }

    #[test]
    fn test_load_balancer_enqueue_and_rebalance() {
        let topo = SmpTopology::detect();
        let mut balancer = SmpLoadBalancer::new(topo);
        for i in 0..8 {
            balancer.enqueue_task(Task::new(i, 0, 0));
        }
        let stats = balancer.stats();
        assert_eq!(stats.total_tasks, 8);
        // Trigger rebalance
        balancer.rebalance(0);
        balancer.rebalance(1);
    }

    #[test]
    fn test_apic_timer() {
        let timer = ApicTimer::new(0, 1000);
        timer.interrupt();
        timer.interrupt();
        assert_eq!(timer.elapsed_ns(), 2_000_000);
    }

    #[test]
    fn test_numa_aware_task_placement() {
        let topo = SmpTopology::detect();
        let mut balancer = SmpLoadBalancer::new(topo);
        // NUMA node 1 → CPUs 2,3
        let task = Task::new(99, 0, 1);
        balancer.enqueue_task(task);
        let cpu2_tasks = balancer.run_queues[&2].nr_running.load(Ordering::Relaxed);
        let cpu3_tasks = balancer.run_queues[&3].nr_running.load(Ordering::Relaxed);
        assert!(
            cpu2_tasks + cpu3_tasks == 1,
            "Task should land on NUMA node 1 CPUs"
        );
    }
}
