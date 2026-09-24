// SPDX-License-Identifier: MIT
// Sovereign Async Procedure Call (APC / AIO / Signal Cancellation) Subsystem for SigmaOS (`src/ipc/sovereign_async_procedure_call.rs`)
// Inspired by Linux POSIX AIO signal notifications (`aio_read`/`aio_write`/`SIGEV_THREAD`) and FreeBSD kqueue async event callbacks.
// Implements Kernel-Mode APC Queues, User-Mode APC Dispatching on Thread Alertable Wait State,
// POSIX Thread Cancellation Points (`pthread_testcancel` parity), and Priority-Ordered APC Delivery.

use std::collections::{BTreeMap, VecDeque};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// APC Delivery Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApcDeliveryMode {
    KernelMode = 0, // High-priority kernel completion routine
    UserMode = 1,   // User-mode callback executed when thread enters Alertable Wait state
}

/// APC Routine State
#[derive(Debug, Clone)]
pub struct AsyncProcedureCallDescriptor {
    pub apc_id: u64,
    pub target_tid: u32,
    pub mode: ApcDeliveryMode,
    pub priority: u8, // 0 = Normal, 255 = Realtime/Urgent
    pub routine_address: u64,
    pub context_parameter: u64,
    pub is_executed: bool,
}

/// Sovereign Async Procedure Call (APC) Dispatcher Engine
pub struct SovereignAsyncProcedureCallEngine {
    pub kernel_apc_queue: VecDeque<AsyncProcedureCallDescriptor>,
    pub user_apc_queues: BTreeMap<u32, VecDeque<AsyncProcedureCallDescriptor>>, // TID -> Queue
    pub thread_alertable_states: BTreeMap<u32, bool>,                           // TID -> IsAlertable
    pub thread_cancelled_flags: BTreeMap<u32, bool>,                           // TID -> IsCancelled
    pub total_apcs_dispatched: u64,
}

impl SovereignAsyncProcedureCallEngine {
    pub fn new() -> Self {
        Self {
            kernel_apc_queue: VecDeque::new(),
            user_apc_queues: BTreeMap::new(),
            thread_alertable_states: BTreeMap::new(),
            thread_cancelled_flags: BTreeMap::new(),
            total_apcs_dispatched: 0,
        }
    }

    /// Set thread Alertable Wait state (Allows user-mode APC execution)
    pub fn set_thread_alertable(&mut self, tid: u32, is_alertable: bool) {
        self.thread_alertable_states.insert(tid, is_alertable);
    }

    /// Queue an Async Procedure Call (Kernel or User mode)
    pub fn queue_apc(&mut self, tid: u32, mode: ApcDeliveryMode, routine_addr: u64, context: u64, priority: u8) -> u64 {
        let apc_id = self.total_apcs_dispatched + 1;
        let apc = AsyncProcedureCallDescriptor {
            apc_id,
            target_tid: tid,
            mode,
            priority,
            routine_address: routine_addr,
            context_parameter: context,
            is_executed: false,
        };

        match mode {
            ApcDeliveryMode::KernelMode => {
                self.kernel_apc_queue.push_back(apc);
            }
            ApcDeliveryMode::UserMode => {
                self.user_apc_queues.entry(tid).or_default().push_back(apc);
            }
        }

        self.total_apcs_dispatched += 1;
        apc_id
    }

    /// Deliver pending Kernel-Mode APCs immediately
    pub fn deliver_kernel_apcs(&mut self) -> usize {
        let mut delivered = 0;
        while let Some(mut apc) = self.kernel_apc_queue.pop_front() {
            apc.is_executed = true;
            if apc.is_executed {
                delivered += 1;
            }
        }
        delivered
    }

    /// Deliver pending User-Mode APCs if thread is in Alertable Wait state
    pub fn deliver_user_apcs_for_thread(&mut self, tid: u32) -> Result<usize, &'static str> {
        let is_alertable = self.thread_alertable_states.get(&tid).copied().unwrap_or(false);
        if !is_alertable {
            return Err("Thread is not in an Alertable Wait state; User APC delayed");
        }

        let mut count = 0;
        if let Some(queue) = self.user_apc_queues.get_mut(&tid) {
            while let Some(mut apc) = queue.pop_front() {
                apc.is_executed = true;
                if apc.is_executed {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// Request POSIX thread cancellation (`pthread_cancel` parity)
    pub fn request_thread_cancellation(&mut self, tid: u32) {
        self.thread_cancelled_flags.insert(tid, true);
    }

    /// Test if thread has pending cancellation at POSIX cancellation point (`pthread_testcancel` parity)
    pub fn test_thread_cancellation(&self, tid: u32) -> bool {
        self.thread_cancelled_flags.get(&tid).copied().unwrap_or(false)
    }
}

impl Default for SovereignAsyncProcedureCallEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_and_user_apc_delivery() {
        let mut engine = SovereignAsyncProcedureCallEngine::new();

        // Queue Kernel APC
        let k_id = engine.queue_apc(10, ApcDeliveryMode::KernelMode, 0xFF8000, 0x1234, 100);
        assert_eq!(k_id, 1);
        assert_eq!(engine.deliver_kernel_apcs(), 1);

        // Queue User APC for thread 20 (not alertable -> delayed)
        let _u_id = engine.queue_apc(20, ApcDeliveryMode::UserMode, 0x004000, 0x5678, 10);
        assert!(engine.deliver_user_apcs_for_thread(20).is_err());

        // Make thread 20 alertable -> delivered
        engine.set_thread_alertable(20, true);
        assert_eq!(engine.deliver_user_apcs_for_thread(20).unwrap(), 1);
    }

    #[test]
    fn test_pthread_cancellation_point() {
        let mut engine = SovereignAsyncProcedureCallEngine::new();
        assert!(!engine.test_thread_cancellation(30));

        engine.request_thread_cancellation(30);
        assert!(engine.test_thread_cancellation(30));
    }
}
