# 🔒 SigmaOS Security Framework

> SigmaOS is built on the principle that **security is not a feature — it is the architecture**. Every syscall, every IPC message, every file access, and every network packet passes through a hardware-enforced capability gate.

---

## 🏛️ Security Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Application                              │
│         (explicit capabilities declared at launch)           │
├─────────────────────────────────────────────────────────────┤
│                sigma_pledge / sigma_unveil                    │
│     (syscall filtering — deny all not explicitly allowed)    │
├─────────────────────────────────────────────────────────────┤
│                   S-SEC Capability Gate                       │
│     (64-bit hardware token validation per syscall)           │
├──────────────┬────────────────┬────────────────────────────┤
│   PQC Layer  │   MAC Engine   │   Audit Logger              │
│ (Kyber-1024  │ (Bell-LaPadula │ (Tamper-evident event log)  │
│ +Dilithium5) │  MLS Policy)   │                             │
├──────────────┴────────────────┴────────────────────────────┤
│               Hardware (TPM 2.0 / Intel TXT)                 │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔑 Capability-Based Security

### What Are Capabilities?

A capability is an unforgeable, hardware-enforced token that grants specific permissions. Unlike traditional UNIX permissions (based on user IDs and group IDs), capabilities are:

- **Object-specific** — "Read `/etc/passwd`" not "read as root"
- **Delegatable** — A process can share a subset of its capabilities with a child
- **Revocable** — The OS can revoke any capability at any time
- **Audit-logged** — Every capability use is recorded

```rust
pub struct CapabilityToken {
    id: u64,
    permissions: Vec<Permission>,
    expiry: Option<u64>,
    delegatable: bool,
}

pub enum Permission {
    ReadFile(PathBuf),
    WriteFile(PathBuf),
    NetworkConnect { host: String, port: u16 },
    NetworkBind { port: u16 },
    ExecuteProcess(PathBuf),
    InvokeAi,
    AccessGpu,
    // ... 50+ fine-grained permissions
}
```

### CapabilityGate

The `CapabilityGate` validates every privileged operation:

```rust
let gate = CapabilityGate::new();
let result = gate.validate(&token, &Permission::ReadFile(path.clone()));
match result {
    Ok(()) => { /* proceed */ }
    Err(e) => { /* denied, log the attempt */ }
}
```

---

## 🛡️ sigma_pledge — Syscall Filtering

Inspired by OpenBSD's `pledge()`, `sigma_pledge` allows processes to declare exactly which syscall categories they need. All others are denied with `EPERM`.

```rust
// A web server only needs networking and file I/O
sigma_pledge!(["inet", "rpath", "wpath", "proc"]);

// A script interpreter needs only execution
sigma_pledge!(["exec", "rpath"]);

// After pledging, attempting a restricted syscall → immediate kill
```

### Pledge Namespaces

| Namespace | Grants | Example use |
|-----------|--------|-------------|
| `inet` | TCP/UDP socket creation | Web servers |
| `rpath` | Read filesystem paths | File readers |
| `wpath` | Write filesystem paths | File writers |
| `exec` | Execute programs | Shells |
| `proc` | Process management | Init systems |
| `ai` | AI inference via sigma-aid | AI-powered apps |
| `crypto` | PQC operations | Security tools |
| `tty` | Terminal interaction | Interactive apps |
| `dns` | DNS resolution | Network clients |
| `unveil` | sigma_unveil syscall | Path-narrowing |

---

## 🌐 Post-Quantum Cryptography

SigmaOS is one of the first OS implementations to use **NIST FIPS 203/204** algorithms natively:

### Key Encapsulation: Kyber-1024

Used for all key exchange operations (TLS, VPN, storage key derivation):

```rust
// Key encapsulation
let (public_key, secret_key) = kyber1024::keypair();
let (ciphertext, shared_secret) = kyber1024::encapsulate(&public_key);
let decrypted_secret = kyber1024::decapsulate(&ciphertext, &secret_key);
assert_eq!(shared_secret, decrypted_secret);
```

**Security level:** 256-bit post-quantum, equivalent to AES-256 classical security.

### Digital Signatures: Dilithium-5

Used for all authentication (package signatures, boot chain, kernel modules):

```rust
// Signing
let (sign_pk, sign_sk) = dilithium5::keypair();
let signature = dilithium5::sign(message, &sign_sk);

// Verification
assert!(dilithium5::verify(message, &signature, &sign_pk).is_ok());
```

**Security level:** NIST Level 5 (highest security parameter set).

### Hybrid Encryption

For bulk data, SigmaOS uses a hybrid approach:
- **Kyber-1024** for key encapsulation
- **AES-256-GCM** for data encryption (hardware-accelerated)
- **ChaCha20-Poly1305** as software fallback on non-AES hardware

---

## 🔍 Intrusion Detection System (IDS)

The `IntrusionDetectionSystem` monitors for security events in real-time:

```rust
pub trait IntrusionDetectionSystem {
    fn add_rule(&mut self, rule: DetectionRule);
    fn analyze_event(&self, event: &SecurityEvent) -> DetectionResult;
    fn get_alerts(&self) -> Vec<TrafficAlert>;
}
```

### Detection Strategies

1. **Signature Detection** — Known attack pattern matching (CVE signatures).
2. **Anomaly Detection** — Statistical baseline deviation detection.
3. **Behavioral Analysis** — Process syscall pattern analysis using the AI shard.
4. **Network Traffic Analysis** — Protocol anomaly detection on all packets.

### Alert Severity Levels

| Level | Meaning | Auto-Response |
|-------|---------|--------------|
| Info | Informational event | Log only |
| Low | Minor policy violation | Log + notify |
| Medium | Suspicious activity | Log + throttle |
| High | Active attack pattern | Log + block + notify |
| Critical | Kernel exploitation attempt | Log + kill process + emergency snapshot |

---

## 🔐 Vault: Encrypted Secret Storage

The `EncryptedFileVault` provides secure secret storage for credentials, keys, and sensitive data:

```rust
let vault = EncryptedFileVault::new(Aes256GcmEncryption::new(&master_key));
vault.store("database_password", password_bytes)?;
let retrieved = vault.retrieve("database_password")?;
```

### Encryption Algorithms Supported

| Algorithm | Type | Hardware Accel | Status |
|-----------|------|----------------|--------|
| AES-256-GCM | Symmetric | ✅ AES-NI | ✅ Complete |
| ChaCha20-Poly1305 | Symmetric | ❌ (SW) | ✅ Complete |
| Kyber-1024 | KEM | ❌ (SW) | ✅ Complete |

---

## 🔏 PKI: Certificate Management

The `PKIManager` handles X.509 certificates for the local system:

```rust
let pki = PKIManager::new();
let cert = SimpleCertificate::new(subject, public_key, validity_days);
pki.issue_certificate(cert, &ca_key)?;
pki.verify_chain(&certificate)?;
```

Key features:
- **Self-signed CA** — Each SigmaOS installation generates its own root CA at first boot.
- **Short-lived certs** — Default 24-hour validity; auto-renewed.
- **CRL** — Certificate Revocation List maintained in the secure vault.
- **OCSP stapling** — For external certificate verification.

---

## 📋 Audit Logging

Every security event is recorded in a tamper-evident audit log:

```rust
let logger = SimpleAuditLogger::new();
logger.log(SimpleAuditEvent::new(
    event_id,
    EventType::Authentication,
    user_id,
    b"User logged in from terminal",
))?;
```

### Log Integrity

- Each log entry includes a **hash chain** linking to the previous entry.
- Log entries are **Dilithium-5 signed** by the kernel at write time.
- Log rotation includes **Merkle tree root** for batch verification.
- Remote log export uses **Kyber-1024** encrypted transport.

---

## 🛡️ MAC: Mandatory Access Control

The `MACEngine` implements Bell-LaPadula Multi-Level Security:

```rust
pub trait MACEngine {
    fn get_context(&self, subject_id: usize) -> Option<&SecurityContext>;
    fn check_read(&self, subject: &SecurityContext, object: &SecurityContext) -> bool;
    fn check_write(&self, subject: &SecurityContext, object: &SecurityContext) -> bool;
}
```

### Security Levels (Bell-LaPadula)

```
TopSecret > Secret > Confidential > Unclassified
```

- **No read up** — Process at level L cannot read data at level > L.
- **No write down** — Process at level L cannot write data at level < L.
- MLS policy enforced at file open, IPC send, and network connect.

---

## 🔗 Related Pages

- [Advanced Absorption Matrix](Advanced_Absorption) — Security tool absorption
- [Sigma AI Agents](Sigma_AI_Agents) — AI-assisted intrusion detection
- [SigmaFS Innovations](SigmaFS_Innovations) — PQC filesystem encryption
- [Maturity & Distro-Parity Roadmap](Maturity_Parity_Roadmap) — Security phases
