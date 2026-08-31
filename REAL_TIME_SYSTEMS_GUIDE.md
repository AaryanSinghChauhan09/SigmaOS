# SigmaOS Real-Time Systems Guide

## Overview

This guide provides comprehensive information about using SigmaOS for real-time and embedded applications, including hard real-time capabilities, deterministic scheduling, and low-latency I/O operations.

## Table of Contents

1.  [Real-Time Capabilities](#real-time-capabilities)
2.  [Deterministic Scheduling](#deterministic-scheduling)
3.  [Memory Management for Real-Time](#memory-management-for-real-time)
4.  [Interrupt Handling](#interrupt-handling)
5.  [Low-Latency I/O](#low-latency-io)
6.  [Real-Time Configuration](#real-time-configuration)
7.  [Performance Benchmarking](#performance-benchmarking)
8.  [Case Studies](#case-studies)

## Real-Time Capabilities

### Hard Real-Time Support

SigmaOS provides hard real-time capabilities through:

*   **Preemptive Kernel**: Full kernel preemption for minimal latency
*   **Priority Inheritance**: Prevents priority inversion
*   **Deadline Monotonic Scheduling**: Optimal for periodic tasks
*   **Bounded Execution Times**: Guaranteed worst-case execution times
*   **Memory Locking**: Prevents paging delays

### Real-Time Profiles

```rust
pub enum RtProfile {
    /// Hard real-time with microsecond-level guarantees
    HardRealTime {
        max_latency_us: u64,
        max_jitter_us: u64,
    },
    /// Soft real-time with millisecond-level guarantees
    SoftRealTime {
        max_latency_ms: u64,
        target_latency_ms: u64,
    },
    /// Best-effort with quality of service
    BestEffort {
        priority_level: u8,
    },
}
```

## Deterministic Scheduling

### Multi-Level Feedback Queue (MLFQ)

SigmaOS implements an advanced MLFQ scheduler for real-time tasks:

```rust
pub struct MlfqConfig {
    pub queue_count: usize,           // Number of priority queues
    pub time_slice_base_ms: u64,      // Base time slice
    pub boost_interval_ms: u64,       // Priority boost interval
    pub preempt_threshold: u8,        // Preemption threshold
    pub real_time_queue: bool,        // Dedicated RT queue
}

impl MlfqConfig {
    pub fn for_real_time() -> Self {
        Self {
            queue_count: 16,           // More queues for finer granularity
            time_slice_base_ms: 1,     // Smaller time slices
            boost_interval_ms: 100,    // Frequent priority boosts
            preempt_threshold: 1,       // Aggressive preemption
            real_time_queue: true,      // Enable RT queue
        }
    }
}
```

### Earliest Deadline First (EDF)

For hard real-time periodic tasks:

```rust
pub struct EdfTask {
    pub task_id: u32,
    pub period_us: u64,           // Task period in microseconds
    pub deadline_us: u64,         // Absolute deadline
    pub execution_time_us: u64,   // Worst-case execution time
    pub priority: u8,
}

pub struct EdfScheduler {
    ready_queue: BinaryHeap<EdfTask>,
    current_task: Option<EdfTask>,
    timer_resolution_us: u64,
}

impl EdfScheduler {
    pub fn schedule(&mut self) -> Option<&EdfTask> {
        // Return task with earliest deadline
        self.ready_queue.peek()
    }
    
    pub fn add_task(&mut self, task: EdfTask) {
        self.ready_queue.push(task);
    }
    
    pub fn is_schedulable(&self, tasks: &[EdfTask]) -> bool {
        // Liu & Layland utilization bound test
        let total_utilization: f64 = tasks.iter()
            .map(|t| t.execution_time_us as f64 / t.period_us as f64)
            .sum();
        
        let n = tasks.len() as f64;
        let bound = n * (2_f64.powf(1.0 / n) - 1.0);
        
        total_utilization <= bound
    }
}
```

### Rate Monotonic Scheduling

For static priority assignment:

```rust
pub struct RateMonotonicScheduler {
    tasks: Vec<RmTask>,
}

pub struct RmTask {
    pub task_id: u32,
    pub period_us: u64,
    pub execution_time_us: u64,
    pub priority: u8,  // Higher priority for shorter periods
}

impl RateMonotonicScheduler {
    pub fn assign_priorities(&mut self) {
        // Sort by period (shorter period = higher priority)
        self.tasks.sort_by_key(|t| t.period_us);
        
        for (i, task) in self.tasks.iter_mut().enumerate() {
            task.priority = (self.tasks.len() - i) as u8;
        }
    }
}
```

## Memory Management for Real-Time

### Memory Pooling

Pre-allocate memory pools to prevent runtime allocation delays:

```rust
pub struct RealTimeMemoryPool {
    paged_pool: PoolBlock,
    non_paged_pool: PoolBlock,
    locked_regions: Vec<MemoryRegion>,
}

impl RealTimeMemoryPool {
    pub fn new(size: usize) -> Self {
        // Pre-allocate all memory upfront
        let non_paged = allocate_non_paged(size);
        let paged = allocate_paged(size);
        
        Self {
            paged_pool: paged,
            non_paged_pool: non_paged,
            locked_regions: Vec::new(),
        }
    }
    
    pub fn lock_region(&mut self, addr: usize, size: usize) -> Result<(), MemoryError> {
        // Lock memory region to prevent paging
        let region = MemoryRegion { addr, size, locked: true };
        self.locked_regions.push(region);
        Ok(())
    }
    
    pub fn allocate_realtime(&mut self, size: usize) -> Result<*mut u8, MemoryError> {
        // Allocate from non-paged pool for deterministic behavior
        allocate_from_pool(&mut self.non_paged_pool, size)
    }
}
```

### Memory Locking

Lock critical memory regions to prevent paging:

```rust
use sigmaos::kernel::memory::{VirtualMemoryManager, PageFlags};

impl VirtualMemoryManager {
    pub fn lock_pages(&mut self, virtual_addr: u64, num_pages: usize) -> Result<(), &'static str> {
        for i in 0..num_pages {
            let page_addr = virtual_addr + (i as u64 * PAGE_SIZE as u64);
            let flags = PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::NO_EXECUTE;
            self.map_page(page_addr, page_addr, flags)?;
        }
        Ok(())
    }
    
    pub fn mlock(&mut self, addr: u64, len: usize) -> Result<(), &'static str> {
        let num_pages = (len + PAGE_SIZE - 1) / PAGE_SIZE;
        self.lock_pages(addr, num_pages)
    }
}
```

### Deterministic Allocation

Use buddy allocator with fixed-size blocks:

```rust
pub struct DeterministicAllocator {
    block_sizes: Vec<usize>,  // Fixed block sizes
    free_lists: Vec<Vec<*mut u8>>,
}

impl DeterministicAllocator {
    pub fn new(block_sizes: Vec<usize>) -> Self {
        let free_lists = block_sizes.iter()
            .map(|_| Vec::new())
            .collect();
        
        Self {
            block_sizes,
            free_lists,
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        // Find smallest block that fits
        for (i, &block_size) in self.block_sizes.iter().enumerate() {
            if block_size >= size && !self.free_lists[i].is_empty() {
                return self.free_lists[i].pop();
            }
        }
        None
    }
    
    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        // Return to appropriate free list
        for (i, &block_size) in self.block_sizes.iter().enumerate() {
            if block_size >= size {
                self.free_lists[i].push(ptr);
                return;
            }
        }
    }
}
```

## Interrupt Handling

### Low-Latency Interrupt Processing

```rust
pub struct InterruptHandler {
    handler_id: u32,
    priority: u8,
    latency_target_us: u64,
    handler_fn: fn() -> InterruptResult,
}

pub enum InterruptResult {
    Handled,
    Deferred,
    Error,
}

impl InterruptHandler {
    pub fn handle_interrupt(&self) -> InterruptResult {
        let start = sigmaos::time::get_timestamp_us();
        
        let result = (self.handler_fn)();
        
        let elapsed = sigmaos::time::get_timestamp_us() - start;
        if elapsed > self.latency_target_us {
            log_warning!("Interrupt handler exceeded latency target: {}us", elapsed);
        }
        
        result
    }
}
```

### Interrupt Prioritization

```rust
pub struct InterruptController {
    handlers: [Option<InterruptHandler>; 256],
    priority_levels: [u8; 256],
}

impl InterruptController {
    pub fn register_handler(&mut self, irq: u8, handler: InterruptHandler, priority: u8) {
        self.handlers[irq as usize] = Some(handler);
        self.priority_levels[irq as usize] = priority;
    }
    
    pub fn handle_interrupt(&mut self, irq: u8) {
        if let Some(ref handler) = self.handlers[irq as usize] {
            handler.handle_interrupt();
        }
    }
    
    pub fn set_priority(&mut self, irq: u8, priority: u8) {
        self.priority_levels[irq as usize] = priority;
    }
}
```

### Nested Interrupts

```rust
pub struct NestedInterruptController {
    current_priority: u8,
    max_nesting_level: u8,
    current_nesting: u8,
}

impl NestedInterruptController {
    pub fn can_handle_interrupt(&self, irq_priority: u8) -> bool {
        irq_priority > self.current_priority && 
        self.current_nesting < self.max_nesting_level
    }
    
    pub fn enter_interrupt(&mut self, priority: u8) {
        self.current_priority = priority;
        self.current_nesting += 1;
    }
    
    pub fn exit_interrupt(&mut self) {
        self.current_nesting -= 1;
        self.current_priority = 0;
    }
}
```

## Low-Latency I/O

### Zero-Copy I/O

```rust
pub trait ZeroCopyIo {
    fn read_zero_copy(&mut self, buffer: &mut [u8]) -> Result<usize, IoError>;
    fn write_zero_copy(&mut self, data: &[u8]) -> Result<usize, IoError>;
    fn get_dma_buffer(&mut self) -> Option<&mut [u8]>;
}

pub struct ZeroCopyDevice {
    dma_buffer: *mut u8,
    buffer_size: usize,
}

impl ZeroCopyIo for ZeroCopyDevice {
    fn read_zero_copy(&mut self, buffer: &mut [u8]) -> Result<usize, IoError> {
        // Direct DMA transfer to buffer
        let len = buffer.len().min(self.buffer_size);
        unsafe {
            core::ptr::copy_nonoverlapping(self.dma_buffer, buffer.as_mut_ptr(), len);
        }
        Ok(len)
    }
    
    fn write_zero_copy(&mut self, data: &[u8]) -> Result<usize, IoError> {
        // Direct DMA transfer from data
        let len = data.len().min(self.buffer_size);
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.dma_buffer, len);
        }
        Ok(len)
    }
    
    fn get_dma_buffer(&mut self) -> Option<&mut [u8]> {
        unsafe {
            Some(core::slice::from_raw_parts_mut(self.dma_buffer, self.buffer_size))
        }
    }
}
```

### Polling vs Interrupt-Driven

```rust
pub enum IoMode {
    Polling {
        interval_us: u64,
    },
    InterruptDriven {
        irq: u8,
        priority: u8,
    },
    Hybrid {
        primary: IoMode,
        fallback: IoMode,
    },
}

pub struct RealTimeIo {
    mode: IoMode,
    device: ZeroCopyDevice,
}

impl RealTimeIo {
    pub fn process_io(&mut self) -> Result<usize, IoError> {
        match self.mode {
            IoMode::Polling { interval_us } => {
                self.poll_with_interval(interval_us)
            }
            IoMode::InterruptDriven { .. } => {
                // Handled via interrupt handler
                Ok(0)
            }
            IoMode::Hybrid { .. } => {
                // Try interrupt first, fall back to polling
                self.try_interrupt_then_poll()
            }
        }
    }
}
```

### Real-Time Networking

```rust
pub struct RealTimeNetworkStack {
    latency_target_us: u64,
    jitter_target_us: u64,
    priority_queue: PriorityQueue<NetworkPacket>,
}

impl RealTimeNetworkStack {
    pub fn send_realtime_packet(&mut self, packet: NetworkPacket) -> Result<(), NetworkError> {
        let timestamp = sigmaos::time::get_timestamp_us();
        
        // Add timestamp for latency measurement
        let rt_packet = packet.with_timestamp(timestamp);
        
        self.priority_queue.push(rt_packet);
        Ok(())
    }
    
    pub fn receive_realtime_packet(&mut self) -> Option<NetworkPacket> {
        let packet = self.priority_queue.pop()?;
        
        let current_time = sigmaos::time::get_timestamp_us();
        let latency = current_time - packet.timestamp();
        
        if latency > self.latency_target_us {
            log_warning!("Packet exceeded latency target: {}us", latency);
        }
        
        Some(packet)
    }
}
```

## Real-Time Configuration

### Kernel Configuration

```toml
# sigma-core.toml for real-time systems
[profile]
name = "rtos"
target = "x86_64-unknown-none"

[realtime]
enabled = true
preempt_kernel = true
tick_rate_hz = 10000
timer_resolution_us = 100

[scheduler]
algorithm = "edf+mlfq"
real_time_queue = true
priority_levels = 16
time_slice_ms = 1

[memory]
lock_all_rt_memory = true
enable_thp = false
pool_preallocation = true

[interrupts]
nesting_enabled = true
max_nesting_level = 8
priority_levels = 256
```

### Build Configuration

```bash
# Build real-time kernel
cargo build --profile rtos --features "realtime,preempt_rt"

# Optimized for latency
RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C codegen-units=1" cargo build --profile rtos
```

### Runtime Configuration

```rust
pub struct RtSystemConfig {
    pub max_latency_us: u64,
    pub max_jitter_us: u64,
    pub cpu_affinity: Vec<usize>,
    pub memory_locked: bool,
    pub interrupt_priority: u8,
}

impl RtSystemConfig {
    pub fn for_audio_processing() -> Self {
        Self {
            max_latency_us: 100,      // 100 microseconds
            max_jitter_us: 50,        // 50 microseconds
            cpu_affinity: vec![0, 1], // Use first 2 cores
            memory_locked: true,
            interrupt_priority: 15,   // Highest priority
        }
    }
    
    pub fn for_robotics() -> Self {
        Self {
            max_latency_us: 1000,     // 1 millisecond
            max_jitter_us: 200,       // 200 microseconds
            cpu_affinity: vec![0],    // Single core
            memory_locked: true,
            interrupt_priority: 14,
        }
    }
}
```

## Performance Benchmarking

### Latency Measurement

```rust
pub struct LatencyBenchmark {
    samples: Vec<u64>,
    target_us: u64,
}

impl LatencyBenchmark {
    pub fn new(target_us: u64) -> Self {
        Self {
            samples: Vec::new(),
            target_us,
        }
    }
    
    pub fn measure<F: FnOnce()>(&mut self, f: F) {
        let start = sigmaos::time::get_timestamp_us();
        f();
        let elapsed = sigmaos::time::get_timestamp_us() - start;
        self.samples.push(elapsed);
    }
    
    pub fn get_statistics(&self) -> LatencyStats {
        let max = *self.samples.iter().max().unwrap_or(&0);
        let min = *self.samples.iter().min().unwrap_or(&0);
        let avg: u64 = self.samples.iter().sum::<u64>() / self.samples.len() as u64;
        
        let mut sorted = self.samples.clone();
        sorted.sort();
        let percentile_99 = sorted[(sorted.len() * 99 / 100).min(sorted.len() - 1)];
        
        LatencyStats {
            max,
            min,
            avg,
            percentile_99,
            target: self.target_us,
        }
    }
}

pub struct LatencyStats {
    pub max: u64,
    pub min: u64,
    pub avg: u64,
    pub percentile_99: u64,
    pub target: u64,
}
```

### Jitter Measurement

```rust
pub struct JitterAnalyzer {
    expected_interval_us: u64,
    actual_intervals: Vec<u64>,
}

impl JitterAnalyzer {
    pub fn record_interval(&mut self, actual_us: u64) {
        self.actual_intervals.push(actual_us);
    }
    
    pub fn analyze(&self) -> JitterStats {
        let deviations: Vec<i64> = self.actual_intervals.iter()
            .map(|&actual| (actual as i64 - self.expected_interval_us as i64).abs())
            .collect();
        
        let max_jitter = *deviations.iter().max().unwrap_or(&0) as u64;
        let avg_jitter: i64 = deviations.iter().sum::<i64>() / deviations.len() as i64;
        
        JitterStats {
            max_jitter,
            avg_jitter: avg_jitter as u64,
            expected_interval: self.expected_interval_us,
        }
    }
}

pub struct JitterStats {
    pub max_jitter: u64,
    pub avg_jitter: u64,
    pub expected_interval: u64,
}
```

### Schedulability Analysis

```rust
pub struct SchedulabilityAnalyzer;

impl SchedulabilityAnalyzer {
    pub fn analyze_edf(tasks: &[EdfTask]) -> SchedulabilityResult {
        // Check total CPU utilization
        let total_utilization: f64 = tasks.iter()
            .map(|t| t.execution_time_us as f64 / t.period_us as f64)
            .sum();
        
        if total_utilization > 1.0 {
            return SchedulabilityResult::NotSchedulable {
                reason: "Total utilization exceeds 100%".to_string(),
                utilization: total_utilization,
            };
        }
        
        // Check for deadline misses
        for task in tasks {
            if task.execution_time_us > task.deadline_us {
                return SchedulabilityResult::NotSchedulable {
                    reason: format!("Task {} execution time exceeds deadline", task.task_id),
                    utilization: total_utilization,
                };
            }
        }
        
        SchedulabilityResult::Schedulable {
            utilization: total_utilization,
            slack: 1.0 - total_utilization,
        }
    }
}

pub enum SchedulabilityResult {
    Schedulable {
        utilization: f64,
        slack: f64,
    },
    NotSchedulable {
        reason: String,
        utilization: f64,
    },
}
```

## Case Studies

### Audio Processing

```rust
pub struct AudioProcessingSystem {
    sample_rate: u32,
    buffer_size: usize,
    latency_target_us: u64,
}

impl AudioProcessingSystem {
    pub fn process_audio_buffer(&mut self, input: &[f32], output: &mut [f32]) {
        let start = sigmaos::time::get_timestamp_us();
        
        // Real-time audio processing
        for (i, &sample) in input.iter().enumerate() {
            output[i] = self.apply_filter(sample);
        }
        
        let elapsed = sigmaos::time::get_timestamp_us() - start;
        if elapsed > self.latency_target_us {
            log_warning!("Audio processing exceeded latency target");
        }
    }
    
    fn apply_filter(&self, sample: f32) -> f32 {
        // Apply audio filter
        sample * 0.9
    }
}
```

### Industrial Control

```rust
pub struct IndustrialController {
    control_loop_period_us: u64,
    sensor_inputs: Vec<f32>,
    actuator_outputs: Vec<f32>,
}

impl IndustrialController {
    pub fn run_control_loop(&mut self) {
        loop {
            let start = sigmaos::time::get_timestamp_us();
            
            // Read sensors
            self.read_sensors();
            
            // Compute control algorithm
            self.compute_control();
            
            // Update actuators
            self.update_actuators();
            
            let elapsed = sigmaos::time::get_timestamp_us() - start;
            let remaining = self.control_loop_period_us.saturating_sub(elapsed);
            
            if remaining > 0 {
                sigmaos::time::sleep_us(remaining);
            } else {
                log_error!("Control loop exceeded period");
            }
        }
    }
}
```

### Robotics

```rust
pub struct RobotController {
    motion_update_period_us: u64,
    sensor_fusion_period_us: u64,
}

impl RobotController {
    pub fn update_motion(&mut self) {
        // High-priority motion control
        let start = sigmaos::time::get_timestamp_us();
        
        self.read_encoders();
        self.compute_pid_control();
        self.update_motor_outputs();
        
        let elapsed = sigmaos::time::get_timestamp_us() - start;
        self.monitor_timing("motion", elapsed, self.motion_update_period_us);
    }
    
    pub fn sensor_fusion(&mut self) {
        // Medium-priority sensor fusion
        let start = sigmaos::time::get_timestamp_us();
        
        self.read_imu();
        self.read_gps();
        self.fuse_sensors();
        
        let elapsed = sigmaos::time::get_timestamp_us() - start;
        self.monitor_timing("sensor_fusion", elapsed, self.sensor_fusion_period_us);
    }
    
    fn monitor_timing(&self, operation: &str, elapsed: u64, target: u64) {
        if elapsed > target {
            log_warning!("{} exceeded timing: {}us > {}us", operation, elapsed, target);
        }
    }
}
```

## Troubleshooting

### Common Real-Time Issues

1.  **Missed Deadlines**
    *   Check CPU utilization
    *   Verify interrupt priorities
    *   Review memory locking
    *   Profile execution times

2.  **High Jitter**
    *   Disable background processes
    *   Use CPU affinity
    *   Lock memory regions
    *   Optimize interrupt handlers

3.  **Priority Inversion**
    *   Enable priority inheritance
    *   Use proper mutex implementation
    *   Avoid long critical sections

### Debugging Tools

```rust
pub struct RtDebugger {
    trace_buffer: Vec<RtEvent>,
    max_events: usize,
}

pub struct RtEvent {
    timestamp: u64,
    event_type: RtEventType,
    task_id: u32,
    data: u64,
}

pub enum RtEventType {
    TaskSwitch,
    InterruptEntry,
    InterruptExit,
    DeadlineMiss,
    PriorityInversion,
}

impl RtDebugger {
    pub fn log_event(&mut self, event: RtEvent) {
        self.trace_buffer.push(event);
        if self.trace_buffer.len() > self.max_events {
            self.trace_buffer.remove(0);
        }
    }
    
    pub fn analyze_deadline_misses(&self) -> Vec<&RtEvent> {
        self.trace_buffer.iter()
            .filter(|e| matches!(e.event_type, RtEventType::DeadlineMiss))
            .collect()
    }
}
```

## Resources

*   [Kernel Customization Guide](KERNEL_CUSTOMIZATION_GUIDE)
*   [Device Driver Guide](DEVICE_DRIVER_GUIDE)
*   [Performance Tuning Guide](PERFORMANCE_TUNING_GUIDE)
*   [API Reference](API_REFERENCE)

## Contributing

When contributing real-time features:

1.  Provide worst-case execution time analysis
2.  Include schedulability analysis
3.  Add latency benchmarks
4.  Document priority assignments
5.  Test under worst-case conditions

## License

Copyright © 2026 SigmaOS Project. Licensed under MIT License.
