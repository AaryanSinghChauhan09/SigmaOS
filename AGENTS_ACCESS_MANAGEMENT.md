# SigmaOS AI Agent Comprehensive Access Operations Management Directive (`AGENTS_ACCESS_MANAGEMENT.md`)

This document defines technical directives, permission enforcement protocols, and security access guidelines for AI agents managing the complete access lifecycle in SigmaOS.

---

## 1. Core Principles for Comprehensive Access Operations Management

Access operations across SigmaOS govern directory services, filesystem access, remote tools, memory isolation, process migration, and hardware I/O modes. AI agents modifying access routines must observe the following rules across all access categories:

### A. Directory, Identity & Remote Access
1. **Lightweight Directory Access Protocol (LDAP) & SSSD (`LdapAccessClient`, `LdapUserEntry`):**
   - Bind requests (`bind`) and user identity resolution (`search_user`) must utilize encrypted TLS connections (`ldaps://`).
   - Active Directory & SSSD credentials must enforce password complexity and Kerberos TGT ticket caching.
2. **Anonymous vs Authenticated Access:**
   - Anonymous access clients must be restricted to unprivileged read-only public endpoints with rate-limiting.
   - Authenticated sessions require valid security tokens and capability bounding set evaluation.
3. **Remote Access Tools (RAT) & Remote Files:**
   - Remote access controllers must distinguish active controlling sessions from passive view sessions.
   - Remote file mounts require transport encryption and authenticated mount handles.
4. **Wireless Access Points (WAP):**
   - Wireless AP connections must enforce WPA3/Enterprise authentication and landlock network isolation.

### B. Filesystem, Path & Storage I/O Access
1. **Direct vs Relative Path Access:**
   - All relative file path operations must be canonicalized to absolute paths before Landlock v5 or pledge/unveil security checks to prevent path traversal bypasses.
2. **Effective Access Time (`atime`):**
   - File access timestamp updates (`atime`, `mtime`, `ctime`) must observe POSIX `relatime` or `noatime` mount flags to reduce disk wear while preserving audit logging integrity.
3. **Read / Write Permission Enforcers (`FileAttributeAccessControl`):**
   - POSIX mode bits (DAC), Extended ACLs, immutable flags (`SIMMUT`), and Access Control Matrix (`AccessControlMatrix`) permissions must be evaluated before granting file read or write access handles.
4. **Sequential vs Random Storage Access:**
   - Storage I/O engines must optimize read-ahead buffering for sequential workloads and minimize seek latency (random access time) for random access block devices.

### C. Memory, Protection & Process Migration Access
1. **Memory Access Protection:**
   - Enforce Write-XOR-Execute (`W^X`) / Data Execution Prevention (`DEP`), guard page zones around dynamic allocations, and stack clash protection (`has_guard_page`).
2. **Process Migration Control:**
   - Live process state and memory migration between cores or nodes must verify RCU epochs (`rcu_epoch`) and encrypt process memory state before transfer.
3. **Security Access Tokens & Bounding Sets:**
   - User security access tokens must enforce capability bounding sets to prevent unauthorized privilege escalation.

---

## 2. Pre-Commit Access Operations Verification Checklist

Before submitting code modifications, AI agents must verify:
- [ ] Relative file paths canonicalize safely prior to security policy evaluation.
- [ ] Read and write permission checks evaluate DAC, MAC, and `AccessControlMatrix` rules.
- [ ] LDAP directory queries handle unauthenticated/unbound states gracefully without panicking.
- [ ] Memory access protections preserve guard page zones and `W^X` invariant rules.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
