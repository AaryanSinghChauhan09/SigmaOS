//! SigmaOS — Lock-Free IPC Ring Buffer
//! SPSC (Single-Producer Single-Consumer) and MPSC variants.
//! Uses seqlock/atomic ordering via core::sync::atomic — no stdlib.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

type U8    = u8;
type U32   = u32;
type U64   = u64;
type Usize = usize;
type Bool  = bool;

// ── Constants ─────────────────────────────────────────────────────────────────
pub const RING_CAPACITY: Usize = 256;      // must be power of 2
const MASK: Usize = RING_CAPACITY - 1;
pub const MSG_MAX_BYTES: Usize = 512;      // max payload per message

// ── Message header ────────────────────────────────────────────────────────────
#[repr(C, align(64))]   // cache-line aligned
pub struct Message {
    pub src_pid:  U32,
    pub dst_pid:  U32,
    pub msg_type: U32,
    pub len:      U32,
    pub cap_token:U64,
    pub payload:  [U8; MSG_MAX_BYTES],
}

impl Message {
    pub const fn zero() -> Self {
        Message {
            src_pid: 0, dst_pid: 0, msg_type: 0, len: 0,
            cap_token: 0, payload: [0u8; MSG_MAX_BYTES],
        }
    }
}

// ── SPSC Ring Buffer ──────────────────────────────────────────────────────────
/// Cache-line separated head/tail to avoid false sharing.
#[repr(C)]
pub struct SpscRing {
    head: AtomicU32,       // written by consumer
    _pad0: [U8; 60],
    tail: AtomicU32,       // written by producer
    _pad1: [U8; 60],
    buf:  [Message; RING_CAPACITY],
}

impl SpscRing {
    /// Create a new ring buffer (const-zero initialised).
    pub const fn new() -> Self {
        // SAFETY: zero-init is valid for all field types
        unsafe { core::mem::zeroed() }
    }

    /// Try to send a message. Returns `true` on success, `false` if full.
    #[inline]
    pub fn try_send(&self, msg: &Message) -> Bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if (tail.wrapping_sub(head)) as Usize >= RING_CAPACITY {
            return false; // full
        }
        let slot = (tail as Usize) & MASK;
        unsafe {
            let dst = &self.buf[slot] as *const Message as *mut Message;
            core::ptr::copy_nonoverlapping(msg as *const Message, dst, 1);
        }
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        true
    }

    /// Try to receive a message. Returns `true` and fills `out` on success.
    #[inline]
    pub fn try_recv(&self, out: &mut Message) -> Bool {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        if head == tail { return false; } // empty
        let slot = (head as Usize) & MASK;
        unsafe {
            core::ptr::copy_nonoverlapping(
                &self.buf[slot] as *const Message, out as *mut Message, 1,
            );
        }
        self.head.store(head.wrapping_add(1), Ordering::Release);
        true
    }

    /// How many messages are currently in the ring.
    #[inline]
    pub fn len(&self) -> Usize {
        self.tail.load(Ordering::Relaxed)
            .wrapping_sub(self.head.load(Ordering::Relaxed)) as Usize
    }

    pub fn is_empty(&self) -> Bool { self.len() == 0 }
    pub fn is_full(&self)  -> Bool { self.len() >= RING_CAPACITY }
}

// ── MPSC (Multi-Producer, Single-Consumer) Ring ───────────────────────────────
/// Uses a CAS loop on tail to allow multiple producers.
#[repr(C)]
pub struct MpscRing {
    head: AtomicU32,
    _pad0: [U8; 60],
    tail: AtomicU32,
    _pad1: [U8; 60],
    commit: AtomicU32,    // marks how far data has been committed
    _pad2: [U8; 60],
    buf:  [Message; RING_CAPACITY],
}

impl MpscRing {
    pub const fn new() -> Self { unsafe { core::mem::zeroed() } }

    /// Try to enqueue from any thread/CPU. Returns `true` on success.
    pub fn try_send(&self, msg: &Message) -> Bool {
        loop {
            let tail = self.tail.load(Ordering::Relaxed);
            let head = self.head.load(Ordering::Acquire);
            if (tail.wrapping_sub(head)) as Usize >= RING_CAPACITY {
                return false;
            }
            // CAS to claim a slot
            match self.tail.compare_exchange_weak(
                tail, tail.wrapping_add(1),
                Ordering::AcqRel, Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let slot = (tail as Usize) & MASK;
                    unsafe {
                        let dst = &self.buf[slot] as *const Message as *mut Message;
                        core::ptr::copy_nonoverlapping(msg as *const Message, dst, 1);
                    }
                    // Spin-wait until we can advance commit
                    while self.commit.compare_exchange_weak(
                        tail, tail.wrapping_add(1),
                        Ordering::AcqRel, Ordering::Relaxed,
                    ).is_err() {
                        core::hint::spin_loop();
                    }
                    return true;
                }
                Err(_) => core::hint::spin_loop(),
            }
        }
    }

    /// Consume next message. Only safe to call from a single consumer.
    pub fn try_recv(&self, out: &mut Message) -> Bool {
        let head = self.head.load(Ordering::Relaxed);
        let commit = self.commit.load(Ordering::Acquire);
        if head == commit { return false; }
        let slot = (head as Usize) & MASK;
        unsafe {
            core::ptr::copy_nonoverlapping(
                &self.buf[slot] as *const Message, out as *mut Message, 1,
            );
        }
        self.head.store(head.wrapping_add(1), Ordering::Release);
        true
    }
}

// ── Capability-gated Channel ──────────────────────────────────────────────────
/// A named channel with a capability token for sender authentication.
pub struct CapChannel {
    pub channel_id: U32,
    pub owner_cap:  U64,
    ring: SpscRing,
}

impl CapChannel {
    pub const fn new(id: U32, cap: U64) -> Self {
        CapChannel { channel_id: id, owner_cap: cap, ring: SpscRing::new() }
    }

    /// Send with capability check.
    pub fn send(&self, msg: &Message) -> Bool {
        if msg.cap_token != self.owner_cap { return false; } // cap mismatch
        self.ring.try_send(msg)
    }

    pub fn recv(&self, out: &mut Message) -> Bool {
        self.ring.try_recv(out)
    }
}

// ── Global channel table ──────────────────────────────────────────────────────
const MAX_CHANNELS: Usize = 64;

static mut CHANNELS: [Option<CapChannel>; MAX_CHANNELS] = {
    // const-safe None array
    const NONE: Option<CapChannel> = None;
    [NONE; MAX_CHANNELS]
};

/// Register a new IPC channel. Returns channel ID or U32::MAX on error.
#[no_mangle]
pub unsafe extern "C" fn ipc_create_channel(cap: U64) -> U32 {
    for i in 0..MAX_CHANNELS {
        if CHANNELS[i].is_none() {
            CHANNELS[i] = Some(CapChannel::new(i as U32, cap));
            return i as U32;
        }
    }
    U32::MAX
}

/// Send a message on channel `id`. Returns 0 on success, -1 on error.
#[no_mangle]
pub unsafe extern "C" fn ipc_send(
    channel_id: U32, msg: *const Message,
) -> i32 {
    if msg.is_null() || channel_id as Usize >= MAX_CHANNELS { return -1; }
    match &CHANNELS[channel_id as Usize] {
        Some(ch) => if ch.send(&*msg) { 0 } else { -1 },
        None => -1,
    }
}

/// Receive a message from channel `id`. Returns 0 on success, -1 if empty.
#[no_mangle]
pub unsafe extern "C" fn ipc_recv(
    channel_id: U32, out: *mut Message,
) -> i32 {
    if out.is_null() || channel_id as Usize >= MAX_CHANNELS { return -1; }
    match &CHANNELS[channel_id as Usize] {
        Some(ch) => if ch.recv(&mut *out) { 0 } else { -1 },
        None => -1,
    }
}

/// Destroy a channel.
#[no_mangle]
pub unsafe extern "C" fn ipc_destroy_channel(channel_id: U32) {
    if channel_id as Usize < MAX_CHANNELS {
        CHANNELS[channel_id as Usize] = None;
    }
}
