# Debian Reproducible Build & Policy Engine Component Agent

## Role
The **Debian Reproducible Policy Agent** (`SigmaDebianPolicyAgent`) enforces deterministic bit-for-bit package verification, canonical metadata clamping, and strict package lifecycle transitions inspired by Debian Policy and the Reproducible Builds Initiative.

## Linux Inspiration
- **Debian**: Debian Policy Manual, deterministic reproducible builds, timestamp clamping via `SOURCE_DATE_EPOCH`, sorted package entries, and multi-architecture dependency classification (`same`, `foreign`, `allowed`).

## Current SigmaOS Status
Reproducible build policy engine implemented in `src/package/debian_policy_engine.rs` supporting deterministic SHA-256 artifact manifests, timestamp clamping, canonical permission standardization, and strict lifecycle state machines (`NotInstalled` -> `Unpacked` -> `Configured` -> `Installed`).

## Key Files
- `src/package/debian_policy_engine.rs`
- `src/package/mod.rs`
- `src/package/alpm_engine.rs`

## Public API & Invariant Contract
```rust
pub struct ReproducibleFileEntry {
    pub path: String,
    pub sha256_hash: [u8; 32],
    pub file_size: u64,
    pub permissions: u32,
    pub clamped_mtime: u64,
}

pub struct DebianPolicyEngine {
    pub fn register_package(&mut self, name: &str, version: &str, arch: &str, multi_arch: MultiArchType, deps: Vec<String>) -> &mut DebianPolicyPackage;
    pub fn add_file_entry(&mut self, pkg_name: &str, path: &str, content: &[u8], is_executable: bool) -> Result<(), &'static str>;
    pub fn transition_state(&mut self, pkg_name: &str, next_state: PackageState) -> Result<(), &'static str>;
}
```

## Testing Strategy
```bash
rustc --test src/package/debian_policy_engine.rs --edition 2021 -o /tmp/test_deb_policy && /tmp/test_deb_policy --test-threads=1
```
- Verify deterministic manifest order across varying insertion sequences.
- Enforce strict legal state machine transitions.

## Future Enhancements
- Automated multi-architecture dependency graph SAT solving.
- Cryptographic in-toto supply chain metadata attestation.
