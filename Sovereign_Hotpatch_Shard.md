# Sovereign Hotpatch Shard

**Parity:** kpatch · ksplice · Linux `livepatch` · Oracle Ksplice  
**Location:** `kernel/modules/system/SovereignHotpatchShard.c`  
**Standard:** Zenith Industrial Sovereignty v1.0

---

## Overview

The Sovereign Hotpatch Shard provides native, zero-dependency live kernel patching for SigmaOS. It absorbs the defining USPs of `kpatch`, `ksplice`, and Linux `livepatch` by enabling atomic 5-byte JMP trampoline installation at any target silicon function address — achieving zero downtime and zero reboot for any kernel update.

---

## Architecture

```
Patch Matrix (up to 16 concurrent patches)
  ├── PENDING   — Loaded, awaiting quiesce
  ├── APPLIED   — Trampoline active; calls redirected to patch_func
  ├── REVERTED  — Original instruction bytes restored
  └── FAILED    — Quiesce or install failure

Trampoline Engine
  Step 1: Quiesce silicon missions using target function
  Step 2: Install 5-byte JMP at target_func → patch_func
  Step 3: Resume missions — zero downtime achieved
```

---

## CLI Reference — `sigma-hotpatch`

| Sub-command | Action |
|---|---|
| `sigma-hotpatch load <id> <target_addr> <patch_addr>` | Load and apply a live silicon trampoline patch |
| `sigma-hotpatch revert <id>` | Atomically revert a named patch and restore original code |
| `sigma-hotpatch audit` | Display all patches with addresses, state, and ref-counts |

---

## Design Philosophy

- **Zero Reboot**: Kernel updates are applied while all missions continue running.
- **Ref-Count Safety**: A patch refuses to revert while missions are using the patched path.
- **CVE-First**: Boot patch `CVE-SIGMA-001_null_deref` is pre-seeded at init.

---

## Synchronization State

`GLOBAL MESH ACTIVE` — Synchronized with `AaryanSinghChauhan09/SigmaOS`.
