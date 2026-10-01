// SigmaOS Zircon-inspired Channels, Handles, FIFOs, and Signals
// Inspired by Google Fuchsia's Zircon microkernel IPC primitives:
// Handles as unforgeable capability tokens, typed channels, FIFOs, signals.

use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex, atomic::{AtomicU32, AtomicU64, Ordering}};

// ─────────────────────────────────────────────────────────────────────────────
// Handle Rights (capability enforcement)
// ─────────────────────────────────────────────────────────────────────────────

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rights: u32 {
        const NONE       = 0;
        const DUPLICATE  = 1 << 0;
        const TRANSFER   = 1 << 1;
        const READ       = 1 << 2;
        const WRITE      = 1 << 3;
        const EXECUTE    = 1 << 4;
        const MAP        = 1 << 5;
        const GET_PROP   = 1 << 6;
        const SET_PROP   = 1 << 7;
        const ENUMERATE  = 1 << 8;
        const DESTROY    = 1 << 9;
        const SET_POLICY = 1 << 10;
        const GET_CHILD  = 1 << 11;
        const SIGNAL     = 1 << 12;
        const WAIT       = 1 << 13;
        const INSPECT    = 1 << 14;
        const SAME_RIGHTS = 1 << 31;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Kernel Object Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KobjType {
    Channel,
    Fifo,
    Event,
    EventPair,
    Timer,
    Port,
    Vmo,  // Virtual Memory Object
    Process,
    Thread,
    Job,
}

pub type ZxHandle = u32;
pub type Koid = u64;  // Kernel Object ID

static KOID_COUNTER: AtomicU64 = AtomicU64::new(1);
fn new_koid() -> Koid { KOID_COUNTER.fetch_add(1, Ordering::SeqCst) }

// ─────────────────────────────────────────────────────────────────────────────
// Signals
// ─────────────────────────────────────────────────────────────────────────────

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Signals: u32 {
        const NONE             = 0;
        const READABLE         = 1 << 0;  // ZX_CHANNEL_READABLE
        const WRITABLE         = 1 << 1;  // ZX_CHANNEL_WRITABLE
        const PEER_CLOSED      = 1 << 2;  // ZX_CHANNEL_PEER_CLOSED
        const SIGNALED         = 1 << 3;  // ZX_EVENT_SIGNALED
        const FIFO_READABLE    = 1 << 4;
        const FIFO_WRITABLE    = 1 << 5;
        const HANDLE_CLOSED    = 1 << 23; // ZX_SIGNAL_HANDLE_CLOSED
        const LAST_HANDLE      = 1 << 24;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Channel Message
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ZxMessage {
    pub bytes: Vec<u8>,
    pub handles: Vec<ZxHandle>, // handle transfer
}

impl ZxMessage {
    pub fn new(bytes: Vec<u8>) -> Self { ZxMessage { bytes, handles: Vec::new() } }
    pub fn with_handles(bytes: Vec<u8>, handles: Vec<ZxHandle>) -> Self { ZxMessage { bytes, handles } }
}

// ─────────────────────────────────────────────────────────────────────────────
// Channel (bidirectional message-passing endpoint pair)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ChannelEndpoint {
    pub koid: Koid,
    pub queue: Mutex<VecDeque<ZxMessage>>,
    pub signals: AtomicU32,
    pub peer_closed: AtomicU32,
}

impl ChannelEndpoint {
    fn new() -> Arc<Self> {
        Arc::new(ChannelEndpoint {
            koid: new_koid(),
            queue: Mutex::new(VecDeque::new()),
            signals: AtomicU32::new(Signals::WRITABLE.bits()),
            peer_closed: AtomicU32::new(0),
        })
    }

    pub fn write(&self, msg: ZxMessage) -> ZxStatus {
        if self.peer_closed.load(Ordering::Relaxed) != 0 {
            return ZxStatus::PeerClosed;
        }
        self.queue.lock().unwrap().push_back(msg);
        self.signals.fetch_or(Signals::READABLE.bits(), Ordering::Release);
        ZxStatus::Ok
    }

    pub fn read(&self) -> Result<ZxMessage, ZxStatus> {
        let mut q = self.queue.lock().unwrap();
        if let Some(msg) = q.pop_front() {
            if q.is_empty() {
                self.signals.fetch_and(!Signals::READABLE.bits(), Ordering::Release);
            }
            Ok(msg)
        } else {
            Err(ZxStatus::ShouldWait)
        }
    }

    pub fn close(&self) {
        self.peer_closed.store(1, Ordering::Release);
        self.signals.fetch_or(Signals::PEER_CLOSED.bits(), Ordering::Release);
    }

    pub fn signals(&self) -> Signals {
        Signals::from_bits_truncate(self.signals.load(Ordering::Relaxed))
    }

    pub fn pending_messages(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}

/// A channel pair: (local, peer) — like zx_channel_create()
pub struct Channel {
    pub local: Arc<ChannelEndpoint>,
    pub peer: Arc<ChannelEndpoint>,
    pub local_koid: Koid,
    pub peer_koid: Koid,
}

impl Channel {
    pub fn create() -> (Channel, Channel) {
        let a = ChannelEndpoint::new();
        let b = ChannelEndpoint::new();
        let ka = a.koid;
        let kb = b.koid;
        let ch1 = Channel { local: Arc::clone(&a), peer: Arc::clone(&b), local_koid: ka, peer_koid: kb };
        let ch2 = Channel { local: Arc::clone(&b), peer: Arc::clone(&a), local_koid: kb, peer_koid: ka };
        (ch1, ch2)
    }

    pub fn write(&self, msg: ZxMessage) -> ZxStatus { self.peer.write(msg) }
    pub fn read(&self) -> Result<ZxMessage, ZxStatus> { self.local.read() }
    pub fn close(self) { self.local.close(); }
}

// ─────────────────────────────────────────────────────────────────────────────
// FIFO (fixed-element ring buffer channel)
// ─────────────────────────────────────────────────────────────────────────────

pub struct ZxFifo {
    pub koid: Koid,
    pub element_count: usize,
    pub element_size: usize,
    buffer: Mutex<VecDeque<Vec<u8>>>,
    pub signals: AtomicU32,
}

impl ZxFifo {
    pub fn new(count: usize, elem_size: usize) -> Arc<Self> {
        Arc::new(ZxFifo {
            koid: new_koid(),
            element_count: count,
            element_size: elem_size,
            buffer: Mutex::new(VecDeque::with_capacity(count)),
            signals: AtomicU32::new(Signals::FIFO_WRITABLE.bits()),
        })
    }

    pub fn write(&self, items: &[u8], count: usize) -> (ZxStatus, usize) {
        let mut buf = self.buffer.lock().unwrap();
        let avail = self.element_count - buf.len();
        let to_write = count.min(avail);
        for i in 0..to_write {
            let start = i * self.element_size;
            let end = start + self.element_size;
            if end <= items.len() {
                buf.push_back(items[start..end].to_vec());
            }
        }
        if !buf.is_empty() {
            self.signals.fetch_or(Signals::FIFO_READABLE.bits(), Ordering::Release);
        }
        if buf.len() == self.element_count {
            self.signals.fetch_and(!Signals::FIFO_WRITABLE.bits(), Ordering::Release);
        }
        if to_write < count { (ZxStatus::OutOfRange, to_write) } else { (ZxStatus::Ok, to_write) }
    }

    pub fn read(&self, count: usize) -> (ZxStatus, Vec<Vec<u8>>) {
        let mut buf = self.buffer.lock().unwrap();
        let to_read = count.min(buf.len());
        let items: Vec<Vec<u8>> = buf.drain(..to_read).collect();
        if buf.is_empty() {
            self.signals.fetch_and(!Signals::FIFO_READABLE.bits(), Ordering::Release);
        }
        self.signals.fetch_or(Signals::FIFO_WRITABLE.bits(), Ordering::Release);
        (ZxStatus::Ok, items)
    }

    pub fn available_read(&self) -> usize { self.buffer.lock().unwrap().len() }
}

// ─────────────────────────────────────────────────────────────────────────────
// Event / EventPair
// ─────────────────────────────────────────────────────────────────────────────

pub struct ZxEvent {
    pub koid: Koid,
    pub signals: AtomicU32,
}

impl ZxEvent {
    pub fn new() -> Arc<Self> {
        Arc::new(ZxEvent { koid: new_koid(), signals: AtomicU32::new(0) })
    }

    pub fn signal(&self, clear: Signals, set: Signals) -> ZxStatus {
        let mut s = self.signals.load(Ordering::Relaxed);
        s &= !clear.bits();
        s |= set.bits();
        self.signals.store(s, Ordering::Release);
        ZxStatus::Ok
    }

    pub fn wait(&self, mask: Signals) -> Signals {
        Signals::from_bits_truncate(self.signals.load(Ordering::Relaxed)) & mask
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ZxStatus error codes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZxStatus {
    Ok,
    Internal,
    NotSupported,
    NoResources,
    NoMemory,
    InvalidArgs,
    BadHandle,
    WrongType,
    BadSyscall,
    OutOfRange,
    BufferTooSmall,
    BadState,
    TimedOut,
    ShouldWait,
    Canceled,
    PeerClosed,
    NotFound,
    AlreadyExists,
    AlreadyBound,
    Unavailable,
    AccessDenied,
}

// ─────────────────────────────────────────────────────────────────────────────
// Handle Table (kernel-side mapping of ZxHandle → KernelObject)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum KernelObjectRef {
    Channel(Koid),
    Fifo(Koid),
    Event(Koid),
    Vmo(Koid),
}

#[derive(Debug, Clone)]
pub struct HandleEntry {
    pub kobj: KernelObjectRef,
    pub rights: Rights,
    pub koid: Koid,
}

pub struct ZxHandleTable {
    table: BTreeMap<ZxHandle, HandleEntry>,
    next_handle: AtomicU32,
}

impl ZxHandleTable {
    pub fn new() -> Self {
        ZxHandleTable { table: BTreeMap::new(), next_handle: AtomicU32::new(1) }
    }

    pub fn insert(&mut self, entry: HandleEntry) -> ZxHandle {
        let h = self.next_handle.fetch_add(1, Ordering::SeqCst);
        self.table.insert(h, entry);
        h
    }

    pub fn lookup(&self, h: ZxHandle) -> Option<&HandleEntry> { self.table.get(&h) }

    pub fn close(&mut self, h: ZxHandle) -> ZxStatus {
        if self.table.remove(&h).is_some() { ZxStatus::Ok } else { ZxStatus::BadHandle }
    }

    pub fn duplicate(&mut self, h: ZxHandle, new_rights: Rights) -> Result<ZxHandle, ZxStatus> {
        let entry = self.table.get(&h).ok_or(ZxStatus::BadHandle)?;
        if !entry.rights.contains(Rights::DUPLICATE) { return Err(ZxStatus::AccessDenied); }
        let new_entry = HandleEntry {
            kobj: entry.kobj.clone(),
            rights: new_rights,
            koid: entry.koid,
        };
        Ok(self.insert(new_entry))
    }

    pub fn count(&self) -> usize { self.table.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_create_write_read() {
        let (ch1, ch2) = Channel::create();
        let msg = ZxMessage::new(b"hello zircon".to_vec());
        assert_eq!(ch1.write(msg), ZxStatus::Ok);
        let received = ch2.read().unwrap();
        assert_eq!(received.bytes, b"hello zircon");
    }

    #[test]
    fn test_channel_signals() {
        let (ch1, ch2) = Channel::create();
        // Initially writable, not readable
        assert!(ch1.local.signals().contains(Signals::WRITABLE));
        assert!(!ch2.local.signals().contains(Signals::READABLE));
        ch1.write(ZxMessage::new(vec![1, 2, 3]));
        // ch2 should now have READABLE
        assert!(ch2.local.signals().contains(Signals::READABLE));
    }

    #[test]
    fn test_channel_peer_closed() {
        let (ch1, ch2) = Channel::create();
        ch1.close();
        // ch2 should see PEER_CLOSED
        assert!(ch2.local.signals().contains(Signals::PEER_CLOSED));
        // Write to closed peer should fail
        assert_eq!(ch2.write(ZxMessage::new(vec![])), ZxStatus::PeerClosed);
    }

    #[test]
    fn test_fifo_write_read() {
        let fifo = ZxFifo::new(8, 4);
        let data = [1u8, 2, 3, 4,   5, 6, 7, 8]; // 2 elements of 4 bytes
        let (status, written) = fifo.write(&data, 2);
        assert_eq!(status, ZxStatus::Ok);
        assert_eq!(written, 2);
        let (_, items) = fifo.read(2);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_event_signal_wait() {
        let event = ZxEvent::new();
        event.signal(Signals::NONE, Signals::SIGNALED);
        let observed = event.wait(Signals::SIGNALED);
        assert!(observed.contains(Signals::SIGNALED));
        event.signal(Signals::SIGNALED, Signals::NONE);
        assert!(event.wait(Signals::SIGNALED).is_empty());
    }

    #[test]
    fn test_handle_table_duplicate_rights() {
        let mut table = ZxHandleTable::new();
        let entry = HandleEntry {
            kobj: KernelObjectRef::Event(42),
            rights: Rights::READ | Rights::DUPLICATE,
            koid: 42,
        };
        let h1 = table.insert(entry);
        let h2 = table.duplicate(h1, Rights::READ).unwrap();
        assert_ne!(h1, h2);
        assert_eq!(table.count(), 2);
        table.close(h1);
        assert_eq!(table.count(), 1);
    }

    #[test]
    fn test_handle_duplicate_without_right_fails() {
        let mut table = ZxHandleTable::new();
        let entry = HandleEntry {
            kobj: KernelObjectRef::Channel(1),
            rights: Rights::READ | Rights::WRITE, // no DUPLICATE
            koid: 1,
        };
        let h = table.insert(entry);
        assert_eq!(table.duplicate(h, Rights::READ), Err(ZxStatus::AccessDenied));
    }
}
