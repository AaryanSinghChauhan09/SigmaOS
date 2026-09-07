# SigmaOS AI Agent Security Guidelines

## 1. Overview
SigmaOS incorporates autonomous and interactive AI agents (such as `AgentAutomationEngine`, `LocalLlmWrapper`, `AdaptiveUxAgent`, and `AiCodeAssistant`) into the shell and userland environment. To maintain operating system integrity, user privacy, and system availability, all AI agents operating within SigmaOS must adhere to strict security constraints enforced by the kernel and userland security frameworks.

## 2. Core Security Guarantees for AI Agents

### 2.1 Principle of Least Privilege & Capability Gating
All AI agent tasks execute under strict capability bounds (`CapabilityToken`). AI agents do not inherit root or kernel privileges by default.
- **Process Isolation**: Every agent process operates within a dedicated container / sandbox (`SigmaSandbox`).
- **Capability Tokens**: Agents must request explicit, fine-grained capability tokens (`CapabilityToken::ProcessControl`, `CapabilityToken::FileRead`, `CapabilityToken::NetworkTcp`, etc.). Ungranted capabilities result in immediate access rejection (`EPERM`).

### 2.2 Sandboxing via OpenBSD Pledge & Unveil
Agents must drop unneeded kernel syscalls and restrict filesystem access paths before executing arbitrary code or handling external inputs:
- **Pledge Promises**: Agents must issue `pledge("stdio rpath wpath cpath inet")` to restrict syscall access. Execution promises (`execpromises`) must be omitted unless explicitly authorized for build/compilation agents.
- **Unveil Filesystem Rules**: Agents must unveil only required directory paths (e.g., `unveil("/tmp", "rwc")`, `unveil("/usr/share", "r")`). Accessing un-unveiled paths triggers a kernel `SIGABRT` / `EACCES`.

### 2.3 Input Validation & Sanitization
All inputs passed to AI agents (such as user prompts, CLI commands, network packets, or file paths) must be validated via the `InputValidator`:
- **Path Traversal Protection**: Paths containing `..`, null bytes (`\0`), or invalid UTF-8 sequences are rejected.
- **Command Injection Safeguards**: Shell commands generated or parsed by AI agents must pass through the `RedirectionEngine` and AST parser rather than raw subshell invocations.

### 2.4 AI Model & Memory Hardening
- **Zero-Copy Tensor Allocations**: Tensor memory allocated for local LLM execution (`AiTensorMemoryManager`) uses UMA / TTM page-pinning and hardened guard pages to prevent out-of-bounds memory reads or buffer overflow vulnerabilities.
- **Model Checksum & Attestation**: GGUF weights and WASM agent modules must be verified via Dilithium-5 / Ed25519 PQC signatures (`SigpkgSpec` and `sigma attest`) prior to execution.

### 2.5 Audit Logging & Telemetry
Every automated action performed by an AI agent is recorded in the immutable structured journal (`StructuredLogEntry` in `src/logging/structured_logging.rs`).
- **Key Fields Recorded**: Agent PID, timestamp, requested capability, target file/network resource, and outcome status.
- **Log Rotation & Remote Forwarding**: Journal entries are signed and forwarded over RFC 5424 Syslog / TLS to prevent anti-forensic tampering.

---
*Maintained by the SigmaOS Security & SIG-Security Steering Committee.*
