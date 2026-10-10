// SigmaOS Physical Memory Manager
// Bounded, race-free physical frame manager with typed error handling and double-free prevention.

use super::frame::{Frame, FrameDescriptor, FrameState, PAGE_SIZE_4K};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameAllocationError {
    OutOfMemory,
    DoubleFree,
    ReservedFrame,
    InvalidFrameIndex,
    CapacityExceeded,
}

impl core::fmt::Display for FrameAllocationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfMemory => write!(f, "PhysicalMemoryManager: physical frame exhaustion (OOM)"),
            Self::DoubleFree => write!(f, "PhysicalMemoryManager: attempted double free of frame"),
            Self::ReservedFrame => write!(f, "PhysicalMemoryManager: cannot allocate reserved memory frame"),
            Self::InvalidFrameIndex => write!(f, "PhysicalMemoryManager: frame index out of bounds"),
            Self::CapacityExceeded => write!(f, "PhysicalMemoryManager: total physical capacity limit exceeded"),
        }
    }
}

pub type PhysicalResult<T> = Result<T, FrameAllocationError>;

#[derive(Debug)]
pub struct PhysicalMemoryManager {
    base_phys_addr: u64,
    total_frames: usize,
    free_count: AtomicUsize,
    allocated_count: AtomicUsize,
    reserved_count: AtomicUsize,
    descriptors: Vec<FrameDescriptor>,
    free_list: Mutex<Vec<usize>>,
    reclaim_wakeup_notified: AtomicBool,
}

impl PhysicalMemoryManager {
    pub fn new(base_phys_addr: u64, total_frames: usize, reserved_ranges: &[(usize, usize)]) -> Self {
        let mut descriptors = Vec::with_capacity(total_frames);
        let mut free_list = Vec::new();
        let mut res_count = 0;

        for i in 0..total_frames {
            let phys_addr = base_phys_addr + (i as u64 * PAGE_SIZE_4K);
            let is_reserved = reserved_ranges.iter().any(|&(start, count)| i >= start && i < start + count);

            if is_reserved {
                descriptors.push(FrameDescriptor::new(i, phys_addr, FrameState::Reserved));
                res_count += 1;
            } else {
                descriptors.push(FrameDescriptor::new(i, phys_addr, FrameState::Free));
                free_list.push(i);
            }
        }

        let free_count = total_frames - res_count;

        Self {
            base_phys_addr,
            total_frames,
            free_count: AtomicUsize::new(free_count),
            allocated_count: AtomicUsize::new(0),
            reserved_count: AtomicUsize::new(res_count),
            descriptors,
            free_list: Mutex::new(free_list),
            reclaim_wakeup_notified: AtomicBool::new(false),
        }
    }

    /// Allocate a single physical frame
    pub fn allocate_frame(&self) -> PhysicalResult<Frame> {
        let mut list = self.free_list.lock().unwrap();
        if let Some(frame_idx) = list.pop() {
            let desc = &self.descriptors[frame_idx];
            if desc.get_state() == FrameState::Reserved {
                list.push(frame_idx);
                return Err(FrameAllocationError::ReservedFrame);
            }
            desc.set_state(FrameState::Allocated);
            desc.ref_count.store(1, Ordering::SeqCst);

            self.free_count.fetch_sub(1, Ordering::SeqCst);
            self.allocated_count.fetch_add(1, Ordering::SeqCst);

            Ok(Frame::new(frame_idx, desc.phys_addr))
        } else {
            Err(FrameAllocationError::OutOfMemory)
        }
    }

    /// Free a previously allocated physical frame
    pub fn free_frame(&self, frame: Frame) -> PhysicalResult<()> {
        if frame.index >= self.total_frames {
            return Err(FrameAllocationError::InvalidFrameIndex);
        }

        let desc = &self.descriptors[frame.index];
        let current_state = desc.get_state();

        if current_state == FrameState::Reserved {
            return Err(FrameAllocationError::ReservedFrame);
        }

        if current_state == FrameState::Free {
            return Err(FrameAllocationError::DoubleFree);
        }

        let mut list = self.free_list.lock().unwrap();
        desc.set_state(FrameState::Free);
        desc.ref_count.store(0, Ordering::SeqCst);
        list.push(frame.index);

        self.allocated_count.fetch_sub(1, Ordering::SeqCst);
        self.free_count.fetch_add(1, Ordering::SeqCst);

        // Notify potential reclaim wait queue
        self.reclaim_wakeup_notified.store(true, Ordering::SeqCst);

        Ok(())
    }

    /// Trigger lost-wakeup check for frame reclaim waiting threads
    pub fn take_reclaim_wakeup_notification(&self) -> bool {
        self.reclaim_wakeup_notified.swap(false, Ordering::SeqCst)
    }

    pub fn total_frames(&self) -> usize {
        self.total_frames
    }

    pub fn free_frames(&self) -> usize {
        self.free_count.load(Ordering::SeqCst)
    }

    pub fn allocated_frames(&self) -> usize {
        self.allocated_count.load(Ordering::SeqCst)
    }

    pub fn reserved_frames(&self) -> usize {
        self.reserved_count.load(Ordering::SeqCst)
    }

    pub fn base_phys_addr(&self) -> u64 {
        self.base_phys_addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pmm_internal_basic() {
        let pmm = PhysicalMemoryManager::new(0x100000, 5, &[(0, 1)]);
        assert_eq!(pmm.total_frames(), 5);
        assert_eq!(pmm.reserved_frames(), 1);
        assert_eq!(pmm.free_frames(), 4);

        let frame = pmm.allocate_frame().unwrap();
        assert_eq!(pmm.free_frames(), 3);
        assert_eq!(pmm.allocated_frames(), 1);

        pmm.free_frame(frame).unwrap();
        assert_eq!(pmm.free_frames(), 4);
        assert_eq!(pmm.allocated_frames(), 0);
    }
}
