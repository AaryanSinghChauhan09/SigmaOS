<<<<<<< HEAD
# SigmaOS — Contribution Guide

> Join the evolution of digital sovereignty.

---

## 🛠️ Coding Standards

SigmaOS is an industrial-grade project. All contributions must adhere to:

- **Language**: C++20 for shards, Python 3.10+ for tooling.
- **Pattern**: Strict OOP Singleton (Context Manager).

- **Zero-Dependency**: No `libc` or external headers allowed in Layer 1-4 shards.
- **Documentation**: Every code change MUST be accompanied by an update to the corresponding `.md` file in `WIKI/`.

## 🌀 Branching Strategy

We follow a staged release cadence:

- **`main` (Stable)**: The production-ready sovereign lattice. Only merges from `beta` allowed.
- **`beta` (Staging)**: Integration branch for feature-complete topics.

- **`alpha` (Topic)**: Feature-specific branches (e.g., `feat/neural-paging`).

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
=======
﻿# Contributing to SigmaOS

We welcome contributors! As a meritocratic project, we value high-quality code, clear documentation, and a focus on sovereign computing.

## ðŸ›  Setup

1. Clone the repo.
2. Ensure you have a C++20 compatible compiler (GCC 12+, Clang 15+).
3. Follow the [Installation Guide](https://github.com/AaryanSinghChauhan09/SigmaOS.wiki/blob/master/INSTALLATION_GUIDE.md).

## ðŸ§­ Branching Strategy

* `main`: Stable, production-ready code.
* `develop`: Integration branch for new features.
* `feature/*`: Individual feature shards.
* `fix/*`: Bug fixes.

## ðŸ“ Coding Standards

* **OOP Isolation**: All new shards must inherit from `SigmaObject`.
* **No Raw Pointers**: Use Sovereign smart pointers or reference-counted objects.
* **Documentation**: Every public method must be documented in the header.

## ðŸ—³ Sovereign Council

Major architectural changes require an RFC and approval from the [Sovereign Council](https://github.com/AaryanSinghChauhan09/SigmaOS.wiki/blob/master/GOVERNANCE_CHARTER.md).
>>>>>>> 7759f274e222d74141c499a7b379a060016fe9a1
