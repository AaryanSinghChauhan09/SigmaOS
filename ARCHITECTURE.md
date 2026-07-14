# SigmaOS Architecture

> **Status**: Implemented
> **Last Updated**: 2026-07-13

This document describes the high-level architecture of SigmaOS, including the microkernel design, shard architecture, security model, and component interactions.

## Overview

SigmaOS is a sovereign microkernel operating system built on the principle of capability-based security. The system is composed of modular components called "shards" that can be loaded, unloaded, and updated independently.

## System Architecture

### High-Level Architecture

```mermaid
graph TB
    subgraph "User Space"
        App1[Applications]
        App2[Applications]
        App3[Applications]
    end
    
    subgraph "Optional Shards"
        Zenith[Zenith Compositor]
        Desktop[Desktop Shell]
        LLM[LLM Integration]
        Pkg[Package Manager]
    end
    
    subgraph "Essential Shards"
        GPU[GPU Driver]
        Storage[Storage Driver]
        Audio[Audio Driver]
        Network[Network Driver]
        Input[Input Driver]
    end
    
    subgraph "Core Shards"
        MM[S-MM Memory Manager]
        Sched[S-SCHED Scheduler]
        Net[S-NET Network Stack]
        FS[S-FS Filesystem]
        IPC[S-IPC IPC]
        SEC[S-SEC Security Manager]
        SYS[S-SYS Syscall Interface]
    end
    
    subgraph "Kernel"
        Microkernel[Microkernel]
        Hardware[Hardware Abstraction]
    end
    
    App1 --> SYS
    App2 --> SYS
    App3 --> SYS
    
    Zenith --> GPU
    Desktop --> Input
    LLM --> MM
    Pkg --> FS
    
    GPU --> Hardware
    Storage --> Hardware
    Audio --> Hardware
    Network --> Hardware
    Input --> Hardware
    
    MM --> Microkernel
    Sched --> Microkernel
    Net --> Microkernel
    FS --> Microkernel
    IPC --> Microkernel
    SEC --> Microkernel
    SYS --> Microkernel
    
    Microkernel --> Hardware
```

## Microkernel Architecture

### Core Components

The SigmaOS microkernel provides minimal functionality, delegating most services to user-space shards:

```mermaid
graph LR
    subgraph "Microkernel"
        Thread[Thread Management]
        IPC[IPC Primitives]
        Cap[Capability System]
        VM[Virtual Memory]
        IRQ[Interrupt Handling]
    end
    
    subgraph "Core Shards"
        MM[Memory Manager]
        Sched[Scheduler]
        FS[Filesystem]
        Net[Network Stack]
    end
    
    Thread --> Cap
    IPC --> Cap
    VM --> Cap
    IRQ --> Thread
    
    MM --> VM
    Sched --> Thread
    FS --> IPC
    Net --> IPC
```

### Capability-Based Security

SigmaOS uses capability-based security where all access is granted through capabilities:

```mermaid
graph TB
    subgraph "Capability System"
        Cap[Capability Object]
        Rights[Access Rights]
        Revocation[Revocation]
        Audit[Audit Trail]
    end
    
    subgraph "Operations"
        Grant[Grant Capability]
        Revoke[Revoke Capability]
        Check[Check Access]
    end
    
    Cap --> Rights
    Cap --> Revocation
    Cap --> Audit
    
    Grant --> Cap
    Revoke --> Revocation
    Check --> Rights
```

## Shard Architecture

### Shard Types

SigmaOS has four categories of shards:

1. **Core Shards**: Essential kernel components (Rust)
2. **Essential Shards**: Hardware drivers (Rust/Zig)
3. **Optional Shards**: Desktop and AI features (Nim)
4. **Infinite Shards**: Experimental features (Zig)

### Shard Loading

```mermaid
sequenceDiagram
    participant App as Application
    participant Sys as S-SYS
    participant Loader as Shard Loader
    participant Shard as Shard
    participant Cap as Capability System
    
    App->>Sys: Load Shard Request
    Sys->>Loader: Load Shard
    Loader->>Shard: Initialize
    Shard->>Cap: Request Capabilities
    Cap->>Shard: Grant Capabilities
    Shard->>Loader: Ready
    Loader->>Sys: Shard Loaded
    Sys->>App: Shard Handle
```

### Shard Communication

Shards communicate through well-defined interfaces:

```mermaid
graph TB
    subgraph "Shard A"
        Service[Service Interface]
    end
    
    subgraph "Shard B"
        Client[Client Interface]
    end
    
    subgraph "Communication"
        CapChannel[Capability Channel]
        SharedMem[Shared Memory]
        Events[Event Notifications]
    end
    
    Client --> CapChannel
    Service --> CapChannel
    
    Client --> SharedMem
    Service --> SharedMem
    
    Client --> Events
    Service --> Events
```

## Core Shards

### S-MM (Memory Manager)

```mermaid
graph TB
    subgraph "S-MM"
        Buddy[Buddy Allocator]
        Paging[Paging System]
        CapMem[Capability Memory]
    end
    
    subgraph "Operations"
        Alloc[Allocate Memory]
        Free[Free Memory]
        Map[Map Pages]
        Unmap[Unmap Pages]
    end
    
    Alloc --> Buddy
    Free --> Buddy
    Map --> Paging
    Unmap --> Paging
    
    Alloc --> CapMem
    Map --> CapMem
```

### S-SCHED (Scheduler)

```mermaid
graph TB
    subgraph "S-SCHED"
        EEVDF[EEVDF Algorithm]
        RT[Real-Time Support]
        Affinity[CPU Affinity]
        Balance[Load Balancing]
    end
    
    subgraph "Task States"
        Running[Running]
        Ready[Ready]
        Blocked[Blocked]
    end
    
    EEVDF --> Running
    EEVDF --> Ready
    RT --> Running
    Affinity --> Running
    Balance --> Ready
```

### S-NET (Network Stack)

```mermaid
graph TB
    subgraph "S-NET"
        TCP[TCP Protocol]
        UDP[UDP Protocol]
        IP[IP Layer]
        Firewall[Zero-Trust Firewall]
    end
    
    subgraph "Essential"
        NetDriver[Network Driver]
    end
    
    TCP --> IP
    UDP --> IP
    IP --> Firewall
    Firewall --> NetDriver
```

### S-FS (Filesystem)

```mermaid
graph TB
    subgraph "S-FS"
        VFS[VFS Layer]
        Ext4[ext4]
        FAT32[FAT32]
        NTFS[NTFS]
    end
    
    subgraph "Essential"
        StorageDriver[Storage Driver]
    end
    
    VFS --> Ext4
    VFS --> FAT32
    VFS --> NTFS
    
    Ext4 --> StorageDriver
    FAT32 --> StorageDriver
    NTFS --> StorageDriver
```

### S-IPC (Inter-Process Communication)

```mermaid
graph TB
    subgraph "S-IPC"
        MsgPass[Message Passing]
        SharedMem[Shared Memory]
        Sync[Sync/Async]
        ZeroCopy[Zero-Copy]
    end
    
    subgraph "Security"
        CapIPC[Capability IPC]
    end
    
    MsgPass --> CapIPC
    SharedMem --> CapIPC
    Sync --> CapIPC
    ZeroCopy --> SharedMem
```

### S-SEC (Security Manager)

```mermaid
graph TB
    subgraph "S-SEC"
        CapMgmt[Capability Management]
        Access[Access Control]
        Audit[Audit Logging]
        Crypto[Post-Quantum Crypto]
    end
    
    subgraph "Cryptography"
        Kyber[Kyber-1024 KEM]
        Dilithium[Dilithium-5 Signatures]
    end
    
    CapMgmt --> Access
    Access --> Audit
    Crypto --> Kyber
    Crypto --> Dilithium
```

### S-SYS (System Call Interface)

```mermaid
graph TB
    subgraph "S-SYS"
        Syscall[Syscall Handler]
        CapCheck[Capability Check]
        Monitor[Performance Monitor]
        Filter[Syscall Filter]
    end
    
    subgraph "Applications"
        App1[Application 1]
        App2[Application 2]
    end
    
    App1 --> Syscall
    App2 --> Syscall
    
    Syscall --> CapCheck
    Syscall --> Monitor
    Syscall --> Filter
```

## Essential Shards

### GPU Driver

```mermaid
graph TB
    subgraph "GPU Driver"
        Init[Initialization]
        ModeSet[Mode Setting]
        Framebuffer[Framebuffer Management]
        Accel[2D/3D Acceleration]
    end
    
    subgraph "Hardware"
        GPU[NVIDIA/AMD/Intel GPU]
    end
    
    Init --> GPU
    ModeSet --> GPU
    Framebuffer --> GPU
    Accel --> GPU
```

### Storage Driver

```mermaid
graph TB
    subgraph "Storage Driver"
        NVMe[NVMe Controller]
        AHCI[AHCI Controller]
        BlockIO[Block I/O]
        Queue[I/O Queue]
    end
    
    subgraph "Hardware"
        SSD[NVMe SSD]
        HDD[AHCI HDD]
    end
    
    NVMe --> SSD
    AHCI --> HDD
    BlockIO --> Queue
```

### Audio Driver

```mermaid
graph TB
    subgraph "Audio Driver"
        Codec[Codec Init]
        PCM[PCM Playback]
        Capture[PCM Capture]
        Mixer[Mixer Controls]
    end
    
    subgraph "Hardware"
        Audio[Audio Codec]
    end
    
    Codec --> Audio
    PCM --> Audio
    Capture --> Audio
    Mixer --> Audio
```

### Network Driver

```mermaid
graph TB
    subgraph "Network Driver"
        NIC[NIC Init]
        TX[Packet TX]
        RX[Packet RX]
        DMA[DMA Operations]
    end
    
    subgraph "Hardware"
        Eth[Ethernet NIC]
        WiFi[WiFi Adapter]
    end
    
    NIC --> Eth
    NIC --> WiFi
    TX --> DMA
    RX --> DMA
```

### Input Driver

```mermaid
graph TB
    subgraph "Input Driver"
        Keyboard[Keyboard Driver]
        Mouse[Mouse Driver]
        Touch[Touchscreen Driver]
        HID[HID Protocol]
    end
    
    subgraph "Hardware"
        KB[USB Keyboard]
        MouseDev[USB Mouse]
        TouchDev[Touchscreen]
    end
    
    Keyboard --> KB
    Mouse --> MouseDev
    Touch --> TouchDev
    Keyboard --> HID
    Mouse --> HID
    Touch --> HID
```

## Security Architecture

### Capability Model

```mermaid
graph TB
    subgraph "Capability Model"
        DefaultDeny[Default Deny]
        ExplicitGrant[Explicit Grant]
        Revocation[Revocation]
        LeastPrivilege[Least Privilege]
    end
    
    subgraph "Enforcement"
        Kernel[Kernel Enforcement]
        Shard[Shard Enforcement]
        Audit[Audit Trail]
    end
    
    DefaultDeny --> Kernel
    ExplicitGrant --> Kernel
    Revocation --> Shard
    LeastPrivilege --> Shard
    
    Kernel --> Audit
    Shard --> Audit
```

### Post-Quantum Cryptography

```mermaid
graph TB
    subgraph "Post-Quantum Crypto"
        Kyber[Kyber-1024 KEM]
        Dilithium[Dilithium-5 Signatures]
        Hybrid[Hybrid Mode]
    end
    
    subgraph "Use Cases"
        KeyExchange[Key Exchange]
        Signing[Digital Signatures]
        Auth[Authentication]
    end
    
    Kyber --> KeyExchange
    Dilithium --> Signing
    Hybrid --> Auth
```

## Deployment Profiles

### Standalone (Full Desktop)

```mermaid
graph TB
    subgraph "Standalone Profile"
        Core[Core Shards]
        Essential[Essential Shards]
        Optional[Optional Shards]
        Desktop[Desktop Environment]
    end
    
    Core --> Essential
    Essential --> Optional
    Optional --> Desktop
```

### Microkernel (Embedded)

```mermaid
graph TB
    subgraph "Microkernel Profile"
        Core[Core Shards Only]
        Minimal[Minimal Footprint]
    end
    
    Core --> Minimal
```

### Cloud (Headless)

```mermaid
graph TB
    subgraph "Cloud Profile"
        Core[Core Shards]
        Essential[Essential Shards]
        CloudInit[Cloud-Init]
        SSH[SSH Access]
    end
    
    Core --> Essential
    Essential --> CloudInit
    Essential --> SSH
```

## Component Interactions

### Boot Sequence

```mermaid
sequenceDiagram
    participant Boot as Bootloader
    participant Kernel as Microkernel
    participant Core as Core Shards
    participant Essential as Essential Shards
    participant User as User Space
    
    Boot->>Kernel: Load Kernel
    Kernel->>Kernel: Initialize
    Kernel->>Core: Load Core Shards
    Core->>Kernel: Ready
    Kernel->>Essential: Load Essential Shards
    Essential->>Kernel: Ready
    Kernel->>User: Start Init
    User->>User: System Ready
```

### System Call Flow

```mermaid
sequenceDiagram
    participant App as Application
    participant Sys as S-SYS
    participant Cap as S-SEC
    participant Shard as Core Shard
    participant Kernel as Microkernel
    
    App->>Sys: System Call
    Sys->>Cap: Check Capability
    Cap->>Sys: Access Granted
    Sys->>Shard: Forward Request
    Shard->>Kernel: Kernel Operation
    Kernel->>Shard: Result
    Shard->>Sys: Return Result
    Sys->>App: Return to User
```

## Performance Considerations

### Zero-Copy Operations

```mermaid
graph LR
    subgraph "Zero-Copy"
        Shared[Shared Memory]
        Cap[Capability Access]
        Direct[Direct Access]
    end
    
    subgraph "Benefits"
        Perf[Performance]
        Latency[Low Latency]
        CPU[Low CPU Usage]
    end
    
    Shared --> Cap
    Cap --> Direct
    
    Direct --> Perf
    Direct --> Latency
    Direct --> CPU
```

### EEVDF Scheduling

```mermaid
graph TB
    subgraph "EEVDF"
        Virtual[Virtual Deadline]
        Eligible[Earliest Eligible]
        Fair[Fairness]
        RT[Real-Time Support]
    end
    
    subgraph "Benefits"
        O1[O1 Scheduling]
        LowLat[Low Latency]
        Predictable[Predictable]
    end
    
    Virtual --> Eligible
    Eligible --> Fair
    Fair --> RT
    
    Eligible --> O1
    RT --> LowLat
    Fair --> Predictable
```

## Future Architecture

### Self-Evolving System

```mermaid
graph TB
    subgraph "Self-Evolving"
        Genetic[Genetic Algorithms]
        RL[Reinforcement Learning]
        Auto[Auto-Tuning]
        SelfHeal[Self-Healing]
    end
    
    subgraph "Targets"
        Kernel[Kernel Parameters]
        Sched[Scheduler Tuning]
        Resource[Resource Allocation]
    end
    
    Genetic --> Kernel
    RL --> Sched
    Auto --> Resource
    SelfHeal --> Kernel
```

### AI-Native OS

```mermaid
graph TB
    subgraph "AI-Native"
        NPU[NPU Integration]
        ML[ML-Based Scheduling]
        Adaptive[Adaptive Resources]
        Predictive[Predictive Maintenance]
    end
    
    subgraph "Benefits"
        Opt[Optimized Performance]
        Power[Power Efficiency]
        Smart[Smart Resource Use]
    end
    
    NPU --> ML
    ML --> Adaptive
    Adaptive --> Predictive
    
    ML --> Opt
    Adaptive --> Power
    Predictive --> Smart
```

## Summary

SigmaOS architecture is designed around the following principles:

1. **Modularity**: Shards can be loaded/unloaded independently
2. **Security**: Capability-based security with default deny
3. **Performance**: Zero-copy operations and O(1) scheduling
4. **Sovereignty**: Post-quantum cryptography and local-first design
5. **Extensibility**: Support for experimental and self-evolving features

The architecture enables SigmaOS to be deployed in various environments from embedded systems to cloud platforms while maintaining security and performance.

---

*Last Updated: 2026-07-13*
