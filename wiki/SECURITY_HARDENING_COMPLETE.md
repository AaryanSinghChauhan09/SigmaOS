# SigmaOS Security Hardening — Complete Guide

> SigmaOS is built with **security as a first-class architectural property**, not an
> afterthought. This document describes every security layer implemented in SigmaOS,
> with code examples and cross-references to analogous mechanisms in Linux, OpenBSD,
> FreeBSD, and Qubes OS.

---

## Table of Contents

1. [Security Philosophy](#security-philosophy)
2. [Capability-Based Security](#capability-based-security)
3. [pledge / unveil — Syscall & Path Restriction](#pledge--unveil)
4. [Mandatory Access Control (MAC)](#mandatory-access-control)
5. [Memory Safety](#memory-safety)
6. [Cryptography](#cryptography)
7. [Network Security](#network-security)
8. [Qubes-Style Isolation](#qubes-style-isolation)
9. [Audit Logging](#audit-logging)
10. [Exploit Mitigations](#exploit-mitigations)
11. [Supply Chain Security](#supply-chain-security)
12. [Security Checklist](#security-checklist)

---

## Security Philosophy

SigmaOS follows the **principle of least privilege** at every layer:

- **No ambient authority** — processes have no implicit permissions
- **Explicit grants** — capabilities, pledge promises, and unveil paths must be
  explicitly granted before use
- **Fail secure** — security violations cause immediate process termination
  (not graceful errors that can be exploited)
- **Defense in depth** — multiple independent security layers; compromise of one
  does not break the others
- **Zero-trust kernel** — even privileged userland processes cannot bypass MAC policies

---

## Capability-Based Security

**Source**: `src/security/capability.rs`
**Inspired by**: FreeBSD Capsicum, seL4, CHERI

### Concept

Traditional Unix uses UID 0 (root) for privilege — if you have root, you have
everything. SigmaOS replaces this with **capability tokens**: unforgeable handles
that grant specific, limited authority.

### Capability Types

```rust
pub enum Capability {
    // Filesystem
    ReadFile(PathHandle),       // Read specific file/directory
    WriteFile(PathHandle),      // Write specific file/directory
    CreateFile(PathHandle),     // Create files in directory
    DeleteFile(PathHandle),     // Delete specific file

    // Network
    TcpConnect(SocketAddr),     // Connect to specific address
    TcpListen(u16),             // Listen on specific port
    UdpSend(SocketAddr),        // Send UDP to specific address

    // Process
    SpawnProcess(BinaryPath),   // Execute specific binary
    SendSignal(Pid, Signal),    // Signal specific process

    // Kernel
    LoadDriver(DriverId),       // Load specific kernel driver
    ModifyRoute,                // Modify network routing table
    SetClock,                   // Modify system clock
    Reboot,                     // Reboot/shutdown system
}
```

### Usage

```rust
use crate::security::capability::{CapabilitySet, Capability};

// Grant the web server only what it needs
let caps = CapabilitySet::new()
    .add(Capability::TcpListen(443))        // HTTPS port
    .add(Capability::TcpListen(80))         // HTTP port
    .add(Capability::ReadFile("/var/www"))   // Serve files from here
    .add(Capability::ReadFile("/etc/tls"))  // TLS certificates
    .build();

// Apply to process (cannot be escalated afterwards)
sigma_capset(pid, &caps);
```

### Entering Capability Mode

```rust
// After setup, enter capability mode — no new capabilities can be acquired
capability::enter_cap_mode();

// Now: only operations explicitly granted above are possible
// Any attempt to open a new path -> ECAP error
```

---

## pledge / unveil

### pledge — Syscall Promise Restriction

**Source**: `src/pledge.rs`
**Inspired by**: OpenBSD `pledge(2)`

Once a process calls `pledge`, it **permanently** restricts itself to the listed
syscall groups. Any attempt to use a syscall outside the promise list results in
**immediate SIGKILL** — not an error the program can recover from.

```rust
// In a web server after binding ports and loading config:
sigma_pledge("stdio rpath inet dns", "stdio");
// Now: only read-only filesystem + network + DNS allowed
// Cannot open files for writing, fork, exec, etc.
```

**Promise groups**:

| Promise | Syscalls Allowed |
|---------|-----------------|
| `stdio` | read, write, close, fstat, ioctl (tty only) |
| `rpath` | open(O_RDONLY), stat, getdents, readlink |
| `wpath` | open(O_WRONLY), truncate, ftruncate |
| `cpath` | open(O_CREAT), mkdir, rename |
| `dpath` | unlink, rmdir, symlink |
| `inet` | socket(AF_INET/6), connect, bind, accept, send, recv |
| `unix` | socket(AF_UNIX), connect, bind |
| `dns` | getaddrinfo equivalent (restricted socket use) |
| `proc` | fork, kill, getpid, getpgid |
| `exec` | execve (with inherited execpromises) |
| `id` | setuid, setgid, setgroups |
| `pf` | Packet filter (firewall) modifications |
| `crypto` | Access to cryptographic hardware |
| `syslog` | Write to /dev/log |

### Implementation

```rust
// src/pledge.rs
pub fn sys_pledge(task: &mut Task, promises_str: &str, exec_promises: &str) -> i64 {
    let new_mask = PledgeMask::parse(promises_str);
    let exec_mask = PledgeMask::parse(exec_promises);

    // Pledge can only restrict — cannot expand existing restrictions
    if task.pledge_mask.is_some() {
        let current = task.pledge_mask.unwrap();
        if !new_mask.is_subset_of(current) {
            return -EPERM; // Cannot regain dropped promises
        }
    }

    task.pledge_mask = Some(new_mask);
    task.exec_pledge_mask = Some(exec_mask);
    0
}

// Called by syscall dispatcher before every syscall
pub fn check_pledge(task: &Task, syscall_nr: u64) -> Result<(), SigmaError> {
    if let Some(mask) = task.pledge_mask {
        if !mask.allows(syscall_nr) {
            // Pledge violation: kill the process immediately
            task.deliver_signal(Signal::SIGKILL);
            return Err(SigmaError::PledgeViolation);
        }
    }
    Ok(())
}
```

---

### unveil — Filesystem Visibility Restriction

**Source**: `src/security/mac.rs`
**Inspired by**: OpenBSD `unveil(2)`

`unveil` makes the filesystem appear to have only the paths you explicitly reveal.
Paths not unveiled are invisible — attempts to access them return `ENOENT`.

```rust
// Database server example:
sigma_unveil("/var/db/data",     "rwc"); // Read/write/create database files
sigma_unveil("/etc/sigma/db.conf", "r"); // Read config
sigma_unveil("/tmp",             "rwc"); // Temp files
sigma_unveil(NULL, NULL);               // Lock: no more paths can be added

// Now: /etc/passwd, /home/*, /bin/sh etc. are all invisible
```

---

## Mandatory Access Control

**Source**: `src/security/mac.rs`
**Inspired by**: SELinux (NSA), AppArmor (Immunix/Canonical)

### Security Contexts

Every process, file, and socket has a **security context** — a label that determines
what it can interact with.

```
user_u:role_r:type_t:s0  (SELinux format)
```

SigmaOS uses simplified context triples: `domain:type:level`

```
sigma_kernel:kernel_t:s0
web_server:httpd_t:s0
user_browser:browser_t:s0
database:db_t:s0-s3
```

### MAC Policy Rules

```rust
// MAC policy (loaded from /etc/sigma/mac.policy)
// Format: allow <domain> <type> <operation>;

allow httpd_t www_content_t { read getattr };
allow httpd_t httpd_port_t  { tcp_socket name_bind };
deny  httpd_t shadow_t      { read };

// Transitioning context on exec
type_transition shell_t     httpd_exec_t process httpd_t;
```

### Implementation

```rust
// src/security/mac.rs
pub fn mac_check(
    subject: &SecurityContext,
    object: &SecurityContext,
    op: MACOperation,
) -> MACDecision {
    let rule = POLICY.lookup(subject.domain, object.type_, op);
    match rule {
        PolicyRule::Allow => MACDecision::Allow,
        PolicyRule::Deny  => {
            audit_log_mac_denial(subject, object, op);
            MACDecision::Deny
        }
        PolicyRule::Audit => {
            audit_log_mac_access(subject, object, op);
            MACDecision::Allow
        }
    }
}
```

---

## Memory Safety

### Rust's Static Guarantees

SigmaOS leverages Rust's ownership system for compile-time memory safety:

- **No null pointer dereferences** — `Option<T>` instead of nullable pointers
- **No buffer overflows** — bounds-checked slice indexing
- **No use-after-free** — borrow checker prevents dangling references
- **No data races** — `Send`/`Sync` trait constraints on shared state

### unsafe Code Policy

All `unsafe` blocks in SigmaOS **must** include a `// SAFETY:` comment:

```rust
// GOOD — with SAFETY justification
pub fn read_register(addr: usize) -> u32 {
    // SAFETY: addr is a valid MMIO register address verified at driver init.
    // The hardware guarantees 32-bit aligned read returns defined behavior.
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

// BAD — missing SAFETY comment (rejected in code review)
pub fn bad_function(addr: usize) -> u32 {
    unsafe { *(addr as *const u32) } // No SAFETY comment — rejected
}
```

### unwrap() Policy

`unwrap()` and `expect()` are **prohibited** in production code paths:

```rust
// BAD
let val = some_option.unwrap(); // Panics in production

// GOOD — explicit error handling
let val = match some_option {
    Some(v) => v,
    None => return Err(SigmaError::NotFound),
};

// ALSO GOOD — with meaningful message
let val = some_option.expect("SAFETY: initialized in kernel_init before this call");
```

### W^X Enforcement

Pages cannot be both Writable and Executable simultaneously:

```rust
// src/memory/paging.rs
pub fn map_page(vaddr: VirtAddr, paddr: PhysAddr, flags: PageFlags) {
    // Enforce W^X
    if flags.contains(PageFlags::WRITE) && flags.contains(PageFlags::EXECUTE) {
        panic!("W^X violation: page cannot be both writable and executable");
    }
    // ... map the page
}
```

---

## Cryptography

**Source**: `src/crypto/vectorized_pqc.rs`
**Algorithms**: Post-quantum algorithms from NIST PQC competition (finalized 2024)

| Algorithm | Type | Security Level | Source File |
|-----------|------|---------------|-------------|
| Kyber-1024 (ML-KEM) | Key Encapsulation | 256-bit post-quantum | `src/crypto/vectorized_pqc.rs` |
| Dilithium-5 (ML-DSA) | Digital Signature | 256-bit post-quantum | `src/crypto/vectorized_pqc.rs` |
| SPHINCS+-SHA256-256s | Signature | Stateless hash-based | `src/security/pki.rs` |
| ChaCha20-Poly1305 | Symmetric AEAD | 256-bit | `src/net/tls.rs` |
| AES-256-GCM | Symmetric AEAD | 256-bit | `src/crypto/` |
| Blake3 | Hash | 256-bit | `src/klib/hash.rs` |
| X25519 | ECDH (classical) | 128-bit | `src/security/pki.rs` |

```rust
use crate::crypto::vectorized_pqc::{Kyber1024, Dilithium5};

// Key generation
let (pk, sk) = Kyber1024::generate_keypair(&mut rng);

// Encapsulation (sender)
let (ciphertext, shared_secret) = Kyber1024::encapsulate(&pk, &mut rng);

// Decapsulation (receiver)
let shared_secret = Kyber1024::decapsulate(&sk, &ciphertext);

// Sign a message
let keypair = Dilithium5::generate_keypair(&mut rng);
let signature = Dilithium5::sign(&keypair.secret, b"message");
assert!(Dilithium5::verify(&keypair.public, b"message", &signature));
```

---

## Network Security

### PF Firewall

**Source**: `src/net/firewall.rs`
**Inspired by**: OpenBSD PF (Packet Filter)

```
# /etc/sigma/pf.conf — SigmaOS firewall rules

# Default deny all
block all

# Allow established connections
pass in  all keep state
pass out all keep state

# Allow HTTPS
pass in on em0 proto tcp to port 443

# Block all to port 22 except trusted subnet
pass in on em0 proto tcp from 192.168.1.0/24 to port 22
block in on em0 proto tcp to port 22

# Rate limit ICMP
pass in inet proto icmp max-pkt-rate 10/1
```

### TLS 1.3 Integration

**Source**: `src/net/tls.rs`

All network services in SigmaOS use TLS 1.3 minimum:

```rust
let tls_config = TlsConfig::new()
    .min_version(TlsVersion::TLS13)
    .cert_file("/etc/tls/server.crt")
    .key_file("/etc/tls/server.key")
    .cipher_suites(&[
        CipherSuite::TLS_CHACHA20_POLY1305_SHA256,
        CipherSuite::TLS_AES_256_GCM_SHA384,
    ])
    .build();
```

---

## Qubes-Style Isolation

**Source**: `src/security/qubes_isolation.rs`
**Inspired by**: Qubes OS (Invisible Things Lab)

Qubes OS runs each application in a separate VM. SigmaOS implements this model
using lightweight isolation cells — process groups with isolated namespaces,
capability sets, and memory.

```rust
use crate::security::qubes_isolation::IsolationCell;

// Create an isolation cell for the browser
let browser_cell = IsolationCell::builder()
    .name("browser")
    .memory_limit(512 * 1024 * 1024) // 512 MB
    .cpu_limit(2)                     // 2 vCPUs
    .network_policy(NetworkPolicy::External) // Can access internet
    .filesystem_policy(FilesystemPolicy::Template("/var/cells/browser"))
    .build();

// Launch browser in isolated cell
browser_cell.exec("/usr/bin/sigma-browser", &[]);

// Data flows between cells through verified channels
let channel = IsolationChannel::new(browser_cell.id(), documents_cell.id());
channel.send(b"file contents"); // Validated, no arbitrary code execution
```

---

## Audit Logging

**Source**: `src/security/audit.rs`

All security-relevant events are logged to the kernel audit trail:

```rust
// Events automatically audited:
// - pledge violations
// - unveil violations
// - capability violations
// - MAC policy denials
// - Failed authentication
// - Privilege escalation attempts
// - File access to sensitive paths (/etc/shadow, /etc/tls/*)
// - Network connections to/from outside policy
```

**Audit log format**:

```json
{
  "timestamp": "2026-08-11T19:00:00Z",
  "event": "PLEDGE_VIOLATION",
  "pid": 1234,
  "uid": 1000,
  "syscall": "open",
  "syscall_nr": 1,
  "path": "/etc/shadow",
  "promise_required": "rpath",
  "promise_active": "stdio",
  "action": "KILLED"
}
```

---

## Exploit Mitigations

| Mitigation | Status | Implementation |
|-----------|--------|---------------|
| Stack canaries | ✅ | Compiler (`-Z stack-protector-all`) |
| ASLR | ✅ | `src/memory/paging.rs` |
| KASLR | ✅ | `src/kernel/main.rs` |
| W^X | ✅ | `src/memory/paging.rs` |
| SMEP (Supervisor Mode Exec Protection) | ✅ | `src/boot/uefi.rs` |
| SMAP (Supervisor Mode Access Protection) | ✅ | `src/boot/uefi.rs` |
| CET (Control-flow Enforcement) | 🔄 | Compiler flag |
| SafeStack | 🔄 | Compiler flag |
| CFI (Control Flow Integrity) | 🔄 | LLVM `-fsanitize=cfi` |
| Spectre/Meltdown mitigations | ✅ | LFENCE, IBRS, STIBP, IBPB |
| Retpoline (indirect branch) | ✅ | Compiler retpoline mode |

---

## Supply Chain Security

SigmaOS takes supply chain security seriously:

1. **Reproducible builds** — identical source → identical binary (Nix-inspired)
2. **Content-addressed packages** — packages identified by hash, not name/version
3. **Signed packages** — Dilithium-5 signatures on all packages
4. **Minimal dependencies** — `uuid` and `rand` crates are the ONLY external deps
   in Cargo.toml, and roadmap is to remove both by using `klib::uuid` and `klib::rng`
5. **Dependency audit** — `cargo audit` runs in CI on every push
6. **Vendor directory** — all sources vendored; no network fetches at build time

---

## Security Checklist

For every new module or PR, verify:

- [ ] No `use std::` imports (use `klib` equivalents)
- [ ] All `unsafe` blocks have `// SAFETY:` comments
- [ ] No `unwrap()` without `// SAFETY:` justification
- [ ] Security-sensitive operations checked against pledge/unveil/caps
- [ ] New syscalls added to pledge permission table
- [ ] Network code uses TLS 1.3
- [ ] Cryptographic operations use post-quantum algorithms
- [ ] Audit log entries for security events
- [ ] Tests for security properties (not just happy path)
- [ ] No hardcoded secrets, passwords, or keys
- [ ] Error messages don't leak sensitive information
- [ ] Integer overflow checked (use `checked_add`, `saturating_add`)
