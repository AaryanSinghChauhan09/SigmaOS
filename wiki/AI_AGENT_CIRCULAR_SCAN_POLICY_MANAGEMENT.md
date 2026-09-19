# AI Agent Circular SCAN (C-SCAN) Policy Management Guide

## Overview
This wiki guide details Circular SCAN (C-SCAN) elevator block scheduling management protocols for AI coding agents operating on SigmaOS. It covers queue data structures, sector LBA ordering, read/write queue prioritization, head sweep direction, wrap-around semantics, and zero-allocation queue management.

## Key Principles
1. **Unidirectional Sweep**: Disk head services sectors strictly in ascending sector order ($LBA_0 \to LBA_{max}$).
2. **Deterministic Wrap-Around**: When no pending requests remain at or above `head_pos`, the head wraps back to sector 0 without servicing requests during return movement.
3. **Read Prioritization**: `read_queue` requests are dispatched prior to `write_queue` requests to prevent process read stalling.

## C-SCAN Dispatch Logic (`src/kernel/block_dev.rs`)
```rust
pub fn dispatch_next(&mut self) -> Option<Bio> {
    if let Some(bio) = self.dispatch.pop_front() {
        return Some(bio);
    }

    // Serve reads preferentially; find next sector >= head
    if let Some((&sector, _)) = self.read_queue.range(self.head_pos..).next() {
        let bio = self.read_queue.remove(&sector).unwrap();
        self.head_pos = bio.end_sector();
        return Some(bio);
    }
    // Wrap around (C-SCAN)
    if let Some((&sector, _)) = self.read_queue.iter().next() {
        let bio = self.read_queue.remove(&sector).unwrap();
        self.head_pos = bio.end_sector();
        return Some(bio);
    }
    // Then writes
    if let Some((&sector, _)) = self.write_queue.range(self.head_pos..).next() {
        let bio = self.write_queue.remove(&sector).unwrap();
        self.head_pos = bio.end_sector();
        return Some(bio);
    }
    if let Some((&sector, _)) = self.write_queue.iter().next() {
        let bio = self.write_queue.remove(&sector).unwrap();
        self.head_pos = bio.end_sector();
        return Some(bio);
    }
    None
}
```

## Related Documents
- `docs/AI_AGENT_CIRCULAR_SCAN_POLICY_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_CIRCULAR_SCAN_POLICY_MANAGEMENT_GUIDELINES.md`
- `docs/AI_AGENT_IO_MANAGEMENT_ARCHITECTURE.md`
