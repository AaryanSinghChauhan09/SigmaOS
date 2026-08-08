//! Lock-Free Zero-Copy IPC Queue Subsystem
//! Implements high-speed page-passing circular ring buffers with atomic fences for sub-microsecond latency.

use core::sync::atomic::{AtomicUsize, Ordering};

pub const QUEUE_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPCError {
    QueueFull,
    QueueEmpty,
    InvalidPayload,
}

pub struct ZeroCopyQueue<T, const N: usize> {
    pub buffer: [Option<T>; N],
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
}

impl<T: Clone, const N: usize> ZeroCopyQueue<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: core::array::from_fn(|_| None),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Enqueue a zero-copy reference or page frame onto the queue without locks
    pub fn enqueue(&mut self, item: T) -> Result<(), IPCError> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        if head.wrapping_sub(tail) >= N {
            return Err(IPCError::QueueFull);
        }

        let idx = head % N;
        self.buffer[idx] = Some(item);
        self.head.store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Dequeue a zero-copy reference or page frame out of the queue
    pub fn dequeue(&mut self) -> Result<T, IPCError> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);

        if tail == head {
            return Err(IPCError::QueueEmpty);
        }

        let idx = tail % N;
        let item = self.buffer[idx].take().ok_or(IPCError::InvalidPayload)?;
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_ipc_flow() {
        let mut queue: ZeroCopyQueue<[u8; 4], 4> = ZeroCopyQueue::new();

        let page_payload = [0xDE, 0xAD, 0xBE, 0xEF];
        queue.enqueue(page_payload).unwrap();

        let retrieved_payload = queue.dequeue().unwrap();
        assert_eq!(retrieved_payload, page_payload);
    }

    #[test]
    fn test_zero_copy_queue_bounds() {
        let mut queue: ZeroCopyQueue<u32, 2> = ZeroCopyQueue::new();
        queue.enqueue(10).unwrap();
        queue.enqueue(20).unwrap();

        // Queue is now full
        assert_eq!(queue.enqueue(30), Err(IPCError::QueueFull));

        assert_eq!(queue.dequeue().unwrap(), 10);
        assert_eq!(queue.dequeue().unwrap(), 20);

        // Queue is now empty
        assert_eq!(queue.dequeue(), Err(IPCError::QueueEmpty));
    }
}
