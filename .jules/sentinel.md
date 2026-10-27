# 🛡️ Sentinel's Journal - Critical Security & Hardening Learnings

## 2025-05-20 - Multi-Sandbox Isolation & UDF Scriptlet Execution Bounds
**Learning:** Executing untrusted upstream maintainer package scriptlets (pre/post install) directly in userland poses critical privilege escalation and arbitrary code execution hazards.
**Action:** Enforce strict Landlock v4, OpenBSD Pledge/Unveil, and Capsicum capability-based sandboxing across all UDF scriptlet execution hooks.

## 2025-05-21 - PQC Signature Attestation & Memory Protection
**Learning:** Traditional RSA/ECDSA package signing is vulnerable to future quantum computing advances. Implementing post-quantum cryptographic (PQC) Dilithium5 & OpenBSD Signify dual-signature verification guarantees long-term supply chain security.
**Action:** Enforce mandatory PQC attestation and W^X memory page protection across all executable loading pathways.
