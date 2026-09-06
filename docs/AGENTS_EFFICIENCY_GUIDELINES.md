# SigmaOS AI Agent Efficiency Guidelines & Performance Architecture

This document specifies mandatory efficiency rules, SIMD vectorization directives, memory locality optimization patterns, and zero-copy performance standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Computational & Algorithmic Efficiency Principles
- **Zero-Allocation Hot Paths**: Performance-critical routines (packet dispatch, syscall routing, scheduler loops) must perform zero dynamic heap allocations.
- **Optimal Time Complexity**: Use O(1) or O(log N) lookup algorithms (hash maps, static B-trees, binary search) rather than O(N^2) nested search loops.
- **Cache-Friendly Data Structures**: Prefer contiguous slice/array layouts (`Vec`, fixed-size buffers) to minimize CPU L1/L2 cache misses.

## 2. Hardware Acceleration & SIMD Vectorization
- **Instruction Set Architecture Auto-Routing (`src/klib/isa.rs`)**:
  - Automatically detect ISA feature levels (x86-64-v1 through v4, AVX2, AVX-512, NEON, SVE2, RISC-V Vector 1.0).
  - Route bulk memory copies (`memcpy`), hashing, and crypto primitives to vectorized intrinsics.

## 3. Asynchronous Zero-Copy I/O Subsystems
- **Zero-Copy Ring Buffers**: Utilize lock-free ring buffers (`src/klib/ringbuf.rs`, `src/klib/ring_buffer.rs`) for IPC and network packet processing.
- **eBPF & XDP Fast-Paths**: Fast-path packet filtering and socket redirection must utilize zero-copy XDP ring buffers before userland context switches.

## 4. AI Agent Performance Directives
1. **Minimize Memory Footprint**: Bounded data structures with pre-allocated capacities.
2. **Profile & Benchmark**: Verify performance improvements using microbenchmarks and Phoronix-style automated metrics.
