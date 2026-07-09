# SigmaOS Security Model Deep Dive

## 1. sigma_pledge: Syscall Allowlist

`sigma_pledge` is inspired by OpenBSD's `pledge(2)` syscall. It restricts the set of IPC message types and kernel operations a process may perform. Once set, the pledge can only shrink — it is **irreversible**.

### Pledge Groups

| Group | Allowed operations |
|---|---|
| `stdio` | sigma-bus send/recv, read/write to open FDs |
| `rpath` | sigma_open with O_RDONLY, stat |
| `wpath` | sigma_open with O_WRONLY, O_CREAT |
| `cpath` | mkdir, symlink, unlink |
| `inet` | sigma_connect, sigma_bind (TCP/UDP only) |
| `proc` | fork, exec |
| `tty` | ioctl on TTY devices |
| `audio` | sigma-bus send to sigma-audiod |
| `video` | sigma-bus send to Zenith compositor |

### Implementation

```rust
// kernel/src/pledge.rs

pub fn sigma_pledge(process: &mut Process, promises: &str) -> Result<(), SyscallError> {
    let new_pledge = PledgeSet::from_str(promises)?;
    // Pledge can only shrink
    if !process.pledge.is_superset_of(&new_pledge) {
        return Err(SyscallError::EPERM);
    }
    process.pledge = new_pledge;
    Ok(())
}

// On every sigma-bus send:
pub fn check_pledge_for_message(process: &Process, msg: &BusMessage) -> bool {
    process.pledge.allows_message(&msg.payload)
}
```

---

## 2. sigma_unveil: Filesystem Path Restriction

`sigma_unveil` restricts the SigmaFS paths accessible to a process. Calls are **additive and irreversible** — once a path is added, it cannot be removed, but no new paths can be added after `sigma_pledge` narrows the pledge.

```rust
// kernel/src/unveil.rs

pub fn sigma_unveil(process: &mut Process, path: &str, perms: &str) -> Result<(), SyscallError> {
    if process.unveil_locked {
        return Err(SyscallError::EPERM);
    }
    let entry = UnveilEntry {
        path: PathBuf::from(path),
        read:    perms.contains('r'),
        write:   perms.contains('w'),
        create:  perms.contains('c'),
        execute: perms.contains('x'),
    };
    process.unveil_map.push(entry);
    Ok(())
}

// On every VFS operation:
pub fn vfs_access_check(process: &Process, path: &Path, op: VfsOp) -> bool {
    for entry in &process.unveil_map {
        if path.starts_with(&entry.path) {
            return entry.permits(op);
        }
    }
    false // deny by default
}
```

---

## 3. AVC O(1) Cache: MAC Decision Caching

The **Access Vector Cache** (AVC) provides O(1) lookup for Mandatory Access Control decisions. It caches (source_type, target_type, object_class) → allow/deny decisions from the policy.

### Cache Structure

```rust
// kernel/src/avc.rs

const AVC_CACHE_SIZE: usize = 65536; // per-CPU

struct AvcEntry {
    source_type:  u32,
    target_type:  u32,
    object_class: u16,
    decision:     AvcDecision, // Allow | Deny | AuditAllow
    valid:        bool,
}

struct AvcCache {
    entries: [AvcEntry; AVC_CACHE_SIZE],
}

impl AvcCache {
    #[inline(always)]
    pub fn lookup(&self, src: u32, tgt: u32, cls: u16) -> Option<AvcDecision> {
        let idx = avc_hash(src, tgt, cls) % AVC_CACHE_SIZE;
        let e = &self.entries[idx];
        if e.valid && e.source_type == src && e.target_type == tgt && e.object_class == cls {
            Some(e.decision)
        } else {
            None // cache miss → fall through to policy engine
        }
    }
}

fn avc_hash(src: u32, tgt: u32, cls: u16) -> usize {
    let h = (src as u64).wrapping_mul(2654435761)
        ^ (tgt as u64).wrapping_mul(40503)
        ^ (cls as u64).wrapping_mul(57);
    h as usize
}
```

Cache hit rate target: > 99.9% after warm-up (< 100ms into workload).

---

## 4. PQC Chain: Kyber-1024 → Dilithium-5 → TPM2 PCR Sealing

```
Key Establishment (per sigma-bus session):
  Initiator generates:  Kyber-1024 keypair (pk_i, sk_i)
  Responder generates:  Kyber-1024 keypair (pk_r, sk_r)
  Initiator sends pk_i:
    Responder: ct = Kyber1024.Encapsulate(pk_i) → (ct, ss_r)
    Initiator: ss_i = Kyber1024.Decapsulate(sk_i, ct)
    ss_i == ss_r  →  shared secret for AES-256-GCM channel

Signatures (CI artifacts, SVID certificates, policy files):
  sigma-ca signs with Dilithium-5 (NIST PQC Level 5, 4595-byte public key)
  Every shard SVID cert: Dilithium-5 signed, 30-day TTL
  Policy file /etc/sigma/policy.sig: Dilithium-5 signed at build time

TPM2 PCR Sealing (FDE key):
  FDE key sealed to PCRs 0 (firmware) + 4 (bootloader) + 7 (Secure Boot) + 8 (kernel)
  Unsealed by sigma-boot.efi before kernel jump
  If PCRs change (kernel tampered): unseal fails → FDE key not available → encrypted disk inaccessible
```

---

## 5. Zero-Trust: SPIFFE SVID Per Shard

Every shard receives a [SPIFFE](https://spiffe.io/) Verifiable Identity Document (SVID) at registration:

```
SVID URI: spiffe://sigmaos.local/shard/<shard-name>/<instance-id>
```

SVIDs are X.509 certificates signed by `sigma-ca` using Dilithium-5. They are rotated every 30 days. Every sigma-bus cross-shard message includes the sender's SVID; the receiver verifies it before processing.

---

## 6. Attack Surface Reduction vs Linux

| Attack Vector | Linux | SigmaOS |
|---|---|---|
| Kernel syscall interface | ~400 syscalls exposed to all processes | sigma_pledge: process-specific allowlist |
| Filesystem access | DAC (rwx bits) | sigma_unveil: explicit per-path allowlist |
| IPC | Pipes, sockets, signals, shared mem — unrestricted | sigma-bus typed messages + capability token check |
| Kernel modules | Loadable, unsigned by default | No loadable modules — shards in Ring 3 |
| Network | Any process can open raw sockets | `inet` pledge required; raw sockets: `rawnet` pledge only |
| TLS | OpenSSL (C, ~4MB, legacy ciphers) | rustls (Rust, ~400KB, TLS 1.3 only) |
| PQC | None by default | Kyber-1024 + Dilithium-5 in all IPC |
