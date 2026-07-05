# RFC-0002: SigmaOS Kernel Stable ABI Policy

- **RFC number**: 0002

- **Author(s)**: SigmaOS Project

- **Subsystem**: kabi / kernel

- **Status**: Accepted

- **Date proposed**: 2026-07-01

- **Tracking issue**: #kabi

- **Implementation PR**: kabi/src/lib.rs

---

## Summary

Define which interfaces in the SigmaOS kernel are "stable ABI" (never broken
without a major version bump) and which are "internal ABI" (may change between
releases). Provide a mechanical check tool (`sigma-drv abi check`) and a
`kabi/` directory with snapshot headers.

---

## Motivation

### Current behaviour

There is no formal ABI stability promise. Driver authors who target SigmaOS
cannot know if their driver will work after an OS update.

### Desired behaviour

- A documented list of stable-ABI symbols (functions, structs, constants).

- A CI check that detects ABI breakage and fails the build.

- Driver DDK exposes **only** stable-ABI symbols so third-party drivers survive
  OS minor-version updates.

### Non-goals

- Matching Linux's KABI (too complex for v1).

- Stabilising internal kernel-only interfaces (`sigma_slab_alloc`, etc.).

---

## Detailed Design

### Stable ABI surface (v1)

The following categories of symbols are **stable**:

| Category | Examples |
|----------|---------|
| Driver DDK traits | `WifiDriver`, `BlockDriver`, `InputDriver` |
| C-ABI exports used by drivers | `sigma_request_irq`, `sigma_free_irq`, `nic_tx_packet` |
| kabi structs | `KabiVersion`, `DriverInfo`, `IrqDescriptor` |
| Syscall numbers | All values in `syscall_dispatch.rs` (never renumbered) |

### kabi/ directory layout

```
kabi/
  src/
    lib.rs          # Rust stable-ABI types (repr(C), versioned)

    version.rs      # KABI_VERSION constant

  snapshots/
    v15.0.0.json    # Symbol snapshot for v15.0.0

    v15.1.0.json    # Updated on each minor release

  check.py          # CI: compare current headers to snapshot

```

### ABI snapshot format

```json
{
  "version": "15.0.0",
  "symbols": [
    { "name": "sigma_request_irq", "type": "fn(u8, IrqHandler) -> ()", "cabi": true },
    { "name": "WifiDriver", "type": "trait", "methods": ["init","scan","connect"] }
  ]
}
```

### CI enforcement

`kabi/check.py` runs in CI on every PR:

1. Parse current `kabi/src/lib.rs` → current symbol set.

2. Load `kabi/snapshots/<base_version>.json` → baseline.

3. Diff: any **removed** or **changed** symbol = ABI break → CI fails.

4. New symbols = ABI addition → CI passes but appends to snapshot.

---

## Alternatives Considered

### Alternative A: Use Linux's KABI mechanism (Module.symvers)

Too complex; requires full kernel build infrastructure.

### Alternative B: No formal ABI (status quo)

Unacceptable for a driver ecosystem.

---

## Compatibility / ABI Impact

This RFC itself defines the ABI policy; it has no ABI impact.

---

## Implementation Plan

1. ✅ Done: `kabi/src/lib.rs` with `KabiVersion` and `DriverInfo` structs.

2. ✅ Done: `sigma-drv abi check` CLI command.

3. TODO: `kabi/check.py` CI script with snapshot comparison.

4. TODO: Snapshot `v15.0.0.json` generated from current headers.
