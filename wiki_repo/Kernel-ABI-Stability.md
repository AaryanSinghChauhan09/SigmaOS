# Kernel ABI Stability (kabi/)

The `kabi/` library is SigmaOS's answer to Linux's most criticized weakness:
**no stable kernel ABI**. Drivers compiled against DDK v1.0 will load and work
on every future SigmaOS version without recompilation.

---

## The Problem: Linux's Unstable kABI

Linus Torvalds explicitly refuses to stabilize the Linux kernel ABI:

> "We don't promise any stable in-kernel ABI. If you write a kernel module for
> kernel version X, it may or may not work with kernel version X+1."

This means:
- NVIDIA must recompile their driver for every kernel update
- Out-of-tree drivers break regularly
- Vendors don't ship drivers because maintenance cost is too high
- Users get "kernel update broke my Wi-Fi" experiences

## Windows Does It Right (for stability)

Windows WDM/WDDM provides a stable ABI **per OS version**. A WDDM 1.0 driver
(Vista) still loads on Windows 11 via a compatibility shim. The tradeoff is
that Windows carries decades of compatibility baggage.

## SigmaOS kabi: Permanent Stability

```rust
// kabi/src/lib.rs
pub const KABI_VERSION_MAJOR: u32 = 1;  // never changes (frozen)
pub const KABI_VERSION_MINOR: u32 = 0;  // incremented when adding new fields

// Every stable struct starts with this header:
#[repr(C)]
pub struct KabiHeader {
    pub magic:         u32,  // KABI_MAGIC = 0x4B414249 ("KABI")
    pub version_major: u32,  // must match kernel — checked at load time
    pub version_minor: u32,  // driver ≤ kernel minor = compatible
    pub struct_size:   u32,  // sizeof(outer struct) — truncation detection
    pub _reserved:     [u32; 4], // zero today, may be used in future
}
```

Rules that will **never be violated**:
1. `KABI_VERSION_MAJOR` only increments for breaking changes (extremely rare)
2. New fields are added **only at the end** of stable structs
3. The `struct_size` field lets the loader detect and zero-fill unknown trailing fields
4. Stable symbol names are frozen — internal implementation can change, exported names cannot

---

## Compile-Time Layout Verification

```rust
// Verify struct sizes don't accidentally change:
kabi_assert_size!(KabiHeader, 32);         // fails build if size changes
kabi_assert_offset!(KabiHeader, magic, 0); // fails build if field moves
```

These macros run at **compile time** — if any change violates the ABI, the
kernel itself fails to build before it can ship a breaking change.

---

## Stable Symbol Table

Drivers export their functions through a named symbol table, not raw ELF symbols.
This means internal symbol names can be refactored without breaking drivers:

```rust
// Driver exports this table — the kernel looks up functions by name:
pub struct KabiSymbolTable {
    pub header:  KabiHeader,
    pub count:   u32,
    pub symbols: [KabiSymbol; 64],  // name → address mapping
}

// Kernel loads driver and calls:
let probe_addr = table.find(b"sigma_driver_probe");
let init_addr  = table.find(b"sigma_driver_init");
```

---

## Driver Load-Time Validation

When the kernel loads a driver `.so`/`.a`, it calls `kabi_validate_header()`:

```rust
pub fn validate(&self) -> KabiResult {
    if self.magic != KABI_MAGIC {
        return Err(KabiError::BadMagic);     // not a SigmaOS driver
    }
    if self.version_major != KABI_VERSION_MAJOR {
        return Err(KabiError::MajorMismatch); // incompatible major version
    }
    if self.version_minor > KABI_VERSION_MINOR {
        return Err(KabiError::MinorTooNew);  // driver needs newer kernel
    }
    Ok(())
}
```

If validation fails, the driver is **rejected at load time** with a clear error
message — no silent corruption or undefined behavior.

---

## Pledge Capability Check

```rust
// Kernel verifies driver's required capabilities against granted set:
kabi_check_pledge(driver.pledge_required, kernel_grant_for_ring);
// Returns -1 if driver requests more than kernel allows
```

A driver requesting `Cap::All` at ring-3 is **rejected** — ring-3 drivers
can only request the capabilities appropriate for their isolation level.

---

## Versioning Policy

| Scenario | Action |
|----------|--------|
| Bug fix in kernel, no ABI change | Patch bump — all drivers continue working |
| New optional field added to struct | Minor bump — old drivers get zero for new field |
| Field removed or reordered | Major bump — old drivers rejected with clear error |
| Stable symbol renamed | **Never** — old name kept as alias forever |

---

## Comparison

| OS | kABI Stability | Driver Compatibility |
|----|----------------|----------------------|
| Linux | ❌ No stable kABI | Breaks every kernel update |
| Windows | 🔄 Stable per OS version | Breaks across major releases |
| SigmaOS | ✅ Stable forever (v1.0) | DDK v1.0 drivers work on all versions |

---

*Source: `kabi/src/lib.rs` · See also: [Driver Framework](Driver-Framework) · [Driver Development Guide](Driver-Development-Guide)*
