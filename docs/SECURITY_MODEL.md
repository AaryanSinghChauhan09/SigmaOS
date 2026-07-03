# SigmaOS Security Model

SigmaOS implements a multi-layered security model combining the best ideas
from OpenBSD (pledge/unveil), Linux (capabilities, namespaces, seccomp), and
modern zero-trust architecture.

---

## Layers

```
┌───────────────────────────────────────────────────────────────────┐
│  Layer 6: Post-Quantum Cryptography (Kyber-1024 + Dilithium-5)   │
│  All TLS, package signing, and attestation are quantum-safe       │
├───────────────────────────────────────────────────────────────────┤
│  Layer 5: Zero-Trust + TPM2 Attestation                          │
│  SPIFFE workload identities, hardware-sealed keys, measured boot  │
├───────────────────────────────────────────────────────────────────┤
│  Layer 4: Namespaces + cgroups (Container Isolation)             │
│  PID/NET/MNT/IPC/UTS/USER namespaces, resource limits            │
├───────────────────────────────────────────────────────────────────┤
│  Layer 3: Linux Capabilities (41 fine-grained caps)              │
│  CAP_NET_ADMIN, CAP_SYS_BOOT, etc. per-process                   │
├───────────────────────────────────────────────────────────────────┤
│  Layer 2: sigma_pledge + sigma_unveil (Syscall + Path allowlist) │
│  OpenBSD-inspired: process declares capabilities, kernel enforces │
├───────────────────────────────────────────────────────────────────┤
│  Layer 1: Ring-3 Driver Isolation                                │
│  All drivers optionally run as isolated userspace processes       │
└───────────────────────────────────────────────────────────────────┘
```

---

## Layer 1: Ring-3 Driver Isolation

```rust
// drivers/ddk/sigma_ddk.rs
sigma_register_driver!(SigmaDriverDescriptor {
    ring: 3,   // driver runs as isolated process
    // Crash → kernel keeps running, driver restarted
});
```

Windows and Linux: driver crash = kernel crash.  
SigmaOS: driver crash = that driver is restarted, everything else continues.

---

## Layer 2: sigma_pledge + sigma_unveil

```c
// Declare capabilities at process start
sigma_pledge("stdio rpath inet");   // NIC driver
sigma_pledge("stdio video");        // GPU driver
sigma_pledge("stdio audio");        // Audio driver

// Restrict filesystem access
sigma_unveil("/etc", "r");          // read /etc only
sigma_unveil("/tmp", "rwc");        // read/write/create /tmp
// After unveil lock: all other paths DENIED
```

Violations → SIGKILL + audit log entry. No exceptions.

**Source:** `kernel/security/sigma_pledge.rs`

---

## Layer 3: Linux Capabilities (41 caps)

```c
// Check if process has CAP_NET_ADMIN before network config
sigma_cap_check(pid, CAP_NET_ADMIN, syscall_nr);

// Drop capabilities permanently (cannot re-gain)
sigma_cap_drop(pid, CAP_SYS_RAWIO);

// Get/set capability sets
CapabilitySet caps;
sigma_capget(pid, &caps);
```

Caps are tracked in a per-PID table with audit logging for every denied check.

**Source:** `kernel/security/sigma_capability.rs`

---

## Layer 4: Namespaces + cgroups

```c
// Create a new PID namespace for a container
uint32_t ns_id = sigma_ns_create(NS_PID, parent_ns);
sigma_ns_attach(ns_id, container_pid);

// Set resource limits via cgroups
uint32_t cg = sigma_cgroup_create("my-container", 4, 1);
sigma_cgroup_set_memory(cg, 256 * 1024 * 1024);  // 256 MB
sigma_cgroup_set_cpu(cg, 512);                     // half weight
sigma_cgroup_attach(cg, container_pid);
```

Supported namespace types: PID, NET, MNT, IPC, UTS, USER.

**Sources:** `kernel/core/sigma_namespaces.rs`, `kernel/core/sigma_cgroups.rs`

---

## Layer 5: Zero-Trust + TPM2

Every workload has a SPIFFE identity:
```
spiffe://sigmaos.local/workload/nginx
spiffe://sigmaos.local/kernel/driver/e1000
```

TPM2 seals disk encryption keys against PCR values:
- PCR[0]: sigma-boot.efi hash
- PCR[1]: kernel hash  
- PCR[2]: initramfs hash
- PCR[3]: kernel command line

If any boot stage is tampered with → TPM refuses to unseal → disk stays encrypted.

---

## Layer 6: Post-Quantum Cryptography

All cryptographic operations use PQC algorithms:

| Operation | Algorithm | File |
|-----------|-----------|------|
| Key exchange (TLS) | Kyber-1024 | `crypto/sigma_kyber.rs` |
| Signatures (packages) | Dilithium-5 | `crypto/sigma_dilithium.rs` |
| Hash (integrity) | SHA-3-256 | `crypto/` |
| Random | SHAKE-256 PRNG | `kernel/core/sigma_irq.rs` |

Standard algorithm names (FIPS 203/204): ML-KEM-1024, ML-DSA-87.

---

## Syscall Security

Every syscall goes through three checks before execution:

```
syscall_dispatch(nr, args)
  │
  ├─ 1. sigma_pledge check: is this syscall in allowed set?
  │     NO → SIGKILL + audit entry
  │
  ├─ 2. sigma_capability check: does process have required CAP?
  │     NO → return EPERM
  │
  └─ 3. sigma_unveil check (for path syscalls): is path allowed?
        NO → return EACCES
```

**Source:** `kernel/core/syscall_dispatch.rs`

---

## Audit Trail

Every security event is logged:
- pledge violations (SIGKILL)
- capability denials
- unveil path denials
- TPM attestation events

Logs are immutable (append-only) and signed with Dilithium-5.

---

## Comparison

| Feature | Windows | Linux | SigmaOS |
|---------|---------|-------|---------|
| Driver isolation | ❌ ring-0 | ❌ ring-0 | ✅ ring-3 option |
| Syscall allowlist | ❌ | ✅ seccomp | ✅ sigma_pledge |
| Path restriction | ❌ | ❌ | ✅ sigma_unveil |
| Linux capabilities | ❌ | ✅ 41 caps | ✅ 41 caps |
| Namespaces | ❌ | ✅ | ✅ |
| PQC crypto | ❌ | ❌ | ✅ ML-KEM + ML-DSA |
| TPM measured boot | 🔄 | 🔄 | ✅ |

---

*Sources: `kernel/security/` · `drivers/ddk/sigma_ddk.rs` · `crypto/`*
