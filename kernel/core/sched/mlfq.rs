// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Multi-Level Feedback Queue (Rust, no_std)
//! =========================================================================

pub const MAX_PROCESSES: usize = 256;
const NUM_QUEUES: usize = 4;

type U32 = u32;

/// A simple static circular queue for tracking PIDs at a specific priority level.
pub struct PidQueue {
    pids: [U32; MAX_PROCESSES],
    head: usize,
    tail: usize,
    count: usize,
}

impl PidQueue {
    pub const fn new() -> Self {
        PidQueue {
            pids: [0; MAX_PROCESSES],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, pid: U32) -> bool {
        if self.count >= MAX_PROCESSES {
            return false;
        }
        self.pids[self.tail] = pid;
        self.tail = (self.tail + 1) % MAX_PROCESSES;
        self.count += 1;
        true
    }

    pub fn pop(&mut self) -> Option<U32> {
        if self.count == 0 {
            return None;
        }
        let pid = self.pids[self.head];
        self.head = (self.head + 1) % MAX_PROCESSES;
        self.count -= 1;
        Some(pid)
    }
}

pub struct MlfqQueue {
    queues: [PidQueue; NUM_QUEUES],
}

impl MlfqQueue {
    pub const fn new() -> Self {
        MlfqQueue {
            queues: [
                PidQueue::new(),
                PidQueue::new(),
                PidQueue::new(),
                PidQueue::new(),
            ],
        }
    }

    pub fn init(&mut self) {
        // Initialization if needed
    }

    pub fn enqueue(&mut self, priority: U32, pid: U32) -> bool {
        let p = if priority >= NUM_QUEUES as U32 {
            NUM_QUEUES - 1
        } else {
            priority as usize
        };
        self.queues[p].push(pid)
    }

    pub fn dequeue(&mut self) -> Option<U32> {
        let mut i = 0;
        while i < NUM_QUEUES {
            if let Some(pid) = self.queues[i].pop() {
                return Some(pid);
            }
            i += 1;
        }
        None
    }
}
