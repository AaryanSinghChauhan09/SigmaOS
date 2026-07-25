# SigmaOS

**SigmaOS** is a sovereign, zero-allocation, `no_std` operating system built entirely in Rust. It discards legacy POSIX assumptions to build a hyper-secure, capability-based microkernel designed for an AI-first, object-oriented ecosystem.

## Core Capabilities

SigmaOS has undergone a massive architectural upgrade to include modern Computer Science and Data Science paradigms natively.

### 1. Sovereign Kernel (`no_std`)
- **Capability Tokens**: A 64-bit hardware-enforced permission model (bypassing legacy ACLs).
- **Zero-Copy IPC**: SPSC ring-buffers for inter-shard communication.
- **SigmaFS & VFS**: Copy-on-Write (CoW) filesystem with deterministic extent mapping.
- **Self-Healing Watchdogs**: Autonomous fault detection and exponential backoff restart policies.

### 2. Advanced Technology Suites
- **5G/6G Network OS**: Implements E2/A1/O1 O-RAN Interfaces, 3GPP network slicing (eMBB, URLLC, mMTC), and automatic TRAI QoS verification.
- **ROS 2 Robotics**: Built-in DDS Participant registration, complementary filter sensor fusion, and trapezoidal trajectory planning.
- **Brain-Computer Interface**: Integration for OpenBCI Cyton/Daisy and Neurosity Crown EEG headsets with band power Goertzel extraction and motor imagery classifiers.
- **IN-SPACe Developer Tools**: CCSDS framing, orbit element propagation, and link budget estimators.
- **Formal Verification**: Bounded model checking specs for key ring-buffer IPC paths.

### 3. Cyber Security & Isolation
- **Security Center Daemon**: Actively monitors the immutable, BLAKE3-linked kernel audit logs. Applies temporal decay heuristics to identify threats (e.g., sandbox escape attempts) and kills malicious shards autonomously.
- **Sovereign Sandboxes**: Fine-grained capability systems (sandboxctl) restricting filesystem and network accesses.

### 4. Zenith Desktop (UI/UX)
- **Object-Oriented UI**: A Trait-based Widget framework operating entirely without heap allocations.
- **BSP Window Manager**: Binary Space Partitioning tiling engine.
- **Multi-Monitor KMS**: Direct hardware Modesetting supporting cloned and extended CRTC outputs.

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

See [INSTALL.md](INSTALL.md) for build instructions using the new `Justfile` toolchain.

## Linux Distro Parity

SigmaOS matches standard Linux system interfaces:
- [Linux-Inspired Subsystems](Linux-Inspired-Subsystems) (udev, sysfs, procfs, tmpfs, inotify, dmesg, mount, sysctl, logrotate)
- [Coreutils Reference](Coreutils-Reference) (chmod, chown, cp, mv, touch, wc, grep, head, tail, df, du)
- [Device Management](Device-Management) (probe detection, naming rules, node hierarchy)

