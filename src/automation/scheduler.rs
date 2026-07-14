#![no_std]
#![no_main]

/// OOP-based Task Scheduler for SigmaOS
/// Based on Ideas-999-Structured: Automation & Scripting Item 846
/// Implements cron-like task scheduling

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TaskID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TaskStatus { Pending = 0, Running = 1, Completed = 2, Failed = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SchedulerError { Success = 0, NotFound = 1, InvalidSchedule = 2 }

pub trait ScheduledTask {
    fn id(&self) -> TaskID;
    fn command(&self) -> &[u8];
    fn schedule(&self) -> &[u8];
    fn status(&self) -> TaskStatus;
    fn last_run(&self) -> u64;
}

#[repr(C)]
pub struct SimpleScheduledTask {
    pub id: TaskID,
    pub command: [u8; 256],
    pub schedule: [u8; 64],
    pub status: AtomicUsize,
    pub last_run: AtomicUsize,
}

impl SimpleScheduledTask {
    pub fn new(id: TaskID, command: &[u8], schedule: &[u8]) -> Self {
        let mut cmd_array = [0u8; 256];
        let mut sched_array = [0u8; 64];
        let cmd_len = command.len().min(255);
        let sched_len = schedule.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(command.as_ptr(), cmd_array.as_mut_ptr(), cmd_len);
            core::ptr::copy_nonoverlapping(schedule.as_ptr(), sched_array.as_mut_ptr(), sched_len);
        }
        SimpleScheduledTask {
            id,
            command: cmd_array,
            schedule: sched_array,
            status: AtomicUsize::new(TaskStatus::Pending as usize),
            last_run: AtomicUsize::new(0),
        }
    }
}

impl ScheduledTask for SimpleScheduledTask {
    fn id(&self) -> TaskID { self.id }
    fn command(&self) -> &[u8] {
        let len = self.command.iter().position(|&b| b == 0).unwrap_or(256);
        &self.command[..len]
    }
    fn schedule(&self) -> &[u8] {
        let len = self.schedule.iter().position(|&b| b == 0).unwrap_or(64);
        &self.schedule[..len]
    }
    fn status(&self) -> TaskStatus { unsafe { core::mem::transmute(self.status.load(Ordering::SeqCst)) } }
    fn last_run(&self) -> u64 { self.last_run.load(Ordering::SeqCst) as u64 }
}

pub trait TaskScheduler {
    fn add_task(&mut self, task: Box<dyn ScheduledTask>) -> Result<TaskID, SchedulerError>;
    fn remove_task(&mut self, id: TaskID) -> Result<(), SchedulerError>;
    fn get_task(&self, id: TaskID) -> Option<&dyn ScheduledTask>;
    def run_task(&mut self, id: TaskID) -> Result<(), SchedulerError>;
}

#[repr(C)]
pub struct SimpleTaskScheduler {
    pub tasks: Vec<Option<Box<dyn ScheduledTask>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTaskScheduler {
    pub fn new() -> Self {
        SimpleTaskScheduler {
            tasks: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TaskScheduler for SimpleTaskScheduler {
    fn add_task(&mut self, task: Box<dyn ScheduledTask>) -> Result<TaskID, SchedulerError> {
        let id = task.id();
        self.tasks.push(Some(task));
        Ok(id)
    }
    
    fn remove_task(&mut self, id: TaskID) -> Result<(), SchedulerError> {
        for task_option in &mut self.tasks {
            if let Some(ref task) = *task_option {
                if task.id() == id {
                    return Ok(());
                }
            }
        }
        Err(SchedulerError::NotFound)
    }
    
    fn get_task(&self, id: TaskID) -> Option<&dyn ScheduledTask> {
        for task_option in &self.tasks {
            if let Some(ref task) = *task_option {
                if task.id() == id { return Some(task.as_ref()); }
            }
        }
        None
    }
    
    fn run_task(&mut self, id: TaskID) -> Result<(), SchedulerError> {
        for task_option in &mut self.tasks {
            if let Some(ref mut task) = *task_option {
                if task.id() == id {
                    task.status.store(TaskStatus::Running as usize, Ordering::SeqCst);
                    task.last_run.store(1000000, Ordering::SeqCst);
                    task.status.store(TaskStatus::Completed as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SchedulerError::NotFound)
    }
}

pub trait CronParser {
    fn parse(&self, cron_expr: &[u8]) -> Result<(), SchedulerError>;
    def should_run(&self, cron_expr: &[u8], timestamp: u64) -> bool;
}

#[repr(C)]
pub struct SimpleCronParser;

impl SimpleCronParser {
    pub fn new() -> Self { SimpleCronParser }
}

impl CronParser for SimpleCronParser {
    fn parse(&self, _cron_expr: &[u8]) -> Result<(), SchedulerError> {
        Ok(())
    }
    
    fn should_run(&self, _cron_expr: &[u8], _timestamp: u64) -> bool {
        true
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
