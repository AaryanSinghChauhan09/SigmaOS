//! SigmaOS I/O Scheduler Subsystem

use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use super::scheduler_queue::{IoOperationKind, IoPriority, IoRequest, IoStage};

/// Typed errors for I/O scheduler operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    QueueFull,
    InvalidStageTransition,
    RequestNotFound(u64),
    ResourceExhausted,
    IoFailed(i32),
}

/// Multi-queue I/O scheduler enforcing the strict 10-stage lifecycle contract.
pub struct IoScheduler {
    max_queue_depth: usize,
    next_req_id: AtomicU64,
    rt_queue: VecDeque<IoRequest>,
    be_queue: VecDeque<IoRequest>,
    idle_queue: VecDeque<IoRequest>,
    active_requests: BTreeMap<u64, IoRequest>,
}

impl IoScheduler {
    pub fn new(max_queue_depth: usize) -> Self {
        Self {
            max_queue_depth,
            next_req_id: AtomicU64::new(1),
            rt_queue: VecDeque::new(),
            be_queue: VecDeque::new(),
            idle_queue: VecDeque::new(),
            active_requests: BTreeMap::new(),
        }
    }

    /// Stage 1 & 2: Submit and validate a new I/O request.
    pub fn submit_request(
        &mut self,
        dev_id: u32,
        op: IoOperationKind,
        block_num: u64,
        length_bytes: usize,
        priority: IoPriority,
    ) -> Result<u64, SchedulerError> {
        let total = self.rt_queue.len() + self.be_queue.len() + self.idle_queue.len() + self.active_requests.len();
        if total >= self.max_queue_depth {
            return Err(SchedulerError::QueueFull);
        }

        let id = self.next_req_id.fetch_add(1, Ordering::SeqCst);
        let mut req = IoRequest::new(id, dev_id, op, block_num, length_bytes, priority);

        // Stage 2: Validate
        req.advance(IoStage::Validate);

        // Stage 3: Enqueue
        req.advance(IoStage::Enqueue);

        match priority {
            IoPriority::RealTime => self.rt_queue.push_back(req),
            IoPriority::BestEffort => self.be_queue.push_back(req),
            IoPriority::Idle => self.idle_queue.push_back(req),
        }

        Ok(id)
    }

    /// Stage 4: Dequeue and execute next scheduled I/O request.
    pub fn dispatch_next(&mut self) -> Option<IoRequest> {
        let mut req = self.rt_queue.pop_front()
            .or_else(|| self.be_queue.pop_front())
            .or_else(|| self.idle_queue.pop_front())?;

        req.advance(IoStage::Execute);
        self.active_requests.insert(req.id, req.clone());
        Some(req)
    }

    /// Stage 5 to 10: Process completion, callback, reclaim, and free.
    pub fn complete_request(&mut self, id: u64, result: Result<(), i32>) -> Result<IoStage, SchedulerError> {
        if let Some(mut req) = self.active_requests.remove(&id) {
            match result {
                Ok(()) => {
                    req.advance(IoStage::InterruptDma);
                    req.advance(IoStage::Complete);
                    req.advance(IoStage::Callback);
                    req.advance(IoStage::Reclaim);
                    req.advance(IoStage::Free);
                    Ok(IoStage::Free)
                }
                Err(err) => {
                    req.fail(err);
                    Ok(IoStage::ErrorRecovery)
                }
            }
        } else {
            Err(SchedulerError::RequestNotFound(id))
        }
    }

    pub fn pending_count(&self) -> usize {
        self.rt_queue.len() + self.be_queue.len() + self.idle_queue.len()
    }

    pub fn active_count(&self) -> usize {
        self.active_requests.len()
    }
}
