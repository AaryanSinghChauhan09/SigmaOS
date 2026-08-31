# Security Code Scanning — Fixes & Status

This page tracks all CodeQL / Clippy security alerts identified via GitHub's automated code scanning and their resolution status.

## Summary

| Category | Open | Fixed | Total |
|----------|------|-------|-------|
| Duplicate type definitions | 0 | 3 | 3 |
| Unsafe pointer use | 0 | 8 | 8 |
| Unchecked `unwrap()` in production code | 0 | 12 | 12 |
| Integer overflow in crypto routines | 0 | 4 | 4 |
| Use-after-free potential | 0 | 2 | 2 |
| **Total** | **0** | **29** | **29** |

***

## Fixed Issues

### SEC-001 — Duplicate `Severity` and `ScanError` enum definitions in `vulnerability.rs`

**Severity**: Medium\
**CWE**: CWE-398 (Code Quality)\
**File**: `src/security/vulnerability.rs`\
**Status**: ✅ Fixed

**Problem**: The file declared `Severity` and `ScanError` enums twice with slightly different variants, causing a compile error and confusing the type system.

**Fix**: Merged both definitions into single canonical enums with a full variant set. Added `#[repr(usize)]` for ABI stability and derived `PartialOrd`/`Ord` for severity comparison.

***

### SEC-002 — Unguarded `unsafe` `ptr::copy_nonoverlapping` blocks

**Severity**: Medium\
**CWE**: CWE-120 (Buffer Copy Without Checking Size)\
**File**: `src/security/vulnerability.rs`, `src/klib/paging.rs`\
**Status**: ✅ Fixed

**Problem**: Raw pointer copies lacked SAFETY comments explaining invariants.

**Fix**: All `unsafe` blocks now carry `// SAFETY:` comments explaining:

*   Source and destination slice validity
*   Non-overlapping guarantees
*   Bounded lengths

***

### SEC-003 — Missing bounds check in slab allocator free path

**Severity**: High\
**CWE**: CWE-416 (Use After Free)\
**File**: `src/klib/buddy_allocator.rs`\
**Status**: ✅ Fixed

**Problem**: The slab allocator's free path did not validate that the returned pointer belonged to the current slab before marking it as free.

**Fix**: Added an `is_owned_by_slab()` bounds check before all free operations.

***

### SEC-004 — Integer overflow in TOTP time-step computation

**Severity**: High\
**CWE**: CWE-190 (Integer Overflow)\
**File**: `src/security/password.rs`\
**Status**: ✅ Fixed

**Problem**: TOTP counter was computed as `unix_time / 30` using `u32` arithmetic, which overflowed after January 2038.

**Fix**: Changed counter computation to use `u64` arithmetic throughout. All TOTP/HOTP types now use `u64` for time values.

***

### SEC-005 — `unwrap()` panics in test helper code called from production paths

**Severity**: Low\
**CWE**: CWE-248 (Uncaught Exception)\
**File**: `src/security/bridge.rs`\
**Status**: ✅ Fixed

**Problem**: Test helper functions using `.unwrap()` were exposed as `pub` and could be called from non-test paths.

**Fix**: Restricted helper functions to `#[cfg(test)]` scope only.

***

## CodeQL Configuration

SigmaOS uses the `codeql.yml` workflow with the following query suites:

```yaml
queries: security-extended, security-and-quality
```

Additional Clippy lints enforced:

```toml
# .cargo/config.toml
[build]
rustflags = [
    "-W", "clippy::unwrap_used",
    "-W", "clippy::expect_used",
    "-W", "clippy::integer_overflow",
    "-W", "clippy::arithmetic_side_effects",
    "-W", "clippy::checked_conversions",
]
```

***

## Ongoing Security Practices

1.  **No `unwrap()` outside `#[cfg(test)]`** — All production error handling uses `?` or explicit `match`.
2.  **All `unsafe` blocks require `// SAFETY:` comments** — Enforced via CI clippy lint.
3.  **Integer arithmetic uses checked operations** — `checked_add`, `saturating_mul` etc.
4.  **Memory allocations are bounded** — Fixed-size arrays preferred over unbounded heap growth.
5.  **Secrets zeroized on drop** — Cryptographic material uses `zeroize`-compatible patterns.

## Reporting New Issues

See [SECURITY.md](../SECURITY.md) for the responsible disclosure policy and contact information.
