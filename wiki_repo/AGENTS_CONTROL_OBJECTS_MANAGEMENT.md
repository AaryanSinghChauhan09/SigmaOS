# SigmaOS Control Objects, Access Control Matrix & Zero Trust Gate Guide for AI Agents

This guide provides technical specifications, access control matrix evaluation, Discretionary Access Control (`DacPermission`), Extended POSIX ACLs, AppArmor path-based MAC (`UbuntuAppArmorProfile`), capability bounding sets, Capsicum descriptor rights, BSD securelevels, and Zero Trust access gate policies for AI agents working in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Access Control Architecture

SigmaOS implements a multi-layered security and access control model under `#![no_std]` Rust (`src/access/control.rs`, `src/security/capability.rs`, `src/distro/missing_distro_innovations.rs`):

* **Discretionary Access Control (`DacPermission` in `src/access/control.rs`):**
  Evaluates POSIX octal mode bits (`owner_uid`, `group_gid`, `mode_bits`), SUID, SGID, and Sticky bit deletion restrictions for directory contents.
* **Access Control Matrix (ACM):**
  Provides 2D rights mapping (Subject, Object, Right Mask) for explicit permission granting and revocation.
* **Extended POSIX ACLs:**
  Provides user, group, named user, named group, mask, and other entry evaluation with automatic mask filtering.
* **AppArmor Path-Based MAC (`UbuntuAppArmorProfile` in `src/distro/missing_distro_innovations.rs`):**
  Enforces path pattern matching and permission evaluation under `Enforcing` or `Complain` modes.
* **Capability Bounding Sets:**
  Restricts process capabilities using bitmask evaluation (`CapabilityToken::has_permission`).
* **Capsicum Descriptor Rights:**
  Provides fine-grained file descriptor rights limiting (read, write, seek, mmap, fcntl, accept, connect).
* **Zero Trust Access Gate:**
  Unifies network MAC address filtering, capability bounding, and access control matrix verification before granting resource access.

---

## 2. DAC & Capability Evaluation Rules for AI Agents

When evaluating or granting access control rights:

1. **Root Bypass Exception:**
   `UID 0` (Root) bypasses standard DAC and POSIX ACL checks, but MUST remain bounded by capability bounding sets and active MAC policies.
2. **Capability Bitmask Evaluation:**
   `CapabilityToken::has_permission` MUST perform bit shifting (`(self.bits & (1 << (permission as u64))) != 0`) so bit shift indices correspond directly to capability variant discriminants.
3. **Sticky Bit Deletion Restriction:**
   In sticky-bit directories (`0o1000`), file deletion MUST be restricted to the directory owner, file owner, or root (`UID 0`).

---

## 3. Checklist for AI Agents Managing Access Control Objects

1. **Verify Capability Shift Logic:** Ensure capability token bitmask checks use `1 << permission`.
2. **Test Access Control Subsystem Pipelines:**
   Run access control unit tests:
   ```bash
   cargo test --lib -- access::control::tests
   ./run_sigma_tests.sh
   ```
