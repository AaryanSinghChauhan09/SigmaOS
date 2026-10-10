# ADR 001: PR-Only Research and Implementation Pipeline & Reproducible CI Build Gate

- **Status:** Accepted
- **Date:** 2026-10-10
- **Authors:** SigmaOS Core Architecture Team

## Context

SigmaOS needs a structured pipeline to study upstream operating systems (Linux, FreeBSD, OpenBSD, xv6, Debian, Fedora, Arch) and integrate missing components safely through reviewable, testable pull requests. Code presence alone without evidence does not constitute verified feature integration.

## Decision

1. **PR-Only Operating Model:** Every study, design, implementation, test, documentation, and status update must be represented by a reviewable PR.
2. **Reproducible CI Build Gate:** All PRs must pass `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, and `make format`, and generate a build manifest (`reports/build_manifest.json`).
3. **Structured Upstream Research:** Mandatory study notes in `research/upstream/`, Architecture Decision Records (ADRs) in `research/decisions/`, and comparisons in `research/comparisons/`.

## Consequences

- Prevents unverified parity claims.
- Enforces evidence-based status updates in `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`.
- Protects codebase health through automated CI workflows.
