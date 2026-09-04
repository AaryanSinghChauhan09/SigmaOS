# 📋 SigmaOS Developer Rules & Guidelines

**Version:** 1.0.0  
**Last Updated:** September 2026  
**Scope:** All developers, designers, contributors, and AI agents

---

## Table of Contents

1. [Core Principles](#core-principles)
2. [Development Workflow](#development-workflow)
3. [Code Standards](#code-standards)
4. [Security & Safety](#security--safety)
5. [Testing & Verification](#testing--verification)
6. [Documentation](#documentation)
7. [Contribution Areas](#contribution-areas)
8. [Special Interest Groups (SIGs)](#special-interest-groups-sigs)
9. [AI Agent Guidelines](#ai-agent-guidelines)
10. [Review & Approval Process](#review--approval-process)

---

## Core Principles

### 1. **Sovereignty & Zero Dependencies**
- **No External Unverified Dependencies**: Core kernel and userspace modules must use Rust's `#![no_std]` with explicit capability bounds
- **Self-Sufficiency**: Avoid external crates unless explicitly approved by maintainers
- **Minimal Surface**: Each component should have the smallest possible attack surface

### 2. **Memory Safety & Security-First**
- **Prefer Safe Rust**: Memory safety is non-negotiable
- **Unsafe Blocks**: Only use `unsafe` for hardware interaction, driver development, or low-level OS primitives
- **Document Invariants**: Every `unsafe` block must document all safety invariants
- **Post-Quantum Cryptography**: All cryptographic operations must use Kyber-1024 (KEM) or Dilithium-5 (signatures)

### 3. **Performance & Efficiency**
- **Target Metrics**:
  - Context Switch Latency: < 0.12 µs
  - Zero-Copy IPC: > 14.2 GB/s
  - Boot Time: < 180 ms
  - Memory Allocation Overhead: 0% (zero-alloc path preferred)
- **No Premature Optimization**: Profile before optimizing; measure impact post-change

### 4. **Modularity & Isolation**
- **Microkernel Architecture**: Decompose functionality into isolated shards
- **Capability-Based Security**: Every component runs under explicit capability constraints
- **IPC-First Design**: Inter-process communication is the primary execution model
- **Loose Coupling**: Minimize dependencies between modules

### 5. **Vendor Independence & Sovereignty**
- **No Foreign Control**: SigmaOS must not depend on external OS binaries, drivers, or toolchains for core functionality
- **India-First Compliance**: Native support for GST, Income Tax, UPI, and 22-language support
- **Long-Term Vision**: Design decisions must support 10+ years of maintenance without external vendor changes

---

## Development Workflow

### 1. **Setup & Environment**

```bash
# Clone repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install Rust toolchain (stable + nightly)
rustup toolchain install stable nightly
rustup default stable

# Verify build environment
cargo --version
rustc --version
```

### 2. **Branch Strategy**

**Branch Naming Convention:**

```
feat/description           — New features
fix/description            — Bug fixes
refactor/description       — Code refactoring (no functional changes)
docs/description           — Documentation updates
kernel/description         — Kernel-level changes
pkg/description            — Package manager updates
arch/description           — Architecture/infrastructure changes
shard/app-name             — New shard/module implementations
```

**Example:**

```bash
git checkout -b feat/zenith-gesture-support
# or
git checkout -b fix/scheduler-race-condition
```

### 3. **Commit Guidelines**

**Conventional Commit Format:**

```
type(scope): description

[optional body]

[optional footer]
```

**Types:** feat, fix, docs, style, refactor, test, chore, shard, security, perf

**Examples:**

```
feat(scheduler): implement EEVDF scheduling algorithm
fix(memory): resolve buddy allocator fragmentation issue
docs(kernel): add VMM architecture guide
security(pledge): strengthen capability boundary checking
perf(ipc): optimize zero-copy message passing
```

**Rules:**
- One logical change per commit
- Descriptive messages (min 20 characters)
- Reference related issues: `Closes #123` or `Relates to #456`
- No commits with temporary/debug code

### 4. **Pull Request Process**

**Before Submitting:**

1. Create Feature Branch off `main`
2. Local Testing:

```bash
cargo build
cargo test --lib
./run_sigma_tests.sh
./scripts/sigma_quality_check.sh --strict
```

3. Code Formatting:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
```

4. Update Documentation (README, rustdoc, wiki if applicable)
5. Link Issues in PR description

**PR Template (Required):**

```markdown
## Description
Brief summary of changes

## Related Issues
Closes #123
Relates to #456

## Changes Made
- Change 1
- Change 2

## Testing
- [ ] Unit tests added/updated
- [ ] Manual testing completed
- [ ] All tests passing
- [ ] Quality checks passing

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
- [ ] Security implications reviewed
```

**Approval Requirements:**

- Minimum 2 approvals from maintainers
- All CI/CD checks passing
- No unresolved conversations
- SIG lead approval for module-specific changes

---

## Code Standards

### 1. **Rust Formatting & Linting**

```bash
# Format code
cargo fmt

# Check linting
cargo clippy -- -D warnings

# Format check (pre-commit)
cargo fmt -- --check
```

**Style Rules:**
- 4-space indentation (automatic via rustfmt)
- Line length: 100 characters (soft limit, 120 hard limit)
- Naming: `snake_case` for functions/variables, `CamelCase` for types
- Imports: Group standard library, external crates, internal modules

### 2. **Documentation Standards**

**Rustdoc Comments (Required for Public APIs):**

```rust
/// Brief one-line summary.
///
/// Longer explanation if needed. Describe the purpose, behavior,
/// and any important constraints.
///
/// # Examples
///
/// ```
/// let result = my_function(input);
/// assert_eq!(result, expected);
/// ```
///
/// # Panics
///
/// Panics if `condition` is true.
///
/// # Safety
///
/// (Required for unsafe functions) Describe safety invariants.
pub fn my_function(input: T) -> Result<U, E> {
    // implementation
}
```

**Module-Level Documentation:**

```rust
//! High-level description of the module.
//!
//! Explain the module's purpose, key types, and usage patterns.
```

### 3. **Memory Management**

**Allocation Strategy:**
- Prefer stack allocation for small, fixed-size data
- Use no_std allocators (BuddyAllocator, SlabAllocator)
- Zero-copy operations where possible
- Audit all heap allocations for safety

**Banned Patterns:**
- Global mutable state (use interior mutability if necessary)
- Unbounded collections (set capacity limits)
- Recursive allocations without depth limits

### 4. **Unsafe Code**

**When Unsafe is Necessary:**
- Hardware register access
- Low-level CPU operations (context switches, interrupts)
- FFI to C code (rare, requires vetting)
- Performance-critical paths with memory layout guarantees

**Unsafe Code Rules:**

```rust
/// SAFETY: Explain why this is safe here.
/// Requirements:
/// - `ptr` must be valid and properly aligned
/// - Caller must ensure no other references exist
unsafe {
    // Code with documented invariants
}
```

**Checklist for Unsafe Blocks:**
- [ ] Document all safety invariants in `/// SAFETY:` comments
- [ ] Mark with `// SAFETY:` inline comments
- [ ] Add bounds checking where applicable
- [ ] Include tests that verify invariants
- [ ] Request additional review in PR

### 5. **Error Handling**

**Preferred Pattern:**

```rust
// Use Result<T, E> for fallible operations
pub fn operation() -> Result<T, CustomError> {
    // ...
}

// Custom error types
#[derive(Debug)]
pub enum CustomError {
    InvalidInput(String),
    NotFound,
    PermissionDenied,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::NotFound => write!(f, "Resource not found"),
            Self::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}
```

**Anti-Patterns:**
- ❌ `unwrap()` in production code (except main entry point)
- ❌ `panic!()` for non-fatal errors
- ❌ Silent error swallowing with `.ok().flatten()`
- ❌ Generic error types without context

---

## Security & Safety

### 1. **Capability-Based Access Control**

All new features must enforce capability-based access:

```rust
// Define capabilities
pub struct Capability {
    allow_network: bool,
    allow_read: PathBuf,
    allow_write: PathBuf,
    allow_execute: bool,
}

// Enforce at runtime
pub fn secure_operation(cap: &Capability, resource: &Path) -> Result<()> {
    if !cap.allow_read.starts_with(resource) {
        return Err(SecurityError::PermissionDenied);
    }
    // Proceed with operation
}
```

### 2. **Post-Quantum Cryptography**

Required for:
- Driver module signing
- Package attestation
- Security-critical configuration

```rust
use pqc_kyber::{Kyber1024, Keypair};
use pqc_dilithium::{Dilithium5, Signature};

// Generate keypair
let (public_key, secret_key) = Kyber1024::keygen();

// Sign module
let signature = Dilithium5::sign(&secret_key, module_bytes);

// Verify signature
Dilithium5::verify(&public_key, module_bytes, &signature)?;
```

### 3. **Pledge & Unveil Sandboxing**

All user-facing applications should use OpenBSD-style sandboxing:

```rust
// Pledge capabilities
pledge(&["stdio", "rpath", "inet"])?;

// Restrict filesystem access
unveil("/var/www", "r")?;
unveil(null, null)?; // Lock down filesystem
```

### 4. **Input Validation**

Mandatory for All User Input:

```rust
pub fn validate_input(input: &str) -> Result<ValidatedInput> {
    // Length checks
    if input.len() > MAX_LENGTH {
        return Err(ValidationError::TooLong);
    }

    // Character validation
    if !input.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(ValidationError::InvalidCharacters);
    }

    Ok(ValidatedInput(input.to_string()))
}
```

### 5. **Threat Model**

Consider these threat scenarios:
- Local Privilege Escalation: Verify capability boundaries
- Information Disclosure: Check for side-channel leaks
- Denial of Service: Validate resource limits
- Code Injection: Sanitize all external inputs
- Timing Attacks: Use constant-time comparisons for secrets

---

## Testing & Verification

### 1. **Unit Testing**

Mandatory for:
- All public APIs
- Core algorithms
- Security-critical code
- Error paths

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operation() {
        let input = InputData::default();
        let result = perform_operation(&input).unwrap();
        assert_eq!(result.value, expected_value);
    }

    #[test]
    #[should_panic(expected = "invalid input")]
    fn test_panic_on_invalid_input() {
        perform_operation(&invalid_input());
    }

    #[test]
    fn test_error_handling() {
        let result = perform_operation(&bad_data());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ExpectedError);
    }
}
```

### 2. **Integration Testing**

Run integration tests before PR submission:

```bash
cargo test --test '*' --lib
```

### 3. **Stress Testing**

For performance-critical components:

```bash
# Run sigma test suite
./run_sigma_tests.sh

# Run quality checks
./scripts/sigma_quality_check.sh --strict
```

### 4. **Security Testing**

- Use Clippy's security lints: `cargo clippy -- -W security`
- Run miri for undefined behavior detection (nightly)
- Fuzz critical parsing code
- Run address sanitizers on unsafe code

### 5. **Testing Checklist**

- [ ] All tests pass locally
- [ ] New functionality has unit tests (>80% coverage)
- [ ] Edge cases covered (null, empty, max size)
- [ ] Error paths tested
- [ ] Performance impact measured
- [ ] Security implications reviewed

---

## Documentation

### 1. **Rustdoc**

Required Documentation:
- All public modules, structs, enums, traits, functions
- Complex algorithms with inline explanations
- Unsafe blocks with safety invariants
- Error conditions and recovery strategies

```bash
# Generate and view docs locally
cargo doc --open
```

### 2. **Architecture Documentation**

Location: `wiki/` directory and GitHub Wiki

Required for Major Features:
- System design overview
- Component interaction diagrams
- Data flow description
- Performance characteristics
- Security considerations

### 3. **Inline Comments**

- Use `//` for single-line explanations
- Use `/* */` for multi-line comments only when necessary
- Explain "why", not "what" (code shows what)
- Reference GitHub issues: `// See #123 for context`

**Bad Comment:**

```rust
// Increment counter
counter += 1;
```

**Good Comment:**

```rust
// Increment counter to track number of context switches
// Required for scheduler fairness accounting
counter += 1;
```

### 4. **README Updates**

Update repo README if changes affect:
- Build process
- Dependencies
- Architecture
- Feature set
- Performance characteristics

### 5. **Wiki Maintenance**

Keep GitHub Wiki updated with:
- Strategic decisions and rationale
- Architecture diagrams
- Performance benchmarks
- Known limitations
- Roadmap status

---

## Contribution Areas

### 1. **Kernel Subsystems**
- Microkernel hybrid primitives
- EEVDF scheduler implementation
- eBPF tracing infrastructure
- Hardware device drivers (NVMe, e1000e, xHCI USB, Intel HDA)
- Virtual memory management (paging, MMU)

### 2. **Userland & Utilities**
- Shell (sigma-sh) enhancements
- Init system services
- Package manager adapters (ALPM, APT, RPM, Portage, Alpine, XBPS, Nix, Moss)
- System utilities and tooling

### 3. **Zenith Desktop Compositor**
- Layout engines
- GTK3/GTK4 native toolkit adapters
- Display/monitor management
- Wayland protocol support

### 4. **Shards Ecosystem**
- Sandboxed application development
- Productivity applications (office suite, email, calendar)
- Multimedia tools (video editor, audio DSP)
- Security auditing stacks

### 5. **Documentation & Community**
- Architecture guides and tutorials
- API reference documentation
- Installation manuals
- Community engagement

---

## Special Interest Groups (SIGs)

### SIG-Kernel
**Focus:** Low-level OS primitives  
**Scope:** Scheduling, virtual memory, IPC, eBPF, syscall interfaces  
**Lead:** [TBD]  
**Meeting:** Bi-weekly (virtual)

### SIG-Drivers
**Focus:** Hardware abstraction and drivers  
**Scope:** HAL design, PCIe, NVMe, NICs, USB, HDA  
**Lead:** [TBD]  
**Meeting:** Bi-weekly (virtual)

### SIG-Apps & Shards
**Focus:** Application ecosystem  
**Scope:** .sigma-app manifests, SquashFS/OverlayFS, Shards Marketplace  
**Lead:** [TBD]  
**Meeting:** Weekly (virtual)

### SIG-Security
**Focus:** Security and cryptography  
**Scope:** Sandboxing (pledge/unveil), MAC policies, PQC, binary hardening  
**Lead:** [TBD]  
**Meeting:** Bi-weekly (virtual)

**Participation:**
- Attend SIG meetings for your area of contribution
- Review and approve PRs in your domain
- Mentor junior contributors
- Drive strategic decisions for your domain

---

## AI Agent Guidelines

### 1. **Scope & Limitations**

**AI agents may:**
- ✅ Generate code following established patterns
- ✅ Refactor existing code for readability/performance
- ✅ Create unit tests and documentation
- ✅ Suggest improvements based on security analysis
- ✅ Automate boilerplate and repetitive tasks
- ✅ Analyze performance bottlenecks

**AI agents must not:**
- ❌ Modify security-critical code without human review
- ❌ Change unsafe block implementations
- ❌ Modify cryptographic code without domain expert review
- ❌ Approve PRs or merge code
- ❌ Make architectural decisions
- ❌ Override established patterns without justification

### 2. **Code Generation Rules**

For All Generated Code:
- Follow Rust style guidelines exactly
- Add comprehensive comments explaining logic
- Include rustdoc for public items
- Generate unit tests (minimum 80% coverage)
- Document any unsafe blocks with full safety arguments
- Add security review checklist for critical sections

**Generated Code Template:**

```rust
/// Generated by [AI Agent Name] on [date]
/// Human review required for: [specific areas]
pub fn generated_function(input: T) -> Result<U> {
    // Implementation
    todo!()
}
```

### 3. **Error Messages & Feedback**

When AI-generated code encounters issues:
- Provide detailed error explanations
- Suggest specific fixes with code examples
- Reference relevant documentation or patterns
- Highlight security concerns prominently
- Request human review for uncertain changes

### 4. **Documentation Generation**

AI agents should generate:
- Clear, concise rustdoc comments
- Architecture diagrams (using Mermaid syntax)
- Performance notes and complexity analysis
- Usage examples
- Error condition documentation

### 5. **Security Review Process**

For AI-generated security/cryptographic code:
- **Mandatory Human Review** by domain expert
- **Code Audit Checklist:**
  - [ ] Follows NIST standards (for crypto)
  - [ ] No obvious side-channel vulnerabilities
  - [ ] Proper input validation
  - [ ] Error handling comprehensive
  - [ ] No timing dependencies on secrets
- **Testing Requirements:**
  - [ ] Fuzz testing completed
  - [ ] Edge cases covered
  - [ ] Performance impact measured

---

## Review & Approval Process

### 1. **Code Review Criteria**

Reviewers must verify:
- ✅ Code follows style and architectural guidelines
- ✅ All tests pass and coverage is adequate
- ✅ Documentation is clear and complete
- ✅ No security vulnerabilities introduced
- ✅ Performance impact is acceptable
- ✅ Error handling is comprehensive
- ✅ Breaking changes are clearly documented

### 2. **Approval Levels**

| Change Type | Reviews Needed | Domain Expert | Approval |
|-------------|---|---|---|
| Documentation | 1 | No | Maintainer |
| Bug fix | 2 | Domain-specific | Maintainer |
| Feature | 2 | Yes | SIG Lead + Maintainer |
| Security | 2 | Security expert | SIG-Security + Maintainer |
| Kernel | 2 | Kernel expert | SIG-Kernel + Maintainer |
| Crypto | 2 | Security expert | SIG-Security + Maintainer |

### 3. **Review Timeline**

- Routine PRs: Review within 2 business days
- Security PRs: Review within 24 hours
- Critical bugs: Review within 4 hours
- Revert requests: Review immediately

### 4. **Feedback Guidelines**

**For Reviewers:**
- Be respectful and constructive
- Explain reasoning behind suggestions
- Provide specific code examples
- Distinguish blocking issues from suggestions
- Approve when concerns are resolved

**For Contributors:**
- Respond to all feedback within 48 hours
- Ask for clarification if needed
- Don't resolve conversations yourself (let reviewers)
- Thank reviewers for their time

### 5. **Merge Requirements**

A PR can merge only when:
- [ ] All required approvals obtained
- [ ] All CI/CD checks passing
- [ ] Conversations resolved
- [ ] Commits squashed if needed
- [ ] Latest main branch merged in
- [ ] No conflicts remaining

---

## Enforcement & Escalation

### Violations

Consequences for guideline violations:

1. **First Violation:** Friendly reminder + guidance
2. **Second Violation:** PR blocked + requirements to continue
3. **Third Violation:** Contributor access review by maintainers
4. **Security Violation:** Immediate escalation to security team

### Reporting Issues

- **Code of Conduct Violations:** [See CODE_OF_CONDUCT.md]
- **Security Issues:** [See SECURITY_POLICY.md]
- **Process Questions:** Open GitHub Discussion or contact SIG lead

---

## Resources

- **Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS
- **Documentation:** Wiki
- **Discussions:** GitHub Discussions
- **Issue Tracker:** GitHub Issues
- **Roadmap:** ROADMAP.md
- **Security Policy:** SECURITY_POLICY.md

---

**Last Updated:** September 4, 2026  
**Maintained By:** SigmaOS Core Team  
**License:** MIT (same as SigmaOS)

> "Sovereignty is the ultimate efficiency." — SigmaOS Principle
