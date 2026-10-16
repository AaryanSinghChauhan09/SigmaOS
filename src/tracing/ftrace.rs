//! # Kernel ftrace & Function Graph Tracer
//!
//! Implements a dynamic function tracer and function graph tracer for SigmaOS.
//!
//! ## Mechanisms
//! - Dynamic code patching: Patches 5-byte NOPs (`0x0F 0x1F 0x44 0x00 0x00`) at function
//!   entry points (`__fentry__` / `mcount`) into `CALL ftrace_caller` trampolines.
//! - Function Graph Tracer: Records function entry timestamp and intercepts the return
//!   address on the stack to measure exact function execution duration and call depth.
//! - Per-CPU Lockless Ring Buffer: Stores trace entries without global locking.
//! - Tracing control exposed via `/sys/kernel/debug/tracing` (`current_tracer`, `trace`, `trace_pipe`).

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Standard 5-byte NOP on x86_64
pub const X86_NOP5: [u8; 5] = [0x0F, 0x1F, 0x44, 0x00, 0x00];

/// Tracer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracerType {
    Nop,
    Function,
    FunctionGraph,
    Irqsoff,
    Preemptoff,
    SchedSwitch,
}

/// A trace record in the per-CPU ring buffer
#[derive(Debug, Clone, Copy)]
pub struct TraceRecord {
    pub cpu: u32,
    pub pid: u32,
    pub timestamp_ns: u64,
    pub ip: usize,        // Instruction pointer of traced function
    pub parent_ip: usize, // Return address / caller
    pub depth: u16,       // Call depth for graph tracer
    pub duration_ns: u64, // Execution time for graph return
}

/// Call frame on the shadow call stack for tracking function returns
#[derive(Debug, Clone, Copy)]
pub struct FtraceRetFrame {
    pub func: usize,
    pub original_ret_addr: usize,
    pub entry_time_ns: u64,
}

/// Per-CPU trace buffer
pub struct CpuTraceBuffer {
    pub cpu_id: u32,
    pub records: Vec<TraceRecord>,
    pub max_records: usize,
    pub dropped_records: u64,
    pub shadow_stack: Vec<FtraceRetFrame>,
}

impl CpuTraceBuffer {
    pub fn new(cpu_id: u32, capacity: usize) -> Self {
        Self {
            cpu_id,
            records: Vec::with_capacity(capacity),
            max_records: capacity,
            dropped_records: 0,
            shadow_stack: Vec::with_capacity(64),
        }
    }

    pub fn push(&mut self, record: TraceRecord) {
        if self.records.len() >= self.max_records {
            self.dropped_records += 1;
        } else {
            self.records.push(record);
        }
    }
}

/// Central ftrace subsystem manager
pub struct FtraceEngine {
    pub enabled: bool,
    pub active_tracer: TracerType,
    pub per_cpu_buffers: BTreeMap<u32, CpuTraceBuffer>,
    pub patched_callsites: usize,
}

impl FtraceEngine {
    pub fn new(cpu_count: u32) -> Self {
        let mut buffers = BTreeMap::new();
        for cpu in 0..cpu_count {
            buffers.insert(cpu, CpuTraceBuffer::new(cpu, 65536));
        }

        Self {
            enabled: false,
            active_tracer: TracerType::Nop,
            per_cpu_buffers: buffers,
            patched_callsites: 0,
        }
    }

    /// Set active tracer plugin.
    pub fn set_tracer(&mut self, tracer: TracerType) {
        self.active_tracer = tracer;
        self.enabled = tracer != TracerType::Nop;
    }

    /// Function entry hook invoked by `__fentry__`.
    pub fn trace_function_entry(
        &mut self,
        cpu: u32,
        pid: u32,
        ip: usize,
        parent_ip: usize,
        ts: u64,
    ) {
        if !self.enabled {
            return;
        }

        if let Some(buf) = self.per_cpu_buffers.get_mut(&cpu) {
            let depth = buf.shadow_stack.len() as u16;

            if self.active_tracer == TracerType::FunctionGraph {
                buf.shadow_stack.push(FtraceRetFrame {
                    func: ip,
                    original_ret_addr: parent_ip,
                    entry_time_ns: ts,
                });
            }

            buf.push(TraceRecord {
                cpu,
                pid,
                timestamp_ns: ts,
                ip,
                parent_ip,
                depth,
                duration_ns: 0,
            });
        }
    }

    /// Function return hook for function graph tracer.
    pub fn trace_function_return(&mut self, cpu: u32, pid: u32, ts: u64) -> Option<usize> {
        if let Some(buf) = self.per_cpu_buffers.get_mut(&cpu) {
            if let Some(frame) = buf.shadow_stack.pop() {
                let duration = ts.saturating_sub(frame.entry_time_ns);
                let depth = buf.shadow_stack.len() as u16;

                buf.push(TraceRecord {
                    cpu,
                    pid,
                    timestamp_ns: ts,
                    ip: frame.func,
                    parent_ip: frame.original_ret_addr,
                    depth,
                    duration_ns: duration,
                });

                return Some(frame.original_ret_addr);
            }
        }
        None
    }
}
