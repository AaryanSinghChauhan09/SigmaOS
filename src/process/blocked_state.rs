// Linux & BSD Inspired Blocked Process States & Process Control Block (PCB) Management for SigmaOS

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// Detailed Blocked / Suspended Process State (Linux D / S / T / t states & BSD SSLEEP / SSTOP parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockedProcessState {
    WaitingIo,             // Linux TASK_UNINTERRUPTIBLE (D state)
    WaitingSemaphore,      // Linux TASK_INTERRUPTIBLE (S state)
    WaitingSocket,         // Network socket read/write block
    BlockedPageFault,      // Demand paging fault resolution stall
    SuspendedDisk,         // Swapped out / suspended to disk (BSD SSTOP/SWAP)
    SuspendedUserSignal,   // SIGSTOP / SIGTSTP signal suspension
    BlockedProcessControl, // Process scheduler block state
}

/// Reasons for Process Blocking / Waiting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockReason {
    BlockDeviceIo { device_id: usize, block_num: usize },
    PageFaultAddress { virtual_address: u64 },
    SemaphoreId(usize),
    MutexId(usize),
    SignalWaiting(u32),
}

/// Process Control Block (PCB) for Blocked State Tracking
#[derive(Debug, Clone)]
pub struct ProcessControlBlock {
    pub pid: usize,
    pub ppid: usize,
    pub name: String,
    pub state: BlockedProcessState,
    pub block_reason: Option<BlockReason>,
    pub blocked_at_timestamp: u64,
    pub is_swapped_out: bool,
    pub priority: u8,
}

impl ProcessControlBlock {
    pub fn new(pid: usize, ppid: usize, name: &str) -> Self {
        ProcessControlBlock {
            pid,
            ppid,
            name: String::from(name),
            state: BlockedProcessState::WaitingIo,
            block_reason: None,
            blocked_at_timestamp: 0,
            is_swapped_out: false,
            priority: 120,
        }
    }

    pub fn transition_to_blocked(
        &mut self,
        state: BlockedProcessState,
        reason: BlockReason,
        timestamp: u64,
    ) {
        self.state = state;
        self.block_reason = Some(reason);
        self.blocked_at_timestamp = timestamp;
        if state == BlockedProcessState::SuspendedDisk {
            self.is_swapped_out = true;
        }
    }

    pub fn unblock(&mut self) {
        self.block_reason = None;
        self.is_swapped_out = false;
    }
}

/// Blocked / Suspended Process Manager Queue
pub struct BlockedProcessManager {
    pub blocked_queue: Vec<ProcessControlBlock>,
}

impl BlockedProcessManager {
    pub fn new() -> Self {
        BlockedProcessManager {
            blocked_queue: Vec::new(),
        }
    }

    pub fn block_process(&mut self, pcb: ProcessControlBlock) {
        self.blocked_queue.push(pcb);
    }

    pub fn wake_process_by_io(
        &mut self,
        device_id: usize,
        block_num: usize,
    ) -> Vec<ProcessControlBlock> {
        let mut woken = Vec::new();
        let mut i = 0;
        while i < self.blocked_queue.len() {
            let is_match = match &self.blocked_queue[i].block_reason {
                Some(BlockReason::BlockDeviceIo {
                    device_id: d,
                    block_num: b,
                }) => *d == device_id && *b == block_num,
                _ => false,
            };

            if is_match {
                let mut pcb = self.blocked_queue.remove(i);
                pcb.unblock();
                woken.push(pcb);
            } else {
                i += 1;
            }
        }
        woken
    }

    pub fn get_blocked_count(&self) -> usize {
        self.blocked_queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocked_process_queue() {
        let mut manager = BlockedProcessManager::new();
        let mut pcb = ProcessControlBlock::new(42, 1, "disk-reader");
        pcb.transition_to_blocked(
            BlockedProcessState::WaitingIo,
            BlockReason::BlockDeviceIo {
                device_id: 1,
                block_num: 128,
            },
            5000,
        );

        manager.block_process(pcb);
        assert_eq!(manager.get_blocked_count(), 1);

        let woken = manager.wake_process_by_io(1, 128);
        assert_eq!(woken.len(), 1);
        assert_eq!(woken[0].pid, 42);
        assert_eq!(manager.get_blocked_count(), 0);
    }
}
