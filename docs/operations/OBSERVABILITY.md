# SigmaOS Observability & Diagnostics Policy

## Diagnostic Logging Standard

Kernel and system services emit structured log records adhering to the JSON diagnostic schema:

```json
{
  "timestamp": "2026-10-14T02:30:00Z",
  "subsystem": "kernel::mm::paging",
  "level": "ERROR",
  "message": "Page fault at non-canonical address",
  "context": {
    "cr2": "0xDEADBEEF00000000",
    "rip": "0xFFFFFFFF80100020",
    "pid": 42
  }
}
```

---

## Diagnostics Collector

System diagnostic reports are compiled using `scripts/diagnostics/collect_system_report.sh`.

### Report Sections
1. **Boot Initialization Ledger**: Chronological status of registered boot components.
2. **Panic Diagnostics**: Register context (`RAX`, `RBX`, `RIP`, `RSP`, `CR2`, `CR3`) and kernel backtrace.
3. **Subsystem Health Status**: Memory allocation limits, VFS fd usage, and process lifecycle counters.
