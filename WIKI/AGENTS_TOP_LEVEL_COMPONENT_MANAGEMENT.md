# AI Agent Guidelines: Top-Level Component Management in SigmaOS

## Overview
This document defines operational guidelines and architectural directives for AI agents working on **Top-Level Component Management** in SigmaOS. It specifies subsystem boundaries, cross-component interface invariants, capability-ring isolation, zero-dependency inter-component communication, and the unified cross-subsystem distro bridge (`SovereignUniversalDistroBridge`) across `#![no_std]` runtime environments in SigmaOS.

---

## 1. Top-Level Component Architecture

SigmaOS organizes its bare-metal operating system architecture into 8 primary top-level components:

```
+-----------------------------------------------------------------------------------+
|                            ZENITH DESKTOP PLATFORM                                |
|        (Direct Framebuffer, Zero Wayland/X11, Inclusive Accessibility)            |
+-----------------------------------------------------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|               UNIVERSAL DISTRO BRIDGE (SovereignUniversalDistroBridge)            |
|       (Cross-subsystem router: Landlock v5, eBPF XDP, Jails, Pledge/Unveil)       |
+-----------------------------------------------------------------------------------+
       |                  |                   |                 |                |
       v                  v                   v                 v                v
+--------------+  +---------------+  +-----------------+  +------------+  +------------+
| Security Ring|  | Universal Pkg |  | VFS & Storage   |  | Network    |  | HAL &      |
| & Hardening  |  | (SigmaPkg)    |  | (SigmaFS / CoW) |  | (ZenithNet)|  | Drivers    |
+--------------+  +---------------+  +-----------------+  +------------+  +------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|                    SIGMAOS BARE-METAL MICROKERNEL CORE                            |
|        (EEVDF/BORE Scheduler, SovereignVMM Paging, Lock-Free IPC Ring)            |
+-----------------------------------------------------------------------------------+
```

---

## 2. The 8 Top-Level Component Specifications

| Component | Primary Location | Scope & Responsibilities |
| :--- | :--- | :--- |
| **1. Microkernel Core** | `src/kernel/`, `src/klib/`, `src/memory/`, `src/arch/` | EEVDF/BORE thread schedulers, 4-level paging virtual memory managers (`SovereignVMM`), lock-free IPC rings, and CPU trap/exception dispatchers. |
| **2. HAL & Driver Shards** | `src/hal/`, `src/drivers/`, `src/driver/` | Multi-arch interrupt routing (APIC, GICv3, PLIC, ExtIOI), hot-swappable Driver Shards (`DriverShardManager`), NVMe/xHCI/E1000 controller abstractions, and legacy-to-modern bus bridges. |
| **3. VFS & Storage Engine** | `src/filesystem/`, `src/storage/`, `src/fs/` | Copy-on-Write (CoW) Merkle tree storage, JBD2 ext4 journals, HAMMER2 PFS multi-volume deduplication, and mount namespace isolation. |
| **4. Asynchronous Networking** | `src/network/`, `src/net/`, `src/cloud/` | Zero-copy packet ring buffers (AF_XDP / Netmap parity), Noise protocol PQC VPN tunnels (Kyber-1024 / Dilithium-5), eBPF-style fast packet inspection, and P2P cloud state sync. |
| **5. Security & Capability Ring** | `src/security/`, `src/crypto/` | Capability rings (`src/security/capability.rs`), OpenBSD `pledge`/`unveil` path sandboxing, TPM 2.0 PCR attestation, and amnesic RAM scrubbing. |
| **6. Universal Package System** | `src/package/`, `src/sigpkg/` | Multi-format package translation (.deb, .rpm, PKGBUILD, ebuild, apk, snap, flatpak, hpkg), content-addressed store (CAS), and zero-allocation SAT solvers. |
| **7. Zenith Compositor Visual Core** | `zenith_desktop/`, `src/desktop/`, `src/ui/` | Direct framebuffer display engine without X11/Wayland dependencies, multi-threaded tiling layouts, and WCAG 2.1 AA accessibility overlays. |
| **8. Universal Distro Bridge** | `src/distro/linux_bsd_inspirations.rs` | Central cross-subsystem router (`SovereignUniversalDistroBridge`) integrating Landlock v5, eBPF XDP zero-copy, FreeBSD jails, and Illumos zones across operating modes. |

---

## 3. Architectural Rules & Component Invariants

AI agents modifying or extending top-level components must enforce the following 4 core invariants:

### 1. Capability-Gated Component Isolation
- **Invariant:** Top-level components MUST NOT share mutable raw global state across subsystem boundaries.
- **Rule:** Inter-component requests must pass through capability-gated IPC ring buffers (`SovereignUniversalDistroBridge`) or explicit trait interfaces verified by `CapabilityToken`.

### 2. Absolute Zero-Dependency Constraint (`#![no_std]`)
- **Invariant:** All top-level components operate under strict sovereign zero-dependency rules.
- **Rule:** Do not introduce third-party dynamic libraries or external crates under `[dependencies]` in `Cargo.toml`. Use custom `klib` allocation primitives and bare-metal hardware memory mappings (`0x...`).

### 3. Interface Stability & Non-Breaking Evolution
- **Invariant:** Public traits, enums, and module entry points exposed by top-level components MUST maintain backward compatibility with existing subsystem callers.
- **Rule:** Before changing a public interface in `src/kernel/`, `src/drivers/`, `src/filesystem/`, or `src/distro/`, inspect dependent modules across `src/` to prevent compilation breakages.

### 4. Zero Ring 0 Panic Invariant
- Component entry points must return explicit `Result<T, &'static str>` or status codes instead of triggering unhandled kernel panics.

---

## 4. Verification & Testing Protocols

Every top-level component change must be verified via standalone module compilation and integrated test execution:

```bash
# Run standalone unit test for Universal Distro Bridge component
rustc --test --edition 2021 --cfg 'feature="standalone_test"' src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# Run full test suite
./run_sigma_tests.sh
```

---

## 5. AI Agent Self-Assessment Checklist

Before finalizing changes touching top-level components:

- [ ] Does the change respect top-level subsystem boundaries without introducing invalid cross-module coupling?
- [ ] Are inter-component communications capability-gated and thread-safe?
- [ ] Has `#![no_std]` zero-dependency purity been maintained across modified component files?
- [ ] Have all unit tests passed with 0 failures in `./run_sigma_tests.sh`?
