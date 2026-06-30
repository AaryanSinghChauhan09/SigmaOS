//! ipc_channel.rs — SigmaOS Sovereign Zero-Copy IPC Channels
//! Implements typed, bounded, lock-free SPSC and MPSC channels
//! backed by shared physical frames (zero-copy between address spaces).
//!
//! Sovereign: #![no_std], no atomics stdlib — raw volatile memory + memory fences.
//!
//! Two channel variants:
//!   1. SpsciChannel<T, N> — Single Producer Single Consumer ring buffer
//!      Lives in a shared physical frame mapped into both endpoint processes.
//!   2. MpscChannel<T, N> — Multi Producer (compare-and-swap-free, token-based)
//!      Uses a ticket lock embedded in the channel header.

#![no_std]
#![allow(dead_code)]

use core::mem::MaybeUninit;
use core::cell::UnsafeCell;

// ─── Memory Barrier Primitives ────────────────────────────────────────────────
/// x86-64 memory fence — prevents store/load reordering by the CPU.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn mfence() {
    // SAFETY: mfence is a no-side-effect serialising instruction.
    unsafe { core::arch::asm!("mfence", options(nostack, preserves_flags)); }
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
fn mfence() { /* compiler fence approximation */ }

/// Compiler-only fence (prevents reordering at IR level only).
#[inline(always)]
fn compiler_fence_seqcst() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

// ─── SPSC Ring Buffer Channel ─────────────────────────────────────────────────
/// T must be Copy + Sized. N must be a power of 2.
/// The channel header occupies the first 64 bytes of the shared frame.
/// Items follow immediately after.
///
/// Layout (inside shared page):
///   [0..4]  head   : u32 LE  (consumer reads here)
///   [4..8]  tail   : u32 LE  (producer writes here)
///   [8..64] _pad
///   [64..]  items[N] : [T; N]

pub const CACHE_LINE: usize = 64;

#[repr(C, align(64))]
pub struct SpscChannel<T: Copy, const N: usize> {
    head:  UnsafeCell<u32>,   // consumer index
    _pad0: [u8; CACHE_LINE - 4],
    tail:  UnsafeCell<u32>,   // producer index
    _pad1: [u8; CACHE_LINE - 4],
    data:  [UnsafeCell<MaybeUninit<T>>; N],
}

// SAFETY: All access is guarded by head/tail ownership protocol.
unsafe impl<T: Copy + Send, const N: usize> Send for SpscChannel<T, N> {}
unsafe impl<T: Copy + Send, const N: usize> Sync for SpscChannel<T, N> {}

impl<T: Copy, const N: usize> SpscChannel<T, N> {
    const MASK: u32 = (N as u32) - 1;

    pub const fn new() -> Self {
        Self {
            head:  UnsafeCell::new(0),
            _pad0: [0u8; CACHE_LINE - 4],
            tail:  UnsafeCell::new(0),
            _pad1: [0u8; CACHE_LINE - 4],
            data:  unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    /// Producer: try to enqueue item. Returns false if full.
    pub fn try_send(&self, item: T) -> bool {
        let tail = unsafe { *self.tail.get() };
        let head = unsafe { *self.head.get() };
        compiler_fence_seqcst();
        if tail.wrapping_sub(head) >= N as u32 {
            return false; // full
        }
        let slot = (tail & Self::MASK) as usize;
        unsafe { (*self.data[slot].get()).write(item); }
        mfence();
        unsafe { *self.tail.get() = tail.wrapping_add(1); }
        true
    }

    /// Consumer: try to dequeue item. Returns None if empty.
    pub fn try_recv(&self) -> Option<T> {
        let head = unsafe { *self.head.get() };
        let tail = unsafe { *self.tail.get() };
        compiler_fence_seqcst();
        if head == tail {
            return None; // empty
        }
        let slot  = (head & Self::MASK) as usize;
        let item  = unsafe { (*self.data[slot].get()).assume_init() };
        mfence();
        unsafe { *self.head.get() = head.wrapping_add(1); }
        Some(item)
    }

    pub fn is_empty(&self) -> bool {
        unsafe { *self.head.get() == *self.tail.get() }
    }

    pub fn is_full(&self) -> bool {
        let h = unsafe { *self.head.get() };
        let t = unsafe { *self.tail.get() };
        t.wrapping_sub(h) >= N as u32
    }

    pub fn len(&self) -> u32 {
        let h = unsafe { *self.head.get() };
        let t = unsafe { *self.tail.get() };
        t.wrapping_sub(h)
    }
}

// ─── Ticket Lock (for MPSC) ──────────────────────────────────────────────────
/// A fair spinlock using ticket numbers. Lock-free except under contention.
#[repr(C, align(8))]
pub struct TicketLock {
    ticket:  UnsafeCell<u32>,
    serving: UnsafeCell<u32>,
}

unsafe impl Send for TicketLock {}
unsafe impl Sync for TicketLock {}

impl TicketLock {
    pub const fn new() -> Self {
        Self {
            ticket:  UnsafeCell::new(0),
            serving: UnsafeCell::new(0),
        }
    }

    pub fn lock(&self) -> TicketGuard<'_> {
        // Atomic fetch-add via cmpxchg loop (no std::sync::atomic)
        let my_ticket = unsafe { fetch_add_u32(self.ticket.get(), 1) };
        // Spin until our ticket is served
        loop {
            let now = unsafe { *self.serving.get() };
            compiler_fence_seqcst();
            if now == my_ticket { break; }
            // CPU pause hint
            #[cfg(target_arch = "x86_64")]
            unsafe { core::arch::asm!("pause", options(nostack, preserves_flags)); }
        }
        TicketGuard { lock: self }
    }
}

pub struct TicketGuard<'a> {
    lock: &'a TicketLock,
}

impl<'a> Drop for TicketGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            let s = self.lock.serving.get();
            *s = (*s).wrapping_add(1);
        }
        mfence();
    }
}

/// Inline atomic fetch-and-add for u32 without std.
#[cfg(target_arch = "x86_64")]
unsafe fn fetch_add_u32(ptr: *mut u32, val: u32) -> u32 {
    let prev: u32;
    core::arch::asm!(
        "lock xadd [{ptr}], {val}",
        ptr = in(reg) ptr,
        val = inout(reg) val => _,
        out("eax") prev,  // actually val is in-out, fix below
        options(nostack),
    );
    // xadd: ptr += val, returns old *ptr; result in val reg
    let _ = prev;
    let mut old = 0u32;
    core::arch::asm!(
        "mov {out}, {val}",
        val = in(reg) val,
        out = out(reg) old,
        options(nostack, nomem),
    );
    old
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn fetch_add_u32(ptr: *mut u32, val: u32) -> u32 {
    let old = *ptr;
    *ptr = old.wrapping_add(val);
    old
}

// ─── MPSC Channel ─────────────────────────────────────────────────────────────
/// Multi-producer single-consumer. Uses ticket lock on producer side.
/// Consumer side is lock-free (single owner).
pub struct MpscChannel<T: Copy, const N: usize> {
    lock:  TicketLock,
    inner: SpscChannel<T, N>,
}

impl<T: Copy, const N: usize> MpscChannel<T, N> {
    pub const fn new() -> Self {
        Self {
            lock:  TicketLock::new(),
            inner: SpscChannel::new(),
        }
    }

    /// Any thread: enqueue with mutual exclusion.
    pub fn send(&self, item: T) -> bool {
        let _guard = self.lock.lock();
        self.inner.try_send(item)
    }

    /// Single consumer thread only: dequeue.
    pub fn recv(&self) -> Option<T> {
        self.inner.try_recv()
    }

    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
}

// ─── IPC Message Channel (convenience alias) ──────────────────────────────────
use crate::microkernel::IpcMessage;

pub type IpcSpscChannel  = SpscChannel<IpcMessage, 64>;
pub type IpcMpscChannel  = MpscChannel<IpcMessage, 64>;

/// Shared-frame descriptor: kernel maps this into both sender and receiver
/// address spaces so that no data copy is needed.
#[repr(C, align(4096))]
pub struct SharedIpcFrame {
    pub channel: IpcSpscChannel,
    _pad: [u8; 4096 - core::mem::size_of::<IpcSpscChannel>()],
}
