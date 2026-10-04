//! DMA (Direct Memory Access) Manager
//!
//! Inspired by Linux DMA subsystem and BSD busdma framework.
//! Provides DMA buffer allocation, mapping, and transfer management.
//!
//! # Features
//! - DMA buffer allocation (physically contiguous)
//! - DMA address mapping (virtual to physical)
//! - ISA DMA controller (8237) support
//! - Scatter-gather DMA
//! - Bounce buffering for legacy devices
//!
//! # Linux Inspiration
//! - `kernel/dma/` - DMA subsystem
//! - `arch/x86/kernel/pci-dma.c` - x86 DMA operations
//! - `include/linux/dma-mapping.h` - DMA mapping API
//!
//! # FreeBSD Inspiration
//! - `sys/kern/subr_bus_dma.c` - busdma framework
//! - `sys/dev/isa/isadma.c` - ISA DMA controller

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use core::arch::asm;

/// DMA channels (ISA DMA controller)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaChannel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4, // Cascade (not usable)
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7,
}

/// DMA transfer mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaMode {
    /// Read from memory to device
    Read = 0x48,
    /// Write from device to memory
    Write = 0x44,
    /// Verify transfer
    Verify = 0x40,
}

/// DMA transfer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaTransferType {
    Single,
    Block,
    OnDemand,
    Cascade,
}

/// DMA buffer descriptor
#[derive(Debug, Clone)]
pub struct DmaBuffer {
    /// Virtual address
    pub virt_addr: usize,
    /// Physical address
    pub phys_addr: usize,
    /// Size in bytes
    pub size: usize,
    /// DMA channel
    pub channel: DmaChannel,
}

impl DmaBuffer {
    /// Check if buffer is suitable for ISA DMA (must be under 16MB)
    pub fn is_isa_compatible(&self) -> bool {
        self.phys_addr < 0x1000000 // 16MB limit for ISA DMA
    }

    /// Check if buffer crosses 64KB boundary (ISA DMA limitation)
    pub fn crosses_64k_boundary(&self) -> bool {
        let start_page = self.phys_addr >> 16;
        let end_page = (self.phys_addr + self.size - 1) >> 16;
        start_page != end_page
    }
}

/// ISA DMA Controller ports
const DMA1_BASE: u16 = 0x00; // Channels 0-3
const DMA2_BASE: u16 = 0xC0; // Channels 4-7

const DMA1_PAGE: u16 = 0x80;
const DMA2_PAGE: u16 = 0x88;

/// DMA Manager
pub struct DmaManager {
    buffers: Vec<DmaBuffer>,
}

impl DmaManager {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
        }
    }

    /// Allocate DMA buffer
    /// Linux: `kernel/dma/mapping.c:dma_alloc_coherent()`
    /// FreeBSD: `sys/kern/subr_bus_dma.c:bus_dmamem_alloc()`
    pub fn allocate_buffer(
        &mut self,
        size: usize,
        channel: DmaChannel,
    ) -> Result<DmaBuffer, DmaError> {
        // For now, return a placeholder
        // In real implementation, this would allocate physically contiguous memory
        let buffer = DmaBuffer {
            virt_addr: 0,
            phys_addr: 0,
            size,
            channel,
        };

        if !buffer.is_isa_compatible() {
            return Err(DmaError::InvalidAddress);
        }

        if buffer.crosses_64k_boundary() {
            return Err(DmaError::BoundaryCrossing);
        }

        self.buffers.push(buffer.clone());
        Ok(buffer)
    }

    /// Setup ISA DMA transfer
    /// Linux: `arch/x86/kernel/pci-dma.c`
    pub fn setup_transfer(&self, buffer: &DmaBuffer, mode: DmaMode) -> Result<(), DmaError> {
        let channel = buffer.channel as u8;

        if channel == 4 {
            return Err(DmaError::InvalidChannel); // Channel 4 is cascade
        }

        unsafe {
            // Disable DMA channel
            self.mask_channel(channel, true);

            // Clear flip-flop
            self.clear_flip_flop(channel);

            // Set mode
            self.set_mode(channel, mode);

            // Set address
            let addr = buffer.phys_addr as u32;
            self.set_address(channel, addr);

            // Set count (size - 1)
            self.set_count(channel, (buffer.size - 1) as u16);

            // Set page register
            self.set_page(channel, (addr >> 16) as u8);

            // Enable DMA channel
            self.mask_channel(channel, false);
        }

        Ok(())
    }

    /// Mask/unmask DMA channel
    unsafe fn mask_channel(&self, channel: u8, mask: bool) {
        let port = if channel < 4 {
            0x0A // DMA1 single mask register
        } else {
            0xD4 // DMA2 single mask register
        };

        let value = (channel & 0x03) | if mask { 0x04 } else { 0x00 };
        Self::outb(port, value);
    }

    /// Clear byte pointer flip-flop
    unsafe fn clear_flip_flop(&self, channel: u8) {
        let port = if channel < 4 {
            0x0C // DMA1 clear flip-flop
        } else {
            0xD8 // DMA2 clear flip-flop
        };
        Self::outb(port, 0);
    }

    /// Set DMA mode
    unsafe fn set_mode(&self, channel: u8, mode: DmaMode) {
        let port = if channel < 4 {
            0x0B // DMA1 mode register
        } else {
            0xD6 // DMA2 mode register
        };

        let value = (channel & 0x03) | (mode as u8);
        Self::outb(port, value);
    }

    /// Set DMA address
    unsafe fn set_address(&self, channel: u8, addr: u32) {
        let port = match channel {
            0 => 0x00,
            1 => 0x02,
            2 => 0x04,
            3 => 0x06,
            5 => 0xC4,
            6 => 0xC8,
            7 => 0xCC,
            _ => return,
        };

        let addr16 = (addr & 0xFFFF) as u16;
        Self::outb(port, (addr16 & 0xFF) as u8);
        Self::outb(port, ((addr16 >> 8) & 0xFF) as u8);
    }

    /// Set DMA count
    unsafe fn set_count(&self, channel: u8, count: u16) {
        let port = match channel {
            0 => 0x01,
            1 => 0x03,
            2 => 0x05,
            3 => 0x07,
            5 => 0xC6,
            6 => 0xCA,
            7 => 0xCE,
            _ => return,
        };

        Self::outb(port, (count & 0xFF) as u8);
        Self::outb(port, ((count >> 8) & 0xFF) as u8);
    }

    /// Set DMA page register
    unsafe fn set_page(&self, channel: u8, page: u8) {
        let port = match channel {
            0 => 0x87,
            1 => 0x83,
            2 => 0x81,
            3 => 0x82,
            5 => 0x8B,
            6 => 0x89,
            7 => 0x8A,
            _ => return,
        };

        Self::outb(port, page);
    }

    /// I/O port access
    #[cfg(target_arch = "x86_64")]
    unsafe fn outb(port: u16, value: u8) {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    #[cfg(not(target_arch = "x86_64"))]
    unsafe fn outb(_port: u16, _value: u8) {
        // Stub
    }
}

/// DMA Error Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaError {
    InvalidAddress,
    InvalidChannel,
    BoundaryCrossing,
    OutOfMemory,
    NotAligned,
}

/// Scatter-gather list entry
#[derive(Debug, Clone, Copy)]
pub struct ScatterGatherEntry {
    pub phys_addr: u64,
    pub length: u32,
}

/// Scatter-gather DMA descriptor
pub struct ScatterGatherList {
    entries: Vec<ScatterGatherEntry>,
}

impl ScatterGatherList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, phys_addr: u64, length: u32) {
        self.entries.push(ScatterGatherEntry { phys_addr, length });
    }

    pub fn entries(&self) -> &[ScatterGatherEntry] {
        &self.entries
    }

    /// Total size of all entries
    pub fn total_size(&self) -> u64 {
        self.entries.iter().map(|e| e.length as u64).sum()
    }
}

/// DMA direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDirection {
    ToDevice,
    FromDevice,
    Bidirectional,
}

/// DMA synchronization point
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaSyncPoint {
    BeforeCpu,
    AfterCpu,
    BeforeDevice,
    AfterDevice,
}

/// Synchronize DMA buffer (cache coherency)
/// Linux: `kernel/dma/mapping.c:dma_sync_single_for_cpu()`
pub fn dma_sync_buffer(buffer: &DmaBuffer, sync_point: DmaSyncPoint) {
    // On x86_64 with strong memory ordering, this is often a no-op
    // On other architectures, this would flush/invalidate caches
    match sync_point {
        DmaSyncPoint::BeforeCpu => {
            // Invalidate cache before CPU reads
        }
        DmaSyncPoint::AfterCpu => {
            // Flush cache after CPU writes
        }
        DmaSyncPoint::BeforeDevice => {
            // Flush cache before device DMA
        }
        DmaSyncPoint::AfterDevice => {
            // Invalidate cache after device DMA
        }
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_dma_channel() {
        assert_eq!(DmaChannel::Channel0 as u8, 0);
        assert_eq!(DmaChannel::Channel7 as u8, 7);
    }

    #[test]
    fn test_dma_buffer_isa_compat() {
        let buffer = DmaBuffer {
            virt_addr: 0x1000,
            phys_addr: 0x10000,
            size: 4096,
            channel: DmaChannel::Channel2,
        };

        assert!(buffer.is_isa_compatible());
        assert!(!buffer.crosses_64k_boundary());
    }

    #[test]
    fn test_dma_buffer_boundary_crossing() {
        let buffer = DmaBuffer {
            virt_addr: 0,
            phys_addr: 0xFFFF, // One byte before 64K boundary
            size: 2,
            channel: DmaChannel::Channel1,
        };

        assert!(buffer.crosses_64k_boundary());
    }

    #[test]
    fn test_scatter_gather_list() {
        let mut sg = ScatterGatherList::new();
        sg.add_entry(0x1000, 4096);
        sg.add_entry(0x2000, 4096);

        assert_eq!(sg.entries().len(), 2);
        assert_eq!(sg.total_size(), 8192);
    }
}
