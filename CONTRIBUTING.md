# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS!

---

## 📜 Principles & Standards

SigmaOS enforces strict engineering standards inspired by Arch Linux, FreeBSD, and OpenBSD:

1. **Zero External Unverified Dependencies**: Core kernel and userspace modules are written in standard-library-free, memory-safe Rust with explicit capability bounds.
2. **PQC Cryptographic Signing**: All driver modules, package recipes, and security advisories must be signed using post-quantum Kyber-1024 or Dilithium-5 signatures.
3. **Capability-Gated Isolation**: Every new feature or driver shard must run in userland under `pledge` and `unveil` sandboxing primitives.
4. **Comprehensive Verification**: All pull requests must pass the atomic test suite (`./run_sigma_tests.sh`) and pass quality gates (`./scripts/sigma_quality_check.sh`).

---

## 🛠️ Contribution Workflow

1. **Fork & Branch**: Create a feature branch off `main`.
2. **Implement & Test**: Add code/tests and verify locally using `./run_sigma_tests.sh`.
3. **Quality Check**: Run `./scripts/sigma_quality_check.sh --strict`.
4. **Submit PR**: Open a pull request against `main`.

---

## 🛡️ Security Disclosures

Please report security vulnerabilities directly to the security team following `.github/SECURITY.md`.
