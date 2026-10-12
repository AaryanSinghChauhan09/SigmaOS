# SigmaOS Observability and Diagnostics Architecture

## Diagnostic System Architecture

SigmaOS provides structured observability across boot-time, kernel runtime, panics, and userspace diagnostics.

---

## Logging & Event Format

Kernel and subsystem logs are emitted in a structured format:

```text
[TIMESTAMP] [SUBSYSTEM] [LEVEL] Message payload (key=value pairs)
```

Example:
```text
[   0.012400] [PAGING] [INFO] Map virtual range 0xffff800000000000 -> 0x00000000 flags=R|W
```

---

## Panic Diagnostic Format

When an unrecoverable kernel error or invariant violation occurs, the diagnostic panic reporter prints:

1. Subsystem name & location string.
2. Register context dumps (RAX, RBX, RCX, RDX, RSP, RIP, CR2 page fault address).
3. Backtrace frames.
4. Greppable diagnostic string: `SIGMA_PANIC_DIAGNOSTIC_BEGIN` ... `SIGMA_PANIC_DIAGNOSTIC_END`.

---

## Diagnostics Collection Tool

System logs and diagnostic reports can be collected via `scripts/diagnostics/collect_system_report.sh`, producing a single JSON/Tar diagnostic bundle for failure analysis.
