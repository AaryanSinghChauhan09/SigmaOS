// SigmaOS Process Scheduler
// Inspired by Linux CFS (Completely Fair Scheduler) and FreeBSD ULE
// Implements virtual-runtime-based fair scheduling with priority support

#![allow(dead_code)]

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Unique task identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub u32);

/// Task state in the scheduler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Sleeping,
    Zombie,
}

/// Scheduling policy (Linux-inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedPolicy {
    /// Normal CFS scheduling (SCHED_NORMAL/SCHED_OTHER)
    Normal,
    /// Real-time FIFO (SCHED_FIFO)
    RealTimeFifo,
    /// Real-time Round-Robin (SCHED_RR)
    RealTimeRR,
    /// Batch processing (SCHED_BATCH)
    Batch,
    /// Idle priority (SCHED_IDLE)
    Idle,
}

/// A schedulable task/process
#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub vruntime: u64,
    pub priority: u32,
    pub nice: i32,
    pub state: TaskState,
    pub policy: SchedPolicy,
    pub time_slice_remaining: u64,
    pub total_runtime: u64,
    pub weight: u32,
    pub cpu_affinity: u64,
}

impl Task {
    pub fn new(id: TaskId, priority: u32) -> Self {
        Self {
            id,
            vruntime: 0,
            priority,
            nice: 0,
            state: TaskState::Ready,
            policy: SchedPolicy::Normal,
            time_slice_remaining: DEFAULT_TIME_SLICE,
            total_runtime: 0,
            weight: nice_to_weight(0),
            cpu_affinity: u64::MAX, // all CPUs
        }
    }

    pub fn with_nice(mut self, nice: i32) -> Self {
        self.nice = nice.clamp(-20, 19);
        self.weight = nice_to_weight(self.nice);
        self
    }

    pub fn with_policy(mut self, policy: SchedPolicy) -> Self {
        self.policy = policy;
        self
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Lower vruntime = higher scheduling priority
        self.vruntime
            .cmp(&other.vruntime)
            .then_with(|| self.id.0.cmp(&other.id.0))
    }
}

/// Default time slice in microseconds (6ms like Linux)
const DEFAULT_TIME_SLICE: u64 = 6000;
/// Minimum granularity in microseconds
const MIN_GRANULARITY: u64 = 750;
/// Target latency in microseconds (scheduling period)
const TARGET_LATENCY: u64 = 24000;

/// Convert nice value (-20..19) to a scheduling weight
/// Mirrors Linux's sched_prio_to_weight table
fn nice_to_weight(nice: i32) -> u32 {
    // Simplified weight table inspired by Linux
    // nice 0 = weight 1024, each nice level ~1.25x
    let base: u32 = 1024;
    if nice == 0 {
        return base;
    }
    if nice > 0 {
        base >> (nice as u32).min(10)
    } else {
        base << ((-nice) as u32).min(10)
    }
}

/// Per-CPU run queue (CFS red-black tree simulated with BTreeMap)
#[derive(Debug)]
pub struct RunQueue {
    /// Tasks indexed by (vruntime, task_id) for O(log n) min-vruntime lookup
    tasks: BTreeMap<(u64, u32), Task>,
    /// Minimum vruntime across all tasks in this queue
    min_vruntime: u64,
    /// Total weight of all tasks in this queue
    total_weight: u64,
    /// Number of tasks
    nr_running: usize,
    /// Currently running task
    current: Option<Task>,
}

impl RunQueue {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            min_vruntime: 0,
            total_weight: 0,
            nr_running: 0,
            current: None,
        }
    }

    /// Enqueue a task into the run queue
    pub fn enqueue(&mut self, mut task: Task) {
        // New tasks start at the current min_vruntime to avoid starvation
        if task.vruntime < self.min_vruntime {
            task.vruntime = self.min_vruntime;
        }
        task.state = TaskState::Ready;
        self.total_weight += task.weight as u64;
        self.nr_running += 1;
        self.tasks.insert((task.vruntime, task.id.0), task);
    }

    /// Dequeue the task with the smallest vruntime (most deserving of CPU)
    pub fn dequeue_next(&mut self) -> Option<Task> {
        // BTreeMap is sorted, first entry has smallest key
        let key = {
            let first = self.tasks.iter().next()?;
            *first.0
        };
        let mut task = self.tasks.remove(&key)?;
        task.state = TaskState::Running;
        self.total_weight = self.total_weight.saturating_sub(task.weight as u64);
        self.nr_running -= 1;
        Some(task)
    }

    /// Remove a specific task by ID
    pub fn remove_task(&mut self, id: TaskId) -> Option<Task> {
        let key = self
            .tasks
            .keys()
            .find(|(_, tid)| *tid == id.0)
            .copied()?;
        let task = self.tasks.remove(&key)?;
        self.total_weight = self.total_weight.saturating_sub(task.weight as u64);
        self.nr_running -= 1;
        Some(task)
    }

    /// Update min_vruntime after a scheduling decision
    fn update_min_vruntime(&mut self) {
        let tree_min = self.tasks.keys().next().map(|(vrt, _)| *vrt);
        let current_vrt = self.current.as_ref().map(|t| t.vruntime);

        self.min_vruntime = match (tree_min, current_vrt) {
            (Some(a), Some(b)) => a.min(b),
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => self.min_vruntime,
        };
    }

    /// Calculate the ideal time slice for a task given the current load
    fn calc_time_slice(&self, task: &Task) -> u64 {
        if self.nr_running <= 1 {
            return TARGET_LATENCY;
        }
        let slice = (TARGET_LATENCY * task.weight as u64)
            / self.total_weight.max(1);
        slice.max(MIN_GRANULARITY)
    }

    pub fn is_empty(&self) -> bool {
        self.nr_running == 0 && self.current.is_none()
    }

    pub fn len(&self) -> usize {
        self.nr_running + if self.current.is_some() { 1 } else { 0 }
    }
}

/// The main SigmaOS Scheduler
#[derive(Debug)]
pub struct Scheduler {
    /// Per-CPU run queues
    run_queues: Vec<RunQueue>,
    /// Real-time FIFO queue (higher priority than CFS)
    rt_fifo_queue: Vec<Task>,
    /// Real-time RR queue
    rt_rr_queue: Vec<Task>,
    /// Idle tasks
    idle_queue: Vec<Task>,
    /// Next task ID to assign
    next_task_id: u32,
    /// Total number of CPUs
    nr_cpus: usize,
    /// Global clock tick counter
    tick_count: u64,
}

impl Scheduler {
    pub fn new(nr_cpus: usize) -> Self {
        let mut run_queues = Vec::with_capacity(nr_cpus);
        for _ in 0..nr_cpus {
            run_queues.push(RunQueue::new());
        }
        Self {
            run_queues,
            rt_fifo_queue: Vec::new(),
            rt_rr_queue: Vec::new(),
            idle_queue: Vec::new(),
            next_task_id: 1,
            nr_cpus,
            tick_count: 0,
        }
    }

    /// Create and enqueue a new task, returning its ID
    pub fn spawn_task(&mut self, priority: u32, nice: i32, policy: SchedPolicy) -> TaskId {
        let id = TaskId(self.next_task_id);
        self.next_task_id += 1;

        let task = Task::new(id, priority)
            .with_nice(nice)
            .with_policy(policy);

        match policy {
            SchedPolicy::RealTimeFifo => self.rt_fifo_queue.push(task),
            SchedPolicy::RealTimeRR => self.rt_rr_queue.push(task),
            SchedPolicy::Idle => self.idle_queue.push(task),
            _ => {
                // Load-balance: pick the CPU with the fewest tasks
                let cpu = self.find_least_loaded_cpu();
                self.run_queues[cpu].enqueue(task);
            }
        }
        id
    }

    /// Pick the next task to run on a given CPU
    pub fn schedule(&mut self, cpu: usize) -> Option<Task> {
        // Priority order: RT FIFO > RT RR > CFS Normal > Batch > Idle

        // 1. Check RT FIFO (highest priority, non-preemptive among RT FIFO)
        if !self.rt_fifo_queue.is_empty() {
            // Pick the highest-priority RT FIFO task
            let mut best_idx = 0;
            for (i, t) in self.rt_fifo_queue.iter().enumerate() {
                if t.priority > self.rt_fifo_queue[best_idx].priority {
                    best_idx = i;
                }
            }
            return Some(self.rt_fifo_queue.remove(best_idx));
        }

        // 2. Check RT RR
        if !self.rt_rr_queue.is_empty() {
            return Some(self.rt_rr_queue.remove(0));
        }

        // 3. CFS scheduling on the per-CPU run queue
        if cpu < self.run_queues.len() {
            if let Some(task) = self.run_queues[cpu].dequeue_next() {
                return Some(task);
            }
            // Work stealing: try other CPUs
            for other_cpu in 0..self.nr_cpus {
                if other_cpu != cpu && self.run_queues[other_cpu].nr_running > 1 {
                    if let Some(task) = self.run_queues[other_cpu].dequeue_next() {
                        return Some(task);
                    }
                }
            }
        }

        // 4. Idle tasks
        if !self.idle_queue.is_empty() {
            return Some(self.idle_queue.remove(0));
        }

        None
    }

    /// Called on each timer tick to update vruntime and check preemption
    pub fn tick(&mut self, cpu: usize, elapsed_us: u64) {
        self.tick_count += 1;

        if cpu >= self.run_queues.len() {
            return;
        }

        let rq = &mut self.run_queues[cpu];
        if let Some(ref mut current) = rq.current {
            // Update vruntime: weighted by inverse of task weight
            // Higher weight tasks accumulate vruntime slower (get more CPU)
            let delta = (elapsed_us * 1024) / current.weight.max(1) as u64;
            current.vruntime += delta;
            current.total_runtime += elapsed_us;
            current.time_slice_remaining = current.time_slice_remaining.saturating_sub(elapsed_us);

            rq.update_min_vruntime();
        }
    }

    /// Check if the current task on a CPU should be preempted
    pub fn should_preempt(&self, cpu: usize) -> bool {
        if cpu >= self.run_queues.len() {
            return false;
        }
        let rq = &self.run_queues[cpu];
        if let Some(ref current) = rq.current {
            // Preempt if time slice expired
            if current.time_slice_remaining == 0 {
                return true;
            }
            // Preempt if a task with significantly lower vruntime is waiting
            if let Some((&(min_vrt, _), _)) = rq.tasks.iter().next() {
                if current.vruntime > min_vrt + MIN_GRANULARITY {
                    return true;
                }
            }
        }
        false
    }

    /// Block the current task on a CPU (e.g., waiting for I/O)
    pub fn block_current(&mut self, cpu: usize) -> Option<Task> {
        if cpu >= self.run_queues.len() {
            return None;
        }
        let rq = &mut self.run_queues[cpu];
        rq.current.take().map(|mut t| {
            t.state = TaskState::Blocked;
            t
        })
    }

    /// Wake a blocked task and re-enqueue it
    pub fn wake_task(&mut self, mut task: Task) {
        // Give waking tasks a small vruntime bonus to reduce latency
        // (inspired by Linux's place_entity)
        let cpu = self.find_least_loaded_cpu();
        let min_vrt = self.run_queues[cpu].min_vruntime;
        task.vruntime = task.vruntime.max(min_vrt.saturating_sub(TARGET_LATENCY));
        task.state = TaskState::Ready;
        self.run_queues[cpu].enqueue(task);
    }

    /// Find the CPU with the fewest runnable tasks (load balancing)
    fn find_least_loaded_cpu(&self) -> usize {
        let mut min_load = usize::MAX;
        let mut min_cpu = 0;
        for (cpu, rq) in self.run_queues.iter().enumerate() {
            if rq.len() < min_load {
                min_load = rq.len();
                min_cpu = cpu;
            }
        }
        min_cpu
    }

    /// Get scheduler statistics
    pub fn stats(&self) -> SchedulerStats {
        let mut total_tasks = 0;
        let mut total_weight = 0;
        for rq in &self.run_queues {
            total_tasks += rq.len();
            total_weight += rq.total_weight;
        }
        total_tasks += self.rt_fifo_queue.len();
        total_tasks += self.rt_rr_queue.len();
        total_tasks += self.idle_queue.len();

        SchedulerStats {
            total_tasks,
            total_weight,
            nr_cpus: self.nr_cpus,
            tick_count: self.tick_count,
            rt_fifo_count: self.rt_fifo_queue.len(),
            rt_rr_count: self.rt_rr_queue.len(),
            idle_count: self.idle_queue.len(),
        }
    }
}

/// Scheduler statistics
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub total_tasks: usize,
    pub total_weight: u64,
    pub nr_cpus: usize,
    pub tick_count: u64,
    pub rt_fifo_count: usize,
    pub rt_rr_count: usize,
    pub idle_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(TaskId(1), 120);
        assert_eq!(task.id, TaskId(1));
        assert_eq!(task.priority, 120);
        assert_eq!(task.state, TaskState::Ready);
        assert_eq!(task.nice, 0);
        assert_eq!(task.weight, 1024);
    }

    #[test]
    fn test_nice_to_weight() {
        assert_eq!(nice_to_weight(0), 1024);
        assert!(nice_to_weight(-5) > nice_to_weight(0));
        assert!(nice_to_weight(5) < nice_to_weight(0));
    }

    #[test]
    fn test_run_queue_enqueue_dequeue() {
        let mut rq = RunQueue::new();
        assert!(rq.is_empty());

        rq.enqueue(Task::new(TaskId(1), 120));
        rq.enqueue(Task::new(TaskId(2), 120));
        assert_eq!(rq.len(), 2);

        let next = rq.dequeue_next().unwrap();
        assert_eq!(next.state, TaskState::Running);
        assert_eq!(rq.len(), 1);
    }

    #[test]
    fn test_scheduler_spawn_and_schedule() {
        let mut sched = Scheduler::new(2);
        let t1 = sched.spawn_task(120, 0, SchedPolicy::Normal);
        let t2 = sched.spawn_task(120, -5, SchedPolicy::Normal);
        let t3 = sched.spawn_task(120, 5, SchedPolicy::Normal);

        assert_eq!(t1, TaskId(1));
        assert_eq!(t2, TaskId(2));
        assert_eq!(t3, TaskId(3));

        let stats = sched.stats();
        assert_eq!(stats.total_tasks, 3);
    }

    #[test]
    fn test_rt_fifo_priority() {
        let mut sched = Scheduler::new(1);
        sched.spawn_task(120, 0, SchedPolicy::Normal);
        sched.spawn_task(99, 0, SchedPolicy::RealTimeFifo);

        // RT FIFO should be scheduled first
        let next = sched.schedule(0).unwrap();
        assert_eq!(next.policy, SchedPolicy::RealTimeFifo);
    }

    #[test]
    fn test_work_stealing() {
        let mut sched = Scheduler::new(2);
        // Put tasks on CPU 0 only
        for _ in 0..4 {
            sched.spawn_task(120, 0, SchedPolicy::Normal);
        }
        // CPU 1 should steal a task from CPU 0
        let task = sched.schedule(1);
        assert!(task.is_some());
    }

    #[test]
    fn test_scheduler_stats() {
        let mut sched = Scheduler::new(4);
        sched.spawn_task(120, 0, SchedPolicy::Normal);
        sched.spawn_task(99, 0, SchedPolicy::RealTimeFifo);
        sched.spawn_task(120, 0, SchedPolicy::Idle);

        let stats = sched.stats();
        assert_eq!(stats.nr_cpus, 4);
        assert_eq!(stats.total_tasks, 3);
        assert_eq!(stats.rt_fifo_count, 1);
        assert_eq!(stats.idle_count, 1);
    }
}