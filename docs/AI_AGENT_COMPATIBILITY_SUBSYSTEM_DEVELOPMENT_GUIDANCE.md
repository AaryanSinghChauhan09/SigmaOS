# AI Agent Development Guidance: Compatibility Subsystems

This document provides architectural guidance and development standards for AI agents working on POSIX, Linux, and BSD compatibility layers in SigmaOS (`src/compatibility/`).

---

## 1. Overview of Compatibility Subsystems

SigmaOS provides clean-room, zero-dependency compatibility layers for major Operating Systems and distributions:
* **Linux Compatibility (`src/compatibility/linux_compat.rs`):** BPF LSM hooks, `io_uring` SQPOLL, Landlock sandboxing, cgroups v2 PSI.
* **Fedora Compatibility (`src/compatibility/fedora.rs`):** DNF/RPM package resolver, Koji build server, Bodhi update triage, Greenwave CI gating, Ignition provisioning, Dracut initramfs, ABRT crash reporting, Toolbx containers, SELinux policies, and community tools.
* **BSD Kernel Compatibility (`src/compatibility/bsd.rs`):** FreeBSD VNET virtualized network stack, `kqueue`/`kevent` filtering, and OpenBSD KARL section relinking.

---

## 2. Engineering Principles for Compatibility Development

1. **Clean-Room Implementation:**
   * Never copy GPL or restricted C source code directly. All implementations must be independent, clean-room safe Rust code designed from public API specifications, POSIX standards, or man pages.
2. **Deterministic Error Handling:**
   * Return explicit `Result<T, &'static str>` or domain-specific error enums rather than panicking or invoking `unwrap()` on untrusted input data.
3. **Constant-Time Security Checks:**
   * When handling credentials, PAM tokens, or HMAC signatures (such as in `FedoraKeyringPamModule` or `FedoraWebhookMessagingGateway`), use constant-time byte comparisons to prevent timing attacks.

---

## 3. Maintenance Checklists for AI Agents

* [ ] **Syscall Table Alignment:** Ensure any new Linux/BSD syscall translation arms added to `src/syscall/` map cleanly to the corresponding compatibility structs.
* [ ] **Unit Test Coverage:** Every compatibility struct MUST be accompanied by unit tests under `#[cfg(test)]` verifying its primary operations and edge cases.
* [ ] **Standalone Test Runner Verification:** Confirm that standalone tests compile and pass via `rustc --test --crate-type lib src/compatibility/<module>.rs -o build/test_<module>`.

---

*Document Version:* 1.0.0
*Maintained By:* SigmaOS AI Agent Core Engineering Team
