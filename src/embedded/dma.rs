#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

extern crate alloc;

extern crate alloc;

extern crate alloc;

extern crate alloc;

<<<<<<< HEAD
extern crate alloc;

extern crate alloc;

||||||| 23ef22a4a
extern crate alloc;

=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
/// OOP-based DMA for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1146
/// Implements DMA transfers with Linux and BSD-inspired safety wrappers.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
use crate::klib::Vec;
use alloc::boxed::Box;

pub type ChannelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DMADirection { MemoryToMemory = 0, MemoryToPeripheral = 1, PeripheralToMemory = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMAError {
    Success = 0,
    NotFound = 1,
    TransferFailed = 2,
    InvalidParameter = 3,
    UnalignedAddress = 4,
    OutOfBounds = 5,
    PermissionDenied = 6
}

pub trait DMAChannel {
    fn id(&self) -> ChannelID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDMAChannel {
    pub id: ChannelID,
    pub busy: AtomicUsize,
}

impl SimpleDMAChannel {
    pub fn new(id: ChannelID) -> Self {
        SimpleDMAChannel {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl DMAChannel for SimpleDMAChannel {
    fn id(&self) -> ChannelID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait DMAController {
    fn configure(&mut self, channel_id: ChannelID, direction: DMADirection) -> Result<(), DMAError>;
    fn start_transfer(&mut self, channel_id: ChannelID, src: u32, dst: u32, size: u32) -> Result<(), DMAError>;
    fn is_complete(&self, channel_id: ChannelID) -> bool;
}

#[repr(C)]
pub struct SimpleDMAController {
    pub channels: Vec<Option<Box<dyn DMAChannel>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDMAController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleDMAController {
            channels: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DMAController for SimpleDMAController {
    fn configure(&mut self, channel_id: ChannelID, _direction: DMADirection) -> Result<(), DMAError> {
        let channel = SimpleDMAChannel::new(channel_id);
        self.channels.push(Some(Box::new(channel)));
        Ok(())
    }
    
    fn start_transfer(&mut self, channel_id: ChannelID, src: u32, dst: u32, size: u32) -> Result<(), DMAError> {
        if size == 0 {
            return Err(DMAError::InvalidParameter);
        }

        // Linux-inspired Alignment check: verify standard 4-byte (word) aligned buffer addresses
        if (src % 4 != 0) || (dst % 4 != 0) {
            return Err(DMAError::UnalignedAddress);
        }

        // BSD-inspired Bounds check: protect core kernel memory maps (prevent accesses above 0xF0000000)
        if src >= 0xF0000000 || dst >= 0xF0000000 {
            return Err(DMAError::OutOfBounds);
        }

        for i in 0..self.channels.len() {
            if let Some(ref mut channel) = self.channels[i] {
                if channel.id() == channel_id {
                    channel.busy.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DMAError::NotFound)
    }
    
    fn is_complete(&self, channel_id: ChannelID) -> bool {
        for i in 0..self.channels.len() {
            if let Some(ref channel) = self.channels[i] {
                if channel.id() == channel_id {
                    return !channel.is_busy();
                }
            }
        }
        true
    }
}

pub trait CircularBuffer {
    fn write(&mut self, data: &[u8]) -> usize;
    fn read(&mut self, buffer: &mut [u8]) -> usize;
    fn available(&self) -> usize;
}

#[repr(C)]
pub struct SimpleCircularBuffer {
    pub data: [u8; 256],
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
}

impl SimpleCircularBuffer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleCircularBuffer {
            data: [0u8; 256],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }
}

impl CircularBuffer for SimpleCircularBuffer {
    fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            let next_head = (head + 1) % 256;
            if next_head != tail {
                self.data[head] = byte;
                self.head.store(next_head, Ordering::SeqCst);
                written += 1;
            } else {
                break;
            }
        }
        written
    }
    
    fn read(&mut self, buffer: &mut [u8]) -> usize {
        let mut read = 0;
        for byte in buffer.iter_mut() {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            if head != tail {
                *byte = self.data[tail];
                let next_tail = (tail + 1) % 256;
                self.tail.store(next_tail, Ordering::SeqCst);
                read += 1;
            } else {
                break;
            }
        }
        read
    }
    
    fn available(&self) -> usize {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        if head >= tail {
            head - tail
        } else {
            256 - tail + head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dma_controller_success() {
        let mut controller = SimpleDMAController::new();
        assert!(controller.configure(1, DMADirection::MemoryToMemory).is_ok());

        // 4-byte aligned safe transfer
        assert!(controller.start_transfer(1, 0x1000, 0x2000, 64).is_ok());
        assert!(!controller.is_complete(1));
    }

    #[test]
    fn test_dma_alignment_violations() {
        let mut controller = SimpleDMAController::new();
        assert!(controller.configure(2, DMADirection::MemoryToPeripheral).is_ok());

        // Unaligned source (0x1001)
        assert_eq!(controller.start_transfer(2, 0x1001, 0x2000, 64), Err(DMAError::UnalignedAddress));
    }

    #[test]
    fn test_dma_bounds_violations() {
        let mut controller = SimpleDMAController::new();
        assert!(controller.configure(3, DMADirection::PeripheralToMemory).is_ok());

        // Out-of-bounds address (0xF0000000)
        assert_eq!(controller.start_transfer(3, 0x1000, 0xF0000000, 64), Err(DMAError::OutOfBounds));
    }
}
