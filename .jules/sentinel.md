# Sentinel's Journal 🛡️ - Security & Hardening Critical Insights

This journal contains ONLY critical security learnings, vulnerability patterns, and defensive hardening lessons for SigmaOS.

---

## 2025-05-15 - Landlock Unshare Isolation in Scriptlet Execution
**Learning:** Running third-party package post-install scriptlets in shared mount namespaces allowed malicious packages to access temporary file descriptors in `/tmp`. Applying Landlock ABI v3 sandbox rules with `LANDLOCK_ACCESS_FS_READ_FILE` restricts scriptlet filesystem access strictly to the package target path.
**Action:** Always wrap maintainer scriptlet execution in isolated Landlock + SECCOMP sandboxes.

## 2025-05-16 - Post-Quantum Cryptography Signature Verification
**Learning:** Ed25519 alone is vulnerable to quantum cryptanalysis long-term. Integrating dual Dilithium5 + Signify signature verification guarantees post-quantum signature attestation for package updates without noticeable validation slowdown.
**Action:** Enforce hybrid Dilithium5 + Ed25519 signature checks on all sovereign package manifests.
