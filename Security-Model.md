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
