// SigmaOS Physical Frame Management
// Represents physical 4KB RAM frames and frame metadata descriptors.

use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

pub const PAGE_SIZE_4K: u64 = 4096;

/// Frame flags bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameFlags {
    pub reserved: bool,
    pub pinned: bool,
    pub reclaimable: bool,
    pub dirty: bool,
    pub zeroed: bool,
}

/// State of a physical frame
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameState {
    Free,
    Allocated,
    Reserved,
    Reclaimable,
}

/// Physical memory frame metadata descriptor
#[derive(Debug)]
pub struct FrameDescriptor {
    pub frame_index: usize,
    pub phys_addr: u64,
    pub ref_count: AtomicU32,
    pub state: AtomicU8, // Enum mapped to u8 (0: Free, 1: Allocated, 2: Reserved, 3: Reclaimable)
    pub flags: FrameFlags,
}

impl FrameDescriptor {
    pub fn new(frame_index: usize, phys_addr: u64, state: FrameState) -> Self {
        let state_val = match state {
            FrameState::Free => 0,
            FrameState::Allocated => 1,
            FrameState::Reserved => 2,
            FrameState::Reclaimable => 3,
        };
        Self {
            frame_index,
            phys_addr,
            ref_count: AtomicU32::new(if state == FrameState::Allocated { 1 } else { 0 }),
            state: AtomicU8::new(state_val),
            flags: FrameFlags {
                reserved: state == FrameState::Reserved,
                ..Default::default()
            },
        }
    }

    pub fn get_state(&self) -> FrameState {
        match self.state.load(Ordering::SeqCst) {
            0 => FrameState::Free,
            1 => FrameState::Allocated,
            2 => FrameState::Reserved,
            3 => FrameState::Reclaimable,
            _ => FrameState::Free,
        }
    }

    pub fn set_state(&self, state: FrameState) {
        let state_val = match state {
            FrameState::Free => 0,
            FrameState::Allocated => 1,
            FrameState::Reserved => 2,
            FrameState::Reclaimable => 3,
        };
        self.state.store(state_val, Ordering::SeqCst);
    }
}

/// Representation of an allocated physical frame handle
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub index: usize,
    pub phys_addr: u64,
    pub size: u64,
}

impl Frame {
    pub fn new(index: usize, phys_addr: u64) -> Self {
        Self {
            index,
            phys_addr,
            size: PAGE_SIZE_4K,
        }
    }
}
