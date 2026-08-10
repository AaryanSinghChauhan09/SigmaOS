// SigmaOS Kernel Control Unit and Policy-Mechanism Separation Subsystem
// Conforms to zero-dependency, #![no_std] compliant OOP structures

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

// 1. Core Architectural Enums

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptClass {
    Software,
    Io,
    Timer,
    InterProcessor,
    Fault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionCyclePhase {
    Fetch,
    Decode,
    Execute,
    Writeback,
    Commit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoWaitProfile {
    DiskBound,
    NetworkBound,
    DevicePolling,
    Idle,
}

// 2. Policy and Mechanism Separation Traits

/// Defines "how" low-level operations are dispatched (Zero-policy Mechanism layer)
pub trait KernelMechanism {
    fn context_switch_task(&mut self, from_task: usize, to_task: usize);
    fn force_io_wait(&mut self, wait_profile: IoWaitProfile);
    fn execute_instruction(&mut self, phase: InstructionCyclePhase);
}

/// Defines "what" scheduling and allocation rules to apply (Mechanism-free Policy layer)
pub trait KernelPolicy {
    fn decide_next_task(&self, active_tasks: &[usize], cpu_load: usize) -> usize;
    fn calculate_dynamic_priority(&self, current_priority: u8, wait_profile: IoWaitProfile) -> u8;
    fn get_max_time_slice_ms(&self, wait_profile: IoWaitProfile) -> usize;
}

// 3. Concrete Implementations

pub struct SovereignMechanism {
    pub active_task_id: usize,
    pub current_phase: InstructionCyclePhase,
    pub io_wait_count: AtomicU32,
    pub instruction_count: AtomicUsize,
}

impl SovereignMechanism {
    pub fn new() -> Self {
        Self {
            active_task_id: 0,
            current_phase: InstructionCyclePhase::Fetch,
            io_wait_count: AtomicU32::new(0),
            instruction_count: AtomicUsize::new(0),
        }
    }
}

impl KernelMechanism for SovereignMechanism {
    fn context_switch_task(&mut self, from_task: usize, to_task: usize) {
        println!(
            "[mechanism] Context Switch: Saving task #{} state -> Loading task #{} state.",
            from_task, to_task
        );
        self.active_task_id = to_task;
    }

    fn force_io_wait(&mut self, wait_profile: IoWaitProfile) {
        println!(
            "[mechanism] Thread yielding context to enter wait profile: {:?}",
            wait_profile
        );
        self.io_wait_count.fetch_add(1, Ordering::SeqCst);
    }

    fn execute_instruction(&mut self, phase: InstructionCyclePhase) {
        self.current_phase = phase;
        self.instruction_count.fetch_add(1, Ordering::SeqCst);
    }
}

pub struct AdaptivePolicy {
    pub boost_io_tasks: bool,
    pub power_save_mode: bool,
}

impl AdaptivePolicy {
    pub fn new() -> Self {
        Self {
            boost_io_tasks: true,
            power_save_mode: false,
        }
    }
}

impl KernelPolicy for AdaptivePolicy {
    fn decide_next_task(&self, active_tasks: &[usize], cpu_load: usize) -> usize {
        if active_tasks.is_empty() {
            return 0;
        }
        if self.power_save_mode && cpu_load < 30 {
            // Power save: select idle task (usually index 0)
            return active_tasks[0];
        }
        // Normal policy: select highest ID task (simple priority approximation)
        *active_tasks.last().unwrap_or(&0)
    }

    fn calculate_dynamic_priority(&self, current_priority: u8, wait_profile: IoWaitProfile) -> u8 {
        if self.boost_io_tasks && wait_profile == IoWaitProfile::DiskBound {
            // Boost dynamic priority for I/O bound threads to yield rapid response on wakeup
            return current_priority.saturating_add(4);
        }
        current_priority
    }

    fn get_max_time_slice_ms(&self, wait_profile: IoWaitProfile) -> usize {
        if wait_profile == IoWaitProfile::DevicePolling {
            // Devices in polling yield short quantum slices (2ms) to prevent CPU hogging
            2
        } else {
            // Normal quantum (10ms)
            10
        }
    }
}

// 4. Policy-Mechanism Coordinator Control Unit

pub struct PolicyMechanismCoordinator {
    pub mechanism: SovereignMechanism,
    pub policy: Box<dyn KernelPolicy>,
    // Event statistics
    pub software_irq_count: AtomicU32,
    pub io_irq_count: AtomicU32,
    pub timer_irq_count: AtomicU32,
}

impl PolicyMechanismCoordinator {
    pub fn new(policy: Box<dyn KernelPolicy>) -> Self {
        Self {
            mechanism: SovereignMechanism::new(),
            policy,
            software_irq_count: AtomicU32::new(0),
            io_irq_count: AtomicU32::new(0),
            timer_irq_count: AtomicU32::new(0),
        }
    }

    /// Dynamic control unit routing for hardware interrupts based on class grouping (Linux/BSD style)
    pub fn dispatch_interrupt_class(&self, class: InterruptClass) {
        match class {
            InterruptClass::Software => {
                self.software_irq_count.fetch_add(1, Ordering::SeqCst);
            }
            InterruptClass::Io => {
                self.io_irq_count.fetch_add(1, Ordering::SeqCst);
            }
            InterruptClass::Timer => {
                self.timer_irq_count.fetch_add(1, Ordering::SeqCst);
            }
            _ => {}
        }
        println!(
            "[control-unit] Dispatched interrupt event class: {:?}",
            class
        );
    }

    /// Triggers dynamic thread prioritization and scheduling slices by coordinating policy and mechanism
    pub fn execute_policy_schedule(&mut self, active_tasks: &[usize], cpu_load: usize) {
        let next_task = self.policy.decide_next_task(active_tasks, cpu_load);
        let current_task = self.mechanism.active_task_id;

        if next_task != current_task {
            self.mechanism.context_switch_task(current_task, next_task);
        }
    }
}
