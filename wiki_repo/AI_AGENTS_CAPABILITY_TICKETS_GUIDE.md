# SigmaOS AI Agents Capability Tickets & Cryptographic Access Control Guide

Welcome to the **SigmaOS AI Agents Capability Tickets Guide**. This document details capability tokens, cryptographic access tickets, OpenBSD pledge promise tokens, FreeBSD Capsicum rights, and Post-Quantum Cryptography (PQC) token gating for autonomous AI agents and security developers in SigmaOS.

---

## 1. Capability Ticket Architecture Overview

In SigmaOS (`src/security/capability_token.rs`, `src/security/capability_enforcer.rs`, `src/security/pqc_enclave.rs`), access permissions are granted via immutable or rotatable **Capability Tickets & Tokens** rather than ambient superuser privileges:

### Capability Ticket Categories
1. **PQC Capability Tickets (`src/security/pqc_enclave.rs`)**: Post-Quantum Cryptographic tokens signed with Dilithium-5 lattice signatures containing process ID, granted permission bits, and expiration timestamps.
2. **POSIX Bounding Capabilities (`src/security/capability_enforcer.rs`)**: Linux-style capability bitmasks (`CAP_NET_BIND_SERVICE_BIT`, `CAP_SYS_ADMIN_BIT`, `CAP_DAC_OVERRIDE_BIT`).
3. **OpenBSD Pledge Promise Tokens (`src/security/pledge.rs`)**: Declarative process promise sets (`stdio`, `rpath`, `wpath`, `inet`, `exec`).
4. **FreeBSD Capsicum Capability Descriptors (`src/security/capsicum.rs`)**: File descriptor rights (`CAP_READ`, `CAP_WRITE`, `CAP_FSTAT`, `CAP_MMAP`).

---

## 2. PQC Capability Token Issuance & Validation

AI agents requiring system call authorization or cross-shard IPC access must request and validate PQC capability tokens using `PqcTokenGate` (`src/security/pqc_enclave.rs`):

```rust
use sigmaos::security::pqc_enclave::{PqcCapabilityToken, PqcTokenGate};

let mut gate = PqcTokenGate::new();

// Issue an ephemeral PQC capability ticket for a process
let ticket = PqcCapabilityToken::issue(
    101,                  // Process ID
    0b0000_1111,          // Permission bitmask (Read/Write/Exec/IPC)
    3600                  // Validity duration in seconds
);

// Verify capability ticket before executing sensitive kernel operation
assert!(gate.validate_token(&ticket, 101, 0b0000_0001)); // Verify read permission
```

---

## 3. Bounding Capability Enforcement in `SecurityEnforcer`

Microkernel system calls check `CapabilityToken` in `src/security/capability_enforcer.rs`:

```rust
use sigmaos::security::capability_enforcer::{CapabilityToken, SecurityEnforcer, CAP_NET_BIND_SERVICE_BIT};

let token = CapabilityToken::new(202)
    .grant_posix_capability(CAP_NET_BIND_SERVICE_BIT);

let enforcer = SecurityEnforcer::new();
// Validate network socket bind attempt below port 1024
assert!(token.has_posix_capability(CAP_NET_BIND_SERVICE_BIT));
```

---

## 4. Best Practices for AI Agents Managing Capability Tickets

1. **Deny-by-Default Principle**: Unassigned or expired capability tickets result in immediate denial of service or syscall execution.
2. **Key Rotation & Ephemeral Lifetimes**: Rotate PQC capability ticket signing keys periodically and keep ticket TTLs as short as practical.
3. **Monotonic Capability Reduction**: Processes can drop capability bits during execution but can NEVER elevate capabilities without re-authentication.

---

## 5. Checklist for AI Agents Managing Capability Tickets

- [ ] Confirmed PQC Dilithium-5 signature verification succeeds before ticket parsing.
- [ ] Checked expiration timestamps (`expiration_timestamp > current_timestamp`).
- [ ] Ensured capability tokens are tied specifically to caller PID (`ticket.pid == caller_pid`).
- [ ] Executed `./run_sigma_tests.sh` to confirm capability and enclave test suites pass cleanly.
