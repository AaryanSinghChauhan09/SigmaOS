# SigmaOS Access Control & Security System: Gap Analysis & Actionable Roadmap

## Executive Summary

Access control and security enforcement in **SigmaOS** bridges Linux Mandatory Access Control (SELinux security contexts, Landlock rulesets in `src/security/mandatory_access_control.rs` and `src/security/landlock.rs`), OpenBSD privilege reduction (`pledge`/`unveil` in `src/security/pledge_impl.rs`), and filesystem permission evaluation (`evaluate_ext4_access`, `evaluate_ntfs_access` in `src/filesystem/ext4_ntfs_security.rs`). This document presents an exhaustive gap analysis comparing SigmaOS access controls against enterprise Linux Security Modules (LSM) and FreeBSD MAC policy framework standards, followed by a 3-phase strategic development roadmap.

---

## 1. Existing Access Control Architecture in SigmaOS

SigmaOS currently implements multi-layered access control mechanisms across security and filesystem modules:

| Component | Implementation File | Capabilities Provided |
| :--- | :--- | :--- |
| **SELinux MAC Policy Engine** | `src/security/mandatory_access_control.rs` | `SelinuxSecurityContext` (`user:role:type:level`), `SelinuxMacPolicyEngine` with rule evaluation, and `SovereignMacLsmHookRegistry` guarding inode, ptrace, and socket operations. |
| **Unprivileged Landlock Sandboxing** | `src/security/landlock.rs` | `LandlockEngine` and `LandlockRuleset` enabling unprivileged processes to restrict path access (`LANDLOCK_ACCESS_FS_READ_FILE`, `WRITE_FILE`, `EXECUTE`) beneath parent path hierarchies. |
| **OpenBSD Pledge & Unveil Sandbox** | `src/security/pledge_impl.rs` | OpenBSD-style process privilege reduction (`pledge("stdio rpath wpath cpath inet")`) and filesystem path visibility masking (`unveil(path, permissions)`). |
| **Ext4 & NTFS Access Evaluation** | `src/filesystem/ext4_ntfs_security.rs` | Ext4 permission flow (Root bypass -> POSIX ACL `system.posix_acl_access` xattr -> mode bits `rwxrwxrwx`) and NTFS Security Descriptor SID evaluation (`evaluate_ntfs_access`). |

---

## 2. Exhaustive Gap Analysis vs. Linux & BSD Standards

While basic SELinux rulesets, Landlock rules, and Ext4 ACL checks exist in SigmaOS, several systemic gaps remain when benchmarked against Linux LSM ABI v4 and FreeBSD MAC framework standards:

```
                  ┌──────────────────────────────────────────────────────────┐
                  │       SigmaOS Access Control Subsystem                   │
                  └────────────────────────────┬─────────────────────────────┘
                                               │
      ┌────────────────────────────────────────┼────────────────────────────────────────┐
      ▼                                        ▼                                        ▼
┌───────────────────────────┐    ┌───────────────────────────┐    ┌───────────────────────────┐
│ POSIX.1e Extended ACLs    │    │  Landlock ABI v4 Network  │    │ FreeBSD Pluggable MAC     │
│ GAP: Full setfacl/getfacl │    │  Restriction Hooks        │    │ Framework                 │
│ parsing & ACL_MASK logic  │    │  GAP: Lacks TCP port bind │    │ GAP: Lacks mac_biba and   │
│ missing for non-owner UIDs│    │  and connect Landlock     │    │ mac_lomac integrity labels│
└───────────────────────────┘    └───────────────────────────┘    └───────────────────────────┘
      │                                        │                                        │
      ▼                                        ▼                                        ▼
┌───────────────────────────┐    ┌───────────────────────────┐    ┌───────────────────────────┐
│ AppArmor Path-Based MAC   │    │ OpenBSD Fine-Grained      │    │ Auditd LSM Event          │
│ Profiles                  │    │ File Descriptor Capabilities│   │ Serialization             │
│ GAP: Lacks regex path profile│   │ GAP: Lacks pledge         │    │ GAP: Lacks structured audit│
│ enforcement (/etc/apparmor)│   │ capability inheritance    │    │ kernel netlink streaming  │
└───────────────────────────┘    └───────────────────────────┘    └───────────────────────────┘
```

### 2.1. POSIX.1e Extended ACL Parsing & `ACL_MASK` Enforcement (`setfacl`/`getfacl`)
* **Linux Baseline**: Linux `ext4`/`btrfs`/`xfs` support full POSIX.1e extended ACLs. Permission evaluation parses multiple user (`ACL_USER`), group (`ACL_GROUP`), and mask (`ACL_MASK`) entries, applying the `ACL_MASK` bitwise AND constraint to cap effective non-owner permissions.
* **SigmaOS Gap**: `evaluate_ext4_access` in `src/filesystem/ext4_ntfs_security.rs` checks for `system.posix_acl_access` xattr presence, but does not parse full multi-ACE binary structures or apply `ACL_MASK` re-evaluation.

### 2.2. Landlock ABI v4 Network & File Hierarchy Restriction Hooks
* **Linux Baseline**: Linux Landlock ABI v4 (`security/landlock/`) extends filesystem sandboxing with network socket rulesets (`LANDLOCK_ACCESS_NET_BIND_TCP`, `LANDLOCK_ACCESS_NET_CONNECT_TCP`), enabling unprivileged network service isolation.
* **SigmaOS Gap**: `LandlockEngine` in `src/security/landlock.rs` handles filesystem path access (`handled_access_fs`), but lacks network socket domain/port filtering rulesets.

### 2.3. FreeBSD Pluggable MAC Framework & Biba / LoMAC Integrity
* **FreeBSD Baseline**: FreeBSD MAC framework (`sys/security/mac/`) provides pluggable policy modules (`mac_biba` multi-level integrity, `mac_lomac` low-watermark integrity, `mac_portacl` restricted network binding), attaching security labels (`mac_label`) to processes, vnodes, and sockets.
* **SigmaOS Gap**: MAC enforcement in `src/security/mandatory_access_control.rs` is modeled exclusively on SELinux type enforcement, lacking FreeBSD-style pluggable multi-level integrity policies (`mac_biba`).

### 2.4. AppArmor Profile Engine & Wildcard Path Matching
* **Linux Baseline**: AppArmor (`security/apparmor/`) enforces path-based Mandatory Access Control using compiled profiles with glob patterns (e.g., `/usr/bin/firefox { /home/*/.mozilla/ r, }`), providing lightweight human-readable security profiles.
* **SigmaOS Gap**: SigmaOS lacks an AppArmor profile parser and path-matching engine.

### 2.5. Auditd LSM Kernel Audit Stream
* **Linux Baseline**: Linux LSM hooks generate structured audit records (`auditd`) streamed over Netlink sockets (`NETLINK_AUDIT`) whenever SELinux or AppArmor denies access, recording subject contexts, target inodes, and syscall arguments.
* **SigmaOS Gap**: `SovereignMacLsmHookRegistry` appends strings to an in-memory `audit_log` vector without structured kernel-to-userland Netlink streaming.

---

## 3. Actionable Strategic Development Roadmap

To bridge these gaps, the following 3-phase strategic development roadmap will be executed:

### Phase 1: POSIX.1e Extended ACL Engine & Landlock Network Hooks (Months 1–3)
1. **Full POSIX.1e Extended ACL Parser & `ACL_MASK` Evaluator**:
   - Implement binary extended ACL parsing in `src/filesystem/ext4_ntfs_security.rs` (`ACL_USER`, `ACL_GROUP`, `ACL_MASK`, `ACL_OTHER`).
   - Enforce `ACL_MASK` filtering during file access permission checks.
2. **Landlock ABI v4 Network Restriction Extensions**:
   - Update `LandlockEngine` in `src/security/landlock.rs` to support `LANDLOCK_ACCESS_NET_BIND_TCP` and `LANDLOCK_ACCESS_NET_CONNECT_TCP`.

### Phase 2: FreeBSD Pluggable MAC Modules & AppArmor Profiles (Months 3–6)
1. **FreeBSD Pluggable MAC Module Architecture**:
   - Implement pluggable MAC policy interfaces in `src/security/mandatory_access_control.rs`.
   - Add `mac_biba` multi-level integrity and `mac_portacl` network port binding policies.
2. **AppArmor Profile Engine**:
   - Build a profile parser for `/etc/apparmor.d/` with wildcard path matching.

### Phase 3: Kernel Audit Stream & Fine-Grained Capability Delegation (Months 6–12)
1. **Netlink Audit Stream Manager**:
   - Replace in-memory audit logs with a Netlink audit stream daemon for real-time security events.
2. **Capability-Aware File Descriptor Inheritance**:
   - Integrate `Landlock` and `Pledge` sandbox constraints into process inheritance (`execve`) across child tasks.

---

## 4. Verification and Benchmark Plan

| Verification Task | Test Target | Success Criterion |
| :--- | :--- | :--- |
| **POSIX.1e ACL Mask Filtering** | `test_posix_acl_mask_evaluation` | `ACL_MASK` restricts user ACE permissions correctly |
| **Landlock TCP Port Restriction** | `test_landlock_net_bind_restriction` | Disallows unprivileged TCP bind outside permitted port set |
| **FreeBSD Biba Integrity Label** | `test_mac_biba_integrity_read_down` | Low-integrity process cannot write to High-integrity vnode |
| **AppArmor Profile Match** | `test_apparmor_path_glob_match` | Evaluates wildcard path profiles for binary execution |
