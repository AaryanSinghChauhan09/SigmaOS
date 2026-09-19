# SigmaOS Seccomp-BPF Filter Engine

## Overview

`src/security/sigma_seccomp_bpf.rs` provides a SigmaOS-native syscall sandbox
engine.  Unlike Linux seccomp-BPF (which requires BPF bytecode), SigmaOS rules
are expressed as composable Rust structs, making them auditable and ergonomic.

---

## Architecture

```
Process calls syscall(nr, args…)
          │
          ▼
  FilterEngine::evaluate(nr, args)
          │
  ┌───────┴──────────┐
  │ Filter stack     │  newest filter evaluated first
  │  [filter N]      │
  │  [filter N-1]    │
  │  [filter 0]      │
  └───────┬──────────┘
          │
  Most restrictive action wins
          │
  ┌───────┴────────────────────────────┐
  │ Allow / Log / Trace / Errno / Trap │
  │ / Kill                             │
  └────────────────────────────────────┘
```

---

## Actions

| Action | Severity | Effect |
|--------|----------|--------|
| `Allow` | 0 | Syscall proceeds normally |
| `Log` | 1 | Syscall allowed; entry appended to audit log |
| `Trace` | 2 | Notify attached ptrace supervisor |
| `Errno(n)` | 3 | Return error `n` to caller; syscall not executed |
| `Trap` | 4 | Deliver `SIGSYS` to thread (handler may run) |
| `Kill` | 5 | Terminate thread immediately |

When multiple installed filters give different actions for the same syscall,
the **most restrictive** (highest severity) action wins.

---

## Rules and Filters

### `SeccompRule`

```rust
SeccompRule::new(syscall_nr, action)
    .with_comparator(ArgComparator::DstPort { index: 0, value: 22 })
    .with_comment("block SSH")
```

A rule matches when `syscall_nr` matches AND **all** comparators hold.
An empty comparator list matches any argument combination.

### Argument Comparators

| Comparator | Meaning |
|------------|---------|
| `Equal { index, value }` | `arg[index] == value` |
| `NotEqual { index, value }` | `arg[index] != value` |
| `LessThan { index, value }` | `arg[index] < value` |
| `LessEqual { index, value }` | `arg[index] <= value` |
| `GreaterThan { index, value }` | `arg[index] > value` |
| `GreaterEqual { index, value }` | `arg[index] >= value` |
| `MaskedEqual { index, mask, value }` | `(arg[index] & mask) == value` |

### `SeccompFilter`

```rust
let filter = SeccompFilter::new("my-filter")
    .with_default(SeccompAction::Kill)
    .allow(SYS_READ)
    .allow(SYS_WRITE)
    .allow(SYS_CLOSE)
    .allow(SYS_EXIT)
    .errno(SYS_SOCKET, 1);      // EPERM on socket(2)
```

---

## Filter Engine

```rust
let mut engine = FilterEngine::new();
engine.install_filter(filter);

let action = engine.evaluate(SYS_MMAP, &[0; 6]);
match action {
    SeccompAction::Allow => { /* proceed */ }
    SeccompAction::Kill  => { /* terminate thread */ }
    SeccompAction::Errno(e) => { /* return -e */ }
    _ => {}
}
```

### Filter Stack

Filters are stacked: each `install_filter` call pushes a new filter.  The
engine evaluates all filters and returns the most restrictive action.

This mirrors Linux's multi-filter support (`PR_SET_SECCOMP` can be called
multiple times and each `exec` inherits the parent's filters).

---

## Pledge-Style Policy

`PledgePolicy` builds a `SeccompFilter` from high-level capability flags,
inspired by OpenBSD `pledge(2)`:

```rust
let filter = PledgePolicy::default()
    .stdio()        // read, write, close, exit
    .file_read()    // open, stat
    .network()      // socket, connect, bind, listen, accept
    .build("app-sandbox");

engine.install_filter(filter);
```

### Pledge Capability Map

| Pledge flag | Allowed syscalls |
|-------------|-----------------|
| `stdio` | read, write, close, exit |
| `file_read` | open, stat |
| `file_write` | open (write mode) |
| `network` | socket, connect, bind, listen, accept |
| `exec` | execve |
| `proc` | fork, kill |

---

## Audit Log

Every non-Allow action is appended to an in-memory audit log:

```rust
let log = engine.audit_log();
for entry in log {
    println!("syscall={} action={:?} filter={}", entry.syscall_nr, entry.action, entry.filter_name);
}
engine.clear_audit_log();
```

---

## Comparison

### vs Linux seccomp-BPF

| Feature | Linux seccomp-BPF | SigmaOS seccomp-BPF |
|---------|-------------------|---------------------|
| Rule language | BPF bytecode | Rust structs |
| Compile-time safety | No | Yes (type-checked) |
| Multi-filter stack | Yes | Yes |
| Most-restrictive wins | Yes | Yes |
| Audit log | auditd / `SECCOMP_RET_LOG` | Built-in `Vec<AuditEntry>` |
| Userspace notification | `SECCOMP_RET_USER_NOTIF` | `Trace` action (stub) |
| TSYNC (thread sync) | Yes | Planned |

### vs OpenBSD `pledge(2)`

| Feature | OpenBSD pledge | SigmaOS PledgePolicy |
|---------|---------------|---------------------|
| Granularity | Capability groups | Same groups + per-syscall rules |
| Irreversible once set | Yes | Yes (`uninstall_filter` test-only) |
| `unveil` integration | Yes (separate) | Planned |

---

## Source Location

`src/security/sigma_seccomp_bpf.rs`
