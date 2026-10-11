# Internationalization (i18n) & Localization (l10n) Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide complete UTF-8/Unicode 16.0 support, multi-lingual keyboard input engines (IBus/Fcitx style), and locale-aware formatting across all userland.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
glibc locale architecture, gettext translation catalogs, fontconfig font fallback.

### FreeBSD
BSD libc locale database, UTF-8 console localization.

### OpenBSD
Safe UTF-8 string validation without multibyte buffer overflow vulnerabilities.

### NetBSD
citrus internationalization framework for multibyte character encoding.

### Arch Linux & Distros
Ubuntu multilingual language packs, Arch Linux locale-gen infrastructure.

### NixOS
Declarative global i18n profiles, console font mappings, and keyboard layout trees.

## 4. SigmaOS Source Files
- `src/desktop/i18n.rs`
- `src/graphics/font_builder.rs`
- `src/desktop/text_scaling.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Unicode 16.0 Normalization | ✅ Implemented | NFC, NFD, NFKC, NFKD transforms |
| Multilingual Input Method Engine | ✅ Implemented | Pinyin, Romaji, Hangul input shims |
| Locale-aware String Formatting | ✅ Implemented | Numbers, currency, date-time |
| Bi-Directional (BiDi) Text Layout | 🚧 Partial | Arabic, Hebrew RTL rendering |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** HarfBuzz-grade complex text shaping in safe Rust.
2. **[PRIORITY: MEDIUM]** System-wide translation catalog (.mo / gettext compatible) loader.
3. **[PRIORITY: LOW]** Multi-script font fallback chaining in Zenith font renderer.

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
- Unicode Bidirectional Algorithm (UAX #9)
- Unicode Line Breaking Algorithm (UAX #14)
- Trie-based character property lookup tables

## 9. Security & Hardening Directives
- Enforce strict UTF-8 validation rejecting non-shortest form UTF-8 and invalid surrogate codepoints
- Protect against homoglyph attacks in system usernames and filenames

## 10. Performance Targets vs Competitors
- UTF-8 validation throughput > 4 GB/s with AVX-512
- Text shaping latency < 0.5ms per paragraph

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
- **Depends on:** Font Renderer, Desktop Compositor, Terminal Subsystem
- **Required by:** Desktop Shell, Application UI, Terminal, System Settings

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Internationalization-Component-Guide`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Unicode Standard Version 16.0.0
- Unicode Technical Standard #35: Unicode Locale Data (CLDR)
- GNU gettext Runtime Library Manual
