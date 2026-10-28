// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - StarBSD minimal RTOS persona (sub-10MB footprint)
// Principles: MINIMAL KERNEL PHILOSOPHY, MEMORY DISCIPLINE, KEEP IT SIMPLE STUPID

use alloc::{vec::Vec, string::String};

const MAX_MEMORY_MB: u32 = 10;

/// RTOS configuration — enforces sub-10MB footprint
#[derive(Debug, Clone)]
pub struct RtosConfig {
    pub max_mem_mb: u32,
    pub task_slots: u8,
    pub headless: bool,
    pub tick_hz: u32,
}

impl Default for RtosConfig {
    fn default() -> Self {
        Self { max_mem_mb: 8, task_slots: 32, headless: true, tick_hz: 1000 }
    }
}

impl RtosConfig {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.max_mem_mb > MAX_MEMORY_MB { return Err("exceeds 10MB footprint limit"); }
        if self.task_slots == 0 { return Err("need at least 1 task slot"); }
        Ok(())
    }
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority { Idle = 0, Low = 1, Normal = 2, High = 3, RealTime = 4 }

/// RTOS task descriptor
#[derive(Debug, Clone)]
pub struct RtosTask {
    pub id: u8,
    pub name: String,
    pub priority: TaskPriority,
    pub stack_size: u32,
    pub entry_fn: u64,   // function pointer as u64 (no_std safe)
    pub state: TaskState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState { Ready, Running, Blocked, Terminated }

impl RtosTask {
    pub fn new(id: u8, name: &str, priority: TaskPriority, stack_size: u32, entry_fn: u64) -> Self {
        Self { id, name: name.into(), priority, stack_size, entry_fn, state: TaskState::Ready }
    }
}

/// Memory footprint guard — tracks allocations
#[derive(Debug, Default)]
pub struct MemFootprintGuard {
    pub allocated_bytes: usize,
    pub limit_bytes: usize,
}

impl MemFootprintGuard {
    pub fn new(limit_mb: u32) -> Self {
        Self { allocated_bytes: 0, limit_bytes: (limit_mb as usize) * 1024 * 1024 }
    }

    pub fn track_alloc(&mut self, bytes: usize) -> Result<(), &'static str> {
        if self.allocated_bytes + bytes > self.limit_bytes {
            return Err("memory footprint would exceed limit");
        }
        self.allocated_bytes += bytes;
        Ok(())
    }

    pub fn track_free(&mut self, bytes: usize) {
        self.allocated_bytes = self.allocated_bytes.saturating_sub(bytes);
    }

    pub fn utilization_pct(&self) -> u8 {
        if self.limit_bytes == 0 { return 100; }
        ((self.allocated_bytes * 100) / self.limit_bytes).min(100) as u8
    }
}

/// StarBSD RTOS kernel
pub struct StarRtosKernel {
    pub config: RtosConfig,
    pub tasks: Vec<RtosTask>,
    pub current_task: Option<u8>,
    pub tick_count: u64,
    pub footprint: MemFootprintGuard,
    pub booted: bool,
}

impl StarRtosKernel {
    pub fn new(config: RtosConfig) -> Result<Self, &'static str> {
        config.validate()?;
        let limit = config.max_mem_mb;
        Ok(Self {
            footprint: MemFootprintGuard::new(limit),
            config,
            tasks: Vec::new(),
            current_task: None,
            tick_count: 0,
            booted: false,
        })
    }

    pub fn boot(&mut self) -> Result<(), &'static str> {
        // Track kernel base footprint (~512KB)
        self.footprint.track_alloc(512 * 1024)?;
        self.booted = true;
        Ok(())
    }

    pub fn create_task(&mut self, name: &str, priority: TaskPriority, stack_size: u32, entry_fn: u64) -> Result<u8, &'static str> {
        if self.tasks.len() >= self.config.task_slots as usize {
            return Err("task slot limit reached");
        }
        self.footprint.track_alloc(stack_size as usize)?;
        let id = self.tasks.len() as u8;
        self.tasks.push(RtosTask::new(id, name, priority, stack_size, entry_fn));
        Ok(id)
    }

    pub fn schedule_next(&mut self) -> Option<u8> {
        // Simple priority-based scheduler — pick highest-priority ready task
        let next = self.tasks.iter()
            .filter(|t| t.state == TaskState::Ready)
            .max_by_key(|t| t.priority)
            .map(|t| t.id);
        self.current_task = next;
        next
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.schedule_next();
    }

    pub fn idle(&self) {
        // In a real RTOS: execute WFI instruction
    }

    pub fn terminate_task(&mut self, id: u8) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            self.footprint.track_free(task.stack_size as usize);
            task.state = TaskState::Terminated;
        }
    }

    pub fn memory_used_kb(&self) -> usize { self.footprint.allocated_bytes / 1024 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_exceeds_limit() {
        let cfg = RtosConfig { max_mem_mb: 11, ..Default::default() };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_boot_within_10mb() {
        let cfg = RtosConfig::default();
        let mut kernel = StarRtosKernel::new(cfg).unwrap();
        kernel.boot().unwrap();
        assert!(kernel.booted);
        assert!(kernel.memory_used_kb() < 10 * 1024);
    }

    #[test]
    fn test_create_and_schedule_tasks() {
        let cfg = RtosConfig { max_mem_mb: 4, task_slots: 8, headless: true, tick_hz: 100 };
        let mut k = StarRtosKernel::new(cfg).unwrap();
        k.boot().unwrap();
        let t1 = k.create_task("idle", TaskPriority::Idle, 4096, 0xDEAD_0000).unwrap();
        let t2 = k.create_task("sensor", TaskPriority::High, 8192, 0xDEAD_0001).unwrap();
        let next = k.schedule_next();
        assert_eq!(next, Some(t2)); // high priority wins
    }

    #[test]
    fn test_footprint_guard() {
        let mut guard = MemFootprintGuard::new(1); // 1MB limit
        guard.track_alloc(512 * 1024).unwrap();
        assert!(guard.track_alloc(600 * 1024).is_err()); // would exceed 1MB
    }
}
