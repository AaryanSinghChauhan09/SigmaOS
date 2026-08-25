# 🛡️ SigmaOS Security

SigmaOS implements **defence-in-depth** security across every system layer.

## Security Model Overview

```
Boot → Kernel → MAC → Application → Network → Crypto
 ↑       ↑       ↑        ↑           ↑         ↑
TPM2   KSPP   SELinux  pledge()   eBPF-FW   Kyber/
Secure KASLR  AppArmor unveil()  WireGuard  Dilithium
Boot   SMEP/            Seccomp    DoH        PQC-TLS
       SMAP             sandbox    Zero-Trust  1.3
```

## Boot Security

### UEFI Secure Boot
- Only signed bootloaders/kernels can boot
- Sigma kernel image signed with Dilithium-5 (post-quantum)
- Key revocation via UEFI DBX list

### TPM 2.0 Integration
- PCR (Platform Configuration Register) measurements during boot
- Sealed disk encryption key released only if boot measurements match
- Remote attestation capability for cloud deployments

### Unified Kernel Image (UKI)
- Kernel, initramfs, and cmdline bundled and signed together
- Prevents cmdline tampering attacks
- Hash verified at load time: `H(kernel) + H(initramfs) + H(cmdline)`

## Kernel Hardening (KSPP)

| Feature | Description |
|---------|-------------|
| KASLR | Kernel Address Space Layout Randomization |
| SMEP | Supervisor Mode Execution Prevention |
| SMAP | Supervisor Mode Access Prevention |
| W^X | Writable XOR Executable memory |
| Stack Canaries | Stack buffer overflow detection |
| CFI | Control Flow Integrity |
| KPTI | Kernel Page Table Isolation (Meltdown) |
| Retpoline | Spectre v2 mitigation |

## Mandatory Access Control

### SELinux
- Type enforcement (TE) policies
- Role-based access control (RBAC)
- Multi-level security (MLS)
- Policy enforcement via LSM hooks

### AppArmor
- Path-based profiles per application
- Network capability restriction
- File operation allowlists
- Signal and ptrace controls

## Application-Level Security

### pledge() / unveil() (OpenBSD-inspired)
```c
// Process declares only capabilities it needs
pledge("stdio rpath wpath network", NULL);

// Filesystem access restricted to declared paths
unveil("/home/user/docs", "r");
unveil("/tmp", "rwc");
```

### Seccomp-BPF
- Syscall allowlist per process
- BPF program filters arbitrary syscall args
- KILL_PROCESS or ERRNO on violation

### Sandboxing
- DistroSandbox: Landlock + seccomp + cgroups v2
- OCI containers: full namespace isolation
- Browser sandbox: multiple layers of restrictions

## Cryptography

### Post-Quantum Algorithms

| Algorithm | Type | Security Level | Use Case |
|-----------|------|---------------|----------|
| Kyber-1024 | KEM | AES-256 equivalent | Key exchange |
| Dilithium-5 | Signature | NIST Level 5 | Code signing |
| SPHINCS+ | Signature | Hash-based | Long-term signing |
| X25519 | KEM (classical) | 128-bit | Hybrid mode |
| Ed25519 | Signature (classical) | 128-bit | Hybrid mode |

### PQC-TLS 1.3
- **Key exchange**: Kyber-1024 + X25519 hybrid
- **Authentication**: Dilithium-5 certificates
- **Cipher suite**: ChaCha20-Poly1305 / AES-256-GCM

## Network Security

### eBPF Firewall
- XDP (eXpress Data Path) for line-rate packet filtering
- TC (Traffic Control) for egress filtering
- Stateful connection tracking
- Rate limiting and DDoS mitigation

### Zero-Trust Network
- Every connection authenticated (never trust, always verify)
- Mutual TLS (mTLS) for service-to-service
- Micro-segmentation via network policies
- Identity-based access (not IP-based)

### WireGuard VPN
- Modern cryptography (ChaCha20, Poly1305, Curve25519)
- Minimal attack surface (~4000 lines of code)
- Fast handshake (1 RTT)
- Kernel-level implementation

## Threat Detection (Sentinel)

Sentinel is SigmaOS's real-time threat detection system:

- **Behavioral analysis**: Detects anomalous process behavior
- **Signature matching**: Known malware pattern detection
- **Syscall auditing**: Records all security-relevant syscalls
- **Network anomaly detection**: Detects C2 traffic patterns
- **Integrity monitoring**: Detects unauthorized file modifications

## Vulnerability Management

- **CVE database**: Local vulnerability database with severity scoring
- **Automated scanning**: Scans installed packages against CVEs
- **Remediation planning**: Generates hardening recommendations
- **Penetration testing framework**: Simulated pentest for security validation

## Security Reporting

Report vulnerabilities via [GitHub Security Advisories](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories).
