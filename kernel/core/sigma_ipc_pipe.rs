// SigmaOS — IPC: Pipes, Message Queues, Shared Memory
// Sovereign implementation — no external dependencies
#![no_std]
#![allow(dead_code)]
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

// ─── Pipe ────────────────────────────────────────────────────────────────────
pub const PIPE_BUF: usize = 65536;

pub struct Pipe {
    buf:   [u8; PIPE_BUF],
    head:  AtomicUsize,
    tail:  AtomicUsize,
    write_closed: AtomicBool,
    read_closed:  AtomicBool,
}

impl Pipe {
    pub const fn new() -> Self {
        Pipe {
            buf: [0u8; PIPE_BUF],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            write_closed: AtomicBool::new(false),
            read_closed:  AtomicBool::new(false),
        }
    }
    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = (tail + 1) % PIPE_BUF;
            if next == head { break; } // full
            self.buf[tail] = b;
            self.tail.store(next, Ordering::Release);
            written += 1;
        }
        written
    }
    pub fn read(&mut self, out: &mut [u8]) -> usize {
        let mut read = 0;
        for slot in out.iter_mut() {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            if head == tail { break; } // empty
            *slot = self.buf[head];
            self.head.store((head + 1) % PIPE_BUF, Ordering::Release);
            read += 1;
        }
        read
    }
    pub fn close_write(&self) { self.write_closed.store(true, Ordering::Release); }
    pub fn close_read(&self)  { self.read_closed.store(true, Ordering::Release); }
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire) == self.tail.load(Ordering::Acquire)
    }
    pub fn available(&self) -> usize {
        let h = self.head.load(Ordering::Acquire);
        let t = self.tail.load(Ordering::Acquire);
        if t >= h { t - h } else { PIPE_BUF - h + t }
    }
}

// ─── Message Queue ───────────────────────────────────────────────────────────
pub const MSGQ_MAX_MSGS: usize = 256;
pub const MSGQ_MAX_SIZE: usize = 4096;

#[derive(Clone, Copy)]
pub struct Message {
    pub mtype: u64,
    pub mlen:  u16,
    pub mdata: [u8; 128],
}

impl Message {
    pub const fn new() -> Self {
        Message { mtype: 0, mlen: 0, mdata: [0u8; 128] }
    }
}

pub struct MessageQueue {
    msgs:   [Message; MSGQ_MAX_MSGS],
    head:   usize,
    tail:   usize,
    count:  usize,
}

impl MessageQueue {
    pub const fn new() -> Self {
        const EMPTY: Message = Message::new();
        MessageQueue { msgs: [EMPTY; MSGQ_MAX_MSGS], head: 0, tail: 0, count: 0 }
    }
    pub fn send(&mut self, msg: Message) -> bool {
        if self.count >= MSGQ_MAX_MSGS { return false; }
        self.msgs[self.tail] = msg;
        self.tail = (self.tail + 1) % MSGQ_MAX_MSGS;
        self.count += 1;
        true
    }
    pub fn recv(&mut self, mtype: u64) -> Option<Message> {
        if self.count == 0 { return None; }
        // type 0 = any
        for i in 0..self.count {
            let idx = (self.head + i) % MSGQ_MAX_MSGS;
            if mtype == 0 || self.msgs[idx].mtype == mtype {
                let msg = self.msgs[idx];
                // Compact
                for j in i..self.count - 1 {
                    let a = (self.head + j) % MSGQ_MAX_MSGS;
                    let b = (self.head + j + 1) % MSGQ_MAX_MSGS;
                    self.msgs[a] = self.msgs[b];
                }
                self.tail = (self.tail + MSGQ_MAX_MSGS - 1) % MSGQ_MAX_MSGS;
                self.count -= 1;
                return Some(msg);
            }
        }
        None
    }
    pub fn len(&self) -> usize { self.count }
}

// ─── Shared Memory Region ────────────────────────────────────────────────────
pub const SHM_MAX_REGIONS: usize = 64;
pub const SHM_MAX_SIZE:    usize = 4 * 1024 * 1024; // 4 MB per region

pub struct ShmRegion {
    pub key:    u32,
    pub size:   usize,
    pub phys:   u64,     // physical address of backing pages
    pub refs:   u32,
    pub flags:  u32,
}

pub const SHM_IPC_PRIVATE: u32 = 0;
pub const SHM_RDONLY: u32 = 0x1000;

pub struct ShmTable {
    regions: [ShmRegion; SHM_MAX_REGIONS],
    count:   usize,
}

impl ShmTable {
    pub const fn new() -> Self {
        const EMPTY: ShmRegion = ShmRegion { key: 0, size: 0, phys: 0, refs: 0, flags: 0 };
        ShmTable { regions: [EMPTY; SHM_MAX_REGIONS], count: 0 }
    }
    pub fn create(&mut self, key: u32, size: usize, phys: u64, flags: u32) -> Option<usize> {
        if self.count >= SHM_MAX_REGIONS { return None; }
        let idx = self.count;
        self.regions[idx] = ShmRegion { key, size, phys, refs: 1, flags };
        self.count += 1;
        Some(idx)
    }
    pub fn attach(&mut self, key: u32) -> Option<u64> {
        for r in &mut self.regions[..self.count] {
            if r.key == key { r.refs += 1; return Some(r.phys); }
        }
        None
    }
    pub fn detach(&mut self, key: u32) {
        for r in &mut self.regions[..self.count] {
            if r.key == key && r.refs > 0 { r.refs -= 1; }
        }
    }
    pub fn remove(&mut self, key: u32) -> bool {
        for i in 0..self.count {
            if self.regions[i].key == key {
                self.regions[i] = self.regions[self.count - 1];
                self.count -= 1;
                return true;
            }
        }
        false
    }
}
