# Security Architecture

SigmaOS implements a 6-layer security model with each layer independently providing defense.

---

## The 6 Layers

| Layer | Mechanism | Status | Source |
|-------|-----------|--------|--------|
| 1 | Ring-3 driver isolation | ✅ | `drivers/ddk/sigma_ddk.rs` |
| 2 | sigma_pledge + sigma_unveil | ✅ | `kernel/security/sigma_pledge.rs` |
| 3 | Linux capabilities (41 caps) | ✅ | `kernel/security/sigma_capability.rs` |
| 4 | Namespaces + cgroups | ✅ | `kernel/core/sigma_namespaces.rs`, `sigma_cgroups.rs` |
| 5 | Zero-trust + TPM2 attestation | 🔄 | `security/` |
| 6 | Post-quantum crypto | ✅ design | `crypto/` |

---

## sigma_pledge — Process Capability Declaration

```c
// Process declares what it needs before doing anything sensitive
sigma_pledge("stdio rpath inet");

// After pledge: any syscall outside the set → SIGKILL + audit
// "stdio"  = read, write, close, etc.
// "rpath"  = open files for reading
// "inet"   = network sockets
// "wpath"  = write files
// "exec"   = fork/exec
// "proc"   = process management
// "audio"  = audio device access
// "video"  = GPU/display access
```

Pledge is **one-way**: you can only restrict further, never expand.

---

## sigma_unveil — Filesystem Path Restriction

```c
sigma_unveil("/etc", "r");     // read-only access to /etc
sigma_unveil("/tmp", "rwc");   // read/write/create in /tmp
// sigma_unveil lock — all other paths now DENIED

// Attempting open("/home/user/.ssh/id_rsa") → EACCES + audit
```

---

## Linux Capabilities (41 fine-grained)

```c
// Check before privileged operation
if (!sigma_cap_check(pid, CAP_NET_ADMIN, SYS_SOCKET)) {
    return -EPERM;
}

// Drop capabilities permanently
sigma_cap_drop(pid, CAP_SYS_RAWIO);    // can never do raw I/O again
sigma_cap_drop(pid, CAP_SYS_MODULE);   // can never load kernel modules

// Key caps for containers:
// CAP_SYS_ADMIN   — broad admin (avoid granting)
// CAP_NET_ADMIN   — network configuration
// CAP_NET_BIND_SERVICE — bind port < 1024
// CAP_SYS_PTRACE  — debug other processes
```

---

## Containers: Namespaces + cgroups

```c
// Isolated container
uint32_t pid_ns  = sigma_ns_create(NS_PID, 1);
uint32_t net_ns  = sigma_ns_create(NS_NET, 1);
uint32_t uts_ns  = sigma_ns_create(NS_UTS, 1);
sigma_ns_set_hostname(uts_ns, "my-container", 12);

uint32_t cgroup = sigma_cgroup_create("my-container", 12, 1);
sigma_cgroup_set_memory(cgroup, 256 * 1024 * 1024);  // 256MB
sigma_cgroup_set_cpu(cgroup, 500);                     // 50% weight

// Attach container PID to all namespaces and cgroup
sigma_ns_attach(pid_ns, container_pid);
sigma_cgroup_attach(cgroup, container_pid);
```

---

## Post-Quantum Cryptography

All crypto is quantum-safe by default:

| Purpose | Algorithm | Security Level |
|---------|-----------|---------------|
| TLS key exchange | ML-KEM-1024 (Kyber) | 256-bit quantum |
| Package signing | ML-DSA-87 (Dilithium) | 256-bit quantum |
| Boot verification | ML-DSA-87 | 256-bit quantum |
| Disk encryption | AES-256-XTS + Kyber | Classical + PQC |

NIST FIPS 203 (ML-KEM) and FIPS 204 (ML-DSA) finalized standards.

---

## Attack Surface Comparison

| Attack | Windows | Linux | SigmaOS |
|--------|---------|-------|---------|
| Driver exploit → kernel | ✅ direct | ✅ direct | ❌ ring-3 isolated |
| Compromised process reads files | ✅ allowed | ✅ allowed | ❌ sigma_unveil blocks |
| Malicious package | 🔄 antivirus | 🔄 package signing | ❌ Dilithium-5 + verity |
| Quantum crypto attack | ✅ vulnerable | ✅ vulnerable | ❌ ML-KEM/ML-DSA |
| Tampered boot | 🔄 Secure Boot | 🔄 UEFI SB | ❌ TPM PCR + verity |

---

*Sources: `kernel/security/`, `kernel/core/sigma_cgroups.rs`, `kernel/core/sigma_namespaces.rs`, `crypto/`*
*See also: [docs/SECURITY_MODEL.md](../docs/SECURITY_MODEL.md) · [Post-Quantum-Security](Post-Quantum-Security)*
