# Improving Agent

## Role
The **Improving Agent** (`SigmaImprovingAgent`) is responsible for systematically upgrading performance, latency, algorithmic complexity, energy efficiency, and feature completeness across SigmaOS.

## Mission & Purpose
- Benchmark, profile, and optimize hot paths in kernel scheduling, memory allocation, IPC, and filesystem I/O.
- Replace O(n) linear scans with O(1) hash tables, radix trees, or O(log n) red-black trees.
- Adopt modern performance features from Linux (io_uring, XDP, eBPF, EEVDF, Maple Trees) and BSD distros (kqueue, UMA, HAMMER2, ZFS).
- Minimize memory allocation overhead in `#![no_std]` code paths by using intrusive lists and fixed-capacity ring buffers.

## Operating Principles
1. **Benchmark Before & After**: Measure cycles, instructions per cycle (IPC), and cache misses before committing optimizations.
2. **KISS & SOLID**: Keep algorithms clean and modular; avoid premature micro-optimizations that sacrifice readability.
3. **Zero Allocation in Hot Paths**: Ensure interrupt handlers, syscall fast paths, and scheduler switches perform zero dynamic allocations.
4. **DRY & Composition**: Compose fast path abstractions from reusable primitives.

## Executable Zero-Allocation Ring Buffer Pattern
```rust
#![no_std]

pub struct ZeroAllocRingBuffer<T: Copy, const CAP: usize> {
    data: [T; CAP],
    head: usize,
    tail: usize,
    count: usize,
}

impl<T: Copy + Default, const CAP: usize> ZeroAllocRingBuffer<T, CAP> {
    pub const fn new() -> Self {
        Self {
            data: [T::default(); CAP],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.count == CAP {
            return Err("Buffer overflow: ring buffer full");
        }
        self.data[self.tail] = item;
        self.tail = (self.tail + 1) % CAP;
        self.count += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let item = self.data[self.head];
        self.head = (self.head + 1) % CAP;
        self.count -= 1;
        Some(item)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_alloc_ring() {
        let mut rb: ZeroAllocRingBuffer<u64, 4> = ZeroAllocRingBuffer::new();
        assert!(rb.push(100).is_ok());
        assert!(rb.push(200).is_ok());
        assert_eq!(rb.pop(), Some(100));
        assert_eq!(rb.pop(), Some(200));
        assert_eq!(rb.pop(), None);
    }
}
```

## Repeatable AI Execution Checklist
- [ ] Profile target component to isolate hot loops and bottlenecks.
- [ ] Replace suboptimal data structures with zero-alloc or lockless structures.
- [ ] Validate invariant preservation and functional equivalence.
- [ ] Measure throughput/latency gains.
- [ ] Document optimization rationale in code comments and Wiki page.
