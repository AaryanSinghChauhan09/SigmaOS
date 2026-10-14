# Structured Kernel Diagnostics & Panic Guidelines

## Diagnostic Guidelines & Architecture

Kernel diagnostics in SigmaOS rely on structured, greppable logging and standardized panic reporting formats.

---

## Boot Initialization Diagnostic Format

During boot, each kernel subsystem reports initialization status using structured log events:

```text
[BOOT] <SUBSYSTEM>: <STATUS> (<METRICS>)
```

*Example:*
```text
[BOOT] Paging: ONLINE (pages_mapped: 1048576, levels: 4)
[BOOT] Scheduler: ONLINE (cpus: 4, queue_depth: 64)
[BOOT] VFS: ONLINE (rootfs: ext4, total_inodes: 65536)
```

---

## Structured Panic Report Schema

When an unrecoverable kernel invariant is violated, the panic handler prints a diagnostic report in the following format:

```text
================================================================================
                            SIGMAOS KERNEL PANIC
================================================================================
Component : <Subsystem Name>
Operation : <Operation during failure>
Reason    : <Detailed description of invariant violation>
Location  : <File>:<Line>:<Column>

CRITICAL REGISTERS:
  RAX: 0x0000000000000000  RBX: 0x00007fff10002000
  RCX: 0x0000000000000004  RDX: 0x0000000000000000
  RSI: 0xffff800000102000  RDI: 0x0000000000000000
  RBP: 0xffff800000200500  RSP: 0xffff8000002004d0
  RIP: 0xffff8000001052a0  CR2: 0x0000000000000000
================================================================================
```

---

## Failure Discipline

1. **No Silent Failures:** Error conditions in boot-critical paths MUST produce a diagnostic message.
2. **Panic Discipline:** Panics are restricted to non-recoverable kernel invariant violations (e.g., corrupt page tables, memory allocator state corruption). All user/driver errors must be handled gracefully.
