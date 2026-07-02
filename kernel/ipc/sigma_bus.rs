// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/ipc/sigma_bus.rs — sigma-bus: Typed IPC Message Bus
// Language: Rust #![no_std]
// Pattern: OOP via Channel + Bus structs

#![no_std]

pub const MAX_CHANNELS:  usize = 64;
pub const CHANNEL_DEPTH: usize = 16;
pub const MAX_MSG_BYTES: usize = 256;

#[derive(Clone, Copy)]
pub struct Message {
    pub src:     u32,
    pub dst:     u32,
    pub type_id: u32,
    pub len:     usize,
    pub data:    [u8; MAX_MSG_BYTES],
}

impl Message {
    pub const fn new(src: u32, dst: u32, type_id: u32) -> Self {
        Self { src, dst, type_id, len: 0, data: [0u8; MAX_MSG_BYTES] }
    }
    pub fn with_payload(mut self, payload: &[u8]) -> Self {
        let n = payload.len().min(MAX_MSG_BYTES);
        self.data[..n].copy_from_slice(&payload[..n]);
        self.len = n;
        self
    }
}

/// Lock-free single-producer / single-consumer ring buffer channel
pub struct Channel {
    buf:   [Option<Message>; CHANNEL_DEPTH],
    head:  usize,
    tail:  usize,
    pub id: u32,
}

impl Channel {
    pub const fn new(id: u32) -> Self {
        Self { buf: [const { None }; CHANNEL_DEPTH], head: 0, tail: 0, id }
    }

    pub fn send(&mut self, msg: Message) -> bool {
        let next = (self.tail + 1) % CHANNEL_DEPTH;
        if next == self.head { return false; } // full
        self.buf[self.tail] = Some(msg);
        self.tail = next;
        true
    }

    pub fn recv(&mut self) -> Option<Message> {
        if self.head == self.tail { return None; }
        let msg = self.buf[self.head].take();
        self.head = (self.head + 1) % CHANNEL_DEPTH;
        msg
    }

    pub fn is_empty(&self) -> bool { self.head == self.tail }
    pub fn is_full(&self) -> bool  { (self.tail + 1) % CHANNEL_DEPTH == self.head }
}

/// Central message bus — routes messages between shards by channel ID
pub struct SigmaBus {
    channels: [Option<Channel>; MAX_CHANNELS],
    count:    usize,
}

impl SigmaBus {
    pub const fn new() -> Self {
        Self { channels: [const { None }; MAX_CHANNELS], count: 0 }
    }

    pub fn open_channel(&mut self, id: u32) -> bool {
        if self.count >= MAX_CHANNELS { return false; }
        for slot in &mut self.channels {
            if slot.is_none() { *slot = Some(Channel::new(id)); self.count += 1; return true; }
        }
        false
    }

    pub fn close_channel(&mut self, id: u32) {
        for slot in &mut self.channels {
            if matches!(slot, Some(ref c) if c.id == id) {
                *slot = None; self.count -= 1; return;
            }
        }
    }

    pub fn send(&mut self, channel_id: u32, msg: Message) -> bool {
        for slot in &mut self.channels {
            if let Some(ref mut ch) = slot {
                if ch.id == channel_id { return ch.send(msg); }
            }
        }
        false
    }

    pub fn recv(&mut self, channel_id: u32) -> Option<Message> {
        for slot in &mut self.channels {
            if let Some(ref mut ch) = slot {
                if ch.id == channel_id { return ch.recv(); }
            }
        }
        None
    }

    /// Broadcast message to all channels except sender
    pub fn broadcast(&mut self, msg: Message) {
        for slot in &mut self.channels {
            if let Some(ref mut ch) = slot {
                if ch.id != msg.src { ch.send(msg); }
            }
        }
    }
}
