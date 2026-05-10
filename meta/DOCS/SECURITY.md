# Security Policy

## Supported Versions

Currently, only the `main` branch (EXTINCTION-1 APEX) receives active security patches.

## Zero-Trust Architecture Guidelines

SigmaOS employs a "Zero-Trust shard namespacing" model. When contributing or modifying suites:

1. **No Implicit Trust:** No S-suite (S01-S33) inherently trusts another. All inter-suite communication MUST use the Sovereign Event Bus.
2. **WASM Isolation:** Untrusted or foreign logic MUST be executed inside the Native WASM JIT Engine within `S11_Virtualization`, adhering to WASI capability-based permissions.

3. **Memory Safety:** Avoid raw pointers outside of the `S05_Memory` suite. Use the native `sigma_sdk_malloc` and bounds-checked wrappers.

## Reporting a Vulnerability

Do not report security vulnerabilities via public GitHub issues. 
Please email `security@sigmaos.dev` with a detailed description, PoC (if available), and potential mitigation. We aim to acknowledge reports within 48 hours.
