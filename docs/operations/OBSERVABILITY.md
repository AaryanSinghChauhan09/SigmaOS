# System Observability & Diagnostic Framework

## Architecture
SigmaOS implements structured logging, kernel panic reports, and automated system report collection.

---

## Logging & Panic Diagnostics

### Structured Log Format
Kernel logs use JSON/KMSG key-value structured formats:
```json
{
  "timestamp_us": 1240500,
  "level": "INFO",
  "subsystem": "scheduler",
  "message": "Process 42 allocated vruntime slice 10ms",
  "cpu_id": 0
}
```

### Panic Report Format
When a kernel panic occurs, `src/diagnostics/kernel_report.rs` captures register state and serializes diagnostic JSON:
```json
{
  "panic_reason": "Page Fault in Ring 0",
  "faulting_address": "0x00007FFF80000010",
  "instruction_pointer": "0xFFFFFFFF80102030",
  "registers": {
    "rax": "0x0000000000000000",
    "rbx": "0xFFFFFFFF80200000",
    "rsp": "0xFFFFFFFF80401F00"
  },
  "stack_trace": [
    "0xFFFFFFFF80102030 (vfs_read+0x40)",
    "0xFFFFFFFF80105100 (sys_read+0x20)"
  ]
}
```

---

## Diagnostics Collection Tool
Run `scripts/diagnostics/collect_system_report.sh` to produce a full system health artifact.
