# SigmaOS Process Table and PID Policy (`docs/process/PID_POLICY.md`)

## 1. Scope & Objective

This document governs Process ID (PID) allocation, process table bounds, lookup thread-safety, PID reuse policies, and exhaustion recovery mechanisms in SigmaOS.

## 2. PID Allocation Principles

1. **Race-Free Allocation**: PID allocation and deallocation operations must be atomic and race-free under concurrent multi-threaded or multi-core process creation.
2. **Bounded Capacity**: Process table capacity is strictly bounded (`MAX_PROCESSES`, e.g., 4096 processes) to prevent uncontrolled kernel memory consumption.
3. **PID Reuse Delay Policy**: Once a PID is reaped, it cannot be immediately reallocated. A bitmap/free queue delay policy prevents PID recycling races and handle confusion.
4. **Typed Error Returns**: PID exhaustion or capacity exhaustion returns explicit typed errors (`PidError::Exhausted`, `ProcessTableError::TableFull`).

## 3. Process Table Structure & Lookup

```text
ProcessTable
 ├── PidAllocator (allocates and tracks active PIDs 1..32768)
 ├── ProcessEntry Table (Bounded Arc<ProcessEntry> slots)
 └── Subreaper Tree (Parent-child hierarchy and orphan remapping)
```

- **Lookup Safety**: `get_process(pid)` returns a thread-safe `Arc<ProcessEntry>` reference, ensuring process structures remain valid even if concurrently reaped.
- **Stale Protection**: Stale process table entries cannot be recycled until all references are dropped and the entry is marked `REAPED`.

## 4. Verification

PID allocation rules and thread safety are verified in `tests/process/pid_allocator.rs` and `src/process/table.rs`.
