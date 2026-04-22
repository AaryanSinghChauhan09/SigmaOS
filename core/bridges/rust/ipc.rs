/// core/ipc.rs — Low-level inter-shard communication
/// Lock-free SPSC ring buffer — no mutexes, no channels from heavy runtimes.
/// Shards communicate by passing fixed-size messages through shared memory slots.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

pub const IPC_MSG_SIZE:  usize = 128;
pub const IPC_RING_CAPACITY: usize = 256;

/// IPC message — fixed-size to avoid heap allocation
#[derive(Clone, Copy)]
pub struct IpcMessage {
    pub tag:     [u8; 16],           // event type / topic
    pub payload: [u8; IPC_MSG_SIZE], // raw data
    pub len:     usize,              // payload length
    pub src:     u32,                // sender shard id
    pub dst:     u32,                // 0 = broadcast
}

impl IpcMessage {
    pub fn new(tag: &str, payload: &[u8], src: u32, dst: u32) -> Self {
        let mut msg = Self {
            tag: [0u8; 16], payload: [0u8; IPC_MSG_SIZE],
            len: payload.len().min(IPC_MSG_SIZE), src, dst,
        };
        let tb = tag.as_bytes();
        let tl = tb.len().min(16);
        msg.tag[..tl].copy_from_slice(&tb[..tl]);
        msg.payload[..msg.len].copy_from_slice(&payload[..msg.len]);
        msg
    }

    pub fn tag_str(&self) -> &str {
        let end = self.tag.iter().position(|&b| b == 0).unwrap_or(16);
        std::str::from_utf8(&self.tag[..end]).unwrap_or("?")
    }
}

/// Single-Producer Single-Consumer ring buffer (lock-free)
pub struct IpcRing {
    buf:  UnsafeCell<[IpcMessage; IPC_RING_CAPACITY]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

// Safety: designed for single producer + single consumer
unsafe impl Send for IpcRing {}
unsafe impl Sync for IpcRing {}

impl IpcRing {
    pub fn new() -> Self {
        Self {
            buf:  UnsafeCell::new([IpcMessage {
                tag: [0; 16], payload: [0; IPC_MSG_SIZE], len: 0, src: 0, dst: 0
            }; IPC_RING_CAPACITY]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn send(&self, msg: IpcMessage) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let next = (head + 1) % IPC_RING_CAPACITY;
        if next == self.tail.load(Ordering::Acquire) {
            return false; // ring full
        }
        unsafe { (*self.buf.get())[head] = msg; }
        self.head.store(next, Ordering::Release);
        true
    }

    pub fn recv(&self) -> Option<IpcMessage> {
        let tail = self.tail.load(Ordering::Relaxed);
        if tail == self.head.load(Ordering::Acquire) {
            return None; // empty
        }
        let msg = unsafe { (*self.buf.get())[tail] };
        self.tail.store((tail + 1) % IPC_RING_CAPACITY, Ordering::Release);
        Some(msg)
    }

    pub fn pending(&self) -> usize {
        let h = self.head.load(Ordering::Relaxed);
        let t = self.tail.load(Ordering::Relaxed);
        (h + IPC_RING_CAPACITY - t) % IPC_RING_CAPACITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_send_recv_roundtrip() {
        let ring = IpcRing::new();
        let msg = IpcMessage::new("shard:boot", b"S01_Genesis", 1, 0);
        assert!(ring.send(msg));
        let got = ring.recv().expect("should have message");
        assert_eq!(got.tag_str(), "shard:boot");
        assert_eq!(&got.payload[..got.len], b"S01_Genesis");
    }
    #[test]
    fn test_empty_recv_returns_none() {
        let ring = IpcRing::new();
        assert!(ring.recv().is_none());
    }
}
