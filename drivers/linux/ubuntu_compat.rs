// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: UBUNTU COMPATIBILITY LAYER (Rust, no_std)
//! =========================================================================
//!
//! Replaces: drivers/linux/ubuntu_compat.cpp
//! Language: Rust  #![no_std]  #![no_builtins]
//!
//! Thin ABI bridge that exposes SigmaOS HAL calls through Linux-compatible
//! `extern "C"` symbols. The Ubuntu build target links against this shim
//! so that SigmaOS can boot under a Linux/Ubuntu kernel module loader.
//!
//! ZERO standard library. ZERO predefined functions. ZERO external crates.
//!
//! Selected at build time with: TARGET_OS=ubuntu
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
}

// ── Primitive types ───────────────────────────────────────────────────────
type U8  = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ═══════════════════════════════════════════════════════════════════════════
// § 1. Linux kernel ABI constants (from linux/errno.h, redefined manually)
//      ZERO #include. ZERO libc.
// ═══════════════════════════════════════════════════════════════════════════

const LINUX_OK         : I32 =  0;
const LINUX_EINVAL     : I32 = -22;  // Invalid argument
const LINUX_ENOMEM     : I32 = -12;  // Out of memory
const LINUX_ENODEV     : I32 = -19;  // No such device
const LINUX_EBUSY      : I32 = -16;  // Device or resource busy
const LINUX_ETIMEDOUT  : I32 = -110; // Connection timed out

// ═══════════════════════════════════════════════════════════════════════════
// § 2. Compatibility layer state
// ═══════════════════════════════════════════════════════════════════════════

/// Maximum number of wrapped drivers the compat layer tracks.
const MAX_WRAPPED_DRIVERS: usize = 32;

/// Record of a driver registered through the compat layer.
#[repr(C)]
struct WrappedDriver {
    /// Static driver name (null-terminated ASCII pointer).
    name     : *const U8,
    /// PCI vendor ID.
    vendor_id: U32,
    /// PCI device ID.
    device_id: U32,
    /// MMIO base address.
    mmio_base: U64,
    /// Whether this slot is occupied.
    active   : bool,
}

// Safety: name points to string literals ('static).
unsafe impl Send for WrappedDriver {}
unsafe impl Sync for WrappedDriver {}

impl WrappedDriver {
    const fn empty() -> Self {
        WrappedDriver {
            name     : b"\0".as_ptr(),
            vendor_id: 0,
            device_id: 0,
            mmio_base: 0,
            active   : false,
        }
    }
}

/// Ubuntu compatibility layer.
pub struct UbuntuCompatLayer {
    drivers      : [WrappedDriver; MAX_WRAPPED_DRIVERS],
    driver_count : usize,
    initialized  : bool,
}

impl UbuntuCompatLayer {
    pub const fn new() -> Self {
        macro_rules! empty_arr {
            () => { [
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
                WrappedDriver::empty(), WrappedDriver::empty(),
            ] }
        }
        UbuntuCompatLayer {
            drivers     : empty_arr!(),
            driver_count: 0,
            initialized : false,
        }
    }

    /// Initialise the Ubuntu compatibility layer.
    /// This sets up the shim table that translates Linux module_init
    /// calls into SigmaOS HAL driver registrations.
    pub unsafe fn init(&mut self) -> I32 {
        if self.initialized { return LINUX_EBUSY; }
        self.initialized = true;
        LINUX_OK
    }

    /// Register a Linux-style driver into the compat shim.
    pub unsafe fn register_driver(
        &mut self,
        name     : *const U8,
        vendor_id: U32,
        device_id: U32,
        mmio_base: U64,
    ) -> I32 {
        if !self.initialized { return LINUX_ENODEV; }
        if self.driver_count >= MAX_WRAPPED_DRIVERS { return LINUX_ENOMEM; }
        if name.is_null() { return LINUX_EINVAL; }

        let slot = &mut self.drivers[self.driver_count];
        slot.name      = name;
        slot.vendor_id = vendor_id;
        slot.device_id = device_id;
        slot.mmio_base = mmio_base;
        slot.active    = true;

        self.driver_count += 1;
        LINUX_OK
    }

    /// Unregister a driver by vendor/device ID pair.
    pub unsafe fn unregister_driver(&mut self, vendor_id: U32, device_id: U32) -> I32 {
        if !self.initialized { return LINUX_ENODEV; }

        let mut i: usize = 0;
        while i < self.driver_count {
            if self.drivers[i].vendor_id == vendor_id
                && self.drivers[i].device_id == device_id
                && self.drivers[i].active
            {
                self.drivers[i].active = false;
                return LINUX_OK;
            }
            i += 1;
        }
        LINUX_ENODEV
    }

    /// Return number of active wrapped drivers.
    pub fn active_count(&self) -> U32 {
        let mut count: U32 = 0;
        let mut i: usize = 0;
        while i < self.driver_count {
            if self.drivers[i].active { count += 1; }
            i += 1;
        }
        count
    }
}

// ── Global singleton ──────────────────────────────────────────────────────
static mut G_COMPAT: UbuntuCompatLayer = UbuntuCompatLayer::new();

// ── C bridge (Linux kernel module ABI) ────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn ubuntu_compat_init() -> I32 {
    G_COMPAT.init()
}

#[no_mangle]
pub unsafe extern "C" fn ubuntu_compat_register(
    name: *const U8, vendor_id: U32, device_id: U32, mmio_base: U64,
) -> I32 {
    G_COMPAT.register_driver(name, vendor_id, device_id, mmio_base)
}

#[no_mangle]
pub unsafe extern "C" fn ubuntu_compat_unregister(vendor_id: U32, device_id: U32) -> I32 {
    G_COMPAT.unregister_driver(vendor_id, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn ubuntu_compat_active_count() -> U32 {
    G_COMPAT.active_count()
}
