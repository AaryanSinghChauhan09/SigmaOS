# SigmaOS: Sovereign Sandbox Hardening

This document outlines the security policies and hardening techniques applied to the SigmaOS Sovereign Sandbox.

## 1. Config-Driven Policies

The sandbox now utilizes `sandbox_policy.json` to declaratively define shard capabilities.

* **Principle of Least Privilege**: All shards start with `DENY` as the default action.
* **Capability-Based Access**: Shards must explicitly request capabilities (e.g., `OBSERVE_LATTICE`).

## 2. Syscall Filtering

Syscalls are intercepted at the `sigma_syscall_gate` and matched against the shard's policy.

* **WASI Compliance**: For WASM-based shards, the sandbox enforces a strict WASI-compatible subset of syscalls.

## 3. Resource Isolation

* **Memory Quotas**: Each shard is allocated a strictly bounded set of physical pages.
* **CPU Limits**: The `SovereignScheduler` enforces millisecond-level time slicing to prevent DoS by a rogue shard.

## 4. Verification

* **Regression Tests**: `tests/cpp_host/test_sandbox.cpp` validates that blocked syscalls correctly trigger a `SIGMA_SECURITY_FAULT`.
* **Formal Verification**: Planned integration of Coq-based proof for the syscall filtering logic.
