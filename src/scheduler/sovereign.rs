#![no_std]
#![no_main]

use core::mem;
/// OOP-based Sovereign Scheduler for SigmaOS
/// Based on Roadmap Item: Functional Kernel Scheduler Implementation (Critical Blocker)
/// Implements MLFQ (Multi-Level Feedback Queue) and MCS (Machine-to-Core Scheduling)
use core::sync::atomic::{AtomicUsize, Ordering};

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub type ThreadID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThreadState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Sleeping = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Priority {
    High = 0,
    Normal = 1,
    Low = 2,
    Idle = 3,
}

pub trait Thread {
    fn id(&self) -> ThreadID;
    fn state(&self) -> ThreadState;
    fn priority(&self) -> Priority;
    fn set_state(&mut self, state: ThreadState);
}

#[repr(C)]
pub struct SimpleThread {
    pub id: ThreadID,
    pub state: AtomicUsize,
    pub priority: Priority,
    pub quantum: AtomicUsize,
}

impl SimpleThread {
    pub fn new(id: ThreadID, priority: Priority) -> Self {
        SimpleThread {
            id,
            state: AtomicUsize::new(ThreadState::Ready as usize),
            priority,
            quantum: AtomicUsize::new(10),
        }
    }
}

impl Thread for SimpleThread {
    fn id(&self) -> ThreadID {
        self.id
    }
    fn state(&self) -> ThreadState {
        {
            let raw = self.state.load(Ordering::SeqCst) as u32;
            match raw {
                1 => ThreadState::Running,
                2 => ThreadState::Blocked,
                3 => ThreadState::Sleeping,
                _ => ThreadState::Ready,
            }
        }
    }
    fn priority(&self) -> Priority {
        self.priority
    }
    fn set_state(&mut self, state: ThreadState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait Scheduler {
    fn add_thread(&mut self, thread: Box<dyn Thread>) -> Result<ThreadID, SchedulerError>;
    fn remove_thread(&mut self, id: ThreadID) -> Result<(), SchedulerError>;
    fn schedule(&mut self) -> Option<ThreadID>;
    fn yield_thread(&mut self, id: ThreadID) -> Result<(), SchedulerError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SchedulerError {
    Success = 0,
    ThreadNotFound = 1,
    QueueFull = 2,
}

pub struct MLFQScheduler {
    pub queues: [Vec<Option<Box<dyn Thread>>>; 4],
    pub current_queue: AtomicUsize,
}

impl MLFQScheduler {
    pub fn new() -> Self {
        MLFQScheduler {
            queues: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            current_queue: AtomicUsize::new(0),
        }
    }
}

impl Scheduler for MLFQScheduler {
    fn add_thread(&mut self, thread: Box<dyn Thread>) -> Result<ThreadID, SchedulerError> {
        let id = thread.id();
        let queue_idx = match thread.priority() {
            Priority::High => 0,
            Priority::Normal => 1,
            Priority::Low => 2,
            Priority::Idle => 3,
        };
        self.queues[queue_idx].push(Some(thread));
        Ok(id)
    }
    fn remove_thread(&mut self, id: ThreadID) -> Result<(), SchedulerError> {
        for queue in &mut self.queues {
            for thread_option in queue {
                if let Some(ref thread) = *thread_option {
                    if thread.id() == id {
                        return Ok(());
                    }
                }
            }
        }
        Err(SchedulerError::ThreadNotFound)
    }
    fn schedule(&mut self) -> Option<ThreadID> {
        for queue_idx in 0..4 {
            if !self.queues[queue_idx].is_empty() {
                self.current_queue.store(queue_idx, Ordering::SeqCst);
                if let Some(ref thread_option) = self.queues[queue_idx].get(0) {
                    if let Some(ref thread) = *thread_option {
                        return Some(thread.id());
                    }
                }
            }
        }
        None
    }
    fn yield_thread(&mut self, id: ThreadID) -> Result<(), SchedulerError> {
        let current = self.current_queue.load(Ordering::SeqCst);
        if current < 3 {
            for i in 0..self.queues[current].len() {
                if let Some(ref thread_option) = self.queues[current].get(i) {
                    if let Some(ref thread) = *thread_option {
                        if thread.id() == id {
                            let thread = self.queues[current].remove(i);
                            let next_queue = current + 1;
                            if next_queue < 4 {
                                self.queues[next_queue].push(thread);
                            }
                            return Ok(());
                        }
                    }
                }
            }
        }
        Err(SchedulerError::ThreadNotFound)
    }
}

pub trait MCSScheduler {
    fn assign_to_core(&mut self, thread_id: ThreadID, core_id: usize)
        -> Result<(), SchedulerError>;
    fn get_core_load(&self, core_id: usize) -> usize;
}

pub struct SimpleMCSScheduler {
    pub scheduler: MLFQScheduler,
    pub core_assignments: Vec<AtomicUsize>,
    pub core_loads: [AtomicUsize; 64],
}

impl SimpleMCSScheduler {
    pub fn new(num_cores: usize) -> Self {
        let mut core_assignments = Vec::new();
        for _ in 0..num_cores {
            core_assignments.push(AtomicUsize::new(0));
        }
        let core_loads = core::array::from_fn(|_| AtomicUsize::new(0));
        SimpleMCSScheduler {
            scheduler: MLFQScheduler::new(),
            core_assignments,
            core_loads,
        }
    }
}

impl MCSScheduler for SimpleMCSScheduler {
    fn assign_to_core(
        &mut self,
        thread_id: ThreadID,
        core_id: usize,
    ) -> Result<(), SchedulerError> {
        if core_id >= self.core_assignments.len() {
            return Err(SchedulerError::ThreadNotFound);
        }
        self.core_assignments[core_id].store(thread_id, Ordering::SeqCst);
        self.core_loads[core_id].fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn get_core_load(&self, core_id: usize) -> usize {
        if core_id >= 64 {
            return 0;
        }
        self.core_loads[core_id].load(Ordering::SeqCst)
    }
}

pub trait RealTimeScheduler {
    fn add_rt_thread(
        &mut self,
        thread: Box<dyn Thread>,
        deadline: usize,
    ) -> Result<ThreadID, SchedulerError>;
    fn get_next_deadline(&self) -> Option<usize>;
}

pub struct SimpleRealTimeScheduler {
    pub rt_queue: Vec<(ThreadID, usize)>,
    pub next_id: AtomicUsize,
}

impl SimpleRealTimeScheduler {
    pub fn new() -> Self {
        SimpleRealTimeScheduler {
            rt_queue: Vec::new(),
            next_id: AtomicUsize::new(1000),
        }
    }
}

impl RealTimeScheduler for SimpleRealTimeScheduler {
    fn add_rt_thread(
        &mut self,
        thread: Box<dyn Thread>,
        deadline: usize,
    ) -> Result<ThreadID, SchedulerError> {
        let id = thread.id();
        self.rt_queue.push((id, deadline));
        Ok(id)
    }
    fn get_next_deadline(&self) -> Option<usize> {
        if self.rt_queue.is_empty() {
            return None;
        }
        let mut min_deadline = self.rt_queue[0].1;
        for &(_, deadline) in &self.rt_queue {
            if deadline < min_deadline {
                min_deadline = deadline;
            }
        }
        Some(min_deadline)
    }
}


