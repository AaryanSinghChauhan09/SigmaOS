# Diagnostics Guide

This guide helps developers understand compilation errors, known blockers, and how to resolve them.

---

## Common Compilation Errors

### E0282 — Type annotation needed
```
error[E0282]: type annotations needed
  --> src/klib/hashmap.rs:42:9
   |
42 |     let mut v = Vec::new();
```
**Fix:** Add explicit type: `let mut v: Vec<u32> = Vec::new();`

### E0277 — Trait not implemented for HashMap keys
```
error[E0277]: the trait bound `MyStruct: Hash` is not satisfied
```
**Fix:** Either derive `#[derive(Hash, PartialEq, Eq)]` on `MyStruct`, or use `klib`'s simpler map with a different key type.

### E0599 — Method not found on enum
```
error[E0599]: no method named `as_str` found for enum `SomeEnum`
```
**Fix:** Implement the method yourself or match explicitly.

### E0512 — Transmute size mismatch
```
error[E0512]: cannot transmute between `u32` and `u64`
```
**Fix:** Never transmute between different sizes. Use explicit `as` casts or `from`/`into`.

---

## Code Scanning Alerts

### `rust/access-invalid-pointer` (bootloader)
**File:** `bootloader/sigma_boot_efi.rs`
**Lines:** ~847–1226 (multiple UEFI raw pointer dereferences)

**Root cause:** UEFI firmware returns `*mut` pointers to table structures. These are dereferenced without bounds checking.

**Remediation plan:**
```rust
// Current (unsafe, flagged):
let table = unsafe { &*efi_table_ptr };

// Target (safe wrapper):
let table = unsafe {
    // SAFETY: UEFI guarantees table pointer is valid during boot services
    efi_table_ptr.as_ref().ok_or(EfiError::NullTable)?
};
```

### `rust/hard-coded-cryptographic-value` (crypto files)
**Files:** `crypto/sigma_key_derive.rs`, `kernel/core/crypto/sigma_luks.rs`, etc.

**Root cause:** Test vectors and example code use literal byte arrays as keys.

**Fix applied:** `src/crypto/kdf.rs` — replaced `b"password"` with `b"sigmaos-password-hash-v1"` domain-separation constant.

**Remaining:** Other files need similar treatment — replace any literal key with:
```rust
// Good: domain-separation label (not a secret)
const KDF_CONTEXT: &[u8] = b"sigmaos-component-purpose-v1";

// Bad: looks like a hardcoded credential
let key = b"secret_key";
```

### `js/xss-through-dom`
**Files:** `zenith.html`, `index.html`, `zenith_desktop.js`

**Fix pattern:**
```javascript
// Bad (XSS risk):
element.innerHTML = userInput;

// Good (safe):
element.textContent = userInput;
// or for trusted HTML:
element.innerHTML = DOMPurify.sanitize(trustedHtml);
```

### `js/prototype-pollution`
**File:** `zenith_desktop/modules/state-manager.js:42`

**Fix pattern:**
```javascript
// Bad (prototype pollution risk):
const config = {};
Object.assign(config, userInput);

// Good:
const config = Object.create(null);  // no prototype chain
Object.assign(config, sanitize(userInput));
```

### `clippy::new_without_default` (mass suppression)
These are suppressed globally with `#![allow(clippy::new_without_default)]` in all module files.

**Proper fix** (when refactoring): implement `Default` for structs with `fn new() -> Self`:
```rust
impl Default for MyStruct {
    fn default() -> Self { Self::new() }
}
```

---

## Build Troubleshooting

### `cargo check` fails with E0463 (`can't find crate for 'std'`)
You're likely building a `#[no_std]` crate without the right target. Try:
```bash
cargo check                           # library (std features)
cargo check --target x86_64-unknown-none  # bare metal
```

### `cargo test` panics in klib tests
klib uses `alloc` crate. Ensure you have `extern crate alloc;` in `lib.rs`.

### QEMU smoke test fails
```bash
python3 scripts/qemu_smoke_test.py --arch x86_64 --verbose
```
Check that `qemu-system-x86_64` is in PATH and at least 512MB RAM is available.

---

## Sequential Verification Workflow

When fixing a compilation blocker:

1. **Isolate**: run `cargo check 2>&1 | grep "^error" | head -5`
2. **Fix one error at a time** — the first error often causes cascading failures
3. **Re-check**: `cargo check 2>&1 | grep "^error" | wc -l` (count should decrease)
4. **Test**: `cargo test --lib 2>&1 | tail -10`
5. **Clippy**: `cargo clippy 2>&1 | grep "^error" | head -10`
6. **Commit**: stage changes, write a descriptive commit message
