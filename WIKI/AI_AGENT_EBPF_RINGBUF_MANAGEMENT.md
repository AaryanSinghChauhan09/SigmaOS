# AI Agent Directive: eBPF Ring Buffer Subsystem Management

## Overview

The `BpfRingBufferEngine` (`src/kernel/ebpf.rs`) implements a zero-dependency, `#![no_std]` lock-free ring buffer engine providing Linux `BPF_MAP_TYPE_RINGBUF` kernel-to-userland event streaming parity for SigmaOS.

## Key Architectural Structures

1. **`BpfRingBufferEngine`**:
   - Manages single-consumer, multi-producer ring buffer allocations.
   - Enforces power-of-two ring buffer capacity constraints for bitwise modulo indexing (`offset & (capacity - 1)`).

2. **`BpfRingBufferSample`**:
   - Encapsulates individual event payloads with header metadata (`length`, `flags`, `producer_index`).

3. **Operations**:
   - `reserve(size: usize)`: Allocates space in the ring buffer or returns `Err("RingBuffer: Buffer full")`.
   - `submit(sample_id: u64)`: Marks reserved sample as committed and available for consumer polling.
   - `discard(sample_id: u64)`: Cancels reserved sample without dispatching to consumer.
   - `consume_next()`: Polling interface retrieving committed event samples.

## Directives for AI Agents

- **Lock-Free Concurrency**: Preserve atomic index mechanics (`consumer_pos`, `producer_pos`) for low-overhead telemetry streaming.
- **Verification**: Run standalone unit tests using:
  ```bash
  rustc --test --edition 2021 src/kernel/ebpf.rs -o build/ebpf_ringbuf_test && ./build/ebpf_ringbuf_test
  ```
