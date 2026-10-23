# Sentinel's Journal 🛡️

## 2026-10-23 - Strict User-Defined Function (UDF) Execution Sandboxing & PQC Signatures
**Vulnerability:** Unsanitized maintainer scriptlets in package managers (e.g. post-install ebuild hooks, PKGBUILD `install` functions) could execute arbitrary shell code outside containerized boundaries.
**Learning:** Parsing package scriptlets requires strict seL4 microkernel-style Landlock/unshare isolation combined with Dilithium5 PQC signature validation to prevent supply-chain execution attacks.
**Prevention:** Enforce pre-execution AST verification and isolated IPC capability gates for all UDF scriptlets.
