# SigmaOS

**SigmaOS** is a sovereign, zero-allocation, `no_std` operating system built entirely in Rust. It discards legacy POSIX assumptions to build a hyper-secure, capability-based microkernel designed for an AI-first, object-oriented ecosystem.

## Core Capabilities

SigmaOS has undergone a massive architectural upgrade to include modern Computer Science and Data Science paradigms natively.

### 1. Sovereign Kernel (`no_std`)

- **Capability Tokens**: A 64-bit hardware-enforced permission model (bypassing legacy ACLs).

- **Zero-Copy IPC**: SPSC ring-buffers for inter-shard communication.

- **SigmaFS & VFS**: Copy-on-Write (CoW) filesystem with deterministic extent mapping.

- **Self-Healing Watchdogs**: Autonomous fault detection and exponential backoff restart policies.

### 2. Artificial Intelligence & Data Science

- **Local LLM Backend**: An embedded AI Task Orchestrator that natively routes prioritized prompts (Background, Interactive, Critical) without requiring external userland wrappers.

- **Embedded ML**: Zero-allocation K-Means Clustering and FFT algorithms baked into the OS telemetry layer for autonomous system optimization.

### 3. Cyber Security & Isolation

- **Security Center Daemon**: Actively monitors the immutable, BLAKE3-linked kernel audit logs. Applies temporal decay heuristics to identify threats (e.g., sandbox escape attempts) and kills malicious shards autonomously.

- **Sovereign Sandboxes**: Strict CPU, Memory, and Network limits applied via CLI to untrusted code.

### 4. Zenith Desktop (UI/UX)

- **Object-Oriented UI**: A Trait-based Widget framework operating entirely without heap allocations.

- **BSP Window Manager**: Binary Space Partitioning tiling engine.

- **Glassmorphism Profiles**: Declarative UI parsing via `~/.sigma_profile` supporting dynamic theming.

## System Architecture

```mermaid
graph TD
    subgraph Userland (Zenith Desktop)
        UI[Zenith Compositor & Shell]
        Logic[Sigma Logic Automation]
        AI[Local LLM Context Mgr]
    end

    subgraph Daemons
        SEC[Security Center]
        MONITOR[Observability Telemetry]
        STORE[Sigma DB Key-Value]
    end

    subgraph Kernel (no_std)
        IPC[SPSC Ring Buffer IPC]
        CAP[Capability Token Auth]
        VFS[Virtual File System]
        MEM[Bitmap Page Allocator]
    end

    UI <--> IPC
    Logic <--> IPC
    AI <--> IPC
    SEC <--> CAP
    MONITOR <--> MEM
    IPC <--> CAP
```

## Getting Started

See [Installation](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Installation) for build instructions using the new `Justfile` toolchain.

## Documentation

Comprehensive documentation is available in the [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki):

- [Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture) - System architecture and design
- [Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Roadmap) - Development roadmap and milestones
- [Installation](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Installation) - Build and installation guide
- [Quickstart](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Quickstart) - Quick start guide
- [Contributing](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Contributing) - Contribution guidelines
- [Security](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security) - Security policy and reporting
- [Testing](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Testing) - Testing framework and tools
- [Developer Tools](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Developer-Tools) - Development tools and IDE integration
