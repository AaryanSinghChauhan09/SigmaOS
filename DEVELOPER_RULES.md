# 📋 SigmaOS Developer Rules & Task Guidelines

**Version:** 3.0.0
**Last Updated:** September 2026  
**Scope:** All developers, maintainers, contributors, and AI agents
**Master Directives Reference:** [`RULES.md`](RULES.md)

---

## Table of Contents

1. [Core Principles](#core-principles)
2. [Distribution Engineering Task Guidelines](#distribution-engineering-task-guidelines)
3. [Development Workflow](#development-workflow)
4. [Code Standards](#code-standards)
5. [Security & Safety](#security--safety)
6. [Testing & Verification](#testing--verification)
7. [Documentation](#documentation)
8. [Special Interest Groups (SIGs)](#special-interest-groups-sigs)
9. [AI Agent Guidelines](#ai-agent-guidelines)
10. [Review & Approval Process](#review--approval-process)

---

## Core Principles

### 1. **Sovereignty & Zero Dependencies**
- **No External Unverified Dependencies**: Core kernel and userspace modules must use Rust's `#![no_std]` with explicit capability bounds.
- **Self-Sufficiency**: Avoid external crates under `[dependencies]` in `Cargo.toml`.
- **Minimal Surface**: Each component should have the smallest possible attack surface.

### 2. **Memory Safety & Security-First**
- **Prefer Safe Rust**: Memory safety is non-negotiable.
- **Unsafe Blocks**: Only use `unsafe` for hardware interaction, driver development, or low-level OS primitives.
- **Document Invariants**: Every `unsafe` block must document all safety invariants.
- **Post-Quantum Cryptography**: All cryptographic operations must use Kyber-1024 (KEM) or Dilithium-5 (signatures).

---

## Distribution Engineering Task Guidelines

Inspired by Linux & BSD distribution development standards (Arch Linux, Debian, Fedora, Alpine, Gentoo, NixOS, FreeBSD, OpenBSD):

### 1. **Arch Linux PKGBUILD & ALPM Purity Standards**
- **Cleanroom Package Recipes**: Maintain PKGBUILD and PKGINFO array variable purity (`depends`, `makedepends`, `provides`, `conflicts`, `sha256sums`).
- **ALPM Topological Resolution**: Guarantee acyclic dependency graph traversal with topological sorting and explicit cycle detection in package managers.
- **AUR Audit Gating**: All third-party package build recipes must pass automated PKGBUILD safety audits before compilation.

### 2. **Debian sbuild & Pristine-Tar Reproducible Build Guidelines**
- **Determinism**: Enforce `SOURCE_DATE_EPOCH` environment variables, zero-timestamp tar header normalization, and canonical directory sorting.
- **Cleanroom Chroots**: Package compilation must execute inside isolated ephemeral build containers (`sbuild` / `poudriere` cleanroom jails).
- **Bit-for-Bit Verification**: Build outputs must be validated using `ReproducibleBuildRecord` diff hashes against published SBOM records.

### 3. **FreeBSD Ports & Poudriere QA Directives**
- **Pre-flight QA Testing**: Port builds must verify stage directory execution (`stage-qa`), test for leftover temporary files (`check-orphans`), and validate shared library dependencies (`lib-depends`).
- **Capsicum Capability Sandboxing**: Desktop applications and utilities must delegate file descriptor capability rights using FreeBSD Capsicum interfaces (`cap_rights_init`).

### 4. **OpenBSD Pledge/Unveil & Syspatch Security Directives**
- **Strict Sandbox Declarations**: Every userspace binary must call OpenBSD `pledge()` to restrict syscall capabilities and `unveil()` to lock down filesystem visibility immediately upon entry.
- **Fastpath Errata Patching**: Maintain atomic kernel live-patching and userland errata update compatibility (`syspatch` parity).

### 5. **NixOS Declarative State & Hermetic CAS Store Directives**
- **Merkle Closure Store**: Software builds must be addressed by input hashes inside a content-addressed storage (CAS) store.
- **Atomic State Hot-Swapping**: System configuration updates and package state transitions must support sub-millisecond atomic generation rollbacks.

---

## Development Workflow

### 1. **Branch Strategy**

**Mandatory Branch Naming Convention:**
- All developer and AI agent branches MUST start with the `jules-` prefix followed by descriptive task text (e.g. `jules-feat-kernel-scheduler`, `jules-fix-hotkey-binding`).

### 2. **Commit Guidelines**

- Short subject line (50 chars max), blank line, detailed body.
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `security`, `perf`.

---

## Testing & Verification

### 1. **Standalone Test Runner**
- Every Rust file modified in `src/` must be verifiable via standalone unit test compilation:
  `rustc --edition=2021 --test <filepath> -o /tmp/test_bin && /tmp/test_bin`

### 2. **Master Test Suite Execution**
- Before submitting changes, execute the native master test runner `./run_sigma_tests.sh` and ensure all 13 test runner stages pass cleanly.

---

**Last Updated:** September 2026
**Maintained By:** SigmaOS Core Architecture Team

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

## AI Agent & Contributor Development Rules

### 1. **Universal Rules for Human Contributors**
- **Zero External Dependencies**: Core microkernel shards and kernel subsystems operate under `#![no_std]` and must maintain 0 external crates in `Cargo.toml`.
- **Memory Safety & Unsafe Invariants**: Safe Rust is mandatory. Every `unsafe` block must include a `// SAFETY:` rationale.
- **Post-Quantum Cryptography (PQC)**: Driver signatures, package recipes, and kernel module attestations must use Dilithium-5 or Kyber-1024.
- **Capability Sandboxing**: Applications must enforce OpenBSD `pledge`/`unveil` rights or FreeBSD Capsicum capabilities.
- **Automated Verification**: Run `cargo check --lib` and `./run_sigma_tests.sh` before submitting pull requests.

### 2. **Operational Rules for AI Agents**
- **Planning Review Protocol**: AI agents must propose a plan via `request_plan_review` before calling `set_plan` for the first time.
- **Pre-Commit Step Requirement**: Plans must include a pre-commit step using the exact phrasing:
  `Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.`
- **Post-Edit Verification**: After every file creation or modification, the AI agent must verify the change with a read-only tool (`read_file` or `list_files`).
- **Code Review & Memory Recording**: AI agents must call `request_code_review` prior to submitting changes, and call `initiate_memory_recording` after review.
- **Secret Scanner Safeguards**: All mock keys or test tokens in code or tests must use names prefixed with `mock_` or `test_` to prevent false positive secret leaks.

### 3. **Scope & Limitations for AI Agents**

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

### 4. **Code Generation Rules**

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

---

## Task-Specific Policies & Operational Guidelines

### 1. Kernel & Subsystem Task Policy
- **Memory Allocations**: Paged pool memory access is strictly prohibited at `IRQL >= DispatchLevel` (triggers `DoubleFault` / `PAGE_FAULT_IN_NONPAGED_AREA`). Non-paged pool memory allocations must use ticket spinlocks or lockless Seqlock primitives (`SequenceLock`).
- **Lock Ordering Discipline**: Lock hierarchy must strictly follow `Irql::Dirql` -> `Irql::DispatchLevel` -> `Irql::ApcLevel` -> `Irql::PassiveLevel`. Deadlock prevention invariants must be documented for every nested lock acquisition.
- **`unsafe` Code Invariants**: Every `unsafe` block must include a `// SAFETY:` rationale explaining pointer alignment, memory bounds, or hardware IO port safety assumptions.

### 2. Package Management & Universal Adapter Policy
- **Format Integrity**: Every universal package adapter (`src/package/universal.rs`) must implement Strategy, Adapter, State, Decorator, and Observer patterns natively without external CLI invocations.
- **Cryptographic Signatures**: All package manifests must enforce Post-Quantum Dilithium-5 digital signatures prior to store extraction under `/sigma/store/`.
- **User-Defined Functions (UDFs)**: Pre-install, post-install, pre-remove, and post-remove UDF hooks must execute inside restricted `DistrictSandbox` isolation containers with CPU/memory limits.

### 3. Security, Privacy & Secret Management Policy
- **Secret Variable Naming**: In test functions and mock credential definitions, variable names containing sensitive keywords MUST include `mock` or `test` (e.g. `mock_client_secret`) to comply with automated repository secret scanning rules.
- **Sandboxing & Privilege Rights**: All new userspace binary components must declare OpenBSD `pledge`/`unveil` rights and FreeBSD Capsicum capability rights before launching.

### 4. Testing & Verification Policy
- **Standalone Test Runner**: Every Rust module in `src/` must be verifiable via standalone unit test runner invocation (`rustc --test <filepath> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_bin && /tmp/test_bin`).
- **Zero Compilation Warnings**: All code submissions must compile without warnings under `-D warnings`.

### 5. Release Engineering Policy
- **Tag & Release Signing**: All release artifacts, ISO images, and Git tags must be signed using GPG or Post-Quantum Dilithium-5 keys.
- **Reproducible Build Verification**: Build outputs must be validated bit-for-bit against published SHA-256 build provenance hashes and SBOM manifests.

### 6. Linux & BSD Distro Innovation & Compatibility Policy
- **Syscall ABI Translation Invariants**: `UniversalSyscallAbiShim` must preserve zero-copy register state for Linux System V x86_64 and BSD POSIX syscall ABI translation vectors.
- **Stateless Configuration Fallback**: Inspired by Clear Linux, system default configurations must reside under `/usr/share/defaults/` while user overrides in `/etc/` take precedence without corrupting vendor defaults.
- **Sandboxing Standards**: Userland services must enforce OpenBSD `pledge`/`unveil` path isolation or FreeBSD Capsicum capability rights before listening on untrusted network interfaces.
- **Declarative State Reconciliation**: Subsystems inspired by NixOS/Guix must support Content-Addressable Store (CAS) hash tracking and atomic generation rollback without modifying live binaries during execution.
