# SigmaOS AI Agent Protection & Access Rights Management Guide

This guide defines memory page protection transitions, OpenBSD pledge/unveil restrictions, FreeBSD Capsicum capability descriptor limits, and AppArmor mandatory access control (MAC) standards for AI coding agents developing on SigmaOS.

---

## 1. Virtual Memory Page Protection Rights (`mprotect`)

SigmaOS enforces strict W^X (Write XOR Execute) memory page protection rules across virtual address spaces (`klib::paging::SimpleVMM`):

* **Read / Write (`PTE_PRESENT | PTE_WRITABLE | PTE_NO_EXECUTE`):** Heap data, stack frames, and mutable buffers.
* **Read / Execute (`PTE_PRESENT | PTE_USER`):** Executable code segments and shared library text sections.
* **Read-Only (`PTE_PRESENT`):** Read-only data (`.rodata`), zero-copy store package slices (`NixGuixZeroCopyStore`).

### Dynamic Page Protection Transition Directive
When changing memory page rights via `mprotect(vaddr, len, prot)`:
1. **Never allow `PROT_WRITE | PROT_EXEC` simultaneously.**
2. TLB invalidation (`invlpg`) MUST be issued on all active SMP CPU cores immediately following page table attribute modifications.

---

## 2. Process Capability Restriction (`pledge` & `unveil`)

Processes restrict their own system access rights monotonically over time (`src/distro/wiki_ideas_implementation.rs`):

* **`pledge(promises)`:** Irreversibly drops system call privileges (e.g. `stdio rpath wpath cpath inet`).
* **`unveil(path, permissions)`:** Restricts VFS filesystem view to explicit white-listed paths (`r` = read, `w` = write, `c` = create, `x` = execute). Once locked via `unveil(NULL, NULL)`, no further paths may be unveiled.

---

## 3. FreeBSD Capsicum Descriptor Capability Delegation

Capability rights are bound to specific file descriptors (`CAP_READ`, `CAP_WRITE`, `CAP_SEEK`, `CAP_FSTAT`):

```rust
use crate::distro::sovereign_distro_dominance::CapsicumRight;

pub fn enforce_fd_rights(fd: usize, required_right: CapsicumRight, rights_mask: u32) -> bool {
    (rights_mask & (required_right as u32)) != 0
}
```

* Descriptor rights can only be reduced (`cap_rights_limit`), never expanded.
* Entering capability mode (`cap_enter`) restricts the process to operating solely on existing open descriptors.

---

## 4. Ubuntu AppArmor Mandatory Access Control (MAC)

AppArmor security profiles (`UbuntuAppArmorEngine` in `src/distro/missing_distro_innovations.rs`) restrict binary path access in real time:

* **`Enforce` Mode:** Blocks unauthorized read/write/exec access attempts with `Err("AppArmor: Access denied by profile")`.
* **`Complain` Mode:** Logs access violations to `dmesg` while allowing execution for audit profiling.
