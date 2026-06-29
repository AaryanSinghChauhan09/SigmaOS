// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER — Core Trait Definitions
//! =========================================================================
//!
//! Language: Rust  #![no_std]  #![no_builtins]  #![no_main]
//!
//! ZERO standard library. ZERO predefined functions. ZERO external crates.
//! All types, results, and interfaces defined from first principles using
//! only raw Rust language primitives.
//!
//! Compile with:
//!   rustc --edition 2021 --crate-type staticlib \
//!         --target x86_64-unknown-none          \
//!         -C opt-level=2 -C panic=abort          \
//!         drivers/hal/mod.rs -o build/libhal.a
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]
#![allow(unused_variables)]

// ── Panic handler (required by no_std) ────────────────────────────────────
// Implemented as a bare-metal infinite halt — no core::fmt, no unwind.
#[panic_handler]
fn sigma_panic(_info: &core::panic::PanicInfo) -> ! {
    // Disable interrupts and halt the CPU forever.
    // Inline assembly — no library call.
    loop {
        unsafe {
            core::arch::asm!("cli; hlt", options(nomem, nostack));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// § 1. Primitive integer types
//      Defined entirely from first principles — NOT imported from core::
// ═══════════════════════════════════════════════════════════════════════════

/// Unsigned 8-bit sovereign type alias.
pub type SigmaU8  = u8;
/// Unsigned 16-bit sovereign type alias.
pub type SigmaU16 = u16;
/// Unsigned 32-bit sovereign type alias.
pub type SigmaU32 = u32;
/// Unsigned 64-bit sovereign type alias.
pub type SigmaU64 = u64;
/// Signed 32-bit sovereign type alias.
pub type SigmaI32 = i32;
/// Physical memory address (64-bit bare-metal pointer).
pub type PhysAddr = u64;
/// MMIO register offset.
pub type MmioOffset = u32;

// ═══════════════════════════════════════════════════════════════════════════
// § 2. HAL Result type — no std::result, no core::result
// ═══════════════════════════════════════════════════════════════════════════

/// Sovereign HAL operation result.
/// Defined without core::result — raw enum with explicit discriminants.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum HalResult {
    /// Operation succeeded.
    Ok         = 0,
    /// Driver not initialised before operation.
    NotReady   = 1,
    /// Null or misaligned MMIO base address.
    BadAddress = 2,
    /// Zero-length or invalid parameter.
    BadParam   = 3,
    /// Hardware did not respond within timeout.
    Timeout    = 4,
    /// Unknown / catch-all error.
    Unknown    = 0xFFFF_FFFF,
}

impl HalResult {
    /// Returns true if this result represents success.
    #[inline(always)]
    pub fn is_ok(self) -> bool {
        // Match on raw discriminant — no core::cmp, no PartialEq derive
        (self as u32) == 0
    }

    /// Convert to a C-compatible i32 (0 = success, negative = error).
    #[inline(always)]
    pub fn to_c_int(self) -> SigmaI32 {
        let v = self as u32;
        if v == 0 { 0 } else { -(v as SigmaI32) }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// § 3. SovereignDriver trait — the unified HAL interface
//      Every driver in every target (sigma / ubuntu / bsd) implements this.
// ═══════════════════════════════════════════════════════════════════════════

/// The core Sovereign Driver trait.
///
/// Rules:
///   - `probe` MUST be safe to call before `init`.
///   - `init` MUST be idempotent (calling twice is a no-op returning `Ok`).
///   - All methods are `unsafe` because they touch raw hardware.
pub trait SovereignDriver {
    /// Probe whether this driver's hardware is present.
    /// Returns `true` if hardware was detected, `false` otherwise.
    unsafe fn probe(&self) -> bool;

    /// Initialise the driver and hardware.
    /// Must be called exactly once before any other method.
    unsafe fn init(&mut self, mmio_base: PhysAddr) -> HalResult;

    /// Shut down the hardware and release all resources.
    unsafe fn shutdown(&mut self) -> HalResult;

    /// Return a static human-readable driver name.
    /// Must be a null-terminated ASCII literal — no heap allocation.
    fn name(&self) -> *const u8;
}

// ═══════════════════════════════════════════════════════════════════════════
// § 4. Driver registry — fixed-size, no Vec, no alloc
// ═══════════════════════════════════════════════════════════════════════════

/// Maximum number of drivers that can be registered simultaneously.
pub const MAX_DRIVERS: usize = 64;

/// A single entry in the driver registry.
pub struct DriverEntry {
    /// Vendor ID (PCI / USB / virtual).
    pub vendor_id: SigmaU32,
    /// Device ID.
    pub device_id: SigmaU32,
    /// MMIO base address assigned at probe time.
    pub mmio_base: PhysAddr,
    /// Static driver name (null-terminated ASCII).
    pub name: *const u8,
    /// Whether this slot is occupied.
    pub occupied: bool,
}

// Safety: DriverEntry contains raw pointers to string literals only.
// These are 'static and never mutated — safe to send across cores.
unsafe impl Send for DriverEntry {}
unsafe impl Sync for DriverEntry {}

impl DriverEntry {
    /// Construct an empty (unoccupied) entry.
    pub const fn empty() -> Self {
        DriverEntry {
            vendor_id: 0,
            device_id: 0,
            mmio_base: 0,
            name: b"\0".as_ptr(),
            occupied: false,
        }
    }
}

/// Global static driver registry — no heap, no Vec.
pub struct DriverRegistry {
    entries: [DriverEntry; MAX_DRIVERS],
    count: usize,
}

impl DriverRegistry {
    /// Construct an empty registry.
    pub const fn new() -> Self {
        // Cannot use array repeat expression with non-Copy struct in const,
        // so we initialise each slot manually via a macro.
        macro_rules! empty64 {
            () => { [
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
                DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(), DriverEntry::empty(),
            ] }
        }
        DriverRegistry { entries: empty64!(), count: 0 }
    }

    /// Register a new driver entry.
    /// Returns `HalResult::Ok` on success, `HalResult::BadParam` if registry full.
    pub fn register(
        &mut self,
        vendor_id: SigmaU32,
        device_id: SigmaU32,
        mmio_base: PhysAddr,
        name: *const u8,
    ) -> HalResult {
        if self.count >= MAX_DRIVERS {
            return HalResult::BadParam;
        }
        let slot = &mut self.entries[self.count];
        slot.vendor_id = vendor_id;
        slot.device_id = device_id;
        slot.mmio_base = mmio_base;
        slot.name = name;
        slot.occupied = true;
        self.count += 1;
        HalResult::Ok
    }

    /// Return the number of registered drivers.
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }
}

// ── Global registry instance (BSS-resident, zero-initialised) ─────────────
pub static mut GLOBAL_REGISTRY: DriverRegistry = DriverRegistry::new();

// ── C bridge ──────────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn hal_register_driver(
    vendor_id: SigmaU32,
    device_id: SigmaU32,
    mmio_base: PhysAddr,
    name: *const u8,
) -> SigmaI32 {
    GLOBAL_REGISTRY
        .register(vendor_id, device_id, mmio_base, name)
        .to_c_int()
}

#[no_mangle]
pub unsafe extern "C" fn hal_driver_count() -> SigmaU32 {
    GLOBAL_REGISTRY.count() as SigmaU32
}
