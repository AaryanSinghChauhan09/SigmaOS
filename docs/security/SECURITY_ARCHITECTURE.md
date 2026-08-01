# SigmaOS Security Architecture

## Overview

SigmaOS implements a comprehensive security architecture focused on defense-in-depth, zero-trust principles, and post-quantum cryptography by default. This document details the security mechanisms, threat model, and implementation strategies.

## Security Principles

### Core Principles

1. **Zero Trust**: Verify everything, trust nothing by default

2. **Defense in Depth**: Multiple layers of security controls

3. **Post-Quantum Ready**: Cryptography resistant to quantum attacks

4. **Minimal TCB**: Smallest possible Trusted Computing Base

5. **Verifiable Supply Chain**: Cryptographically verified build pipeline

6. **Capability-Based Security**: Fine-grained, composable permissions

### Threat Model

| Threat Category | Description | Mitigation |
| ----------------- | ------------- | ------------ |
| Supply Chain Attacks | Malicious code in dependencies | Reproducible builds, signed artifacts |
| Runtime Exploits | Memory corruption, ROP | Capability system, WASM sandboxing |
| Privilege Escalation | Unauthorized access to resources | Capability-based access control |
| Side-Channel Attacks | Timing, cache attacks | Constant-time cryptography |
| Quantum Threats | Harvest-now-decrypt-later | Post-quantum cryptography |
| Insider Threats | Malicious maintainers | Multi-signature verification |

## Post-Quantum Cryptography

### Default Algorithms

SigmaOS uses FIPS 203/204/205 (ML-KEM, ML-DSA, SLH-DSA) as default:

```rust
// Post-quantum cryptographic operations
use pqcrypto::mlkem::{keypair, encrypt, decrypt};
use pqcrypto::mldsa::{sign, verify};

struct PQCKeyPair {
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}

impl PQCKeyPair {
    fn generate() -> Self {
        let (pk, sk) = keypair();
        PQCKeyPair {
            public_key: pk,
            secret_key: sk,
        }
    }

    fn sign(&self, message: &[u8]) -> Vec<u8> {
        sign(message, &self.secret_key)
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        verify(message, signature, &self.public_key)
    }
}
```

### Algorithm Selection

| Operation | Algorithm | Security Level | Performance |
| ----------- | ----------- | ---------------- | ------------- |
| Key Exchange | ML-KEM-768 | NIST Level 1 | Fast |
| Signatures | ML-DSA-65 | NIST Level 3 | Medium |
| Hashing | SHA3-256 | NIST Level 2 | Very Fast |
| KDF | HKDF-SHA3-256 | NIST Level 2 | Fast |

### Hybrid Cryptography

For compatibility during transition:

```rust
// Hybrid encryption (classical + post-quantum)
struct HybridEncryptor {
    classical: RsaOaep,
    post_quantum: MlKem768,
}

impl HybridEncryptor {
    fn encrypt(&self, plaintext: &[u8]) -> HybridCiphertext {
        let ct_classical = self.classical.encrypt(plaintext);
        let ct_pq = self.post_quantum.encrypt(plaintext);

        HybridCiphertext {
            classical: ct_classical,
            post_quantum: ct_pq,
        }
    }
}
```

## Capability-Based Security

### Capability System

SigmaOS uses a capability-based security model instead of traditional Unix permissions:

```rust
// Capability structure
struct Capability {
    namespace: u64,
    permissions: u64,
    expiration: Option<Instant>,
    delegatable: bool,
}

impl Capability {
    fn check(&self, required: u64) -> bool {
        self.permissions & required == required
    }

    fn delegate(&self, subset: u64) -> Option<Capability> {
        if !self.delegatable {
            return None;
        }

        Some(Capability {
            namespace: self.namespace,
            permissions: self.permissions & subset,
            expiration: self.expiration,
            delegatable: false, // Non-delegatable by default
        })
    }
}
```

### Permission Model

```rust
// Permission bits
const PERM_READ: u64 = 0x01;
const PERM_WRITE: u64 = 0x02;
const PERM_EXECUTE: u64 = 0x04;
const PERM_ADMIN: u64 = 0x08;
const PERM_NETWORK: u64 = 0x10;
const PERM_FILE_READ: u64 = 0x20;
const PERM_FILE_WRITE: u64 = 0x40;
const PERM_PROCESS_SPAWN: u64 = 0x80;

// Process pledge system
struct ProcessPledge {
    promised_capabilities: HashSet<Capability>,
}

impl ProcessPledge {
    fn pledge(&mut self, capabilities: Vec<Capability>) {
        self.promised_capabilities = capabilities.into_iter().collect();
    }

    fn check(&self, required: u64) -> bool {
        self.promised_capabilities
            .iter()
            .any(|cap| cap.check(required))
    }
}
```

### Unveil System

```rust
// Filesystem access restriction
struct UnveilPath {
    path: PathBuf,
    permissions: u64,
}

struct ProcessUnveil {
    unveiled_paths: Vec<UnveilPath>,
}

impl ProcessUnveil {
    fn unveil(&mut self, path: &Path, permissions: u64) {
        self.unveiled_paths.push(UnveilPath {
            path: path.to_path_buf(),
            permissions,
        });
    }

    fn check_access(&self, path: &Path, required: u64) -> bool {
        self.unveiled_paths
            .iter()
            .any(|up| path.starts_with(&up.path) && up.permissions & required == required)
    }
}
```

## Secure Boot and Measured Boot

### Secure Boot Chain

```text
UEFI Firmware → Sigma Bootloader → Sigma Kernel → Sigma Init
    ↓              ↓                  ↓              ↓
  Signed         Signed             Signed         Verified
```

### Measured Boot

```rust
// TPM-based measured boot
struct MeasuredBoot {
    tpm: Tpm2Device,
    pcrs: [u8; 24], // 24 PCR registers
}

impl MeasuredBoot {
    fn measure_component(&mut self, component: &[u8]) -> Result<()> {
        let hash = sha3_256(component);
        self.tpm.extend_pcr(0, hash)?;
        Ok(())
    }

    fn verify_boot_chain(&self) -> Result<bool> {
        let expected_pcrs = self.load_expected_pcrs()?;
        Ok(self.pcrs == expected_pcrs)
    }
}
```

### Remote Attestation

```rust
// Remote attestation protocol
struct RemoteAttestation {
    tpm: Tpm2Device,
    aik: AttestationIdentityKey,
}

impl RemoteAttestation {
    async fn attest(&self, challenger: &Challenger) -> Result<Attestation> {
        let quote = self.tpm.quote(&self.aik, &self.pcrs)?;
        let signature = self.aik.sign(&quote)?;

        Ok(Attestation {
            quote,
            signature,
            pcrs: self.pcrs,
        })
    }
}
```

## WASM Sandbox

### WASM Runtime Security

```rust
// WASM sandbox with capability restrictions
struct WasmSandbox {
    runtime: Wasmtime,
    capabilities: CapabilitySet,
}

impl WasmSandbox {
    fn new(capabilities: CapabilitySet) -> Self {
        let mut config = WasmtimeConfig::new();

        // Restrict system calls
        config.wasm_simd(false);
        config.wasm_bulk_memory(true);

        // Set memory limits
        config.max_memory_pages(1024); // 64MB max

        // Set time limits
        config.max_execution_time(Duration::from_secs(10));

        WasmSandbox {
            runtime: Wasmtime::new(&config),
            capabilities,
        }
    }

    fn execute(&self, module: &[u8]) -> Result<WasmOutput> {
        let module = self.runtime.compile(module)?;
        let instance = self.runtime.instantiate(&module)?;

        // Execute with capability checks
        self.runtime.call_with_checks(instance, &self.capabilities)
    }
}
```

### Capability-Based WASM

```rust
// WASM host functions with capability checks
struct WasmHost {
    capabilities: CapabilitySet,
}

impl WasmHost {
    fn file_open(&self, path: &str) -> Result<i32> {
        if !self.capabilities.check(PERM_FILE_READ) {
            return Err(Error::PermissionDenied);
        }

        // Open file
        Ok(0)
    }

    fn network_connect(&self, addr: &str) -> Result<i32> {
        if !self.capabilities.check(PERM_NETWORK) {
            return Err(Error::PermissionDenied);
        }

        // Connect to network
        Ok(0)
    }
}
```

## Supply Chain Security

### Reproducible Builds

```rust
// Reproducible build system
struct ReproducibleBuilder {
    toolchain: FixedToolchain,
    source_hash: [u8; 32],
    build_env: BuildEnvironment,
}

impl ReproducibleBuilder {
    fn build(&self) -> Result<BuildArtifact> {
        // Use fixed toolchain
        let toolchain_hash = self.toolchain.hash();

        // Normalize build environment
        let normalized_env = self.build_env.normalize();

        // Build with deterministic flags
        let artifact = self.run_build(normalized_env)?;

        // Verify reproducibility
        let artifact_hash = sha3_256(&artifact);
        if artifact_hash != self.source_hash {
            return Err(Error::BuildNotReproducible);
        }

        Ok(artifact)
    }
}
```

### Artifact Signing

```rust
// Artifact signing with post-quantum signatures
struct ArtifactSigner {
    signing_key: MlDsa65Key,
}

impl ArtifactSigner {
    fn sign(&self, artifact: &[u8]) -> Signature {
        let hash = sha3_256(artifact);
        self.signing_key.sign(&hash)
    }

    fn verify(&self, artifact: &[u8], signature: &Signature) -> bool {
        let hash = sha3_256(artifact);
        self.signing_key.verify(&hash, signature)
    }
}
```

### Build Provenance

```rust
// Build provenance metadata
struct BuildProvenance {
    source_commit: String,
    toolchain_version: String,
    build_timestamp: i64,
    builder_identity: String,
    environment_hash: [u8; 32],
}

impl BuildProvenance {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn verify(&self, artifact: &[u8]) -> Result<bool> {
        // Verify that artifact was built with claimed provenance
        let expected_hash = self.reconstruct_hash()?;
        let actual_hash = sha3_256(artifact);

        Ok(expected_hash == actual_hash)
    }
}
```

## Kernel Security

### Memory Safety

```rust
// Memory-safe kernel operations
use std::sync::Arc;

struct SafeKernel {
    memory: Arc<RwLock<MemoryManager>>,
}

impl SafeKernel {
    fn allocate(&self, size: usize) -> Result<*mut u8> {
        let mut mem = self.memory.write();

        // Bounds checking
        if size > MAX_ALLOCATION {
            return Err(Error::AllocationTooLarge);
        }

        // Safe allocation with bounds checking
        mem.allocate(size)
    }

    fn deallocate(&self, ptr: *mut u8) -> Result<()> {
        let mut mem = self.memory.write();

        // Verify pointer is valid
        if !mem.is_valid(ptr) {
            return Err(Error::InvalidPointer);
        }

        mem.deallocate(ptr)
    }
}
```

### System Call Security

```rust
// Secure system call dispatch
struct SecureSyscallDispatcher {
    capability_checker: CapabilityChecker,
}

impl SecureSyscallDispatcher {
    fn dispatch(&self, syscall: Syscall, process: &Process) -> Result<SyscallResult> {
        // Check capabilities before dispatch
        if !self.capability_checker.check(process, &syscall) {
            return Err(Error::PermissionDenied);
        }

        // Validate arguments
        syscall.validate_arguments()?;

        // Dispatch with safety checks
        match syscall {
            Syscall::FileOpen { path, flags } => {
                self.secure_file_open(path, flags, process)
            }
            Syscall::NetworkConnect { addr } => {
                self.secure_network_connect(addr, process)
            }
            _ => Err(Error::NotImplemented),
        }
    }
}
```

### Kernel Hardening

```rust
// Kernel hardening features
struct KernelHardening {
    stack_protector: bool,
    aslr: bool,
    kasan: bool,
    kptr_restrict: bool,
}

impl KernelHardening {
    fn enable_all(&mut self) {
        self.stack_protector = true;
        self.aslr = true;
        self.kasan = true;
        self.kptr_restrict = true;
    }

    fn apply(&self) {
        if self.stack_protector {
            enable_stack_protector();
        }

        if self.aslr {
            enable_aslr();
        }

        if self.kasan {
            enable_kasan();
        }

        if self.kptr_restrict {
            enable_kptr_restrict();
        }
    }
}
```

## Network Security

### TLS 1.3 with Post-Quantum

```rust
// Post-quantum TLS
struct PqTls {
    classical_kex: RsaOaep,
    pq_kex: MlKem768,
    cipher: Aes256Gcm,
}

impl PqTls {
    fn handshake(&self) -> Result<TlsSession> {
        // Hybrid key exchange
        let classical_secret = self.classical_kex.exchange()?;
        let pq_secret = self.pq_kex.exchange()?;

        // Combine secrets
        let combined_secret = combine_secrets(&[classical_secret, pq_secret]);

        // Derive session keys
        let session_keys = hkdf_sha3_256(&combined_secret)?;

        Ok(TlsSession {
            keys: session_keys,
            cipher: self.cipher.clone(),
        })
    }
}
```

### DNS-over-HTTPS

```rust
// DNS-over-HTTPS resolver
struct DoHResolver {
    https_client: HttpsClient,
    dns_cache: LruCache<DnsQuery, DnsResponse>,
}

impl DoHResolver {
    async fn resolve(&self, domain: &str) -> Result<Vec<IpAddr>> {
        // Check cache
        if let Some(cached) = self.dns_cache.get(&domain.to_string()) {
            return Ok(cached.addresses);
        }

        // Query via DoH
        let response = self.https_client
            .get(&format!("https://dns.google/resolve?name={}&type=A", domain))
            .await?;

        // Parse response
        let dns_response = parse_dns_response(&response)?;

        // Cache result
        self.dns_cache.put(domain.to_string(), dns_response.clone());

        Ok(dns_response.addresses)
    }
}
```

## Audit and Logging

### Security Event Logging

```rust
// Security event logging
struct SecurityLogger {
    log_file: SecureLogFile,
    tamper_evident: bool,
}

impl SecurityLogger {
    fn log_event(&mut self, event: SecurityEvent) -> Result<()> {
        let timestamp = SystemTime::now();
        let event_record = EventRecord {
            timestamp,
            event,
            signature: None,
        };

        // Sign event for tamper evidence
        if self.tamper_evident {
            let signature = sign_event(&event_record)?;
            event_record.signature = Some(signature);
        }

        self.log_file.append(event_record)
    }

    fn verify_log(&self) -> Result<bool> {
        if !self.tamper_evident {
            return Ok(true);
        }

        self.log_file.verify_signatures()
    }
}
```

### Audit Trail

```rust
// Comprehensive audit trail
struct AuditTrail {
    events: Vec<AuditEvent>,
    index: HashMap<ProcessId, Vec<usize>>,
}

impl AuditTrail {
    fn record(&mut self, event: AuditEvent) {
        let idx = self.events.len();
        self.events.push(event);

        self.index
            .entry(event.process_id)
            .or_insert_with(Vec::new)
            .push(idx);
    }

    fn query(&self, process_id: ProcessId) -> Vec<&AuditEvent> {
        self.index
            .get(&process_id)
            .map(|indices| {
                indices.iter()
                    .map(|&i| &self.events[i])
                    .collect()
            })
            .unwrap_or_default()
    }
}
```

## Security Monitoring

### Intrusion Detection

```rust
// Anomaly-based intrusion detection
struct IntrusionDetector {
    baseline_model: BaselineModel,
    alert_threshold: f64,
}

impl IntrusionDetector {
    fn detect(&self, metrics: SystemMetrics) -> Vec<Alert> {
        let anomaly_score = self.baseline_model.anomaly_score(metrics);

        if anomaly_score > self.alert_threshold {
            vec![Alert {
                severity: Severity::High,
                description: "Anomalous system behavior detected".to_string(),
                score: anomaly_score,
            }]
        } else {
            vec![]
        }
    }
}
```

### Real-Time Protection

```rust
// Real-time protection system
struct RealTimeProtection {
    signature_db: SignatureDatabase,
    heuristics: HeuristicEngine,
}

impl RealTimeProtection {
    fn scan(&self, file: &[u8]) -> ScanResult {
        // Signature-based detection
        if let Some(signature) = self.signature_db.match_signature(file) {
            return ScanResult::ThreatDetected(signature);
        }

        // Heuristic analysis
        if let Some(threat) = self.heuristics.analyze(file) {
            return ScanResult::Suspicious(threat);
        }

        ScanResult::Clean
    }
}
```

## Compliance and Certification

### Security Standards

SigmaOS aims to comply with:

- **FIPS 140-3**: Cryptographic module validation

- **Common Criteria (EAL4+)**: Security certification

- **NIST SP 800-53**: Security controls

- **ISO 27001**: Information security management

### Certification Roadmap

| Milestone | Target Date | Certification |
| ----------- | ------------- | ---------------- |
| FIPS 140-3 | Q4 2027 | Cryptographic module |
| Common Criteria EAL4 | Q2 2028 | Security target |
| FedRAMP | Q4 2028 | Cloud deployment |
| ISO 27001 | Q1 2029 | ISMS |

---

**Last Updated**: 2026-07-05
**Maintained by**: SigmaOS Security Team


---
## Merged from Security-Architecture.md
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
