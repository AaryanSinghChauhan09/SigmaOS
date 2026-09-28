# 🛡️ SIGMAOS OPEN-SOURCE REPOSITORY ABSORPTION POLICY SPECIFICATION

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Document Version:** 1.0.0
> **Status:** Active Mandatory Engineering Governance Specification

---

## 📌 OVERVIEW & CORE PRINCIPLES

Before importing code or specifications from external open-source projects into **SigmaOS**, all developers, contributors, and AI agents must strictly adhere to this **Absorption Policy**.

SigmaOS enforces an **Adaptation-First Policy** where external code is audited, specified, adapted through cleanroom Rust abstractions, and validated via strict hardware/QEMU testing.

---

## 📋 MANDATORY 8-POINT COMPONENT REVIEW CHECKLIST

Every imported or absorbed open-source component must document and satisfy the following 8 criteria:

1. **Source Repository & Exact Commit/Tag:** Full URL and git SHA-1 commit hash / release tag of the upstream source.
2. **License Compatibility Review:** Verification of license terms (MIT, Apache 2.0, GPL-2.0, BSD-2/3-Clause, MPL) and preservation of copyright notices.
3. **Dependency & API Inventory:** Detailed inventory of required kernel/sys call interfaces, C symbols, and crate requirements.
4. **Hardware Architecture Support Declaration:** Specification of target architectures (x86_64, ARM64, RISC-V 64).
5. **Rust / `#![no_std]` Compatibility Classification:** Classification as core `#![no_std]` kernel space or `alloc`/`std` userland space.
6. **Security & Privilege Requirements:** Required privilege ring (Ring 0, Ring 3, eBPF sandbox, capability flags).
7. **Testing Matrix:** Verification across Unit tests, Integration tests, QEMU microVM emulation, and real hardware targets.
8. **Maintainer & Update Ownership:** Designated agent or maintainer responsible for upstream sync and vulnerability patching.

---

## 🚫 FORBIDDEN DIRECT IMPORTS

Developers and AI agents must **NEVER** directly import:

- Raw Linux kernel C drivers into Ring 0 without a safe Rust isolation shim.
- Proprietary firmware or redistributable binary blobs without explicit permission/license compliance.
- Copyleft GPL code into MIT/Apache-only components without preserving license obligations.
- User-space code or libraries directly into kernel space (`#![no_std]` boundary violation).
- Monolithic framework abstractions that conflict with SigmaOS’s existing VFS, scheduler, memory, or driver models.

---

## 🔄 ADAPTATION-FIRST POLICY PIPELINE

```text
External Project / GitHub Repository
              ↓
   License & Dependency Audit
              ↓
  Behavior / Specification Extraction
              ↓
     SigmaOS HAL Adapter
              ↓
Native Safe Rust Implementation / Isolated Compatibility Module
              ↓
   Unit, QEMU & Hardware Validation
```

---

*End of Policy Specification.*
