# Sentinel's Journal — Critical Security & Hardening Learnings

## 2026-04-12 - Unsanitized User-Defined Scriptlet Execution in Package Sandboxes
**Vulnerability:** Upstream Linux distribution maintainer scriptlets (`preinst`, `postinst`) executed without strict filesystem namespace isolation risk modifying host system configuration files outside the target rootfs container.
**Learning:** Legacy package adapters relied solely on process chroot without Landlock filesystem path restrictions or cgroup v2 resource clamping.
**Prevention:** Enforce multi-sandbox isolation combining Landlock rules, cgroup v2 memory/CPU limits, and unprivileged user namespaces (`CLONE_NEWUSER`) across all 12+ package maintainer hook phases.

## 2026-04-12 - Foreign Package Signature Validation Missing PQC Fallbacks
**Vulnerability:** Classic RSA-2048 / Ed25519 signatures in legacy package repositories are vulnerable to future quantum cryptanalysis.
**Learning:** Foreign package translation pipelines required dual-signature verification supporting quantum-resistant Dilithium5 alongside OpenBSD Signify attestation.
**Prevention:** Mandate SLSA Level 4 provenance attestation and dual-key verification (Dilithium5 + Signify) before untrusted foreign package archives are decompressed into the zero-copy CAS store.
