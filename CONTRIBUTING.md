# Contributing to SigmaOS Zenith Supreme

Welcome to the Sovereign developmental workforce. To contribute to SigmaOS, you must adhere to our **Industrial Purity Standards**.

## 1. The Zero-HLL Mandate
SigmaOS is a **Pure C11 and Assembly** environment.
- **NO C++**: Use our struct-based OOP framework (`SigmaOOP.h`) for encapsulation.
- **NO Standard Libraries**: Use our Sovereign LibC shards. Do not include `<stdio.h>`, `<string.h>`, etc.
- **NO External Dependencies**: Every routine must be natively sharded within the repository.

## 2. Shard Alignment
New features should be sharded into their respective modular territories:
- **Core Kernel Suites**: `/kernel/suites/`
- **Userland Apps & GUI**: `/userland/`
- **Tooling & Test Hooks**: `/tools/`

## 3. Verification
All contributions must be accompanied by an update to the corresponding `tools/dev/sovereign_test/sovereign_test_runner.c` suite. A 100% pass rate is mandatory for shard merging.

## 4. Submission Process
1.  **Fork the Shard**.
2.  **Initialize your Shard**.
3.  **Perform Silicon Audit** (ensure no lint errors).
4.  **Submit Pull Request** for Apex Review.

---
**S MAINTAIN THE PURITY. EXPAND THE SOVEREIGNTY.**
