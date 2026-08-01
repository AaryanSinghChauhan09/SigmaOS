# SigmaOS Security Model

SigmaOS implements a multi-layered security model combining the best ideas
from OpenBSD (pledge/unveil), Linux (capabilities, namespaces, seccomp), and
modern zero-trust architecture.

---

## Layers

```text
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

```text
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
| ----------- | ----------- | ------ |
| Key exchange (TLS) | Kyber-1024 | `crypto/sigma_kyber.rs` |
| Signatures (packages) | Dilithium-5 | `crypto/sigma_dilithium.rs` |
| Hash (integrity) | SHA-3-256 | `crypto/` |
| Random | SHAKE-256 PRNG | `kernel/core/sigma_irq.rs` |

Standard algorithm names (FIPS 203/204): ML-KEM-1024, ML-DSA-87.

---

## Syscall Security

Every syscall goes through three checks before execution:

```text
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
| --------- | --------- | ------- | --------- |
| Driver isolation | ❌ ring-0 | ❌ ring-0 | ✅ ring-3 option |
| Syscall allowlist | ❌ | ✅ seccomp | ✅ sigma_pledge |
| Path restriction | ❌ | ❌ | ✅ sigma_unveil |
| Linux capabilities | ❌ | ✅ 41 caps | ✅ 41 caps |
| Namespaces | ❌ | ✅ | ✅ |
| PQC crypto | ❌ | ❌ | ✅ ML-KEM + ML-DSA |
| TPM measured boot | 🔄 | 🔄 | ✅ |

---

*Sources: `kernel/security/` · `drivers/ddk/sigma_ddk.rs` · `crypto/`*


---
## Merged from Security-Model.md
# SigmaOS Security Model

SigmaOS is built security-first: post-quantum cryptography, capability-based isolation, and zero-trust enforcement are baked in from the lowest kernel level.

---

## Threat Model

| Threat | Mitigation |
|--------|-----------|
| Compromised user process | sigma_pledge + sigma_unveil restrict blast radius |
| Kernel memory disclosure | ASLR 42-bit + SMEP/SMAP enforcement |
| Code injection | W^X: no page is write + exec simultaneously |
| Side-channel attacks on crypto | PQC algorithms (Kyber, Dilithium) designed for constant-time |
| Harvest-now-decrypt-later | Kyber-1024 KEM in all TLS, package signing, attestation |
| Malicious kernel module | Signed module loading + formal verification target (Phase H) |
| Compromised boot | TPM2 attestation + dm-verity + immutable root A/B |
| Privilege escalation | Capability token system (sigma-bus) + namespace isolation |

---

## sigma_pledge — Process Capability Restriction

Inspired by OpenBSD `pledge(2)`. A process declares its capability set at `execve()` time. The kernel enforces the allowlist at syscall entry via BPF filter.

```c
// Process declares it only needs file I/O and networking
sigma_pledge("stdio rpath wpath cpath inet", NULL);

// Any syscall outside this set → SIGABRT
```

**Implemented capability sets:**

| Set | Syscalls permitted |
|-----|--------------------|
| `stdio` | read, write, recv, send, fstat, close, exit |
| `rpath` | open (O_RDONLY), stat, access, readdir |
| `wpath` | open (O_WRONLY), truncate, rename |
| `cpath` | open (O_CREAT), mkdir, unlink |
| `inet` | socket, connect, bind, accept, listen |
| `proc` | fork, execve, waitpid, kill |
| `crypto` | getrandom, getentropy |
| `unveil` | use sigma_unveil API |

---

## sigma_unveil — Filesystem Path Restriction

Inspired by OpenBSD `unveil(2)`. A process whitelists specific filesystem paths with specific permissions. All other paths are invisible.

```c
sigma_unveil("/home/user/docs", "rw");    // read+write
sigma_unveil("/usr/lib", "r");            // read only
sigma_unveil(NULL, NULL);                 // lock — no more unveil calls
// Attempt to open /etc/passwd → ENOENT
```

---

## AVC — Access Vector Cache

An O(1) SELinux-inspired MAC policy cache. Every access check (file, socket, IPC) hits the AVC before the slower policy engine. Cache hit ratio typically > 99%.

```
Process context (label) + Object context (label) + Access class
  → AVC lookup
  → HIT: allow/deny immediately
  → MISS: query policy engine → cache result
```

---

## Post-Quantum Cryptography

NIST-standardised algorithms (FIPS 203/204) used throughout:

| Use Case | Algorithm | Key Size |
|----------|-----------|---------|
| Key exchange (TLS) | X25519 + Kyber-1024 hybrid | 1568 bytes (PQ) |
| Package signing | Dilithium-5 | 4595-byte signature |
| Audit trail integrity | BLAKE2b-256 | 32 bytes |
| Package content hash | BLAKE3 | 32 bytes |
| Symmetric encryption | AES-256-GCM | 256-bit key |
| Key derivation (CryptFS) | Argon2id | configurable |

**Why hybrid X25519+Kyber?**
Hybrid key exchange provides classical security today (X25519) plus quantum resistance (Kyber-1024), with no regression if either algorithm is broken.

---

## Zero-Trust Architecture

Every process, driver, and daemon gets a SPIFFE workload identity (SVID). All IPC through sigma-bus requires capability tokens — no ambient authority.

```
Process A wants to call Process B:
  1. A presents its SVID to sigma-bus
  2. sigma-bus verifies SVID against trust fabric
  3. sigma-bus issues a capability token for the specific operation
  4. B validates token before servicing request
  5. Token is single-use (replays rejected)
```

---

## TPM2 Integration

- **CryptFS**: boot-time unsealing of disk encryption key via TPM2 PCR measurements
- **Attestation**: `sigma-trustd` provides remote attestation reports (PCR + event log)
- **Secure boot**: sigma-boot.efi will be signed and its hash sealed into TPM2 (Phase G)

---

## Boot Security Chain

```
UEFI Secure Boot
  └── sigma-boot.efi (Dilithium-5 signed, Phase G)
        └── TPM2 PCR measurement
              └── dm-verity root partition check
                    └── Kernel loads with SMEP/SMAP/KASLR
                          └── CryptFS unseals with TPM2
                                └── Immutable root mounted read-only
```

---

## Known Open Security Issues

| ID | Description | Severity |
|----|-------------|---------|
| #1009 | CryptFS `derive_key()` returns 32 zero bytes — all encryption is currently fake | Critical |
| #1007 | `sigma-boot.efi` does not exist — cannot do verified boot yet | High |
| No KASLR | Kernel ASLR not yet implemented for kernel text | Medium |

---

*See also: [PQC-Hardening](PQC_HARDENING) · [Sandbox-Hardening](Sandbox-Hardening) · [SECURITY.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SECURITY.md)*
