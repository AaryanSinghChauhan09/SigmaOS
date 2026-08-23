# SigmaOS: CI Pipeline & Release Verification Gating Specification

This document defines the automated validation checks, Software Bill of Materials (SBOM) generation, reproducible build verification steps, and release gating checklists for SigmaOS.

---

## 🌀 Continuous Integration (CI) Workflow

```
       [ Developer PR ]
              |
              v
     +-------------------+
     |  Lint & Format    | ---> cargo fmt / clippy / markdownlint
     +-------------------+
              |
              v
     +-------------------+
     | Standalone Tests  | ---> Verify all microkernel shards pass
     +-------------------+
              |
              v
     +-------------------+
     |   Double-Build    | ---> Compile Build A and Build B from source
     |  Reproducibility  |
     +-------------------+
              |
              v
     +-------------------+
     |  PQC Package Sign | ---> Embed Dilithium-5 signatures into SBOM
     +-------------------+
              |
              v
       [ Merge Gate ]
```

### 1. Build Reproducibility Verification
To prevent supply-chain injections, every release artifact must be strictly reproducible:
1. **Parallel Compilations:** The CI environment spawns two separate, isolated build instances (Build A and Build B) from identical source configurations.
2. **Deterministic Stripping:** Compilation outputs are stripped of timestamps, path references, and build-environment metadata.
3. **Cryptographic Comparison:** The CI compares the cryptographic hash values of both binary images:
   $$\text{Hash}(\text{Build A}) == \text{Hash}(\text{Build B})$$
   Any hash mismatch fails the gating pipeline instantly.

### 2. SBOM and Package Security
* **CycloneDX SBOM Generation:** Every build dynamically compiles a complete Software Bill of Materials (SBOM) listing all nested kernel modules, licenses, and library hashes.
* **Dilithium-5 Signing:** The resulting SBOM is signed with post-quantum Dilithium-5 signatures. These are checked by our `CryptoVerifier` and the bootloader before kernel startup.

---

## 🎯 Gating Checklist for Releases

Before any build can transition from `rolling` to `stable`, the release manager must verify the following automated and manual criteria:

- [ ] **Reproducibility Pass:** Hash matching between dual builds checks out perfectly.
- [ ] **PQC Verification:** Kyber-1024 and Dilithium-5 signatures are successfully embedded and validated across all binaries.
- [ ] **OOM LMK Resilience:** The kernel passes stress-testing under simulated memory exhaustion, demonstrating correct Low Memory Killer (LMK) process harvesting.
- [ ] **WDM Driver Clean Unload:** Zero memory leaks detected on pool tags (non-paged/paged pool allocation counter is 0 upon test driver uninstalls).
- [ ] **Zero-Regressions:** 100% of standalone unit tests pass cleanly.
