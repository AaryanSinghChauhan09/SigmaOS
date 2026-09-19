# AI Agent Context Providers Management Specification for SigmaOS

This document specifies operational standards for AI agents managing, querying, and implementing Context Providers in **SigmaOS**.

---

## 1. Context Provider Query & Implementation Protocol

AI agents interacting with Context Providers must adhere to the following rules:

1. **Non-Blocking Telemetry Queries**:
   - Query context snapshots asynchronously using non-blocking read-locks or lock-free RCU data structures.

2. **Secret Redaction**:
   - Automatically redact credentials, private key material, and user authentication tokens prior to passing context data to prompt injection pipelines.

3. **Priority Hierarchy**:
   - Prioritize security, process, and storage context over ephemeral UI state during context window trimming.

4. **Capability Gating**:
   - Context provider registration requires `CAP_CONTEXT_PROVIDER_REGISTER` capability tokens.

---

## 2. Verification Protocol

- Verify context provider implementations by running `./run_sigma_tests.sh` and `tests/stress_and_fuzz_tests.rs`.

---

*Maintained by the SigmaOS AI Agent & Core Architecture Committee.*
