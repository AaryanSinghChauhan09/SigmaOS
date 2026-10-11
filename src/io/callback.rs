//! SigmaOS I/O Completion Callback Manager

use std::collections::BTreeMap;

/// Status outcome for completed I/O operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallbackStatus {
    Success,
    Timeout,
    HardwareError,
    Canceled,
}

/// A registered completion callback handle.
pub type CallbackHandle = fn(req_id: u64, status: CallbackStatus);

/// Callback Manager stores and invokes completion callbacks for scheduled I/O requests.
pub struct CallbackManager {
    callbacks: BTreeMap<u64, Vec<CallbackHandle>>,
}

impl CallbackManager {
    pub fn new() -> Self {
        Self {
            callbacks: BTreeMap::new(),
        }
    }

    /// Register a completion callback for a given request ID.
    pub fn register_callback(&mut self, req_id: u64, callback: CallbackHandle) {
        self.callbacks.entry(req_id).or_insert_with(Vec::new).push(callback);
    }

    /// Invoke all registered callbacks for a completed request exactly once.
    pub fn invoke_callbacks(&mut self, req_id: u64, status: CallbackStatus) -> usize {
        if let Some(handles) = self.callbacks.remove(&req_id) {
            let count = handles.len();
            for cb in handles {
                cb(req_id, status);
            }
            count
        } else {
            0
        }
    }

    pub fn pending_callbacks_count(&self) -> usize {
        self.callbacks.len()
    }
}
