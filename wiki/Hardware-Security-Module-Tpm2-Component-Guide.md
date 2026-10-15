# TPM 2.0 Hardware Security Module & Measured Boot Component Guide

> **Related portals:** [Security, Sandboxing & Cryptography](./Home.md) &bull; **Maintainer Agent:** [`HARDWARE_SECURITY_MODULE_TPM2_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/HARDWARE_SECURITY_MODULE_TPM2_AGENT.md) &bull; **Status:** Canonical Sovereign Component

---

## Contents
1. [Overview & Architecture](#1-overview--architecture)
2. [Technical Specifications & Protocols](#2-technical-specifications--protocols)
3. [Multi-Language Implementations](#3-multi-language-implementations)
4. [Configuration & Usage](#4-configuration--usage)
5. [Diagnostics & Troubleshooting](#5-diagnostics--troubleshooting)
6. [See Also](#6-see-also)

---

## 1. Overview & Architecture
The **TPM 2.0 Hardware Security Module & Measured Boot** provides high-reliability, sovereign system infrastructure for SigmaOS:
- **Component ID**: `HARDWARE_SECURITY_MODULE_TPM2`
- **Subsystem Category**: Security, Sandboxing & Cryptography
- **Autonomous Agent**: `HARDWARE_SECURITY_MODULE_TPM2_AGENT`
- **Upstream Parity Reference**: TCG TPM 2.0 Library Specification, systemd-cryptenroll, and Linux tpmrm0 driver.

Interfaces directly with discrete and firmware TPM 2.0 chips, measuring UEFI firmware, kernel, initrd, and command-line PCR banks to securely seal and unseal LUKS encryption master keys.

---

## 2. Technical Specifications & Protocols
- **Memory Safety**: Memory-safe kernel and userland execution boundaries preventing memory corruption and buffer overflow exploits.
- **Concurrency Model**: Lock-free or fine-grained synchronization primitives using standard mutexes and atomic operations.
- **Error Handling**: Explicit, zero-cost error propagation eliminating silent failures and unhandled edge cases.

---

## 3. Multi-Language Implementations
- **Rust Engine**: Integrated in `src/kernel/sovereign_parity_apex_v100.rs` with safe abstraction boundaries.
- **Zig Accelerator**: Fast deterministic execution buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

### Standard Library Authorization
Per updated SigmaOS architectural governance, this subsystem drops all previous restrictions against standard library usage. Standard libraries (`std::collections::BTreeMap`, `std::sync::Arc`, `std::sync::Mutex`, `VecDeque`, and `libc`) are authorized and actively utilized.

---

## 4. Configuration & Usage

```toml
# Unseal root LUKS volume with TPM 2.0 PCRs 0, 2, 7
$ systemd-cryptenroll --tpm2-device=auto --tpm2-pcrs=0+2+7 /dev/nvme0n1p2
```

---

## 5. Diagnostics & Troubleshooting
To verify the operation and health of this component:

```bash
# Check subsystem status and diagnostics
$ sigma-ctl status hardware_security_module_tpm2

# Inspect system logs for events
$ journalctl -u hardware_security_module_tpm2 --no-pager
```

---

## 6. See Also
- [Main Portal: Security, Sandboxing & Cryptography](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
