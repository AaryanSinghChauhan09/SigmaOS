// SigmaOS Scheduler Core Engine
// Implements authoritative POSIX process state transitions, lost-wakeup protection,
// context switch integrity checking, starvation prevention, and CPU affinity governance.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::run_queue::{BoundedRunQueue, RunQueueError, TaskRunRef, MAX_RUNQUEUE_CAPACITY};

/// Maximum processes in kernel process table.
pub const MAX_PROCESS_TABLE_SIZE: usize = 4096;
/// Default starvation threshold in ticks (e.g. 50 ticks = 500ms at 100Hz).
pub const DEFAULT_STARVATION_THRESHOLD_TICKS: u64 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    New,
    Ready,
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Reaped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerError {
    InvalidStateTransition { from: ProcessState, to: ProcessState },
    ProcessTableFull,
    ProcessNotFound(u64),
    ProcessAlreadyExists(u64),
    RunQueueError(RunQueueError),
    InvalidCpuAffinity,
    CannotReapNonZombie(u64),
    TaskCannotBeScheduled(u64),
}

impl From<RunQueueError> for SchedulerError {
    fn from(err: RunQueueError) -> Self {
        SchedulerError::RunQueueError(err)
    }
}

/// Process descriptor managed by SchedulerCore
#[derive(Debug, Clone)]
pub struct SchedulerProcessControlBlock {
    pub pid: u64,
    pub ppid: u64,
    pub state: ProcessState,
    pub priority: u8,
    pub cpu_affinity: u64,
    pub current_cpu: u32,
    pub exit_code: i32,
    pub vruntime: u64,
    pub total_scheduled_ticks: u64,
    pub remaining_quantum_ticks: u32,
    pub pending_signals: u64,
    pub wake_event_pending: bool,
}

impl SchedulerProcessControlBlock {
    pub fn new(pid: u64, ppid: u64, priority: u8, cpu_affinity: u64) -> Self {
        Self {
            pid,
            ppid,
            state: ProcessState::New,
            priority: priority.min(63),
            cpu_affinity: if cpu_affinity == 0 { 1 } else { cpu_affinity },
            current_cpu: 0,
            exit_code: 0,
            vruntime: 0,
            total_scheduled_ticks: 0,
            remaining_quantum_ticks: 10,
            pending_signals: 0,
            wake_event_pending: false,
        }
    }
}

/// Core Scheduler Manager implementing authoritative state transitions and SMP run queues.
#[derive(Debug)]
pub struct SchedulerCore {
    pub num_cpus: u32,
    pub current_tick: u64,
    pub process_table: BTreeMap<u64, SchedulerProcessControlBlock>,
    pub run_queues: Vec<BoundedRunQueue>,
    pub active_pids: Vec<Option<u64>>, // Current running PID per CPU core
    pub starvation_threshold_ticks: u64,
}

impl SchedulerCore {
    pub fn new(num_cpus: u32) -> Self {
        let cpus = num_cpus.max(1);
        let mut run_queues = Vec::with_capacity(cpus as usize);
        let mut active_pids = Vec::with_capacity(cpus as usize);

        for cpu in 0..cpus {
            run_queues.push(BoundedRunQueue::new(cpu, MAX_RUNQUEUE_CAPACITY));
            active_pids.push(None);
        }

        Self {
            num_cpus: cpus,
            current_tick: 0,
            process_table: BTreeMap::new(),
            run_queues,
            active_pids,
            starvation_threshold_ticks: DEFAULT_STARVATION_THRESHOLD_TICKS,
        }
    }

    /// Allocates and initializes a new process (`NEW` state).
    pub fn create_process(&mut self, pid: u64, ppid: u64, priority: u8, cpu_affinity: u64) -> Result<(), SchedulerError> {
        if self.process_table.len() >= MAX_PROCESS_TABLE_SIZE {
            return Err(SchedulerError::ProcessTableFull);
        }
        if self.process_table.contains_key(&pid) {
            return Err(SchedulerError::ProcessAlreadyExists(pid));
        }

        let pcb = SchedulerProcessControlBlock::new(pid, ppid, priority, cpu_affinity);
        self.process_table.insert(pid, pcb);
        Ok(())
    }

    /// Updates CPU affinity mask for a process and re-evaluates CPU core assignment.
    pub fn set_cpu_affinity(&mut self, pid: u64, cpu_affinity: u64) -> Result<(), SchedulerError> {
        if cpu_affinity == 0 {
            return Err(SchedulerError::InvalidCpuAffinity);
        }
        let (old_cpu, is_ready) = {
            let pcb = self.process_table.get_mut(&pid).ok_or(SchedulerError::ProcessNotFound(pid))?;
            pcb.cpu_affinity = cpu_affinity;
            (pcb.current_cpu, pcb.state == ProcessState::Ready)
        };

        // If task is ready, migrate to new optimal CPU core
        if is_ready {
            let old_cpu_idx = old_cpu as usize;
            if let Ok(task_ref) = self.run_queues[old_cpu_idx].remove_task(pid) {
                let target_cpu = self.select_best_cpu(cpu_affinity)?;
                self.run_queues[target_cpu as usize].enqueue(task_ref, self.current_tick)?;
                if let Some(pcb_mut) = self.process_table.get_mut(&pid) {
                    pcb_mut.current_cpu = target_cpu;
                }
            }
        }
        Ok(())
    }

    /// Enqueues process from `NEW` / `SLEEPING` / `STOPPED` into `READY` state.
    pub fn mark_ready(&mut self, pid: u64) -> Result<(), SchedulerError> {
        let (old_state, priority, cpu_affinity) = {
            let pcb = self.process_table.get(&pid).ok_or(SchedulerError::ProcessNotFound(pid))?;

            // Valid transitions to READY: NEW -> READY, SLEEPING -> READY, STOPPED -> READY
            match pcb.state {
                ProcessState::New | ProcessState::Sleeping | ProcessState::Stopped => {
                    (pcb.state, pcb.priority, pcb.cpu_affinity)
                }
                ProcessState::Ready => return Ok(()), // Already ready
                _ => return Err(SchedulerError::InvalidStateTransition { from: pcb.state, to: ProcessState::Ready }),
            }
        };

        let target_cpu = self.select_best_cpu(cpu_affinity)?;
        let task_ref = TaskRunRef::new(pid, priority, cpu_affinity, self.current_tick);

        // Atomically transition state to Ready only after enqueue succeeds
        if let Err(e) = self.run_queues[target_cpu as usize].enqueue(task_ref, self.current_tick) {
            return Err(SchedulerError::RunQueueError(e));
        }

        if let Some(pcb_mut) = self.process_table.get_mut(&pid) {
            pcb_mut.state = ProcessState::Ready;
            pcb_mut.current_cpu = target_cpu;
        }

        let _ = old_state;
        Ok(())
    }

    /// Schedule next ready process on specified CPU core (`READY -> RUNNING`).
    pub fn schedule_next(&mut self, cpu_id: u32) -> Result<Option<u64>, SchedulerError> {
        if cpu_id >= self.num_cpus {
            return Err(SchedulerError::InvalidCpuAffinity);
        }

        let cpu_idx = cpu_id as usize;

        // If currently running process is active, maintain or yield
        if let Some(curr_pid) = self.active_pids[cpu_idx] {
            if let Some(pcb) = self.process_table.get_mut(&curr_pid) {
                if pcb.state == ProcessState::Running {
                    if pcb.remaining_quantum_ticks > 0 {
                        pcb.remaining_quantum_ticks -= 1;
                        pcb.total_scheduled_ticks += 1;
                        pcb.vruntime += 1;
                        return Ok(Some(curr_pid));
                    } else {
                        // Quantum expired, yield back to READY
                        pcb.remaining_quantum_ticks = 10;
                        let task_ref = TaskRunRef::new(curr_pid, pcb.priority, pcb.cpu_affinity, self.current_tick);
                        self.run_queues[cpu_idx].enqueue(task_ref, self.current_tick)?;
                        pcb.state = ProcessState::Ready;
                        self.active_pids[cpu_idx] = None;
                    }
                }
            }
        }

        // Dequeue next task from CPU run queue
        if self.run_queues[cpu_idx].is_empty() {
            self.active_pids[cpu_idx] = None;
            return Ok(None);
        }

        let task_ref = self.run_queues[cpu_idx].dequeue_next()?;
        let next_pid = task_ref.pid;

        if let Some(pcb) = self.process_table.get_mut(&next_pid) {
            if pcb.state != ProcessState::Ready {
                return Err(SchedulerError::InvalidStateTransition { from: pcb.state, to: ProcessState::Running });
            }
            pcb.state = ProcessState::Running;
            pcb.current_cpu = cpu_id;
            pcb.remaining_quantum_ticks = 10;
            self.active_pids[cpu_idx] = Some(next_pid);
            Ok(Some(next_pid))
        } else {
            Err(SchedulerError::TaskCannotBeScheduled(next_pid))
        }
    }

    /// Puts running or ready task to sleep (`RUNNING`/`READY` -> `SLEEPING`), with lost-wakeup check.
    pub fn sleep_process(&mut self, pid: u64) -> Result<(), SchedulerError> {
        enum SleepAction {
            RemainReadyReenqueue { priority: u8, affinity: u64 },
            RemainReadyInQueue,
            PutToSleep,
        }

        let action = {
            let pcb = self.process_table.get_mut(&pid).ok_or(SchedulerError::ProcessNotFound(pid))?;

            if pcb.state != ProcessState::Running && pcb.state != ProcessState::Ready {
                return Err(SchedulerError::InvalidStateTransition { from: pcb.state, to: ProcessState::Sleeping });
            }

            // Lost wakeup protection: if event pending or signal pending, remain READY
            if pcb.wake_event_pending || pcb.pending_signals != 0 {
                pcb.wake_event_pending = false;
                let is_running = pcb.state == ProcessState::Running;
                pcb.state = ProcessState::Ready;
                if is_running {
                    let cpu_idx = pcb.current_cpu as usize;
                    self.active_pids[cpu_idx] = None;
                    SleepAction::RemainReadyReenqueue { priority: pcb.priority, affinity: pcb.cpu_affinity }
                } else {
                    SleepAction::RemainReadyInQueue
                }
            } else {
                let cpu_idx = pcb.current_cpu as usize;
                if pcb.state == ProcessState::Ready {
                    let _ = self.run_queues[cpu_idx].remove_task(pid);
                } else if self.active_pids[cpu_idx] == Some(pid) {
                    self.active_pids[cpu_idx] = None;
                }

                pcb.state = ProcessState::Sleeping;
                SleepAction::PutToSleep
            }
        };

        match action {
            SleepAction::RemainReadyReenqueue { priority, affinity } => {
                let task_ref = TaskRunRef::new(pid, priority, affinity, self.current_tick);
                let target_cpu = self.select_best_cpu(affinity)?;
                self.run_queues[target_cpu as usize].enqueue(task_ref, self.current_tick)?;
            }
            SleepAction::RemainReadyInQueue | SleepAction::PutToSleep => {}
        }

        Ok(())
    }

    /// Wake up a sleeping process (`SLEEPING -> READY`) with lost-wakeup protection.
    pub fn wake_process(&mut self, pid: u64) -> Result<(), SchedulerError> {
        let pcb = self.process_table.get_mut(&pid).ok_or(SchedulerError::ProcessNotFound(pid))?;

        match pcb.state {
            ProcessState::Sleeping => {
                pcb.wake_event_pending = false;
                self.mark_ready(pid)?;
            }
            ProcessState::New | ProcessState::Ready | ProcessState::Running => {
                // Pre-mark pending event to prevent lost wakeup when it tries to sleep
                pcb.wake_event_pending = true;
            }
            _ => {}
        }
        Ok(())
    }

    /// Stops running or ready task (`RUNNING`/`READY` -> `STOPPED`).
    pub fn stop_process(&mut self, pid: u64) -> Result<(), SchedulerError> {
        let pcb = self.process_table.get_mut(&pid).ok_or(SchedulerError::ProcessNotFound(pid))?;

        if pcb.state != ProcessState::Running && pcb.state != ProcessState::Ready {
            return Err(SchedulerError::InvalidStateTransition { from: pcb.state, to: ProcessState::Stopped });
        }

        let cpu_idx = pcb.current_cpu as usize;
        if pcb.state == ProcessState::Ready {
            let _ = self.run_queues[cpu_idx].remove_task(pid);
        } else if self.active_pids[cpu_idx] == Some(pid) {
            self.active_pids[cpu_idx] = None;
        }

        pcb.state = ProcessState::Stopped;
        Ok(())
    }

    /// Terminates process (`RUNNING`/`ANY` -> `ZOMBIE`).
    pub fn exit_process(&mut self, pid: u64, exit_code: i32) -> Result<(), SchedulerError> {
        let pcb = self.process_table.get_mut(&pid).ok_or(SchedulerError::ProcessNotFound(pid))?;

        if pcb.state == ProcessState::Zombie || pcb.state == ProcessState::Reaped {
            return Ok(());
        }

        let cpu_idx = pcb.current_cpu as usize;
        if pcb.state == ProcessState::Ready {
            let _ = self.run_queues[cpu_idx].remove_task(pid);
        } else if self.active_pids[cpu_idx] == Some(pid) {
            self.active_pids[cpu_idx] = None;
        }

        pcb.state = ProcessState::Zombie;
        pcb.exit_code = exit_code;

        // Reparent orphans to PID 1 (init)
        let child_pids: Vec<u64> = self.process_table.values().filter(|p| p.ppid == pid).map(|p| p.pid).collect();
        for child_pid in child_pids {
            if let Some(child_pcb) = self.process_table.get_mut(&child_pid) {
                child_pcb.ppid = 1;
            }
        }

        Ok(())
    }

    /// Reaps a zombie process (`ZOMBIE -> REAPED`). Releases process resources.
    pub fn reap_process(&mut self, pid: u64) -> Result<i32, SchedulerError> {
        let pcb = self.process_table.get(&pid).ok_or(SchedulerError::ProcessNotFound(pid))?;

        if pcb.state != ProcessState::Zombie {
            return Err(SchedulerError::CannotReapNonZombie(pid));
        }

        let exit_code = pcb.exit_code;
        self.process_table.remove(&pid);
        Ok(exit_code)
    }

    /// Advance kernel timer tick and perform starvation boosting.
    pub fn timer_tick(&mut self) {
        self.current_tick += 1;
        for rq in self.run_queues.iter_mut() {
            rq.boost_starved_tasks(self.current_tick, self.starvation_threshold_ticks);
        }
    }

    /// Selects best CPU core matching task's affinity mask.
    fn select_best_cpu(&self, cpu_affinity: u64) -> Result<u32, SchedulerError> {
        let mut min_tasks = usize::MAX;
        let mut best_cpu = None;

        for cpu in 0..self.num_cpus {
            if (cpu_affinity & (1u64 << cpu)) != 0 {
                let load = self.run_queues[cpu as usize].len();
                if load < min_tasks {
                    min_tasks = load;
                    best_cpu = Some(cpu);
                }
            }
        }

        best_cpu.ok_or(SchedulerError::InvalidCpuAffinity)
    }
}
