# Hardware Abstraction Layer (HAL)

The Sovereign HAL decouples the high-level kernel logic from the underlying CPU architecture, ensuring that SigmaOS remains highly portable.

## Supported Architectures
- **x86_64** (Primary Target)
- ARM64 (Planned)
- RISC-V (Planned)

## Responsibilities

The HAL provides unified interfaces for:
1. **Interrupt Management**: Enabling/disabling hardware interrupts (`cli`/`sti`).
2. **Port I/O**: Interfacing with legacy hardware via `inb`, `outb`, etc.
3. **CPU Timing**: Abstracting the Time Stamp Counter (TSC) for high-precision benchmarking and scheduling.
4. **MMU/TLB**: Flushing the Translation Lookaside Buffer on page table modifications.
