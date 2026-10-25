# 🛡️ Sentinel's Journal - Security & Hardening Critical Learnings

## 2025-02-23 - Post-Quantum Signature Verification & Isolation Sandboxing
**Learning:** Legacy package verification using RSA-2048 or MD5/SHA1 checksums is vulnerable to collision and quantum cryptanalysis. Integrating Dilithium5 PQC signature validation and Signify attestation in package ingestion pipelines (`src/package/sovereign_distro_package_advancements_v70.rs`) prevents malicious scriptlet execution during installation.
**Action:** Enforce PQC signature verification, Landlock/seccomp sandboxing for maintainer scriptlets, and static memory bounds on all deserialized foreign package manifests.
