# SigmaOS Reproducible Builds Specification

## Overview
SigmaOS enforces 100% deterministic builds across its entire package ecosystem. By neutralizing variables like timestamps, compile paths, and filesystem order during compilation, we guarantee that compiling a specific Git commit results in bit-for-bit identical binaries. Each package contains cryptographically signed Software Bills of Materials (SBOMs) to verify package provenance.

## Deterministic Pipeline Flow
```
 [Source Code (Git Commit)] ──► [Neutralize Timestamps & Paths]
                                         │
                                         ▼
 [Isolated Container Sandbox] ──► [Deterministic Compilation]
                                         │
                                         ▼
 [Cryptographic Signature] ◄──► [Bit-for-Bit Parity Check]
         │
         ▼
 [Staged Repository Package + SBOM]
```

## System Properties
Build environment policies are declared in `build.toml`:
```toml
[build]
reproducible = true
env_clear = ["PATH", "LANG", "TZ"]
timezone = "UTC"
timestamp = 1770000000 # Fixed epoch timestamp for build determinism

[sbom]
format = "SPDX"
hash_algorithm = "sha256"
```

## Technical Implementation
Our build verification script compares hash outputs of separately compiled binary files to assert bit-for-bit identity.

```rust
// tools/sigma_iso_builder.rs
pub fn verify_binary_determinism(path_a: &Path, path_b: &Path) -> Result<bool, io::Error> {
    let bytes_a = fs::read(path_a)?;
    let bytes_b = fs::read(path_b)?;
    
    if bytes_a.len() != bytes_b.len() {
        return Ok(false);
    }
    
    Ok(bytes_a == bytes_b)
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Build environment isolation removing local paths, hostnames, and time stamps.
- **Phase 2 (Months 3-6)**: SPDX-compliant SBOM generator integration inside `sigpkg-build`.
- **Phase 3 (Months 6-9)**: Re-builder farms running independent builds to cross-verify binary signatures.
- **Phase 4 (Months 9-12)**: System-wide verification policies preventing installation of any package lacking signed SBOM attestation.
