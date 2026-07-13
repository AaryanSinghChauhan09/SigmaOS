// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Zero-copy network stack for SigmaOS
// Eliminates memory copies in network operations for maximum performance

use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::vec;
use alloc::alloc::{alloc, dealloc};

/// Zero-copy network buffer
/// Shared between network layers without copying
pub struct ZeroCopyBuffer {
    data: NonNull<u8>,
    len: usize,
    capacity: usize,
    ref_count: AtomicUsize,
}

impl ZeroCopyBuffer {
    pub fn new(capacity: usize) -> Option<Self> {
        let layout = core::alloc::Layout::from_size_align(capacity, 1).ok()?;
        let data = unsafe { alloc(layout) };
        
        if data.is_null() {
            return None;
        }
        
        Some(Self {
            data: NonNull::new(data)?,
            len: 0,
            capacity,
            ref_count: AtomicUsize::new(1),
        })
    }

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        let len = data.len();
        let mut buffer = Self::new(len)?;
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), buffer.data.as_ptr(), len);
        }
        buffer.len = len;
        Some(buffer)
    }

    pub fn data(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.len) }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data.as_ptr(), self.len) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn set_len(&mut self, len: usize) {
        if len <= self.capacity {
            self.len = len;
        }
    }

    pub fn clone(&self) -> Self {
        self.ref_count.fetch_add(1, Ordering::Relaxed);
        Self {
            data: self.data,
            len: self.len,
            capacity: self.capacity,
            ref_count: AtomicUsize::new(self.ref_count.load(Ordering::Relaxed)),
        }
    }
}

impl Clone for ZeroCopyBuffer {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl Drop for ZeroCopyBuffer {
    fn drop(&mut self) {
        if self.ref_count.fetch_sub(1, Ordering::Release) == 1 {
            unsafe {
                let layout = core::alloc::Layout::from_size_align(self.capacity, 1).unwrap();
                dealloc(self.data.as_ptr(), layout);
            }
        }
    }
}

/// Zero-copy packet structure
pub struct ZeroCopyPacket {
    buffer: ZeroCopyBuffer,
    offset: usize,
    packet_len: usize,
}

impl ZeroCopyPacket {
    pub fn new(buffer: ZeroCopyBuffer) -> Self {
        let len = buffer.len();
        Self {
            buffer,
            offset: 0,
            packet_len: len,
        }
    }

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        let buffer = ZeroCopyBuffer::from_bytes(data)?;
        Some(Self::new(buffer))
    }

    pub fn data(&self) -> &[u8] {
        &self.buffer.data()[self.offset..self.offset + self.packet_len]
    }

    pub fn len(&self) -> usize {
        self.packet_len
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        if offset + self.packet_len <= self.buffer.len() {
            self.offset = offset;
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            offset: self.offset,
            packet_len: self.packet_len,
        }
    }
}

impl Clone for ZeroCopyPacket {
    fn clone(&self) -> Self {
        self.clone()
    }
}

/// Zero-copy network interface trait
pub trait ZeroCopyNetworkInterface {
    /// Send packet without copying
    fn send_zero_copy(&mut self, packet: ZeroCopyPacket) -> Result<(), NetworkError>;
    
    /// Receive packet without copying
    fn receive_zero_copy(&mut self) -> Result<Option<ZeroCopyPacket>, NetworkError>;
    
    /// Get MTU
    fn mtu(&self) -> u16;
    
    /// Get MAC address
    fn mac_address(&self) -> [u8; 6];
}

/// Zero-copy socket trait
pub trait ZeroCopySocket {
    /// Send data without copying
    fn send_zero_copy(&mut self, packet: ZeroCopyPacket) -> Result<usize, NetworkError>;
    
    /// Receive data without copying
    fn receive_zero_copy(&mut self) -> Result<Option<ZeroCopyPacket>, NetworkError>;
    
    /// Connect to remote address
    fn connect(&mut self, addr: SocketAddr) -> Result<(), NetworkError>;
    
    /// Bind to local address
    fn bind(&mut self, addr: SocketAddr) -> Result<(), NetworkError>;
    
    /// Listen for connections
    fn listen(&mut self, backlog: u32) -> Result<(), NetworkError>;
    
    /// Accept incoming connection
    fn accept_zero_copy(&mut self) -> Result<Option<Box<dyn ZeroCopySocket>>, NetworkError>;
}

/// Network error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    BufferTooSmall,
    InvalidPacket,
    ConnectionRefused,
    ConnectionReset,
    TimedOut,
    WouldBlock,
    AddressInUse,
    AddressNotAvailable,
    NoRouteToHost,
    HostUnreachable,
    NetworkUnreachable,
    ConnectionAborted,
    Other,
}

/// Socket address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddr {
    pub ip: [u8; 4],
    pub port: u16,
}

impl SocketAddr {
    pub const fn new(ip: [u8; 4], port: u16) -> Self {
        Self { ip, port }
    }

    pub const fn localhost(port: u16) -> Self {
        Self::new([127, 0, 0, 1], port)
    }
}

/// Zero-copy buffer pool for efficient buffer management
pub struct BufferPool {
    buffers: Vec<Option<ZeroCopyBuffer>>,
    buffer_size: usize,
    pool_size: usize,
}

impl BufferPool {
    pub fn new(buffer_size: usize, pool_size: usize) -> Self {
        let mut buffers = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            buffers.push(ZeroCopyBuffer::new(buffer_size));
        }
        
        Self {
            buffers,
            buffer_size,
            pool_size,
        }
    }

    pub fn acquire(&mut self) -> Option<ZeroCopyBuffer> {
        for buffer in &mut self.buffers {
            if buffer.is_none() {
                *buffer = ZeroCopyBuffer::new(self.buffer_size);
                return buffer.clone();
            }
        }
        None
    }

    pub fn release(&mut self, buffer: ZeroCopyBuffer) {
        for slot in &mut self.buffers {
            if slot.is_none() {
                *slot = Some(buffer);
                return;
            }
        }
    }

    pub fn available(&self) -> usize {
        self.buffers.iter().filter(|b| b.is_none()).count()
    }

    pub fn used(&self) -> usize {
        self.pool_size - self.available()
    }
}

/// Zero-copy ring buffer for network packet queuing
pub struct ZeroCopyRingBuffer {
    buffers: Vec<Option<ZeroCopyBuffer>>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl ZeroCopyRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffers: vec![None; capacity],
            head: 0,
            tail: 0,
            capacity,
        }
    }

    pub fn push(&mut self, buffer: ZeroCopyBuffer) -> bool {
        let next_tail = (self.tail + 1) % self.capacity;
        
        if next_tail == self.head {
            return false; // Buffer full
        }
        
        self.buffers[self.tail] = Some(buffer);
        self.tail = next_tail;
        true
    }

    pub fn pop(&mut self) -> Option<ZeroCopyBuffer> {
        if self.head == self.tail {
            return None; // Buffer empty
        }
        
        let buffer = self.buffers[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        buffer
    }

    pub fn len(&self) -> usize {
        if self.tail >= self.head {
            self.tail - self.head
        } else {
            self.capacity - self.head + self.tail
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.capacity == self.head
    }
}
