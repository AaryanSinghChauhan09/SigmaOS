# SigmaOS: Sovereign Architecture Omnibus

Welcome to the definitive architecture manual for **SigmaOS**. This document outlines how SigmaOS has absorbed and revolutionized every major Unique Selling Proposition (USP) from industrial-grade operating systems globally—all condensed into a pure C11, zero-dependency environment.

## 1. Kernel Hypervisor (KVM/Hyper-V Parity)
SigmaOS natively operates as a Type-1.5 Hypervisor. Using `SovereignHypervisor.c`, it exposes bare-metal VMCS interfaces, capturing `VM-Exit` traps natively and forwarding hardware faults using Extended Page Tables (EPT).
*   **Documentation:** [Hypervisor Design Overview]
*   **CLI Command:** `sigma hypervisor spawn --vcpus=4 --ram=2048`

## 2. eXpress Data Path (XDP Parity)
Network traffic (Layer 2) is intercepted via `SovereignXDP.c` directly within the NIC RX ring buffers, ensuring DDoS mitigation triggers wire-speed packet `XDP_DROP` verdicts before kernel scheduling occurs.

## 3. Sandboxing & Access Control
By aggregating Windows AppContainers, Linux SECCOMP BPF, and macOS Seatbelt profiles, the `SovereignAppContainer.c` provisions isolated execution envelopes. `SovereignLSM.c` enforces Mandatory Access Control by stacking policies symmetrically.
*   **CLI Command:** `sigma container spawn --strict /bin/bash`

## 4. Multi-Queue Scheduling (CFS)
O(1) deadlocking is avoided using Red-Black Tree virtual runtime mechanics found in `SovereignSchedulerMQ.c`, combined with automated work-stealing algorithms spanning identical NUMA boundaries.

## 5. Storage Sovereignty (LUKS FDE)
Filesystems are suspended asynchronously using device-mapper intercepted cryptography via `SovereignFDE.c`, executing AES-256-XTS uniformly over all disk geometry. 

## 6. Real-Time Forensic Instrumentation
The system employs zero-overhead instrumentation (`SovereignFTrace.c`) dynamically patching kernel executions strings directly inside code pages. If the kernel drops into a panicked deadlock, physical RAM states are immediately transplanted utilizing `SovereignKexec.c`, avoiding BIOS delays, handing over completely unwound diagnostic crash cores via `SigmaCrashDump.c`.

## 7. SigmaInit (PID 1) Service Array
Daemon orchestration begins immediately utilizing the Service Manager built into `SigmaInit.c`. Socket activation, DAG dependency routing, and zombie reparenting bring userland spaces online safely and deterministically without legacy scripting requirements.

***
**"No external lib. No compromise. Absolute Kernel Sovereignty."**
