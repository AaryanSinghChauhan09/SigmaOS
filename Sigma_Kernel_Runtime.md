# Kernel Runtime Architecture in SigmaOS

> **Status**: 🔄 Active | **Subsystem**: `SigmaKernel`

## 1. Executive Summary

Legacy Linux relies heavily on monolithic kernel modules and massive userspace init systems (like `systemd`). SigmaOS introduces a **Capability-Native Runtime**, merging the security model of a microkernel (seL4/QubesOS) with the operational simplicity of lightweight init systems, all governed by AI-driven predictive scheduling.

---

## 2. Absorbed Distro Capabilities

| Linux Tech | Inspiration | SigmaOS Capability |
| :--- | :--- | :--- |
| **Systemd** | Service management | Deterministic, dependency-based service tracking without the bloat. |
| **QubesOS** | VM Compartmentalization| Hardware-backed virtual isolation for critical subsystems. |
| **Void Linux / Runit** | Lightweight init | Fast, minimal execution paths for booting the system. |

---

## 3. SigmaOS Innovations

### 3.1 Capability-Native Runtime

Instead of relying solely on Discretionary Access Control (users/groups), SigmaOS enforces capability-based tokens at the syscall level. A service cannot open a socket or read a file unless it has been explicitly granted a cryptographic capability token during initialization. 

```rust
// kernel/runtime/capability.rs
// SPDX-License-Identifier: MIT

pub fn spawn_service(name: &str) -> Result<Process> {
    // Generate an unforgeable token for the service
    let token = CapabilityToken::new()
        .allow_network("tcp", 80)
        .allow_read("/var/www")
        .deny_all_others();
        
    ProcessBuilder::new(name)
        .with_capabilities(token)
        .spawn()
}
```

### 3.2 Self-Healing Kernel

If a driver or kernel module encounters an unrecoverable error or memory fault, the kernel does not panic. The offending subsystem is automatically isolated, cleanly terminated, and restarted transparently. Hardware state is recovered via driver-specific reset routines.

### 3.3 AI-Driven Scheduling

Standard completely fair schedulers (CFS) are unaware of user intent. SigmaOS utilizes a highly optimized neural network inside the kernel scheduler to predict thread execution patterns. It automatically identifies interactive threads (UI, audio) and grants them preemption priority, while batching background tasks to maximize CPU sleep states.

### 3.4 Zero-Trust Boot

SigmaOS integrates deeply with the hardware TPM (Trusted Platform Module). The bootloader (`sigma-boot`) measures the cryptographic hash of the kernel, the initramfs, and the root file system. If any measurement fails or indicates tampering, the boot sequence halts and falls back to the `SigmaFS` forensic recovery snapshot.
