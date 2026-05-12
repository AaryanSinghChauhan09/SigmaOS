# SigmaOS — Contribution Guide

> Join the evolution of digital sovereignty.

---

## 🛠️ Coding Standards

SigmaOS is an industrial-grade project. All contributions must adhere to:

* **Language**: C++20 for shards, Python 3.10+ for tooling.
* **Pattern**: Strict OOP Singleton (Context Manager).

* **Zero-Dependency**: No `libc` or external headers allowed in Layer 1-4 shards.
* **Documentation**: Every code change MUST be accompanied by an update to the corresponding `.md` file in `WIKI/`.

## 🌀 Branching Strategy

We follow a staged release cadence:

* **`main` (Stable)**: The production-ready sovereign lattice. Only merges from `beta` allowed.
* **`beta` (Staging)**: Integration branch for feature-complete topics.

* **`alpha` (Topic)**: Feature-specific branches (e.g., `feat/neural-paging`).

## 🚀 CI/CD Pipeline

Every Pull Request triggers the following automated suite:

1. **Lattice Rebuild**: All 600+ shards must compile with zero warnings.
2. **Regression Suite**: IRQ handlers and SHS v2 are verified for RDTSC-cycle precision.

3. **Security Scan**: Verify PQC signatures and TPM handshake protocols.
4. **Doc Lint**: Ensure all WIKI files follow the GitHub Flavored Markdown standard.

## 🤝 How to Contribute Shards

1. **Fork** the repository and create an `alpha` branch.
2. **Develop** your shard in the appropriate `suites/` directory.

3. **Sync** documentation in `WIKI/`.
4. **Submit** a PR to `beta` for review.

---
"Sovereignty is a collective intent."
