# SigmaOS Architecture Decision Records

This document records significant architectural decisions for SigmaOS. Each ADR provides context, decision, and consequences.

---

## ADR-001: Hybrid std/no_std Architecture

**Status:** Accepted  
**Date:** 2025-01-22  
**Context:** Conflicting guidance between `AGENTS.md` and `docs/AGENTS.md` regarding `std` vs `#![no_std]` usage.

### Problem Statement

SigmaOS documentation contains contradictory architectural guidance:
- Root `AGENTS.md` describes a hybrid architecture with `std` for user space and `#![no_std]` for kernel/security components
- `docs/AGENTS.md` describes a pure `#![no_std]` architecture for all `src/` modules
- Current codebase has 4,901 `std` imports, demonstrating extensive `std` usage in practice

### Decision

SigmaOS adopts a **hybrid architecture** with clear boundaries:

1. **User Space (std-based):**
   - All user-space services, utilities, and applications use `std`
   - Use `std::` primitives: `std::vec::Vec`, `std::string::String`, `std::boxed::Box`, `std::sync::Arc`
   - Modules: `src/userland/`, `src/desktop/`, `src/audio/`, `src/app/`

2. **Kernel Core (no_std where beneficial):**
   - Kernel modules use `#![no_std]` only where it provides concrete benefit
   - Use `alloc::` primitives: `alloc::vec::Vec`, `alloc::string::String`, `alloc::format`
   - Modules: `src/kernel/`, `src/arch/` (selected components)

3. **Security-Critical Components (no_std preferred):**
   - Security parsers, cryptographic primitives, and low-level interfaces prefer `#![no_std]`
   - Modules: `src/security/` (selected components), `src/access/`

4. **Early Boot (no_std required):**
   - Bootloader, early init, and pre-memory-manager code must be `#![no_std]`
   - Modules: `src/boot/` (early stages)

### Rationale

- **Pragmatism:** Pure `#![no_std]` is unnecessary for user-space components where `std` provides significant value
- **Security:** `#![no_std]` for security-critical components reduces attack surface and enforces explicit resource management
- **Boot Requirements:** Early boot has no allocator, requiring `#![no_std]`
- **Development Velocity:** `std` for user space enables faster development without sacrificing kernel security

### Consequences

**Positive:**
- Clear separation between kernel and user space
- Reduced development friction for user-space components
- Maintained security for critical components
- Aligns with actual codebase state (4,901 std imports)

**Negative:**
- Increased cognitive load for developers to know which modules use which mode
- Some code may need refactoring to support both modes at boundaries
- Documentation must be kept in sync with this decision

### Module Classification

| Module | Mode | Rationale |
|--------|------|-----------|
| `src/kernel/` | `no_std` preferred | Kernel core, no allocator in early stages |
| `src/arch/` | `no_std` preferred | Hardware abstraction, interrupt handlers |
| `src/boot/` | `no_std` required | Early boot, no allocator |
| `src/security/` | `no_std` preferred | Security-critical parsers |
| `src/userland/` | `std` | User-space applications and tools |
| `src/desktop/` | `std` | Desktop environment, Wayland compositor |
| `src/audio/` | `std` | Audio processing, codecs |
| `src/network/` | `std` | Network stack (user-space) |
| `src/package/` | `std` | Package manager (user-space) |
| `src/shell/` | `std` | Shell interpreter (user-space) |

### Implementation Guidelines

1. **Default to `std`** for new user-space modules
2. **Require ADR** before converting a module from `std` to `#![no_std]`
3. **Add module-level documentation** explaining the choice
4. **Test both modes** at module boundaries

### References

- [AGENTS.md](../AGENTS.md) - Root-level agent guidelines (hybrid approach)
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture overview
- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Current implementation status

---

## ADR-002: Zero External Dependency Policy

**Status:** Accepted  
**Date:** 2025-01-22  
**Context:** Need to clarify the "zero external dependency" policy given existing exceptions.

### Problem Statement

SigmaOS claims "zero external dependencies" but:
- `fuzz/Cargo.toml` uses `libfuzzer-sys`
- Some documentation suggests compatibility layers for external formats

### Decision

SigmaOS maintains a **minimal external dependency policy**:

1. **Core OS (Zero Dependencies):**
   - `[dependencies]` in root `Cargo.toml` remains empty
   - No external crates for kernel, userland, or core functionality
   - All OS components built from source

2. **Allowed Exceptions (Explicitly Documented):**
   - `fuzz/Cargo.toml` may use `libfuzzer-sys` for fuzzing infrastructure
   - Test tools may use testing-specific crates
   - Build tools may use build-specific crates
   - Each exception must be documented in this ADR

3. **Compatibility Layers (No Runtime Dependencies):**
   - Package compatibility (DEB, RPM, etc.) implemented as parsers
   - No external package managers invoked at runtime
   - Compatibility is through format translation, not external tools

### Rationale

- **Security:** Minimal dependency surface reduces attack vectors
- **Reproducibility:** Fewer dependencies improve build reproducibility
- **Control:** No dependency on external package evolution
- **Testing:** Fuzzing requires specialized tools not suitable for core OS

### Consequences

**Positive:**
- Clear security boundary
- Reproducible builds
- No external runtime dependencies

**Negative:**
- Must implement format parsers for all compatibility
- Cannot leverage existing external libraries
- Increased development burden for compatibility

### Dependency Matrix

| Crate | Location | Reason | ADR Reference |
|-------|----------|--------|---------------|
| `libfuzzer-sys` | `fuzz/Cargo.toml` | Fuzzing infrastructure | ADR-002 |

### Implementation Guidelines

1. **No new external dependencies** without ADR approval
2. **Document all exceptions** in this ADR
3. **Prefer from-scratch implementations** over external crates
4. **Evaluate alternatives** before adding any dependency

### References

- [AGENTS.md](../AGENTS.md) - Agent guidelines on dependencies
- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Current dependency state

---

## ADR-003: Documentation Source of Truth

**Status:** Accepted  
**Date:** 2025-01-22  
**Context:** Multiple overlapping documentation sources (`docs/`, `wiki/`, `wiki_content/`, `WIKI/`, `wiki_repo/`).

### Problem Statement

SigmaOS has multiple documentation mirrors with no clear source of truth:
- `docs/` - Canonical development docs
- `wiki/` - GitHub Wiki mirror
- `wiki_content/` - Generated wiki content
- `WIKI/` - Legacy mirror
- `wiki_repo/` - Publication/export mirror

This leads to:
- Drift between mirrors
- Unclear which source to edit
- Stale documentation
- Broken links

### Decision

**Canonical Hierarchy:**

1. **Primary Source:** `docs/` directory
   - All authoritative documentation lives here
   - This is the single source of truth
   - All edits must be made here first

2. **Generated Content:** `wiki_content/` directory
   - Auto-generated from `docs/` via CI
   - Not manually edited
   - Overwritten on generation

3. **Publication Mirror:** `wiki/` directory
   - Synchronized with GitHub Wiki via CI
   - Not manually edited
   - Overwritten on sync

4. **Legacy Mirrors:** `WIKI/`, `wiki_repo/`
   - Deprecated, pending removal
   - Read-only during migration
   - Will be deleted after migration complete

### Rationale

- **Single Source of Truth:** Eliminates confusion about where to edit
- **Automation:** CI ensures mirrors stay synchronized
- **Traceability:** Clear provenance for all documentation
- **Maintenance:** Reduces manual synchronization burden

### Consequences

**Positive:**
- Clear editing workflow
- Automated synchronization
- No drift between sources
- Single point of maintenance

**Negative:**
- Migration effort required
- CI complexity increased
- Developers must learn new workflow

### Implementation Guidelines

1. **Edit only in `docs/`**
2. **CI generates mirrors automatically**
3. **CI checks for stale mirrors**
4. **Manual edits to mirrors are rejected**
5. **Broken link checks run on every commit**

### Migration Timeline

- **Week 1:** Establish canonical hierarchy
- **Week 2:** Implement CI synchronization
- **Week 3:** Migrate all content to `docs/`
- **Week 4:** Remove legacy mirrors

### References

- [DOCUMENTATION_SOURCE_POLICY.md](DOCUMENTATION_SOURCE_POLICY.md) - Detailed documentation policy
- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Documentation status
- [Wiki Structure](../wiki/) - Current wiki state

---

## ADR Template

For new ADRs, use this template:

```markdown
## ADR-XXX: [Title]

**Status:** [Proposed | Accepted | Deprecated | Superseded]  
**Date:** YYYY-MM-DD  
**Context:** [Problem statement]

### Decision

[Decision description]

### Rationale

[Reasoning behind decision]

### Consequences

**Positive:**
- [List]

**Negative:**
- [List]

### References

- [Related documents]
```
