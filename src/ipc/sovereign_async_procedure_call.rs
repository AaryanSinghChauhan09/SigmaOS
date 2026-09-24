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
            next_id: 1,
            total_apcs_dispatched: 0,
            total_cancellations: 0,
        }
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

    /// Evaluate POSIX thread cancellation point (`pthread_testcancel` parity)
    pub fn evaluate_cancellation_point(&self, token_id: u64) -> bool {
        if let Some(token) = self.cancellation_tokens.get(&token_id) {
            token.is_cancelled && token.state == AsyncCancellationState::Enable
        } else {
            false
        }
    }

    /// Process all pending APC messages
    pub fn process_pending_apcs(&mut self) -> Vec<(u64, Result<Vec<u8>, String>)> {
        let mut results = Vec::new();
        let msgs = core::mem::take(&mut self.pending_messages);

        for msg in msgs {
            // Check cancellation token state & POSIX cancellation points
            if self.evaluate_cancellation_point(msg.token_id) {
                results.push((msg.msg_id, Err("Procedure call cancelled".to_string())));
                continue;
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

        assert!(engine.evaluate_cancellation_point(task_id));

        let results = engine.process_pending_apcs();
        assert_eq!(results.len(), 0); // Removed from pending
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
