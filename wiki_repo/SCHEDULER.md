# SigmaOS Scheduler

## Overview

SigmaOS implements a round-robin scheduler for CPU time management. This document describes the scheduler architecture and implementation.

## Scheduler Architecture

### Round-Robin Scheduler

**Location**: `kernel/scheduler/round_robin_scheduler.rs`

**Features**:
- Fair time sharing among processes
- Configurable time slice quantum
- Task queue management
- Context switching
- RDTSC-based timestamps

**Data Structures**:

```rust
pub struct TaskControlBlock {
    pub tid: u64,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub context: TaskContext,
    pub time_slice: u64,
    pub total_runtime: u64,
    pub last_run: u64,
    pub next: Option<u64>,
    pub prev: Option<u64>,
}

pub struct TaskState {
    pub running: bool,
    pub ready: bool,
    pub blocked: bool,
}

pub struct TaskPriority {
    pub level: u8,
    pub static_prio: u8,
    pub dynamic_prio: u8,
}

pub struct TaskContext {
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub cr3: u64,
}
```

**API**:

```rust
pub unsafe fn sigma_sched_init() -> i32;
pub unsafe fn sigma_sched_add_task(tcb: *const TaskControlBlock) -> i32;
pub unsafe fn sigma_sched_remove_task(tid: u64) -> i32;
pub unsafe fn sigma_sched_tick() -> u64;
pub unsafe fn sigma_sched_yield() -> u64;
pub unsafe fn sigma_sched_get_current() -> u64;
pub unsafe fn sigma_sched_get_count() -> usize;
pub unsafe fn sigma_sched_get_switches() -> u64;
```

## Scheduling Algorithm

### Round-Robin Algorithm

1. **Initialization**: Create empty task queue
2. **Add Task**: Add task to end of queue
3. **Schedule Tick**:
   - Decrement current task's time slice
   - If time slice expired, schedule next task
4. **Schedule Next**:
   - Move current task to end of queue
   - Get next task from head
   - Switch context to new task
5. **Yield**: Voluntarily give up CPU time

### Time Slice Management

- **Default**: 10ms quantum
- **Configurable**: Can be adjusted per system
- **Per-Task**: Each task has its own time slice counter

### Task States

- **Running**: Task currently executing on CPU
- **Ready**: Task waiting for CPU time
- **Blocked**: Task waiting for I/O or resource

## Context Switching

### Context Save

When switching away from a task:
1. Save general-purpose registers
2. Save stack pointer
4. Save instruction pointer
5. Save flags
6. Save page table base (CR3)

### Context Restore

When switching to a task:
1. Restore page table base (CR3)
2. Restore flags
3. Restore instruction pointer
4. Restore stack pointer
5. Restore general-purpose registers

### Switch Assembly

```asm
; Save current context
push rax
push rbx
push rcx
push rdx
push rsi
push rdi
push rbp
push r8
push r9
push r10
push r11
push r12
push r13
push r14
push r15
pushfq

; Switch to new task
mov [old_rsp], rsp
mov rsp, [new_rsp]

; Restore new context
popfq
pop r15
pop r14
pop r13
pop r12
pop r11
pop r10
pop r9
pop r8
pop rbp
pop rdi
pop rsi
pop rdx
pop rcx
pop rbx
pop rax
```

## Timer Integration

### APIC Timer

The scheduler uses the APIC timer for periodic interrupts:

```rust
// Initialize APIC timer
pub unsafe fn sigma_apic_timer_init(frequency: u64) -> i32 {
    // Configure timer for periodic interrupts
    // Set interrupt vector
    // Set divisor
}
```

### Timer Interrupt Handler

```rust
pub unsafe fn timer_interrupt_handler() {
    // Call scheduler tick
    let next_task = sigma_sched_tick();

    // If task switch needed
    if next_task != 0 {
        context_switch(next_task);
    }
}
```

## Priority Support

### Static Priority

Each task has a static priority level:
- **0-31**: Real-time tasks
- **32-63**: Interactive tasks
- **64-95**: Normal tasks
- **96-127**: Background tasks

### Dynamic Priority

Dynamic priority adjusts based on:
- CPU usage
- I/O wait time
- Nice value

### Priority Inheritance

To prevent priority inversion:
- High-priority task waiting on low-priority task
- Temporarily boost low-priority task's priority
- Restore priority after resource release

## Scheduler Statistics

### Tracking Metrics

```rust
pub unsafe fn sigma_sched_get_switches() -> u64;
pub unsafe fn sigma_sched_get_count() -> usize;
```

### Statistics Output

```rust
pub unsafe fn print_stats(&mut self, buf: &mut [u8]) -> usize {
    // Format: "Tasks: X Switches: Y Quantum: Z"
}
```

## Future Enhancements

### Planned Features

1. **Multi-level feedback queue**: Better interactive performance
2. **Real-time scheduler**: For real-time tasks
3. **CPU affinity**: Bind tasks to specific CPUs
4. **Load balancing**: Distribute tasks across CPUs
5. **Power management**: CPU frequency scaling

### Research Areas

1. **Predictive scheduling**: AI-based task placement
2. **Energy-aware scheduling**: Minimize power consumption
3. **Heterogeneous scheduling**: Big.LITTLE CPU support
4. **Deadline scheduling**: Real-time deadline guarantees

## Best Practices

### For Kernel Developers

1. Keep critical sections short
2. Avoid holding locks across context switches
3. Use appropriate time slice values
4. Consider priority when designing interfaces
5. Test with various workloads

### For Userland Developers

1. Use yield() when waiting for I/O
2. Set appropriate priorities
3. Avoid busy-wait loops
4. Profile CPU usage
5. Consider batch processing for CPU-intensive tasks

## Troubleshooting

### High Context Switch Rate

**Symptoms**: Poor performance, high overhead

**Solutions**:
1. Increase time slice quantum
2. Reduce number of tasks
3. Check for busy-wait loops
4. Profile scheduler behavior

### Starvation

**Symptoms**: Task never gets CPU time

**Solutions**:
1. Check priority settings
2. Verify task is not blocked
3. Review scheduling algorithm
4. Implement priority boosting

### Latency Issues

**Symptoms**: Poor interactive response

**Solutions**:
1. Reduce time slice for interactive tasks
2. Implement priority boosting
3. Use multi-level feedback queue
4. Profile interrupt latency

## References

- [Round-Robin Scheduling](https://en.wikipedia.org/wiki/Round-robin_scheduling)
- [OSDev Scheduler](https://wiki.osdev.org/Scheduling_Algorithms)
- [Linux CFS Scheduler](https://www.kernel.org/doc/html/latest/scheduler/sched-design-CFS.html)
