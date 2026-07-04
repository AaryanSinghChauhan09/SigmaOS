# Transactional Updates (A/B)

SigmaOS uses an atomic A/B slot update system that guarantees safe updates
with automatic rollback if the new system fails to boot.

---

## How It Works

```
Slot A (active):   /sigma/system-a/  ← currently running
Slot B (inactive): /sigma/system-b/  ← update target

Step 1: Download + verify image → write to slot B
Step 2: Compute SHA-256 + verify Dilithium-5 signature
Step 3: Write dm-verity hash to /sigma/system-b/.sigma-verity
Step 4: Update boot pointer: /boot/sigma-active-slot = "B"
Step 5: Reboot
Step 6: Bootloader reads boot pointer → boots slot B
Step 7a: If boot succeeds → slot B becomes permanent active
Step 7b: If boot fails   → bootloader falls back to slot A
```

---

## Command Reference

```bash
# Check current status
sigma-updater status
# Active slot:   A
# Inactive slot: B
# Slot A verity: abc123...
# Slot B verity: not present

# Apply an update
sigma-updater apply /sigma/updates/manifest.toml

# Manual rollback
sigma-updater rollback
# Rolling back from slot B to slot A
# ✓ Rollback complete — reboot required

# Reboot to apply
reboot
```

---

## Update Manifest (`manifest.toml`)

```toml
version    = "15.1.0"
sha256     = "abc123def456..."
sig_path   = "sigmaos-15.1.0.sig"
image_url  = "https://github.com/AaryanSinghChauhan09/SigmaOS/releases/..."
image_path = "sigmaos-15.1.0-x86_64.tar.gz"
size_bytes = 524288000
```

---

## Verification Steps

The updater verifies before writing to the inactive slot:

1. **SHA-256 checksum** — image integrity
2. **Dilithium-5 signature** — authenticity (via `sigma-verify-sig`)
3. **dm-verity hash** — written to `.sigma-verity` for boot-time check

If any step fails, the update is aborted and the active slot is unchanged.

---

## Comparison with Other Systems

| System | Update Method | Rollback |
|--------|--------------|---------|
| Windows | In-place | System Restore |
| Ubuntu | `apt upgrade` (in-place) | Manual |
| Fedora | rpm-ostree A/B | Automatic |
| **SigmaOS** | **Atomic A/B + verity** | **Automatic** |
| Chrome OS | Automatic A/B | Automatic |
| Talos Linux | Immutable A/B | Automatic |

---

## Source

`sigmad/updater/main.rs` — Rust std, ~280 lines. Hand-rolled SHA-256, no external crates.

*See also: [Reproducible Builds](../docs/REPRODUCIBLE_BUILD.md) · [sigpkg Spec](../docs/SIGPKG_SPEC.md)*
