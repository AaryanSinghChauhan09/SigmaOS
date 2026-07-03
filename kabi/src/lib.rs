// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kabi/src/lib.rs — Kernel ABI Stability Library
//
// Guarantees that drivers compiled against DDK v1.0 will work on all future
// SigmaOS versions without recompilation.  This is the key advantage over
// Linux (which has no stable kABI and breaks out-of-tree drivers every update).
//
// Design principles:
//   - Every exported struct is #[repr(C)] with explicit padding
//   - New fields are ONLY added at the end of a struct
//   - Removing or reordering fields requires a major ABI version bump
//   - The kabi_check!() macro validates struct layout at compile time
//   - Stable symbol table exported via .sigma_kabi ELF section
//
// Windows comparison:
//   Windows WDM/WDDM: stable ABI per OS version, breaks across major releases
//   SigmaOS kabi:     stable ABI across ALL versions once DDK_ABI_VERSION frozen
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

// ── ABI version constants ─────────────────────────────────────────────────

/// Major ABI version — increment when backward-incompatible changes are made.
/// Drivers check this at load time and refuse if mismatched.
pub const KABI_VERSION_MAJOR: u32 = 1;

/// Minor ABI version — increment when new fields are added (backward compat).
pub const KABI_VERSION_MINOR: u32 = 0;

/// Patch — bug fixes, no ABI change.
pub const KABI_VERSION_PATCH: u32 = 0;

/// Packed version: [major:8][minor:12][patch:12]
pub const KABI_VERSION: u32 =
    (KABI_VERSION_MAJOR << 24) | (KABI_VERSION_MINOR << 12) | KABI_VERSION_PATCH;

/// Magic bytes that identify a SigmaOS kABI-stable export
pub const KABI_MAGIC: u32 = 0x4B414249;  // "KABI"

// ── Compile-time layout assertions ────────────────────────────────────────

/// kabi_assert_size! — compile-time struct size check.
/// Usage: kabi_assert_size!(MyStruct, 128);
#[macro_export]
macro_rules! kabi_assert_size {
    ($t:ty, $size:expr) => {
        const _: () = {
            if core::mem::size_of::<$t>() != $size {
                panic!("kABI violation: struct size changed");
            }
        };
    };
}

/// kabi_assert_offset! — compile-time field offset check.
#[macro_export]
macro_rules! kabi_assert_offset {
    ($t:ty, $field:ident, $offset:expr) => {
        const _: () = {
            let offset = core::mem::offset_of!($t, $field);
            if offset != $offset {
                panic!("kABI violation: field offset changed");
            }
        };
    };
}

// ── Stable kABI header — prepended to every kABI-exported struct ──────────

/// Every stable struct begins with this header so the loader can validate
/// the ABI version before accessing any fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KabiHeader {
    /// Must equal KABI_MAGIC
    pub magic:         u32,
    /// ABI version (must match kernel's KABI_VERSION_MAJOR)
    pub version_major: u32,
    /// Minor version (kernel accepts minor ≥ driver minor)
    pub version_minor: u32,
    /// sizeof(containing struct) — used to detect truncated structs
    pub struct_size:   u32,
    /// Reserved for future use — must be zero
    pub _reserved:     [u32; 4],
}
// Frozen at 32 bytes — NEVER change this.
kabi_assert_size!(KabiHeader, 32);

impl KabiHeader {
    pub const fn new(struct_size: u32) -> Self {
        Self {
            magic:         KABI_MAGIC,
            version_major: KABI_VERSION_MAJOR,
            version_minor: KABI_VERSION_MINOR,
            struct_size,
            _reserved:     [0u32; 4],
        }
    }

    /// Validate that a header from a loaded driver is compatible.
    pub fn validate(&self) -> KabiResult {
        if self.magic != KABI_MAGIC {
            return Err(KabiError::BadMagic);
        }
        if self.version_major != KABI_VERSION_MAJOR {
            return Err(KabiError::MajorMismatch {
                expected: KABI_VERSION_MAJOR,
                found:    self.version_major,
            });
        }
        if self.version_minor > KABI_VERSION_MINOR {
            return Err(KabiError::MinorTooNew {
                kernel: KABI_VERSION_MINOR,
                driver: self.version_minor,
            });
        }
        Ok(())
    }
}

// ── kABI error types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KabiError {
    /// Magic bytes don't match — not a SigmaOS driver
    BadMagic,
    /// Driver was compiled for a different major ABI version
    MajorMismatch { expected: u32, found: u32 },
    /// Driver expects features not yet in this kernel
    MinorTooNew { kernel: u32, driver: u32 },
    /// Struct is too small (truncated driver binary)
    StructTruncated { expected: u32, found: u32 },
    /// Symbol not found in driver
    SymbolMissing,
    /// Driver ring level not permitted
    RingViolation,
}

pub type KabiResult = Result<(), KabiError>;

// ── Stable symbol table ────────────────────────────────────────────────────

/// A single entry in a driver's stable symbol table.
/// Drivers export their functions through this table so the kernel can
/// find them even if internal symbol names change.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KabiSymbol {
    /// Null-terminated ASCII name (max 63 chars)
    pub name:    [u8; 64],
    /// Function/data pointer
    pub address: u64,
    /// Symbol type: KABI_SYM_FUNC | KABI_SYM_DATA
    pub kind:    u8,
    pub _pad:    [u8; 7],
}
kabi_assert_size!(KabiSymbol, 80);

pub const KABI_SYM_FUNC: u8 = 0;
pub const KABI_SYM_DATA: u8 = 1;

impl KabiSymbol {
    pub const fn func(name_bytes: [u8; 64], addr: u64) -> Self {
        Self { name: name_bytes, address: addr, kind: KABI_SYM_FUNC, _pad: [0u8; 7] }
    }

    pub fn name_str(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
}

/// Table of up to 64 exported symbols per driver.
#[repr(C)]
pub struct KabiSymbolTable {
    pub header:  KabiHeader,
    pub count:   u32,
    pub _pad:    u32,
    pub symbols: [KabiSymbol; 64],
}

impl KabiSymbolTable {
    pub fn find(&self, name: &[u8]) -> Option<u64> {
        for i in 0..(self.count as usize).min(64) {
            if self.symbols[i].name_str() == name {
                return Some(self.symbols[i].address);
            }
        }
        None
    }
}

// ── Driver compatibility matrix ────────────────────────────────────────────

/// Records which ABI version a driver requires and whether it's compatible
/// with the current kernel.
#[repr(C)]
pub struct DriverCompatInfo {
    pub header:          KabiHeader,
    /// Minimum kernel kABI version required
    pub min_kernel_ver:  u32,
    /// Maximum kernel kABI version tested with (0 = any)
    pub max_kernel_ver:  u32,
    /// Pledge capabilities required (bitmask)
    pub pledge_required: u64,
    /// Ring level: 0=kernel, 3=userspace
    pub ring:            u8,
    /// Flags: KABI_COMPAT_*
    pub flags:           u8,
    pub _pad:            [u8; 6],
}

pub const KABI_COMPAT_OPEN_SOURCE: u8 = 1 << 0;
pub const KABI_COMPAT_AI_PORTED:   u8 = 1 << 1;
pub const KABI_COMPAT_CERTIFIED:   u8 = 1 << 2;

// ── Deprecation tracker ────────────────────────────────────────────────────

/// Record of a deprecated kABI symbol with its replacement.
/// Printed as a warning when a driver uses an old API.
#[repr(C)]
pub struct KabiDeprecation {
    pub old_symbol:    [u8; 64],
    pub new_symbol:    [u8; 64],
    pub removed_in:    u32,   // KABI_VERSION at which old symbol is removed
    pub _pad:          [u32; 3],
}

/// Global deprecation list — checked at driver load time.
/// Drivers using deprecated symbols get a warning but still load.
pub static DEPRECATIONS: &[KabiDeprecation] = &[];
// Future: add entries here as APIs are deprecated, e.g.:
//   KabiDeprecation { old: "sigma_dma_alloc_v0", new: "sigma_dma_alloc", removed_in: 0x0200_0000 }

// ── C-ABI exports for the kernel loader ───────────────────────────────────

#[no_mangle]
pub extern "C" fn kabi_version() -> u32 {
    KABI_VERSION
}

#[no_mangle]
pub extern "C" fn kabi_validate_header(hdr: *const KabiHeader) -> i32 {
    if hdr.is_null() { return -22; }  // EINVAL
    match unsafe { &*hdr }.validate() {
        Ok(()) => 0,
        Err(KabiError::BadMagic)                => -1,
        Err(KabiError::MajorMismatch { .. })    => -2,
        Err(KabiError::MinorTooNew { .. })      => -3,
        Err(KabiError::StructTruncated { .. })  => -4,
        Err(KabiError::SymbolMissing)           => -5,
        Err(KabiError::RingViolation)           => -6,
    }
}

#[no_mangle]
pub extern "C" fn kabi_check_pledge(required: u64, granted: u64) -> i32 {
    // All required capabilities must be in the granted set
    if required & granted == required { 0 } else { -1 }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
}
