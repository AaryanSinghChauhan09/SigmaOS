# Deterministic Reproducibility

> **Status**: ACTIVE | **Component**: `sigpkg` & `sigma-sdk`

SigmaOS guarantees that every system binary and Sovereign Shard is **100% bit-for-bit reproducible**. This eliminates "trusting the builder" and ensures that malicious code cannot be injected during the compilation pipeline without detection.

---

## 1. Deterministic Build Architecture

To achieve absolute reproducibility, the `sigma-sdk` build pipeline controls all sources of non-determinism:

*   **Compiler Sandboxing**: Builds occur within a pristine Sovereign Sandbox. Host environment variables (e.g., `USER`, `HOME`) are stripped.
*   **Normalized Timestamps**: All file modification times (`mtime`) in the build output are set to the `SOURCE_DATE_EPOCH` (typically the timestamp of the git commit).
*   **Path Stripping**: Absolute paths are stripped from debug symbols and panic handlers using `-Z remap-cwd-prefix` (Rust) and `-fdebug-prefix-map` (C/C++).
*   **Seeded RNG**: The compiler's random number generators (used for symbol generation) are seeded deterministically based on the package hash.

## 2. The Sovereign Finality Certificate

When a package is published to the `sigma-recipes` repository, it is accompanied by a **Sovereign Finality Certificate (SFC)**. 

The SFC contains:
1. The BLAKE3 hash of the source code.
2. The exact compiler version and environment parameters.
3. The expected BLAKE3 hash of the final `.spkg` binary.
4. A Dilithium5 post-quantum cryptographic signature.

## 3. Verification by `sigpkg`

When a user runs `sigpkg install <shard>`, the package manager performs the following checks:

1. Downloads the `.spkg` and the SFC.
2. Validates the Dilithium5 signature on the SFC.
3. Computes the BLAKE3 hash of the downloaded `.spkg`.
4. Asserts that the computed hash matches the expected hash in the SFC **exactly**.

If the hashes mismatch by even a single bit, the installation is aborted, and an anomaly is logged to the forensic audit ring.

## 4. Local Rebuild Verification

Any user can independently verify a package by running:
```bash
sigma-sdk verify --rebuild <package-name>
```
This command downloads the source, recompiles it locally in a sandbox, and proves that the resulting binary hash matches the upstream SFC.
