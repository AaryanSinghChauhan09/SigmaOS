# Live Kernel Patch (kpatch) Agent

| Field | Value |
|---|---|
| **Portal** | Kernel Core |
| **Maintainer Agent** | LIVE_PATCH_KPATCH_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_livepatch_engine.rs` |
| **Language** | Rust (primary), Nim (controller) |

---

## Table of Contents
1. [Overview](#overview)
2. [Technical Specifications](#technical-specifications)
3. [Implementation](#implementation)
4. [Standard Library Authorization](#standard-library-authorization)
5. [Configuration](#configuration)
6. [Diagnostics](#diagnostics)
7. [See Also](#see-also)

---

## 1. Overview
Manages live kernel patching via Linux `livepatch` subsystem, kpatch infrastructure, and `klp_patch` hot-fix loading. Enables applying security patches to a running kernel without rebooting, using `ftrace`-based function redirection.

## 2. Technical Specifications
- Kernel ABI: `CONFIG_LIVEPATCH=y`
- Mechanism: ftrace trampoline → patched function
- Module format: `.ko` with `KLP_MODULE` metadata
- State: enabled/disabled per-patch via `/sys/kernel/livepatch/`
- Consistency model: per-task consistency (stop_machine or tasks_freezer)

## 3. Implementation
Engine: [`sigma_livepatch_engine.rs`](../src/kernel/sigma_livepatch_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized.

## 5. Configuration
```toml
[livepatch]
patch_dir = "/var/lib/sigma/kpatch"
auto_apply = true
consistency_model = "tasks_freezer"
verify_signature = true
```

## 6. Diagnostics
```bash
ls /sys/kernel/livepatch/
cat /sys/kernel/livepatch/*/enabled
dmesg | grep -i "livepatch\|kpatch"
sigma-diag livepatch --status
```

## 7. See Also
- [KERNEL_MODULE_LOADING_AGENT](./KERNEL_MODULE_LOADING_AGENT.md)
- [CRASH_DUMP_CORE_AGENT](./CRASH_DUMP_CORE_AGENT.md)
