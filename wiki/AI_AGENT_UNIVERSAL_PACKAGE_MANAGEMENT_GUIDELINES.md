# AI Agent Universal Package Management Guidelines

## Purpose
These guidelines define operational protocols, implementation patterns, and safety guardrails for AI coding agents extending foreign package format adapters or command dispatchers in SigmaOS.

---

## Directives for AI Agents

1. **Foreign Format Adapter Implementation**:
   - Always implement header magic byte detection in `detect_format_by_header` and extension checking in `detect_format_by_extension`.
   - Register foreign dependencies through `UniversalDependencyMapper::to_canonical_name` to ensure cross-distro package harmonization.

2. **Foreign Command Dispatching**:
   - Support dry-run simulation flags (`--dry-run`, `-n`, `-p`, `-s`, `--simulate`) across all foreign PM command dispatches.

3. **Code Pattern: Absorbing Foreign Package**:
```rust
let mut bridge = SigPkgUniversalBridgeEngine::new();
let pkg = bridge.absorb_and_register("app.deb", deb_control_bytes)?;
assert!(bridge.is_package_registered("app"));
```

4. **Testing and Verification**:
   - Execute `./run_sigma_tests.sh` to confirm universal package adapter unit tests pass across all formats.

---

## Related Files
- `src/sigpkg/universal_adapter.rs`
- `src/bin/sigpkg.rs`
- `docs/AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT.md`
