# SigmaOS Access Control Development Master Plan: Linux & BSD Inspired Security Architecture

## Executive Summary

Access control in **SigmaOS** is engineered as a multi-layered security subsystem combining Linux Security Modules (SELinux type enforcement, Landlock ABI v4, AppArmor path profiles, POSIX.1e extended ACLs), BSD security mechanisms (OpenBSD `pledge`/`unveil`, FreeBSD pluggable MAC `mac_biba`/`mac_lomac`/`mac_portacl`, FreeBSD Capsicum capability rights), and Zorin Exec Guard default-deny execution policies (`src/security/exec_guard.rs`).

This document defines the master development plan for SigmaOS access control across architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and verification benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Security Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │            SigmaOS Universal Access Control              │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│       Linux LSM Stack     │            │    FreeBSD Security MAC   │            │    OpenBSD Hardening      │
│ • SELinux Contexts & Rules│            │ • Pluggable MAC Framework │            │ • Pledge Syscall Promises │
│ • Landlock ABI v4 Rules   │            │ • Biba / LoMAC Integrity  │            │ • Unveil Path Visibility  │
│ • AppArmor Path Profiles  │            │ • Capsicum cap_rights_t   │            │ • KARL Kernel Relinking   │
│ • POSIX.1e Extended ACLs  │            │ • PortACL Binding Rules   │            │ • PinSyscall Hardening    │
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Six Core Access Control Pillars

### Pillar 1: Filesystem Permissions & POSIX.1e Extended ACLs (`src/filesystem/ext4_ntfs_security.rs`)
* **POSIX.1e Extended ACL Engine**:
  - Parse binary xattrs (`system.posix_acl_access`, `system.posix_acl_default`) representing `ACL_USER`, `ACL_GROUP`, `ACL_MASK`, and `ACL_OTHER` entries.
  - Enforce `ACL_MASK` bitwise filtering during non-owner file permission evaluation (`evaluate_ext4_access`).
* **NTFS Security Descriptors**:
  - Evaluate Security Identifiers (SIDs) and Access Control Entries (ACEs) for NTFS volume parity (`evaluate_ntfs_access`).

### Pillar 2: Mandatory Access Control - SELinux & AppArmor (`src/security/mandatory_access_control.rs`)
* **SELinux Security Contexts & LSM Hooks (`SovereignMacLsmHookRegistry`)**:
  - Support `user:role:type:level` security contexts (`SelinuxSecurityContext`).
  - Intercept inode operations (`open`/`create`/`unlink`), process `ptrace` inspection, and network socket operations (`bind`/`connect`).
* **AppArmor Profile Engine**:
  - Parse `/etc/apparmor.d/` human-readable path profiles with glob wildcard matching.

### Pillar 3: Unprivileged Landlock Sandboxing & Network Hooks (`src/security/landlock.rs`)
* **Landlock ABI v4 Hierarchy & Network Control**:
  - Unprivileged filesystem path restriction (`LANDLOCK_ACCESS_FS_READ_FILE`, `WRITE_FILE`, `EXECUTE`) beneath parent path hierarchies (`LandlockRuleset`).
  - Extend Landlock to enforce network socket rulesets (`LANDLOCK_ACCESS_NET_BIND_TCP`, `LANDLOCK_ACCESS_NET_CONNECT_TCP`).

### Pillar 4: BSD Process Sandboxing & Capability Rights (`src/security/pledge_impl.rs`, `src/security/bsd_hardening.rs`)
* **OpenBSD `pledge(2)` & `unveil(2)`**:
  - Restrict process syscall promise surface (`pledge("stdio rpath wpath cpath inet")`) and hide un-unveiled file paths (`unveil(path, permissions)`).
* **FreeBSD Capsicum Capability Bitfields (`cap_rights_t`)**:
  - Attach fine-grained capability masks (`cap_rights_t`) to file descriptors, restricting operations (`CAP_READ`, `CAP_WRITE`, `CAP_IOCTL`, `CAP_FSTAT`).

### Pillar 5: FreeBSD Pluggable MAC Integrity Labels (`mac_biba`, `mac_lomac`)
* **Pluggable MAC Framework**:
  - Support pluggable security policy modules attaching labels (`mac_label`) to vnodes, processes, and sockets.
  - Implement `mac_biba` multi-level integrity (no read-up, no write-down) and `mac_lomac` low-watermark integrity policies.

### Pillar 6: Security Audit Stream & Execution Guard (`src/security/exec_guard.rs`, `src/security/defensive_audit.rs`)
* **Zorin Exec Guard Default-Deny Policy (`ZorinExecGuardPolicyEngine`)**:
  - Default-deny untrusted binary execution, with developer TOML overrides (`/system/profile.toml`, `/user/preferences.toml`), certificate trust rules, path prefix rules, and Zenith GUI alternative prompt routing.
* **Kernel Netlink Security Audit Stream**:
  - Stream structured audit records over Netlink (`NETLINK_AUDIT`) whenever LSM or MAC hooks deny access.

---

## 3. Four-Phase Chronological Development Roadmap

```
  Phase 1: Extended ACLs & Landlock Network Extensions (Months 1–3)
  ├── POSIX.1e Extended ACL Binary Parser & `ACL_MASK` Filter
  ├── Landlock ABI v4 `LANDLOCK_ACCESS_NET_BIND_TCP` & `CONNECT_TCP`
  └── Zorin Exec Guard TOML Exception Parser & Zenith Prompt Routing

  Phase 2: Pluggable MAC Framework & Integrity Labels (Months 3–6)
  ├── FreeBSD Pluggable MAC Policy Architecture & Vnode Labeling
  ├── `mac_biba` Multi-Level Integrity & `mac_lomac` Low-Watermark Policies
  └── `mac_portacl` Restricted Network Binding Policy

  Phase 3: AppArmor Profiles & Netlink Audit Stream (Months 6–9)
  ├── AppArmor Wildcard Path Profile Engine & `/etc/apparmor.d/` Parser
  ├── Netlink Audit Stream Daemon (`NETLINK_AUDIT`) & Log Serialization
  └── Capsicum Capability Rights Enforcement on File Descriptors

  Phase 4: Hardware Capabilities & Release Hardening (Months 9–12)
  ├── CHERI & ARM MTE Hardware Capability Bounds Checking
  ├── OpenBSD KARL Kernel Section Relinking & PinSyscall Validation
  └── Comprehensive Security Benchmark & Audit Verification
```

---

## 4. Verification and Benchmark Metrics

| Verification Subsystem | Test Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **POSIX.1e ACL Evaluation** | Extended ACL Test Suite | < 1 microsecond evaluation overhead per inode open |
| **SELinux LSM Hooks** | `stress-ng --apparmor` / SELinux Test | Zero unhandled MAC hook bypasses across 10,000 operations |
| **Landlock Sandboxing** | Landlock ABI v4 Compatibility Test | 100% path & TCP port restriction compliance for unprivileged tasks |
| **Exec Guard Default-Deny** | Untrusted Execution Test | Instant default-deny execution blocking with Zenith prompt fallback |
