# AI Agent Mapping Management in SigmaOS

## Overview
SigmaOS employs autonomous AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨) to oversee, validate, and optimize five core Mapping Subsystems across kernel, hardware, containerization, and packaging architectures:
1. **Virtual Memory Page Mapping (VMM/PMM)** (`src/memory/pmm_vmm.rs`)
2. **User Namespace UID/GID Mapping** (`src/syscall/user_syscalls.rs`)
3. **Hardware MMIO & PCI BAR Memory Mapping** (`src/driver/distro_drivers.rs`)
4. **Process PID & Namespace Mapping** (`src/runtime/process/`)
5. **Universal Package Dependency Normalization Mapping** (`src/package/universal.rs`)

---

## 1. Mapping Subsystems & Operational Directives

### 1.1 Virtual Memory Page Mapping & Self-Referential Page Tables
* **Physical & Virtual Memory Mapping**:
  Implemented in `src/memory/pmm_vmm.rs` (`map_page`, `BitmapFrameAllocator`). AI agents inspect page table entries (`PTE`), mapping 4 KiB / 2 MiB / 1 GiB pages with page protection flags (`PRESENT`, `WRITABLE`, `USER_ACCESSIBLE`, `NO_EXECUTE`).
* **Self-Referential Page Table Mapping**:
  Agents enable recursive self-mapping to allow fast OS page directory modifications without consuming extra physical frames.

### 1.2 User Namespace UID/GID Mapping
* **Isolated Container Identity Mapping**:
  Implemented in `src/syscall/user_syscalls.rs` (`UidGidMapping`). Container processes map container-local root (`UID 0`) to unprivileged host UIDs (e.g. `UID 100000..165535`):
  ```rust
  pub struct UidGidMapping {
      pub container_id: u32,
      pub host_id: u32,
      pub count: u32,
  }
  ```
* **Security Validation**:
  Agents verify that no user namespace mapping bridges container `UID 0` to host `root (UID 0)` without explicit `PqcEnclave` capability authorization.

### 1.3 Hardware MMIO & PCI BAR Mapping
* **Device Ring Buffer Memory Mapping**:
  AI agents validate PCIe Base Address Register (BAR) MMIO mappings for storage controllers (`NvmePCIeHostController`), NICs (`IntelE1000eNicDriver`), and graphics framebuffers (`GopLinearFramebufferDriver`), ensuring ring descriptors do not overlap kernel text.

### 1.4 Package Dependency Normalization Mapping
* **Cross-Distro Dependency Translation**:
  Implemented in `UniversalPackageTranslator` (`src/package/universal.rs`). Foreign package dependency strings (`libc6`, `glibc`, `libssl-dev`, `openssl-devel`) are mapped to canonical SigmaOS sovereign dependencies (`sovereign-libc`, `sovereign-openssl`).

---

## 2. AI Agent Operational Rules for Mapping

1. **ASLR Guard Mapping**:
   AI agents must map memory regions with unmapped guard pages on both boundaries to prevent buffer overflow attacks.
2. **Page Table Zeroization**:
   Unmapped pages must be zeroed immediately upon deallocation to eliminate data leakage.
3. **Audit Trails**:
   All user namespace UID/GID map modifications are logged to `DefensiveAuditLogger` for ISO 27001 audit compliance.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active user namespace UID/GID mappings
sigma-map uidgid --pid 2048

# Inspect kernel virtual memory page table mappings
sigma-map vmm-inspect --virt 0x7fff00000000

# Audit foreign package dependency normalization mapping
sigma-map package-deps --format deb --pkg curl
```
