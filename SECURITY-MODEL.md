# SigmaOS Security Model

> Comprehensive security architecture of SigmaOS - a defense-in-depth approach inspired by OpenBSD, Qubes OS, grsecurity, and modern Linux hardening.

---

## 🔐 Security Philosophy

SigmaOS follows the **"Secure by Default, Sovereign by Design"** philosophy:

1. **Principle of Least Privilege** - Every component gets minimum required permissions
2. **Defense in Depth** - Multiple independent security layers
3. **Fail Secure** - Failures default to denial, not permission
4. **Transparency** - All security decisions are logged and auditable
5. **Zero Trust** - No implicit trust, even for local processes

---

## 🛡️ Security Layers

```
┌───────────────────────────────────────────┐
│           Application Layer                  │
│  ┌────────────────────────────────────┐  │
│  │  Sandboxing (pledge/unveil/seccomp)  │  │
│  │  Capability Tokens                   │  │
│  │  AppArmor/SELinux Profiles           │  │
│  └────────────────────────────────────┘  │
├───────────────────────────────────────────┤
│           Network Layer                       │
│  ZenithNet Zero-Trust Router                  │
│  WireGuard Encryption, pf-style Firewall      │
├───────────────────────────────────────────┤
│           Kernel Layer                        │
│  ASLR + PIE, Stack Canaries, FORTIFY_SOURCE   │
│  Kernel Lockdown, LSM (MAC), W^X              │
│  Secure Boot (UEFI), TPM Integration          │
└───────────────────────────────────────────┘
```

---

## 🔑 Capability System

Inspired by **FreeBSD Capsicum** and **OpenBSD pledge/unveil**:

```rust
// Example: capability-restricted file access
let cap = CapabilityToken::new()
    .allow_read("/home/user/docs")
    .allow_write("/tmp")
    .deny_network()
    .deny_exec();

process.apply_capability(cap);
```

### Available Capabilities
| Capability | Description |
|-----------|-------------|
| `CAP_READ` | Read files within unveil path |
| `CAP_WRITE` | Write files within unveil path |
| `CAP_EXEC` | Execute binaries |
| `CAP_NET` | Network access |
| `CAP_AUDIO` | Audio device access |
| `CAP_VIDEO` | Camera/screen capture |
| `CAP_GPU` | GPU compute access |
| `CAP_USB` | USB device access |

---

## 🔐 Post-Quantum Cryptography

SigmaOS implements **NIST PQC standards** for quantum-resistant security:

| Algorithm | Type | Standard | Status |
|-----------|------|----------|--------|
| **ML-KEM** (Kyber) | Key Encapsulation | FIPS 203 | ✅ Implemented |
| **ML-DSA** (Dilithium) | Digital Signature | FIPS 204 | ✅ Implemented |
| **SLH-DSA** (SPHINCS+) | Hash-based Sig | FIPS 205 | 🚧 In Progress |
| **X25519** | Classical KEM | RFC 7748 | ✅ Implemented |
| **Ed25519** | Classical Sig | RFC 8032 | ✅ Implemented |

---

## 🛡️ Rootkit Detection

The `RootkitDetector` (inspired by rkhunter + Linux Kernel Runtime Guard):

```bash
# Run rootkit scan
sigma-security scan --rootkits --comprehensive

# Continuous monitoring
systemctl enable --now sigma-sentinel

# View scan results
sigma-security report --last 24h
```

### Detection Methods
- **Syscall table integrity** - Verify no hooks
- **Kernel module verification** - Check module signatures
- **Process hiding detection** - Compare /proc with kernel data
- **File integrity** - AIDE-style monitoring
- **Network anomalies** - Detect hidden connections

---

## 🔒 Boot Security Chain

```
UEFI Secure Boot
    ↓
Shim (Microsoft-signed)
    ↓
GRUB (Shim-validated)
    ↓
TPM2 PCR Measurement
    ↓
SigmaOS Kernel (signed)
    ↓
initramfs (verified)
    ↓
Root filesystem (dm-verity)
    ↓
Full Disk Encryption (LUKS2+Argon2id)
    ↓
Runtime: Kernel Lockdown Mode
```

---

## 🌐 Zero-Trust Network

ZenithNet implements **BeyondCorp-style zero-trust**:

- **No implicit trust** for local network devices
- **Per-packet authentication** using WireGuard
- **Microsegmentation** - isolated network namespaces per app
- **Continuous verification** - re-auth on behavior anomaly
- **Encrypted DNS** - DoH/DoT by default

```bash
# Configure zero-trust policy
sigma-net policy add --app firefox \
  --allow-out 443 \
  --allow-out 80 \
  --deny-in all \
  --isolate
```

---

## 📖 Security Audit

```bash
# Run full security audit
sigma-security audit --full

# Check CVE exposure
sigma-security cve-check --severity critical,high

# Verify system integrity
sigma-security integrity --verify-all

# Generate security report
sigma-security report --format pdf > security-report.pdf
```

---

*SigmaOS Security Model Documentation | Updated: 2026-08-23*