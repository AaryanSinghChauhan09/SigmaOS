# AI Agent Abstract Data Type (ADT) Operation Management Guidelines

## Purpose
These guidelines define operational rules, implementation patterns, and safety guardrails for AI coding agents implementing, modifying, or managing Abstract Data Types (ADTs) in SigmaOS (`src/klib/ring_buffer.rs`, `src/klib/queue.rs`, `src/klib/arena.rs`).

---

## Directives for AI Agents

1. **Memory Safety & Non-Overlapping Access**:
   - Always ensure pointer arithmetic in low-level unsafe ADT buffers adheres to strict non-overlapping slice rules (`core::slice::from_raw_parts_mut` and `copy_nonoverlapping`).

2. **Atomic Index Masking**:
   - Use bitwise masking (`head & (CAPACITY - 1)`) instead of modulo arithmetic (`head % CAPACITY`) for ring buffer index calculations. Enforce static power-of-two capacity assertions.

3. **Code Pattern: SPSC Lock-Free Ring Buffer Usage**:
```rust
let mut ring = SpscRingBuffer::<u32, 64>::new();
assert!(ring.push(42).is_ok());
assert_eq!(ring.pop(), Some(42));
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm ADT unit tests pass across kernel and userland modules.

---

## Related Files
- `src/klib/ring_buffer.rs`
- `src/klib/queue.rs`
- `src/klib/arena.rs`
- `docs/AGENTS_ABSTRACT_DATA_TYPE_OPERATION_MANAGEMENT.md`
- `wiki/AI_AGENT_ABSTRACT_DATA_TYPE_OPERATION_MANAGEMENT_SPEC.md`
