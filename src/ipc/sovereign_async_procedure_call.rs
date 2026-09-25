#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::new_without_default)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::{BTreeMap, HashMap, HashSet};

#[cfg(any(feature = "standalone_test", test))]
use std::collections::{BTreeMap, HashMap, HashSet};

// ============================================================================
// Sovereign Async Procedure Call (APC) & Asynchronous IPC Engine
// Inspired by Linux POSIX cancellation, io_uring ASYNC_CANCEL, Windows ALPC, Mach IPC
// ============================================================================

/// Async Cancellation State (`pthread_setcancelstate` / `pthread_setcanceltype`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncCancellationState {
    Enable,   // PTHREAD_CANCEL_ENABLE
    Disable,  // PTHREAD_CANCEL_DISABLE
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncCancellationType {
    Deferred,     // PTHREAD_CANCEL_DEFERRED (cancellation points)
    Asynchronous, // PTHREAD_CANCEL_ASYNCHRONOUS (immediate termination)
}

/// Priority level for Kernel Async Procedure Calls (Linux POSIX signal, FreeBSD `ast()`, NT Kernel APC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KernelApcPriority {
    UserMode = 0,
    Normal = 1,
    SpecialKernel = 2,
    HighPriority = 3,
}

/// Enqueued Kernel/User Async Procedure Entry targeted at specific thread/process
#[derive(Debug, Clone)]
pub struct KernelApcEntry {
    pub apc_id: u64,
    pub target_thread_id: u64,
    pub priority: KernelApcPriority,
    pub routine_name: String,
    pub payload: Vec<u8>,
    pub is_kernel_mode: bool,
}

/// Cancellation Token for tracked async procedures & write operations
#[derive(Debug, Clone)]
pub struct SovereignCancellationToken {
    pub token_id: u64,
    pub state: AsyncCancellationState,
    pub cancel_type: AsyncCancellationType,
    pub is_cancelled: bool,
}

impl SovereignCancellationToken {
    pub fn new(token_id: u64) -> Self {
        Self {
            token_id,
            state: AsyncCancellationState::Enable,
            cancel_type: AsyncCancellationType::Deferred,
            is_cancelled: false,
        }
    }

    pub fn cancel(&mut self) -> bool {
        if self.state == AsyncCancellationState::Enable {
            self.is_cancelled = true;
            true
        } else {
            false
        }
    }
}

/// Asynchronous Procedure Call Message Payload (ALPC / Mach IPC)
#[derive(Debug, Clone)]
pub struct SovereignApcMessage {
    pub msg_id: u64,
    pub caller_pid: u64,
    pub procedure_name: String,
    pub payload_bytes: Vec<u8>,
    pub token_id: u64,
}

/// Asynchronous Procedure Execution Routine
pub struct SovereignApcRoutine {
    pub procedure_name: String,
    pub handler: Box<dyn Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync>,
}

/// Asynchronous Write Operation Handle (io_uring style async write)
#[derive(Debug, Clone)]
pub struct SovereignAsyncWriteOp {
    pub write_id: u64,
    pub target_fd: i32,
    pub buffer: Vec<u8>,
    pub bytes_written: usize,
    pub is_completed: bool,
    pub is_cancelled: bool,
}

/// Sovereign Async Procedure Call & Message Passing Engine
pub struct SovereignAsyncProcedureCallEngine {
    pub cancellation_tokens: HashMap<u64, SovereignCancellationToken>,
    pub routines: HashMap<String, SovereignApcRoutine>,
    pub pending_messages: Vec<SovereignApcMessage>,
    pub active_writes: HashMap<u64, SovereignAsyncWriteOp>,
    pub kernel_apc_queue: Vec<KernelApcEntry>,
    pub next_id: u64,
    pub total_apcs_dispatched: u64,
    pub total_cancellations: u64,
}

impl SovereignAsyncProcedureCallEngine {
    pub fn new() -> Self {
        Self {
            cancellation_tokens: HashMap::new(),
            routines: HashMap::new(),
            pending_messages: Vec::new(),
            active_writes: HashMap::new(),
            kernel_apc_queue: Vec::new(),
            next_id: 1,
            total_apcs_dispatched: 0,
            total_cancellations: 0,
        }
    }

    /// Queue a targeted Kernel/User Async Procedure Call for a specific thread ID
    pub fn queue_kernel_apc(
        &mut self,
        target_thread_id: u64,
        priority: KernelApcPriority,
        routine_name: &str,
        payload: Vec<u8>,
        is_kernel_mode: bool,
    ) -> u64 {
        let apc_id = self.next_id;
        self.next_id += 1;

        let entry = KernelApcEntry {
            apc_id,
            target_thread_id,
            priority,
            routine_name: routine_name.to_string(),
            payload,
            is_kernel_mode,
        };

        self.kernel_apc_queue.push(entry);
        // Maintain priority ordering (highest priority first)
        self.kernel_apc_queue.sort_by(|a, b| b.priority.cmp(&a.priority));

        self.total_apcs_dispatched += 1;
        apc_id
    }

    /// Deliver and execute targeted APCs for a specific thread ID upon context switch or interrupt exit
    pub fn deliver_thread_apcs(&mut self, target_thread_id: u64) -> Vec<(u64, Result<Vec<u8>, String>)> {
        let mut results = Vec::new();
        let mut remaining = Vec::new();

        let queue = core::mem::take(&mut self.kernel_apc_queue);

        for entry in queue {
            if entry.target_thread_id == target_thread_id {
                if let Some(routine) = self.routines.get(&entry.routine_name) {
                    let res = (routine.handler)(&entry.payload);
                    results.push((entry.apc_id, res));
                } else {
                    results.push((
                        entry.apc_id,
                        Err(format!("Routine '{}' not found", entry.routine_name)),
                    ));
                }
            } else {
                remaining.push(entry);
            }
        }

        self.kernel_apc_queue = remaining;
        results
    }

    /// Register a named Async Procedure Call (APC) handler
    pub fn register_procedure<F>(&mut self, procedure_name: &str, handler: F)
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync + 'static,
    {
        let routine = SovereignApcRoutine {
            procedure_name: procedure_name.to_string(),
            handler: Box::new(handler),
        };
        self.routines.insert(procedure_name.to_string(), routine);
    }

    /// Dispatch an asynchronous procedure call, returning task ID and cancellation token
    pub fn dispatch_async_call(
        &mut self,
        caller_pid: u64,
        procedure_name: &str,
        payload_bytes: Vec<u8>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let token = SovereignCancellationToken::new(id);
        self.cancellation_tokens.insert(id, token);

        let msg = SovereignApcMessage {
            msg_id: id,
            caller_pid,
            procedure_name: procedure_name.to_string(),
            payload_bytes,
            token_id: id,
        };

        self.pending_messages.push(msg);
        self.total_apcs_dispatched += 1;
        id
    }

    /// Initiate an asynchronous write operation (`io_uring` async write)
    pub fn queue_async_write(&mut self, target_fd: i32, buffer: Vec<u8>) -> u64 {
        let write_id = self.next_id;
        self.next_id += 1;

        let token = SovereignCancellationToken::new(write_id);
        self.cancellation_tokens.insert(write_id, token);

        let write_op = SovereignAsyncWriteOp {
            write_id,
            target_fd,
            buffer,
            bytes_written: 0,
            is_completed: false,
            is_cancelled: false,
        };

        self.active_writes.insert(write_id, write_op);
        write_id
    }

    /// Cancel a pending async procedure or write operation (`pthread_cancel` / `IORING_OP_ASYNC_CANCEL`)
    pub fn cancel_async_operation(&mut self, op_id: u64) -> Result<bool, String> {
        let token = self
            .cancellation_tokens
            .get_mut(&op_id)
            .ok_or_else(|| format!("Operation token '{}' not found", op_id))?;

        if token.cancel() {
            self.total_cancellations += 1;

            // Remove from pending messages if queued
            self.pending_messages.retain(|m| m.msg_id != op_id);

            // Mark active write as cancelled
            if let Some(write_op) = self.active_writes.get_mut(&op_id) {
                write_op.is_cancelled = true;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Process all pending APC messages
    pub fn process_pending_apcs(&mut self) -> Vec<(u64, Result<Vec<u8>, String>)> {
        let mut results = Vec::new();
        let msgs = core::mem::take(&mut self.pending_messages);

        for msg in msgs {
            // Check cancellation token state
            if let Some(token) = self.cancellation_tokens.get(&msg.token_id) {
                if token.is_cancelled {
                    results.push((msg.msg_id, Err("Procedure call cancelled".to_string())));
                    continue;
                }
            }

            if let Some(routine) = self.routines.get(&msg.procedure_name) {
                let res = (routine.handler)(&msg.payload_bytes);
                results.push((msg.msg_id, res));
            } else {
                results.push((
                    msg.msg_id,
                    Err(format!("Procedure '{}' not found", msg.procedure_name)),
                ));
            }
        }

        results
    }

    /// Execute pending async write operations
    pub fn flush_async_writes(&mut self) -> Vec<(u64, usize)> {
        let mut completed = Vec::new();

        for (write_id, write_op) in self.active_writes.iter_mut() {
            if !write_op.is_completed && !write_op.is_cancelled {
                write_op.bytes_written = write_op.buffer.len();
                write_op.is_completed = true;
                completed.push((*write_id, write_op.bytes_written));
            }
        }

        completed
    }
}

impl Default for SovereignAsyncProcedureCallEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Standalone Unit Test Suite
// ============================================================================

#[cfg(test)]
mod apc_tests {
    use super::*;

    #[test]
    fn test_async_procedure_call_dispatch_and_execution() {
        let mut engine = SovereignAsyncProcedureCallEngine::new();

        engine.register_procedure("echo_uppercase", |input| {
            let str_val = core::str::from_utf8(input).map_err(|_| "Utf8 error")?;
            Ok(str_val.to_uppercase().into_bytes())
        });

        let call_id = engine.dispatch_async_call(1001, "echo_uppercase", b"hello sigmaos".to_vec());
        assert_eq!(call_id, 1);

        let results = engine.process_pending_apcs();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 1);

        let output_bytes = results[0].1.as_ref().unwrap();
        assert_eq!(core::str::from_utf8(output_bytes).unwrap(), "HELLO SIGMAOS");
    }

    #[test]
    fn test_async_cancellation_token_flow() {
        let mut engine = SovereignAsyncProcedureCallEngine::new();

        engine.register_procedure("slow_task", |_| Ok(b"done".to_vec()));

        let task_id = engine.dispatch_async_call(1001, "slow_task", b"data".to_vec());

        // Cancel before processing
        let cancel_res = engine.cancel_async_operation(task_id);
        assert!(cancel_res.unwrap());

        let results = engine.process_pending_apcs();
        assert_eq!(results.len(), 0); // Removed from pending
    }

    #[test]
    fn test_kernel_apc_priority_queueing_and_thread_delivery() {
        let mut engine = SovereignAsyncProcedureCallEngine::new();

        engine.register_procedure("sys_signal_handler", |input| {
            Ok(b"sig_handled".to_vec())
        });

        // Queue normal user APC for Thread 42
        let apc1 = engine.queue_kernel_apc(42, KernelApcPriority::UserMode, "sys_signal_handler", b"user".to_vec(), false);
        // Queue high priority kernel APC for Thread 42
        let apc2 = engine.queue_kernel_apc(42, KernelApcPriority::HighPriority, "sys_signal_handler", b"kernel".to_vec(), true);
        // Queue APC for Thread 99
        let apc3 = engine.queue_kernel_apc(99, KernelApcPriority::Normal, "sys_signal_handler", b"other".to_vec(), false);

        // Deliver thread APCs for Thread 42
        let delivered_42 = engine.deliver_thread_apcs(42);
        assert_eq!(delivered_42.len(), 2);
        // High priority APC delivered first
        assert_eq!(delivered_42[0].0, apc2);
        assert_eq!(delivered_42[1].0, apc1);

        // Verify Thread 99 APC remains queued
        assert_eq!(engine.kernel_apc_queue.len(), 1);
        let delivered_99 = engine.deliver_thread_apcs(99);
        assert_eq!(delivered_99.len(), 1);
        assert_eq!(delivered_99[0].0, apc3);
    }

    #[test]
    fn test_async_write_cancellation_and_flush() {
        let mut engine = SovereignAsyncProcedureCallEngine::new();

        let w1 = engine.queue_async_write(1, b"write_data_1".to_vec());
        let w2 = engine.queue_async_write(2, b"write_data_2".to_vec());

        // Cancel w1
        engine.cancel_async_operation(w1).unwrap();

        let completed = engine.flush_async_writes();
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].0, w2);
        assert_eq!(completed[0].1, 12);
    }
}
