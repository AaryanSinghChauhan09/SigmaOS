//! # DMA Engine
//!
//! Direct Memory Access subsystem for high-speed peripheral data transfers.
//! Inspired by Linux drivers/dma/dmaengine.c and FreeBSD sys/dev/dma/.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU32, AtomicU64, AtomicBool, Ordering};

/// DMA transfer direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaDirection {
    MemToMem = 0,     // Memory to memory
    MemToDev = 1,     // Memory to device
    DevToMem = 2,     // Device to memory
    DevToDev = 3,     // Device to device
}

/// DMA transfer width
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaWidth {
    Byte = 1,
    HalfWord = 2,
    Word = 4,
    DoubleWord = 8,
}

/// DMA burst size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaBurst {
    Single = 1,
    Incr4 = 4,
    Incr8 = 8,
    Incr16 = 16,
}

/// DMA transfer descriptor
#[derive(Debug, Clone)]
pub struct DmaDescriptor {
    pub src_addr: u64,
    pub dst_addr: u64,
    pub length: usize,
    pub direction: DmaDirection,
    pub src_width: DmaWidth,
    pub dst_width: DmaWidth,
    pub src_burst: DmaBurst,
    pub dst_burst: DmaBurst,
    pub cyclic: bool,          // Cyclic transfer for audio/video
}

impl DmaDescriptor {
    pub fn new(src: u64, dst: u64, len: usize, dir: DmaDirection) -> Self {
        Self {
            src_addr: src,
            dst_addr: dst,
            length: len,
            direction: dir,
            src_width: DmaWidth::Word,
            dst_width: DmaWidth::Word,
            src_burst: DmaBurst::Single,
            dst_burst: DmaBurst::Single,
            cyclic: false,
        }
    }
}

/// DMA channel state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaChannelState {
    Idle = 0,
    Configured = 1,
    Running = 2,
    Paused = 3,
    Error = 4,
}

/// DMA channel
pub struct DmaChannel {
    id: u32,
    state: AtomicU32,
    descriptor: Option<DmaDescriptor>,
    bytes_transferred: AtomicU64,
    callback: Option<fn(u32, bool)>, // (channel_id, success)
    priority: AtomicU32,
    hardware_id: u32,
}

impl DmaChannel {
    pub fn new(id: u32, hardware_id: u32) -> Self {
        Self {
            id,
            state: AtomicU32::new(DmaChannelState::Idle as u32),
            descriptor: None,
            bytes_transferred: AtomicU64::new(0),
            callback: None,
            priority: AtomicU32::new(0),
            hardware_id,
        }
    }
    
    pub fn configure(&mut self, desc: DmaDescriptor) -> Result<(), DmaError> {
        let state = self.get_state();
        if state != DmaChannelState::Idle {
            return Err(DmaError::ChannelBusy);
        }
        
        self.descriptor = Some(desc);
        self.state.store(DmaChannelState::Configured as u32, Ordering::Release);
        Ok(())
    }
    
    pub fn start(&self) -> Result<(), DmaError> {
        let state = self.get_state();
        if state != DmaChannelState::Configured && state != DmaChannelState::Paused {
            return Err(DmaError::InvalidState);
        }
        
        // In production: program hardware DMA controller registers
        self.state.store(DmaChannelState::Running as u32, Ordering::Release);
        Ok(())
    }
    
    pub fn pause(&self) -> Result<(), DmaError> {
        let state = self.get_state();
        if state != DmaChannelState::Running {
            return Err(DmaError::InvalidState);
        }
        
        self.state.store(DmaChannelState::Paused as u32, Ordering::Release);
        Ok(())
    }
    
    pub fn stop(&self) -> Result<(), DmaError> {
        let state = self.get_state();
        if state == DmaChannelState::Idle {
            return Ok(());
        }
        
        self.state.store(DmaChannelState::Idle as u32, Ordering::Release);
        self.bytes_transferred.store(0, Ordering::Release);
        Ok(())
    }
    
    pub fn get_state(&self) -> DmaChannelState {
        let state_val = self.state.load(Ordering::Acquire);
        unsafe { core::mem::transmute(state_val as u8) }
    }
    
    pub fn get_progress(&self) -> (u64, usize) {
        let transferred = self.bytes_transferred.load(Ordering::Acquire);
        let total = self.descriptor.as_ref().map(|d| d.length).unwrap_or(0);
        (transferred, total)
    }
    
    pub fn set_callback(&mut self, callback: fn(u32, bool)) {
        self.callback = Some(callback);
    }
    
    pub fn set_priority(&self, priority: u32) {
        self.priority.store(priority, Ordering::Release);
    }
    
    /// Called by interrupt handler when transfer completes
    pub fn on_transfer_complete(&self, success: bool) {
        if let Some(desc) = &self.descriptor {
            self.bytes_transferred.fetch_add(desc.length as u64, Ordering::Relaxed);
        }
        
        if !self.descriptor.as_ref().map(|d| d.cyclic).unwrap_or(false) {
            self.state.store(DmaChannelState::Idle as u32, Ordering::Release);
        }
        
        if let Some(cb) = self.callback {
            cb(self.id, success);
        }
    }
}

/// DMA controller
pub struct DmaController {
    channels: Vec<Arc<DmaChannel>>,
    next_channel: AtomicU32,
    capabilities: DmaCapabilities,
    base_addr: usize,
}

impl DmaController {
    pub fn new(num_channels: usize, base_addr: usize) -> Self {
        let mut channels = Vec::with_capacity(num_channels);
        for i in 0..num_channels {
            channels.push(Arc::new(DmaChannel::new(i as u32, i as u32)));
        }
        
        Self {
            channels,
            next_channel: AtomicU32::new(0),
            capabilities: DmaCapabilities::default(),
            base_addr,
        }
    }
    
    /// Allocate a DMA channel
    pub fn alloc_channel(&self) -> Result<Arc<DmaChannel>, DmaError> {
        // Find first idle channel
        for channel in &self.channels {
            if channel.get_state() == DmaChannelState::Idle {
                return Ok(Arc::clone(channel));
            }
        }
        
        Err(DmaError::NoChannelsAvailable)
    }
    
    /// Allocate specific channel by ID
    pub fn alloc_channel_by_id(&self, id: u32) -> Result<Arc<DmaChannel>, DmaError> {
        if id as usize >= self.channels.len() {
            return Err(DmaError::InvalidChannel);
        }
        
        let channel = &self.channels[id as usize];
        if channel.get_state() != DmaChannelState::Idle {
            return Err(DmaError::ChannelBusy);
        }
        
        Ok(Arc::clone(channel))
    }
    
    /// Get channel by ID
    pub fn get_channel(&self, id: u32) -> Option<Arc<DmaChannel>> {
        if id as usize < self.channels.len() {
            Some(Arc::clone(&self.channels[id as usize]))
        } else {
            None
        }
    }
    
    /// Perform simple memory-to-memory copy
    pub fn memcpy(&mut self, src: u64, dst: u64, len: usize) -> Result<(), DmaError> {
        let channel = self.alloc_channel()?;
        
        let desc = DmaDescriptor::new(src, dst, len, DmaDirection::MemToMem);
        
        // Configure and start
        let mut ch = unsafe { &mut *(Arc::as_ptr(&channel) as *mut DmaChannel) };
        ch.configure(desc)?;
        channel.start()?;
        
        // In production: wait for completion or return handle
        Ok(())
    }
    
    /// Perform scatter-gather DMA
    pub fn scatter_gather(&mut self, src_list: &[(u64, usize)], dst: u64) -> Result<(), DmaError> {
        let mut current_dst = dst;
        
        for (src_addr, len) in src_list {
            let channel = self.alloc_channel()?;
            let desc = DmaDescriptor::new(*src_addr, current_dst, *len, DmaDirection::MemToMem);
            
            let mut ch = unsafe { &mut *(Arc::as_ptr(&channel) as *mut DmaChannel) };
            ch.configure(desc)?;
            channel.start()?;
            
            current_dst += *len as u64;
        }
        
        Ok(())
    }
    
    /// Get controller capabilities
    pub fn get_capabilities(&self) -> &DmaCapabilities {
        &self.capabilities
    }
    
    /// Handle DMA interrupt
    pub fn handle_interrupt(&self) {
        // In production: read interrupt status register
        // For each channel with pending interrupt:
        // channel.on_transfer_complete(success);
    }
}

/// DMA controller capabilities
#[derive(Debug, Clone)]
pub struct DmaCapabilities {
    pub max_burst_size: usize,
    pub max_transfer_size: usize,
    pub supports_mem_to_mem: bool,
    pub supports_scatter_gather: bool,
    pub supports_cyclic: bool,
    pub supports_interrupt_coalescing: bool,
    pub num_channels: usize,
}

impl Default for DmaCapabilities {
    fn default() -> Self {
        Self {
            max_burst_size: 16,
            max_transfer_size: 1 << 20, // 1 MB
            supports_mem_to_mem: true,
            supports_scatter_gather: true,
            supports_cyclic: true,
            supports_interrupt_coalescing: false,
            num_channels: 8,
        }
    }
}

/// DMA memory pool for common buffer allocations
pub struct DmaPool {
    size: usize,
    alignment: usize,
    buffers: Vec<DmaBuffer>,
}

impl DmaPool {
    pub fn new(size: usize, alignment: usize) -> Self {
        Self {
            size,
            alignment,
            buffers: Vec::new(),
        }
    }
    
    /// Allocate DMA-coherent buffer
    pub fn alloc(&mut self) -> Result<DmaBuffer, DmaError> {
        // In production: allocate from DMA-coherent memory zone
        let data = alloc::vec![0u8; self.size];
        let phys_addr = data.as_ptr() as u64; // Simplified: should use proper PA translation
        
        Ok(DmaBuffer {
            virt_addr: data.as_ptr() as u64,
            phys_addr,
            size: self.size,
            data,
        })
    }
}

/// DMA-coherent buffer
pub struct DmaBuffer {
    pub virt_addr: u64,
    pub phys_addr: u64,
    pub size: usize,
    data: Vec<u8>,
}

impl DmaBuffer {
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

/// DMA errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaError {
    NoChannelsAvailable,
    InvalidChannel,
    ChannelBusy,
    InvalidState,
    InvalidAddress,
    InvalidLength,
    TransferError,
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dma_channel_lifecycle() {
        let mut channel = DmaChannel::new(0, 0);
        assert_eq!(channel.get_state(), DmaChannelState::Idle);
        
        let desc = DmaDescriptor::new(0x1000, 0x2000, 4096, DmaDirection::MemToMem);
        assert!(channel.configure(desc).is_ok());
        assert_eq!(channel.get_state(), DmaChannelState::Configured);
        
        assert!(channel.start().is_ok());
        assert_eq!(channel.get_state(), DmaChannelState::Running);
        
        assert!(channel.pause().is_ok());
        assert_eq!(channel.get_state(), DmaChannelState::Paused);
        
        assert!(channel.stop().is_ok());
        assert_eq!(channel.get_state(), DmaChannelState::Idle);
    }
    
    #[test]
    fn test_dma_controller() {
        let controller = DmaController::new(4, 0x40000000);
        
        let ch = controller.alloc_channel();
        assert!(ch.is_ok());
        
        let ch2 = controller.alloc_channel_by_id(1);
        assert!(ch2.is_ok());
    }
    
    #[test]
    fn test_dma_descriptor() {
        let desc = DmaDescriptor::new(0x1000, 0x2000, 1024, DmaDirection::MemToMem);
        assert_eq!(desc.src_addr, 0x1000);
        assert_eq!(desc.dst_addr, 0x2000);
        assert_eq!(desc.length, 1024);
        assert_eq!(desc.direction, DmaDirection::MemToMem);
    }
}
