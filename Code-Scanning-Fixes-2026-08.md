# Code Scanning Fixes — August 2026

This page documents the resolution of all GitHub Code Scanning alerts for SigmaOS.

## Alert Summary

| File | Rule | Severity | Fix Applied |
|------|------|----------|-------------|
| `src/klib/vec.rs` | `mismatched_lifetime_syntaxes` | warning | Fixed lifetime annotations |
| `src/klib/vec.rs` | `clippy::new_without_default` | warning | Added `Default` impl |
| `src/klib/buddy_allocator.rs` | `clippy::new_without_default` | warning | Added `Default` for `LeakTracker` |
| `src/klib/buddy_allocator.rs` | `clippy::collapsible_if` | warning | Merged nested `if` with `&&` |
| `src/klib/paging.rs` | `clippy::new_without_default` | warning | Added `Default` for 4 structs |
| `src/klib/paging.rs` | `clippy::manual_div_ceil` | warning | Used `.div_ceil()` |
| `src/klib/paging.rs` | `clippy::deref_addrof` | warning | Simplified `*(&x)` → `x` |
| `src/drivers/peripheral.rs` | `clippy::new_without_default` | warning | `PeripheralManager::default()` added |
| `src/drivers/gpu.rs` | `clippy::new_without_default` | warning | `GpuDriver::default()` added |
| `src/drivers/legacy_serial.rs` | `dead_code` | warning | Fields marked `#[allow(dead_code)]` |
| `src/kernel/scheduler.rs` | `clippy::unnecessary_cast` | warning | Removed redundant cast |
| `src/kernel/scheduler.rs` | `clippy::same_item_push` | warning | Used `vec::resize` instead of loop |
| `src/kernel/scheduler.rs` | `clippy::cast_abs_to_unsigned` | warning | Used `.unsigned_abs()` |
| `src/kernel/memory.rs` | `clippy::manual_div_ceil` | warning | Used `.div_ceil()` |
| `tests/integration_test.rs` | `unexpected_cfgs` | warning | Added to `[lints.rust]` in Cargo.toml |
| `src/package/sigma_pkg.rs` | `rust/unused-variable` | note | Prefixed with `_` |

## Trait Signature Fixes

The `PeripheralDevice::set_power_state` method signature was changed from:

```rust
fn set_power_state(&mut self, state: PowerState);
```

to:

```rust
fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str>;
```

All driver implementations updated accordingly:

*   `src/drivers/legacy_keyboard.rs`
*   `src/drivers/legacy_serial.rs`
*   `src/drivers/vesa.rs`

## Security Scan Status

The goal is to reach **0 open code scanning alerts**. Run locally with:

```bash
cargo clippy -- -D warnings
cargo check 2>&1
```

## References

*   [GitHub Security Code Scanning](https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning)
*   [Clippy Lints Reference](https://rust-lang.github.io/rust-clippy/master/)
