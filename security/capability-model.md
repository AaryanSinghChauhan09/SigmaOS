# SigmaOS Capability-Ring Security Model

## Overview
SigmaOS enforces a **Zero-Trust capability-ring architecture**. Every process, driver, and subsystem is isolated in a strict capability ring. Privilege escalation is not monolithic (like Unix `root`); instead, it is handled via granular, cryptographically-signed capability tokens governed by the Zero-Trust Threat Scoring Engine.

## The Ring Hierarchy
- **Ring 0 (Sovereign Core)**: Microkernel scheduler, VM manager, and Zero-Trust arbiter.
- **Ring 1 (Core Drivers)**: High-trust subsystems like the Ext4 journal and `sigma_kms`.
- **Ring 2 (Hardware HAL)**: User-space driver wrappers (NVMe, USB, Network).
- **Ring 3 (Userland)**: All user applications (`sigma-sh`, `zenith_terminal`).

## Worked Example: NVMe DMA Transfer across Ring Boundaries

When a userland process (Ring 3) wants to read a file from the NVMe drive (Ring 2), the following boundary crossings occur:

1. **The Request (Ring 3 -> Ring 0)**
   - The user application issues a read syscall.
   - The Zero-Trust Engine (Ring 0) intercepts the request. It calculates a real-time **Threat Score** based on the process's current state, recent IPC anomalies, and memory bounds.
   - If the score is within the acceptable threshold, a temporal, one-time Capability Token is minted.

2. **The Delegation (Ring 0 -> Ring 2)**
   - The kernel passes the token to the NVMe driver (Ring 2).
   - The NVMe driver parses the token. Because it runs in Ring 2, it does not have direct access to Ring 3 memory.

3. **The DMA Translation (Ring 2 -> Ring 1)**
   - The NVMe driver requests the Memory Manager (Ring 1) to pin the user's memory pages and translate them to physical addresses for Direct Memory Access (DMA).
   - The Memory Manager validates the Capability Token cryptographically using Dilithium-5 signatures to ensure the NVMe driver is not spoofing a request.

4. **Execution & Hardware Isolation**
   - The NVMe controller performs the DMA transfer.
   - Upon completion, the IOMMU (configured by Ring 0) ensures the hardware only wrote to the specifically authorized physical pages.
   - The temporal Capability Token expires, and the threat score is updated.

This mechanism ensures that even if the NVMe driver (Ring 2) is compromised, it cannot arbitrarily read or write memory belonging to other rings or processes without a valid, kernel-issued capability token.
