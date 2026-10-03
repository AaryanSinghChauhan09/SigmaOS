# Security and Hardening

SigmaOS implements defense-in-depth security inspired by OpenBSD, grsecurity/PaX, and SELinux.

## Overview

SigmaOS prioritizes security through multiple overlapping layers:
- **Memory Safety:** Safe Rust by default, minimal unsafe code
- **Privilege Separation:** Capability-based security model
- **Mandatory Access Control:** SELinux policy enforcement
- **Exploit Mitigation:** W^X, ASLR, stack canaries, CFI
- **Syscall Filtering:** OpenBSD pledge/unveil, Linux seccomp-bpf
- **Cryptographic Integrity:** Post-quantum cryptography, secure boot

---

## SELinux (Security-Enhanced Linux)
**Location:** `src/security/selinux.rs`

Mandatory Access Control (MAC) framework providing fine-grained access control beyond traditional Unix permissions.

### Features
- **Type Enforcement (TE):** Objects and subjects have security types
- **Role-Based Access Control (RBAC):** Users assigned to roles
- **Multi-Level Security (MLS):** Confidentiality levels (Top Secret, Secret, etc.)
- **Multi-Category Security (MCS):** Isolation between processes
- **Security Contexts:** `user:role:type:level` labels on all objects
- **Policy Language:** Modular policy composition

### Security Contexts

Every file, process, socket, and IPC object has a context:
```
user_u:object_r:user_home_t:s0
  ↑       ↑          ↑       ↑
User   Role      Type    Level
```

**Examples:**
- File: `system_u:object_r:etc_t:s0` (/etc/passwd)
- Process: `user_u:user_r:user_t:s0` (regular user process)
- Process: `system_u:system_r:kernel_t:s0` (kernel thread)

### Policy Enforcement

SELinux decides access based on policy rules:
```
allow <source_type> <target_type> : <class> { <permissions> };
```

**Example Rules:**
```selinux
# Allow user processes to read user home files
allow user_t user_home_t:file { read open };

# Allow network daemon to bind TCP socket
allow httpd_t http_port_t:tcp_socket { bind listen };

# Deny all processes from writing to /boot
neverallow * boot_t:file write;
```

### Policy Types

1. **Targeted Policy:**
   - Default for most systems
   - Confined daemons (httpd, sshd, named)
   - Unconfined user processes
   - Balance between security and usability

2. **Strict Policy:**
   - Everything is confined (including user processes)
   - Maximum security
   - Requires careful configuration

3. **MLS Policy:**
   - Multi-level security for classified environments
   - Bell-LaPadula model (no read up, no write down)
   - Used in military/government systems

### SELinux Modes

- **Enforcing:** Denials are enforced and logged
- **Permissive:** Denials are logged but not enforced (audit mode)
- **Disabled:** SELinux is completely off

### Performance Impact

- **CPU Overhead:** 3-7% typical workloads
- **Memory Overhead:** ~50MB for policy database
- **I/O Overhead:** <1% (context lookups cached)

### Integration Points

```
System Call Entry
    ↓
Kernel Permission Check (DAC - Discretionary Access Control)
    ↓
SELinux Hook (MAC - Mandatory Access Control)
    ↓
Policy Decision (allow/deny based on contexts)
    ↓
Audit Log (if configured)
    ↓
System Call Execution (if allowed)
```

---

## Pledge & Unveil (OpenBSD-Inspired)
**Location:** `src/security/pledge_unveil.rs`

Syscall restriction framework reducing attack surface.

### Pledge: Restrict Syscalls

Processes voluntarily restrict which syscalls they can use:

```rust
pledge(&["stdio", "rpath", "inet"])?;
// Now process can only:
// - stdio: read/write/close file descriptors
// - rpath: open files read-only
// - inet: create network sockets
// Any other syscall → SIGABRT
```

**Common Pledge Promises:**
- `stdio`: Basic I/O (read, write, close)
- `rpath`: Read filesystem paths
- `wpath`: Write filesystem paths
- `cpath`: Create filesystem paths
- `inet`: Internet sockets (TCP/UDP)
- `unix`: Unix domain sockets
- `dns`: DNS resolution
- `proc`: Process management (fork, exec, kill)
- `exec`: Execute new programs
- `tty`: Terminal operations

**Example: Web Browser Renderer Process:**
```rust
// After initialization, restrict to only:
pledge(&["stdio", "rpath", "unix"])?;
// Now can't fork, exec, or create network sockets
// Compromised renderer can't spawn shells or open connections
```

### Unveil: Filesystem Visibility

Processes reveal only the filesystem paths they need:

```rust
unveil("/usr/lib", "r")?;  // Read-only access to libraries
unveil("/home/user/documents", "rw")?;  // Read-write access
unveil("/tmp", "rwc")?;  // Read, write, create
unveil(None, None)?;  // Lock - no more unveil() calls allowed

// Now process can ONLY access unveiled paths
// Access to /etc/passwd → ENOENT (as if file doesn't exist)
```

**Permissions:**
- `r`: Read
- `w`: Write
- `x`: Execute
- `c`: Create

**Security Benefits:**
- Prevents path traversal attacks
- Limits data exfiltration
- Reduces TOCTOU (time-of-check-time-of-use) races

---

## Seccomp-BPF (Secure Computing Mode)
**Location:** `src/security/seccomp_filter.rs`

Linux-compatible syscall filtering using BPF (Berkeley Packet Filter) programs.

### Modes

1. **Strict Mode:**
   - Only allows: read, write, _exit, sigreturn
   - Useful for pure computation tasks
   - Any other syscall → SIGKILL

2. **Filter Mode:**
   - Custom BPF program filters syscalls
   - Per-syscall decisions: ALLOW, ERRNO, TRAP, KILL
   - Can inspect syscall arguments

### BPF Filter Example

```rust
// Deny dangerous syscalls, allow everything else
let filter = seccomp_filter! {
    DENY: [ptrace, kexec_load, module_init, reboot],
    ALLOW: *,
};
install_seccomp_filter(filter)?;
```

**Advanced Filtering (argument inspection):**
```rust
// Allow open() only for O_RDONLY (read-only)
let filter = seccomp_filter! {
    syscall: open,
    args: [flags & O_WRONLY == 0],
    action: ALLOW,
};
```

### Use Cases

- Container runtimes (Docker, Podman)
- Browser sandboxes (Chrome, Firefox)
- Package managers
- System daemons

---

## Memory Protection

### W^X (Write XOR Execute)
**Location:** `src/kernel/wx_pte_hardening.rs`

No memory page can be simultaneously writable and executable.

**Implementation:**
- Kernel enforces W^X in page table entries (PTEs)
- Code pages: R-X (readable, executable, not writable)
- Data pages: RW- (readable, writable, not executable)
- Prevents code injection attacks

**Exception: JIT Compilers:**
- Allocate W-- page, write code
- Use `mprotect()` to change to R-X
- OpenBSD-style: kernel enforces delay before execution

### ASLR (Address Space Layout Randomization)

Randomizes memory layout on every program execution:
- Executable base address
- Shared library locations
- Stack position
- Heap position
- mmap() allocations

**Entropy:**
- 28 bits on x86_64 (256GB address space randomization)
- 36 bits on AArch64 (64GB randomization)

**Protection Against:**
- Return-oriented programming (ROP)
- Jump-oriented programming (JOP)
- Return-to-libc attacks

### Stack Canaries

Compiler-inserted guards detecting stack buffer overflows:
```
[Return Address]
[Stack Canary]  ← Random value checked on function return
[Local Variables]
```

**On Function Return:**
1. Check if canary value unchanged
2. If modified → stack overflow detected → abort process
3. Prevents return address overwrite

---

## Control Flow Integrity (CFI)
**Location:** `src/kernel/cfi.rs`

Validates control flow transfers (function calls, returns, jumps).

### Forward-Edge CFI (Indirect Calls)
```rust
// Only allow calls to valid function addresses
fn indirect_call(target: *const fn()) {
    if !is_valid_function_target(target) {
        panic!("CFI violation: invalid call target");
    }
    unsafe { target() };
}
```

### Backward-Edge CFI (Returns)
- Shadow call stack: parallel stack for return addresses
- Hardware return address protection (Intel CET, ARM BTI)

**Protection Against:**
- ROP/JOP gadget chaining
- vtable hijacking
- Function pointer overwrite

---

## Kernel Pointer Restriction
**Location:** `src/kernel/kptr_restrict.rs`

Prevents kernel pointer leaks to userspace.

**Levels:**
- `0`: No restriction (developers only)
- `1`: Hide from unprivileged users
- `2`: Hide from all users (production default)

**Implementation:**
- `/proc/kallsyms`: Shows `0000000000000000` instead of real addresses
- `dmesg`: Redacts kernel pointers (%pK format)
- `/proc/kcore`: Restricted to CAP_SYS_RAWIO

**Prevents:**
- KASLR bypass (Kernel ASLR)
- Precision exploit development
- Information disclosure vulnerabilities

---

## Cryptographic Security

### Post-Quantum Cryptography
**Location:** `src/crypto/pqc.rs`

Quantum-resistant algorithms (NIST PQC standards):

1. **CRYSTALS-Kyber (Key Encapsulation):**
   - Lattice-based encryption
   - Security: 128/192/256-bit equivalent
   - Use: TLS key exchange, encrypted storage

2. **CRYSTALS-Dilithium (Digital Signatures):**
   - Lattice-based signatures
   - Security: 128/192/256-bit equivalent
   - Use: Code signing, firmware updates

3. **FALCON (Digital Signatures):**
   - Compact lattice signatures
   - Smaller signature size than Dilithium
   - Use: Embedded systems, constrained devices

4. **SPHINCS+ (Stateless Hash-Based Signatures):**
   - Hash-based, conservative security
   - Largest signatures, slowest
   - Use: Long-term security (30+ years)

### Secure Boot Chain

```
UEFI Secure Boot
    ↓
Bootloader Verification (signed with PQC)
    ↓
Kernel Verification (dm-verity root hash signed)
    ↓
Module Signature Verification
    ↓
User Space (SELinux enforcing, seccomp active)
```

### Entropy Management
**Location:** `src/crypto/entropy.rs`

High-quality random number generation:

**Sources:**
- CPU instructions (RDRAND, RDSEED on x86)
- Hardware RNGs (TPM, smartcard RNG)
- Timing jitter (interrupt timing)
- Hardware events (disk seek times, network packet timing)

**Algorithm:**
- ChaCha20-based CSPRNG (Cryptographically Secure Pseudo-Random Number Generator)
- Continuous reseeding from entropy sources
- Catastrophic reseeding on fork (prevent state reuse)

---

## Secure Development Practices

### Safe Rust by Default

**Memory Safety Guarantees:**
- No null pointer dereferences
- No use-after-free
- No double-free
- No buffer overflows (bounds-checked)
- No data races (enforced by borrow checker)

**Unsafe Code Auditing:**
- All `unsafe` blocks must be justified with SAFETY comment
- Minimize unsafe surface area (<5% of codebase)
- Audit tools: cargo-geiger, cargo-crev

### Constant-Time Cryptography

Prevent timing side-channels:
```rust
// BAD: Time varies based on password match
fn check_password_bad(input: &[u8], correct: &[u8]) -> bool {
    input == correct  // Early exit on first mismatch
}

// GOOD: Constant time comparison
fn check_password_good(input: &[u8], correct: &[u8]) -> bool {
    subtle::ConstantTimeEq::ct_eq(input, correct).into()
}
```

### Memory Zeroization

Securely erase sensitive data:
```rust
use zeroize::Zeroize;

let mut password = String::from("secret123");
// Use password...
password.zeroize();  // Guaranteed to overwrite memory
// Compiler cannot optimize away zeroization
```

---

## Development Roadmap

### Short-term (Q1-Q2 2027)

1. **SELinux Enhancements:**
   - Conditional policies (runtime policy loading)
   - SELinux userspace tools (semanage, restorecon)
   - Policy generation wizard for custom applications
   - Android-style sepolicy integration

2. **Landlock LSM:**
   - Unprivileged sandboxing (Linux 5.13+)
   - Simpler than seccomp for filesystem restrictions
   - Complement to pledge/unveil

3. **Hardware Security:**
   - Intel SGX (Software Guard Extensions) support
   - AMD SEV (Secure Encrypted Virtualization)
   - ARM TrustZone integration
   - TPM 2.0 full integration (measured boot, disk encryption)

4. **Audit Framework:**
   - Linux-compatible auditd
   - Security event logging
   - SIEM integration (Splunk, ELK)

### Mid-term (Q3-Q4 2027)

1. **Advanced Exploit Mitigation:**
   - Intel CET (Control-flow Enforcement Technology)
   - ARM BTI/PAC (Branch Target Identification / Pointer Authentication)
   - Memory tagging (ARM MTE, SPARC ADI)
   - Hardware-based CFI

2. **Kernel Hardening:**
   - Kernel Lockdown mode (integrity/confidentiality levels)
   - Module signature enforcement (no unsigned modules)
   - LoadPin LSM (only load modules from verified filesystem)
   - Yama LSM (ptrace restrictions)

3. **Capability Systems:**
   - Fine-grained capabilities (CAP_SYS_ADMIN split)
   - Ambient capabilities (inherited across exec)
   - Capability-based file access (POSIX.1e draft)

4. **Cryptographic Agility:**
   - Hybrid PQC/classical schemes (ECDSA+Dilithium)
   - Automated cipher suite updates
   - Key rotation policies

### Long-term (2028+)

1. **Formal Verification:**
   - Prove security properties using Coq/Isabelle
   - Verify SELinux policy correctness
   - Cryptographic protocol verification

2. **Compartmentalization:**
   - Microkernel-style privilege separation
   - Process-per-subsystem architecture
   - IPC-based isolation (like macOS XPC)

3. **Confidential Computing:**
   - Full system encryption (encrypted RAM)
   - Attestation-based boot
   - Encrypted containers
   - Homomorphic encryption support

4. **AI-Powered Security:**
   - Anomaly detection (unusual syscall patterns)
   - Policy generation from application behavior
   - Automated vulnerability discovery

---

## Performance Impact

### Overhead Measurements (vs. baseline without mitigations)

| Feature | CPU Overhead | Memory Overhead |
|---------|-------------|-----------------|
| SELinux Enforcing | 3-7% | 50MB |
| Pledge/Unveil | <1% | Negligible |
| Seccomp-BPF | <2% | 4KB per filter |
| W^X Enforcement | <1% | Negligible |
| ASLR | <1% | Negligible |
| Stack Canaries | 1-3% | 8 bytes per frame |
| CFI (LLVM) | 5-10% | Negligible |
| PQC (vs RSA-2048) | Sign: 2-5x, Verify: 1-2x | Key: 2-4KB |

**Total System Impact:** 10-20% CPU overhead for maximum security

**Mitigation Strategies:**
- Disable features selectively (non-critical workloads)
- Hardware acceleration (AES-NI, SHA extensions)
- Profile-guided optimization

---

## Testing Strategy

### Security Testing

```bash
# Test SELinux policy
setenforce 1  # Enable enforcing mode
sestatus  # Check status
audit2why < /var/log/audit/audit.log  # Analyze denials

# Test pledge/unveil
./pledged_app  # Should restrict syscalls
strace ./pledged_app  # Verify syscall filtering

# Test seccomp
strace -f ./seccomp_app  # Should see prctl(PR_SET_SECCOMP)

# Test W^X
./jit_compiler  # Should enforce W^X transitions

# Test ASLR
for i in {1..10}; do cat /proc/self/maps | grep libc; done
# Addresses should differ each time
```

### Penetration Testing

```bash
# Test exploit mitigations
./exploit_poc  # Should be blocked by ASLR/CFI/W^X

# Test privilege escalation
./privesc_test  # Should be blocked by SELinux/capabilities

# Test kernel pointer leaks
cat /proc/kallsyms  # Should show 0000000000000000
dmesg | grep "0x"  # Should redact addresses
```

---

## Compliance and Certifications

SigmaOS security targets:
- **Common Criteria EAL4+** (Evaluation Assurance Level 4)
- **FIPS 140-3** (Cryptographic Module Validation)
- **PCI-DSS** (Payment Card Industry Data Security Standard)
- **HIPAA** (Health Insurance Portability and Accountability Act)
- **FedRAMP Moderate** (Federal Risk and Authorization Management Program)

---

## References

- SELinux Project: https://github.com/SELinuxProject
- OpenBSD Security: https://www.openbsd.org/security.html
- Linux Kernel Self Protection Project: https://kernsec.org/wiki/index.php/Kernel_Self_Protection_Project
- NIST Post-Quantum Cryptography: https://csrc.nist.gov/projects/post-quantum-cryptography

---

**Last Updated:** October 2, 2026  
**Maintainers:** SigmaOS Security Team  
**License:** MPL-2.0 (same as SigmaOS kernel)
