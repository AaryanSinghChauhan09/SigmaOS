# SigmaOS Security Guidelines for AI Agents (`SECURITY_AGENTS.md`)

This document defines mandatory security constraints and verification requirements for AI autonomous agents contributing code to SigmaOS.

---

## Mandatory AI Security Controls

1. **Zero External Dependencies:**
   - AI agents must NOT introduce third-party crate dependencies under `[dependencies]` in `Cargo.toml`.
   - All cryptographic algorithms (ChaCha20, SHA-256, FNV-1a, Merkle accumulators) and data structures must remain zero-dependency `#![no_std]` native primitives.

2. **Automated Code Review & Auditing:**
   - Every code change must be evaluated using `request_code_review` before completion.
   - Address any blocking issues or nitpicks flagged during review.

3. **Verification & Regression Testing:**
   - Run `./run_sigma_tests.sh` to execute the full Rust test suite and Python pytest integration suite.
   - Verify that all standalone test targets build and pass cleanly.
