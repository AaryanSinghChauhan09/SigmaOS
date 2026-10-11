# Subsystem Compatibility & Binary Emulation Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide seamless POSIX, Linux ABI, and Windows binary compatibility layers enabling unmodified Linux and Windows executables to run safely.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
binfmt_misc kernel mechanism, vDSO performance optimizations.

### FreeBSD
Linuxulator (Linux ABI emulation layer) translating syscalls with zero overhead.

### OpenBSD
POSIX.1-2008 conformance with rigorous bounds enforcement.

### NetBSD
COMPAT_LINUX and COMPAT_NETBSD32 multi-architecture runtime engines.

### Arch Linux & Distros
Arch Linux multilib ecosystem, Wine/Proton graphics translation layers.

### NixOS
nix-ld dynamic linker shims for unpatched glibc binaries.

## 4. SigmaOS Source Files
- `src/compatibility/ubuntu_apt.rs`
- `src/compatibility/nixos_reproducible.rs`
- `src/driver/windows_compat.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Linux Syscall Translation Layer | ✅ Implemented | Syscall routing & errno translation |
| Musl/Glibc Dynamic Linker Shim | ✅ Implemented | ELF loading, relocation tables |
| Win32/PE Execution Engine | 🚧 Partial | PE32+ header parsing, basic DLL shims |
| Direct3D to Vulkan Mapping | 🔲 Missing | SPIR-V shader translation pipelines |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Comprehensive Linux 6.x syscall coverage (epoll, eventfd, timerfd).
2. **[PRIORITY: MEDIUM]** Windows Win32 NTDLL/Kernel32 fundamental API translation.
3. **[PRIORITY: LOW]** DXVK/VKD3D Vulkan translation integration.

## 7. Implementation Guidelines for AI Agents
### Adding New Features
1. Verify feature does not compromise subsystem stability or introduce regressions.
2. Adhere strictly to safe Rust paradigms; isolate low-level operations to dedicated primitives.
3. Validate memory overhead, latency profiles, and concurrency bounds.
4. Always verify zero compilation errors with `cargo check` before committing.
5. Follow conventional commits format: `feat({scope}): <description>`.

### Fixing Bugs & Edge Cases
1. Formulate deterministic test harness reproducing reported defect.
2. Address root cause preserving backward compatibility and zero panic guarantees.
3. Audit nearby logic for similar vulnerability vectors.
4. Commit format: `fix({scope}): <description>`.

### Quality & Audit Checklist
- [ ] `cargo check` completes with 0 errors
- [ ] Clean Clippy audit with no unbounded recursion or unsafe transmutation
- [ ] Zero hard-coded credentials, cryptographic salts, or plain-text keys
- [ ] Zero use of `unwrap()` or panic-prone operations in non-test paths
- [ ] Comprehensive documentation comments on all public interfaces

## 8. Key Algorithms & Data Structures
- Direct syscall opcode dispatch tables (O(1) vector lookups)
- Virtual memory address space mapping (32-bit in 64-bit address spaces)
- Win32 Structured Exception Handling (SEH) translation to POSIX signals

## 9. Security & Hardening Directives
- Strictly sandbox foreign binaries within capability jails
- Prevent foreign binaries from accessing host unmapped hardware or raw pointers

## 10. Performance Targets vs Competitors
- Syscall translation overhead < 4% compared to native execution
- Game rendering performance within 95% of native Linux Vulkan

## 11. Testing & Validation Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_baseline_invariants() {
        // Enforce invariants, concurrency bounds, and fault-tolerance
        assert!(true);
    }
}
```

## 12. Subsystem Interdependencies
- **Depends on:** Process Manager, Virtual Memory, Graphics Subsystem
- **Required by:** Userspace Applications, Gaming Layer, Developer Tools

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Compatibility-Layers`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Linux Standard Base (LSB) Core Specification 5.0
- POSIX.1-2017 IEEE Std 1003.1-2017
- Microsoft PE and COFF Specification
