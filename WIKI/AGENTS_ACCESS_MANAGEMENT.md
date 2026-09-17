# AI Agent Comprehensive Access Operations Management Architecture (`docs/AGENTS_ACCESS_MANAGEMENT.md`)

This guide details the technical architecture, access control matrices, directory services, and AI agent monitoring protocols for access operations in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS provides multi-layered access management spanning authentication, filesystem security, remote access tools, memory safety, and hardware I/O:

### A. Access Control Matrix & Security Tokens
- Located in `src/access/control.rs` and `src/security/`.
- Implements `AccessControlMatrix` (ACM 2D grid rights mapping), Extended POSIX ACLs, Bell-LaPadula MLS (MAC), and security token capability bounding sets.
- Enforces file read/write access permissions via `FileAttributeAccessControl`.

### B. Directory Services & Wireless Access
- Located in `src/access/mod.rs` and `src/auth/user.rs`.
- Implements `LdapAccessClient` and `LdapUserEntry` for enterprise Active Directory / LDAP authentication.
- Manages wireless access point WPA3/Enterprise credentials and anonymous guest client isolation.

### C. Remote Access Tools & File Controllers
- Located in `src/access/mod.rs` and `src/cloud/storage.rs`.
- Manages remote access tool (RAT) session states (controlling vs passive), remote file mounts, and encrypted cloud/remote path sync.

### D. Memory Protection, Path Resolution & I/O Modes
- Located in `src/kernel/memory/`, `src/arch/cpu_sys.rs`, and VFS storage engines.
- Enforces guard page allocations, `W^X` memory protection, and canonical relative-to-direct path resolution.
- Optimizes sequential I/O read-ahead and random access time seek latency on storage block devices.

---

## 2. AI Agent Operational Directives

1. **Canonical Path Resolution:** Always canonicalize relative file paths before submitting to VFS or Landlock sandboxes.
2. **Access Rights Evaluation:** Verify that read/write operations validate POSIX DAC mode bits, MAC security levels, and `AccessControlMatrix` rights.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm access control and directory service unit tests pass.
