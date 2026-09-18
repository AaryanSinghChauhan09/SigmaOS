# AI Agent Network Stack Development & Maintenance Guidance

## Executive Overview

This document provides architectural standards, diagnostic routines, and zero-allocation Rust guidelines for AI coding agents developing and maintaining the **SigmaOS Network Subsystem**. The network stack combines eBPF/XDP zero-copy packet processing (`EbpfXdpZeroCopyRedirector`), Post-Quantum Cryptographic (PQC) WireGuard VPN tunneling (`SovereignPqcWireguardVpnEngine`), and FreeBSD VNET virtualized network stack isolation (`VnetStack`).

---

## Architecture & Subsystem Interactions

```
                            +-----------------------------------+
                            |     Network Interface Card        |
                            +-----------------------------------+
                                              |
                                              v
                            +-----------------------------------+
                            |  eBPF/XDP Zero-Copy Rx Filter     |
                            | (EbpfXdpZeroCopyRedirector)       |
                            +-----------------------------------+
                             /                 |               \
                            /                  |                \
        +-----------------------+  +-----------------------+  +-----------------------+
        | PQC WireGuard VPN     |  | FreeBSD VNET Jail     |  | TCP/IP Native Ring    |
        | Dilithium5 / Kyber    |  | Virtualized Stack     |  | Zero-Alloc Socket     |
        +-----------------------+  +-----------------------+  +-----------------------+
```

---

## Engineering Guidelines for AI Agents

1. **Zero-Allocation Packet Paths**:
   - All packet Rx/Tx buffer rings must be allocated using `#![no_std]` `SovereignRingBuffer` or slab frame allocators.
   - Do not perform `Vec::extend` or string formatting inside the packet processing loop.

2. **PQC WireGuard Tunnel Maintenance**:
   - Verify post-quantum key exchange Handshake v2 state transitions (`SovereignPqcWireguardVpnEngine`).
   - Rotate Kyber-1024 ephemeral keys every 180 seconds or after 1,000,000 transferred packets.

3. **eBPF/XDP Zero-Copy Rules**:
   - Use `redirect_packet_zero_copy` for inter-interface routing without copying memory from DMA ring buffers.

---

## Diagnostic Verification Protocol

AI agents modifying the network stack must verify clean execution:
1. Run `./run_sigma_tests.sh` (Stage 10 and 12 network integration tests).
2. Execute `rustc --test --edition=2021 src/distro/linux_bsd_inspirations.rs -o build/test_net` to verify XDP and PQC VPN unit tests.
