# Arch ALPM Pacman Agent

## Role
The **Arch ALPM Pacman Agent** (`SigmaAlpmAgent`) governs package database synchronization, dependency graph resolution, transactional package installs/removals, and delta updates inspired by Arch Linux's `pacman` and `libalpm` (https://github.com/archlinux/pacman).

## Mission & Purpose
- Provide 100% ABI and behavior parity with Arch Linux's `libalpm`.
- Manage dual database state: `localdb` (installed packages) and `sync_dbs` (repository package mirrors).
- Enforce strict dependency tree validation, preventing the accidental removal of core runtime libraries.
- Support cryptographic verification (PGP/Ed25519) and atomic rollback transactions.

## Source Implementation
- Core Engine: `src/package/alpm_engine.rs`
- Package Module: `src/package/mod.rs`

## Public API & Invariant Contract
```rust
pub struct AlpmPackage {
    pub name: String,
    pub version: String,
    pub arch: PackageArch,
    pub depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
}

pub struct SovereignAlpmEngine {
    pub fn install_package(&mut self, pkg: AlpmPackage, flags: AlpmTransactionFlags) -> Result<(), &'static str>;
    pub fn remove_package(&mut self, pkg_name: &str, flags: AlpmTransactionFlags) -> Result<(), &'static str>;
    pub fn check_status(&self, pkg_name: &str) -> SyncStatus;
    pub fn search_sync(&self, query: &str) -> Vec<&AlpmPackage>;
}
```

## Repeatable AI Execution Checklist
- [ ] Test standalone with `rustc --test src/package/alpm_engine.rs --edition 2021`.
- [ ] Verify dependency tree rejects removal of parent dependencies without `--nodeps`.
- [ ] Ensure package searches execute in O(n) without dynamic heap fragmentation.
- [ ] Maintain parity with Arch Linux repository specifications.
