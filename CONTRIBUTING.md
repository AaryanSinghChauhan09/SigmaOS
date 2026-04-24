# Contributing to SigmaOS

Welcome to the Sovereign Lattice! We aim to build an industrial-grade, zero-dependency, capability-based microkernel OS.

## Core Philosophy: Zero Dependency
SigmaOS strictly adheres to a "pure low-level" philosophy.
- **No high-level interpreters**: Python, JS, and Bash are prohibited for core OS components and build pipelines. We orchestrate entirely in C/C++.
- **No external frameworks**: The OS must be self-reliant. Do not pull in heavy external dependencies (`npm`, large `cargo` trees, etc.).
- **No standard library usage in kernel**: The kernel executes purely on bare metal. Do not `#include <stdlib.h>`, `<stdarg.h>`, etc. Use `__builtin` macros and the internal `SovereignLibC`.

## Architecture & Modularisation
- **Microkernel Isolation**: Every feature (VFS, Networking, Tensor acceleration) must be an isolated shard.
- **Strict IPC Interfaces**: Shards communicate exclusively via the capability-checked message passing ring buffer. Shared memory is prohibited except for DMA buffers explicitly leased to the NPU/GPU.
- **Microservices-style Modularity**: Break down complex subsystems into independent modules within `suites/`.

## Coding Standards
- **C/C++ Standard**: C11 for kernel shards, C++20 for host toolchain (`orchestrator`).
- **Formatting**: We use a unified formatting style. Avoid inline styles or macros that obscure program flow.
- **Inline Assembly**: Permitted only for CPU-specific architectures (e.g., interrupt descriptor setup, page table flushing, TLB operations). Confine all inline assembly to `S04_HAL`.
- **Memory Safety**: Where C is used, allocations must be statically bounded or managed via the custom slab allocator (`sigma_slab_alloc_raw`).

## Development Workflow & Automations
1. **Build Toolchain**: Use the native `s-cli`. Run `./s-cli build <arch>` to build locally.
2. **Static Analysis**: Ensure your code passes `clang-tidy`. Our CI automatically rejects code that fails static analysis checks.
3. **Commit Messages**: Write clear, descriptive commit messages outlining what component was modified.

By contributing to SigmaOS, you agree to uphold the sovereign mandate of architectural purity.
