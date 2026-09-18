# SigmaOS AI Agent Auditor Operation Management Guidelines

## 1. Overview
SigmaOS incorporates a comprehensive, tamper-evident audit and forensic operation framework managed by AI auditing agents (such as `AuditorOperationAgent`, `ChainedAuditTrailLedger`, `EbpfLsmAuditor`, `OpenBsdUnveilAuditTool`, and `WindowsCopilotRecallAuditor`). These guidelines define structured audit logging (`StructuredLogEntry`), cryptographic hash chaining, eBPF LSM syscall auditing, activity tracking, forensic compliance, and remote log forwarding for AI agents in SigmaOS.

## 2. Core Auditor Operation Management Principles

### 2.1 Tamper-Evident Cryptographic Audit Ledgers
- **Chained Audit Trail**: System and agent events are recorded in an append-only, cryptographically hash-chained audit ledger (`ChainedAuditTrailLedger` in `src/unimplemented_tools.rs`).
- **Merkle Accumulator Integration**: Every audit entry includes the parent entry's SHA256 digest, creating an immutable Merkle hash chain that prevents retrospective modification or anti-forensic log deletion.

### 2.2 eBPF LSM Syscall Auditing & Telemetry
- **eBPF Syscall Hooking**: `EbpfLsmAuditor` attaches sandboxed eBPF probes to critical kernel system call entry/exit vectors (e.g. `execve`, `pledge`, `unveil`, `socket`, `mount`).
- **Real-Time Anomaly Scoring**: AI auditor agents evaluate execution contexts against security baseline models (`AiAnomalyFirewall`), flagging suspicious privilege escalation attempts immediately.

### 2.3 Structured Journaling & OpenBSD Policy Auditing
- **Key-Value Journal Fields**: Audit records utilize structured key-value pairs (`StructuredLogEntry` in `src/logging/structured_logging.rs`) capturing PID, UID/GID, capability token ID, executable path, syscall arguments, and outcome status.
- **Unveil & Pledge Audit Tool**: `OpenBsdUnveilAuditTool` validates file access requests against `unveil` policy tables, auditing blocked path access attempts (`EACCES`).

### 2.4 Remote Log Forwarding & Log Rotation
- **RFC 5424 Syslog Forwarding**: Audit records are streamed in real time over TLS / TCP to remote central SIEM / Syslog collectors (`RemoteLogForwarder`).
- **Compressed Multi-Generation Rotation**: Local journal files are rotated and compressed using zstd compression (`LogRotationEngine`), preventing disk space exhaustion while preserving forensic history.

---
*Maintained by the SigmaOS Security, Audit & SIG-Security Steering Committee.*
