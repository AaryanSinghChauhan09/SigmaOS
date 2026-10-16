// SigmaOS Swap Queue and Request Dispatcher
// Queued swap request engine with depth limits, priority levels, and OOM observability metrics.

use super::core::{SwapError, SwapManager, SwapResult};
use core::sync::atomic::{AtomicUsize, Ordering};
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SwapPriority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone)]
pub struct SwapRequest {
    pub request_id: u64,
    pub virt_addr: u64,
    pub phys_frame: u64,
    pub priority: SwapPriority,
    pub is_swap_out: bool,
}

#[derive(Debug)]
pub struct SwapQueueManager {
    queue: Mutex<VecDeque<SwapRequest>>,
    max_queue_depth: usize,
    pending_count: AtomicUsize,
    next_request_id: AtomicUsize,
}

impl SwapQueueManager {
    pub fn new(max_queue_depth: usize) -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            max_queue_depth,
            pending_count: AtomicUsize::new(0),
            next_request_id: AtomicUsize::new(1),
        }
    }

    /// Enqueue a swap request with depth backpressure checks
    pub fn enqueue_request(
        &self,
        virt_addr: u64,
        phys_frame: u64,
        priority: SwapPriority,
        is_swap_out: bool,
    ) -> SwapResult<u64> {
        let mut q = self.queue.lock().unwrap();

        if q.len() >= self.max_queue_depth {
            return Err(SwapError::OutOfSwapSlots);
        }

        let req_id = self.next_request_id.fetch_add(1, Ordering::SeqCst) as u64;
        let req = SwapRequest {
            request_id: req_id,
            virt_addr,
            phys_frame,
            priority,
            is_swap_out,
        };

        // Insert based on priority
        if priority == SwapPriority::High {
            q.push_front(req);
        } else {
            q.push_back(req);
        }

        self.pending_count.fetch_add(1, Ordering::SeqCst);

        Ok(req_id)
    }

    /// Process next swap request from queue using SwapManager
    pub fn process_next(&self, swap_mgr: &SwapManager) -> Option<SwapResult<(u64, usize)>> {
        let mut q = self.queue.lock().unwrap();
        let req = q.pop_front()?;

        self.pending_count.fetch_sub(1, Ordering::SeqCst);

        if req.is_swap_out {
            let res = swap_mgr
                .swap_out_page(req.virt_addr, req.phys_frame)
                .map(|slot| (req.virt_addr, slot));
            Some(res)
        } else {
            let res = swap_mgr.swap_in_page(req.virt_addr);
            Some(res)
        }
    }

    pub fn pending_count(&self) -> usize {
        self.pending_count.load(Ordering::SeqCst)
    }
}
