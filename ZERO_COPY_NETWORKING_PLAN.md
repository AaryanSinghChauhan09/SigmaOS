# ⚡ ZenithNet: Zero-Copy Asynchronous Networking Stack Plan

This plan outlines the architecture and execution roadmap for **ZenithNet**, SigmaOS’s bare-metal, high-performance networking subsystem. ZenithNet is engineered to bypass standard socket bottlenecks by using zero-copy DMA memory rings, lock-free concurrent queues, and integrated post-quantum cryptographic security.

---

## 1. Architectural Highlights

ZenithNet replaces procedural socket syscalls and thread context-switching overhead with a lock-free, memory-mapped asynchronous stack.

```
       +---------------------------------------------+
       |             Application Memory              |
       +---------------------------------------------+
                              |
                     [DMA Frame Mapping] (Zero-Copy)
                              v
       +---------------------------------------------+
       |            Lock-Free Ring Buffer            |
       +---------------------------------------------+
                              |
                    [Direct NIC Memory Access]
                              v
       +---------------------------------------------+
       |         Network Interface Card (NIC)        |
       +---------------------------------------------+
```

### 1.1 Zero-Copy DMA Pipeline
*   **No Kernel Splicing:** Packets are processed directly in pre-allocated page frames.
*   **Direct Mapping:** The network driver maps application-level buffers directly into the NIC's ring buffers via DMA descriptor arrays. This completely removes CPU memory copy operations (`memcpy`), outperforming standard Linux socket boundaries.

### 1.2 Lock-Free Concurrent Channels
*   Packet ingestion and transmission queues are implemented as thread-safe, lock-free ring-buffer channels utilizing Atomic head and tail pointers (`AtomicUsize` with `SeqCst` ordering).
*   Prevents spinlock contention in multi-threaded microkernel servers under heavy network loads.

---

## 2. Protocol Stack Specifications

### 2.1 Layer 2 & 3: Ethernet & IPv4/IPv6
*   **ARP/NDP Cache:** Standardizes dynamic hardware routing tables using zero-allocation arrays.
*   **Checksum Offloading:** Automatically delegates checksum calculations to the NIC's physical ASIC when supported, reducing CPU cycle consumption.

### 2.2 Layer 4: Custom TCP/IP and QUIC
*   **Segment Sliding Window:** Tracks window sizes dynamically, utilizing non-blocking timers for segment retransmissions.
*   **State Machine:** Explicitly models TCP states (`LISTEN`, `SYN_SENT`, `ESTABLISHED`, `FIN_WAIT`) using static transition matrices.

### 2.3 Post-Quantum Cryptographic Tunneling
*   Standard SSL/TLS is replaced with a native, zero-dependency Noise Protocol interface.
*   The handshake leverages post-quantum Dilithium-5 and Kyber-1024 algorithms to guarantee forward secrecy against future quantum adversaries.

---

## 3. Implementation Plan

1.  **Phase 1: Lock-Free Packet Channels (Milestone 1)**
    *   Implement lock-free queue pools inside `src/network/stack.rs`.
    *   Benchmark packet ingestion limits using synthetic workloads.
2.  **Phase 2: Zero-Copy DMA Driver Integration (Milestone 2)**
    *   Modify the Network Driver interface to support DMA page frame mappings.
    *   Write safe page-pinning abstractions inside the microkernel’s memory manager.
3.  **Phase 3: Secure Tunneling (Milestone 3)**
    *   Integrate post-quantum handshake logic inside TCP connection endpoints.
    *   Conduct security validation to ensure that network segments are fully encrypted prior to DMA ring transmission.
