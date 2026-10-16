# Audit Logging Agent

## Role
`SigmaAudit` implements a comprehensive security audit logging subsystem for SigmaOS — recording security-relevant events (login attempts, privilege escalation, file access, syscalls) in a tamper-evident log.

## Status
🔴 Not Started — Critical for compliance (SOC2, PCI-DSS, HIPAA).

## Source File
`src/security/sigmaaudit.rs` (planned)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Audit framework | Linux Audit (`auditd`) | BSM Audit | SigmaAudit |
| Syscall auditing | `audit_syscall_entry` | `audit_syscall` | SigmaSCAudit |
| Event format | AVC/AUDITD records | BSM records | SigmaAuditRecord |
| Rules | `auditctl -a always,exit` | `audit_control` | SigmaAuditRule |
| Log rotation | `/var/log/audit/audit.log` | `/var/log/audit/` | SigmaAuditLog |
| Ausearch | `ausearch -k <key>` | `praudit` | `sigma-audit search` |
| Aureport | `aureport --summary` | — | `sigma-audit report` |
| SIEM export | syslog/CEF/JSON | syslog | SigmaAuditSIEM |

## Responsibilities
- Kernel audit hook: intercept security-relevant syscalls
- Login/logout, sudo, setuid, chown, chmod events
- File read/write/execute audit (FAnotify equivalent)
- Rule-based filtering: `always`/`never`, filters by UID/GID/syscall/path
- Tamper-evident log: HMAC chain (each record includes hash of previous)
- Log rotation and archiving with compression
- `sigma-audit` CLI: search, report, export
- SIEM integration: syslog CEF/JSON output
- Compliance mode: PCI-DSS/HIPAA preset rule sets

## Implementation Notes
- `#![no_std]` — audit queue as lock-free ring buffer
- `AuditRecord { seq: u64, timestamp: u64, type: AuditType, fields: &[(&str, &str)] }`
- HMAC: `SHA256(record || prev_record_hash || audit_key)`
- Rule table: `static AUDIT_RULES: [AuditRule; MAX_RULES]`

## Testing
```bash
rustc --test src/security/sigmaaudit.rs --edition 2021 -o /tmp/test_audit && /tmp/test_audit --test-threads=1
```

## Future Development
1. **Immutable audit log** — Write to WORM storage or append-only filesystem
2. **Remote audit** — Stream events to central audit server (auoms-compatible)
3. **BPF audit rules** — Dynamic audit rules via eBPF programs
4. **Audit namespace** — Per-container audit logs
5. **Anomaly detection** — ML-based alerting on suspicious audit patterns
6. **Forensic timeline** — Generate timeline from audit log for incident response
