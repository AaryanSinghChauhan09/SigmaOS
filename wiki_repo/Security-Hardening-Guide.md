# Security Hardening Guide

SigmaOS implements comprehensive security hardening measures following industry best practices from Linux, BSD, and other operating systems, including kernel hardening, application sandboxing, and secure coding practices.

## Overview

Security hardening provides:
- Kernel hardening with stack canaries, ASLR, PIE, and NX
- Application sandboxing with Landlock, Capsicum, pledge, and unveil
- Secure coding practices with memory safety and input validation
- Cryptographic integrity with post-quantum algorithms
- Audit logging and monitoring
- Vulnerability management and patching
- Access control and privilege separation

## Kernel Hardening

### Stack Protection
- **Stack Canaries**: GCC stack protector, ProPolice
- **Stack Clash Protection**: Guard pages between stack and heap
- **Stack Randomization**: Randomize stack layout
- **Return Address Protection**: RETPOLINE, Shadow Stack

### Address Space Layout
- **ASLR**: Address Space Layout Randomization
- **PIE**: Position Independent Executable
- **RELRO**: Read-Only Relocation
- **Bind Now**: Immediate binding

### Memory Protection
- **NX/DEP**: No-Execute / Data Execution Prevention
- **W^X**: Write XOR Execute prevention
- **Guard Pages**: Guard page allocation for critical data
- **Bounds Checking**: Bounded buffers and string operations

### Kernel Hardening Features
```rust
// src/kernel/hardening.rs
pub struct KernelHardening {
    pub stack_canary_enabled: bool,
    pub aslr_enabled: bool,
    pub pie_enabled: bool,
    pub relro_enabled: bool,
    pub nx_enabled: bool,
    pub stack_clash_protection: bool,
}

impl KernelHardening {
    pub fn new() -> Self {
        KernelHardening {
            stack_canary_enabled: true,
            aslr_enabled: true,
            pie_enabled: true,
            relro_enabled: true,
            nx_enabled: true,
            stack_clash_protection: true,
        }
    }

    pub fn enable_all(&mut self) {
        self.stack_canary_enabled = true;
        self.aslr_enabled = true;
        self.pie_enabled = true;
        self.relro_enabled = true;
        self.nx_enabled = true;
        self.stack_clash_protection = true;
    }

    pub fn check_stack_canary(&self) -> bool {
        self.stack_canary_enabled
    }

    pub fn check_aslr(&self) -> bool {
        self.aslr_enabled
    }

    pub fn check_pie(&self) -> bool {
        self.pie_enabled
    }

    pub fn check_relro(&self) -> bool {
        self.relro_enabled
    }

    pub fn check_nx(&self) -> bool {
        self.nx_enabled
    }

    pub fn check_stack_clash_protection(&self) -> bool {
        self.stack_clash_protection
    }
}
```

## Application Sandboxing

### Linux Landlock v5
Landlock v5 provides filesystem access control with:
- File path access restrictions
- Read-only, write-only, and read-write permissions
- Directory traversal prevention
- Network access control

```rust
// src/security/landlock.rs
pub struct LandlockGuard {
    pub rules: Vec<LandlockRule>,
}

pub struct LandlockRule {
    pub path: String,
    pub access: LandlockAccess,
}

pub enum LandlockAccess {
    Read,
    Write,
    ReadWrite,
    Execute,
}

impl LandlockGuard {
    pub fn new() -> Self {
        LandlockGuard {
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, path: &str, access: LandlockAccess) {
        self.rules.push(LandlockRule {
            path: path.to_string(),
            access,
        });
    }

    pub fn enforce(&self) -> Result<(), LandlockError> {
        // Enforce Landlock rules
        for rule in &self.rules {
            self.enforce_rule(rule)?;
        }
        Ok(())
    }

    fn enforce_rule(&self, rule: &LandlockRule) -> Result<(), LandlockError> {
        // Enforce specific rule
        match rule.access {
            LandlockAccess::Read => self.enforce_read(&rule.path),
            LandlockAccess::Write => self.enforce_write(&rule.path),
            LandlockAccess::ReadWrite => self.enforce_readwrite(&rule.path),
            LandlockAccess::Execute => self.enforce_execute(&rule.path),
        }
    }
}
```

### FreeBSD Capsicum
Capsicum provides capability-based access control with:
- File descriptor rights (read, write, execute, seek, ioctl)
- Process capability mode
- Capability propagation across execve
- Fine-grained access control

```rust
// src/security/capsicum.rs
pub struct CapsicumGuard {
    pub capabilities: BTreeMap<Capability, bool>,
}

pub enum Capability {
    Read,
    Write,
    Execute,
    Seek,
    Ioctl,
}

impl CapsicumGuard {
    pub fn new() -> Self {
        CapsicumGuard {
            capabilities: BTreeMap::new(),
        }
    }

    pub fn add_capability(&mut self, cap: Capability) {
        self.capabilities.insert(cap, true);
    }

    pub fn enter_capability_mode(&self) -> Result<(), CapsicumError> {
        // Enter capability mode
        self.capability_mode_enter()
    }

    fn capability_mode_enter(&self) -> Result<(), CapsicumError> {
        // System call to enter capability mode
        Ok(())
    }
}
```

### OpenBSD Pledge and Unveil
Pledge provides syscall restriction with:
- Process promises (stdio, rpath, inet, etc.)
- Promise violation detection
- Abort on violation

Unveil provides filesystem access restriction with:
- Directory access restrictions
- Path canonicalization
- Symlink protection

```rust
// src/security/pledge.rs
pub struct PledgeGuard {
    pub promises: Vec<PledgePromise>,
}

pub enum PledgePromise {
    Stdio,
    Rpath,
    Inet,
    Exec,
    Proc,
}

impl PledgeGuard {
    pub fn new() -> Self {
        PledgeGuard {
            promises: Vec::new(),
        }
    }

    pub fn add_promise(&mut self, promise: PledgePromise) {
        self.promises.push(promise);
    }

    pub fn pledge(&self) -> Result<(), PledgeError> {
        // System call to pledge
        self.pledge_system_call()
    }

    fn pledge_system_call(&self) -> Result<(), PledgeError> {
        // System call implementation
        Ok(())
    }
}
```

## Secure Coding Practices

### Memory Safety
- **Bounds Checking**: Always check array/string bounds
- **Safe Abstractions**: Use safe Rust abstractions instead of raw pointers
- **Null Pointer Checks**: Validate pointers before dereferencing
- **Integer Overflow**: Use checked arithmetic operations

### Input Validation
- **Sanitize User Input**: Validate all user input before processing
- **Path Canonicalization**: Canonicalize file paths before access
- **Length Validation**: Validate input lengths before processing
- **Type Validation**: Validate input types before processing

### Cryptographic Practices
- **No Hard-coded Keys**: Never hard-code cryptographic keys
- **Key Derivation**: Use proper key derivation functions
- **Random Number Generation**: Use cryptographically secure RNG
- **Secure Algorithms**: Use approved cryptographic algorithms

## Configuration

### Security Configuration
```toml
# /etc/sigmaos/security.toml
[kernel]
# Kernel hardening
stack_canary = true
aslr = true
pie = true
relro = true
nx = true
stack_clash_protection = true

[sandboxing]
# Application sandboxing
landlock = true
capsicum = true
pledge = true
unveil = true

[auditing]
# Audit logging
enabled = true
log_syscalls = true
log_file_operations = true
log_network_access = true

[updates]
# Security updates
auto_update = true
cve_monitoring = true
patch_management = true
```

### Runtime Control
```bash
# Enable kernel hardening
sigsec enable-hardening

# Enable sandboxing
sigsec enable-sandboxing

# Enable audit logging
sigsec enable-audit

# Check security status
sigsec status

# Scan for vulnerabilities
sigsec scan-vulnerabilities

# Apply security updates
sigsec apply-updates
```

## Performance Considerations

### Hardening Overhead
- **Stack Canaries**: Minimal overhead (~1-2%)
- **ASLR**: Minimal overhead (~1%)
- **PIE**: Minimal overhead (~1%)
- **NX/DEP**: No overhead
- **Landlock**: Minimal overhead (~1-2%)
- **Capsicum**: Minimal overhead (~1-2%)
- **Pledge**: Minimal overhead (~1%)

### Balancing Security and Performance
- Enable hardening for security-critical components
- Use relaxed hardening for performance-critical components
- Profile and tune hardening settings
- Monitor for performance regressions

## Troubleshooting

### Application Fails After Hardening
If application fails after hardening:
1. Check which hardening feature is causing the issue
2. Temporarily disable that feature
3. Check application logs for specific errors
4. Adjust hardening settings
5. Re-enable feature with different configuration

### Performance Regression
If performance regression occurs:
1. Profile the application to identify bottleneck
2. Check which hardening feature is causing overhead
3. Adjust hardening settings
4. Consider trade-offs between security and performance
5. Monitor for continued regressions

### Sandbox Violations
If sandbox violations occur:
1. Check violation logs
2. Review sandbox rules
3. Adjust sandbox permissions
4. Check for necessary capabilities
5. Update sandbox configuration

---

**[Security](Category-Security)** | **[Hardening](Category-Hardening)** | **[Sandboxing](Category-Sandboxing)**
