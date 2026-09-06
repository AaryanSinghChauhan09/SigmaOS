# AI Agent Security Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                               Userland Shell REPL                               |
|                  (Natural Language Prompt / CLI Invocation)                     |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                             Input Validation Layer                              |
|          (Null-byte check, Path Traversal Filter, Safe Arithmetic)             |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       SigmaOS Capability & Policy Gate                          |
|             (Pledge promises, Unveil path rules, CapabilityTokens)              |
+---------------------------------------------------------------------------------+
                                        |
                    +-------------------+-------------------+
                    |                                       |
                    v                                       v
+---------------------------------------+ +---------------------------------------+
|        Local LLM Execution            | |       Agent Automation Engine         |
|   (llama.cpp GGUF / Whisper SST)      | |   (Script execution / REPL hooks)     |
|   - Zero-Copy Tensor Memory (UMA)     | |   - Cgroups v2 resource quotas        |
|   - Hardened Guard-Page Allocator     | |   - eBPF LSM Syscall Auditing         |
+---------------------------------------+ +---------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                     Structured Journaling & Audit Trail                         |
|           (RFC 5424 Syslog, Dilithium-5 Attestation, Log Rotation)              |
+---------------------------------------------------------------------------------+
```

## Security Controls

1. **Isolation Modes**:
   - **Userland Agent Sandbox**: Executes inside a Linux/FreeBSD namespace container with restricted vNET networking and unmounted host roots.
   - **WASM Hostcall Runtime**: Agent modules compiled to WebAssembly execute inside `sigma_cli` fast-paths bounded by WASM linear memory limits.

2. **Threat Model & Mitigation**:
   - *Prompt Injection / Malicious Command Generation*: Filtered via `InputValidator` and executed exclusively through parsed AST commands rather than raw string evaluation.
   - *Resource Exhaustion / DoS*: Limited by `AiComputeScheduler` with FreeBSD `SCHED_ULE` CPU priority classes and memory quotas.
   - *Model Tampering / Poisoning*: GGUF weights validated with SHA256 checksums and PQC signatures before model loading.

3. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
