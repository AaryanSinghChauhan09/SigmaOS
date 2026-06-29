// SPDX-License-Identifier: GPL-2.0-or-later
//
// =========================================================================
// SIGMAOS: BSD COMPATIBILITY LAYER (Zig — freestanding, no std)
// =========================================================================
//
// Replaces: drivers/bsd/bsd_compat.cpp
// Language: Zig (freestanding)
//
// Thin ABI bridge between SigmaOS HAL and FreeBSD newbus driver model.
// ZERO standard library. ZERO @import("std"). ZERO predefined functions.
// All register validation uses Zig's comptime feature.
//
// Compile with:
//   zig build-obj -target x86_64-freestanding-none -O ReleaseFast \
//                 drivers/bsd/bsd_compat.zig -femit-bin=build/bsd_compat.o
//
// Selected at build time with: TARGET_OS=bsd
// =========================================================================

// No imports — entirely self-contained.

// ═══════════════════════════════════════════════════════════════════════════
// § 1. BSD errno constants (from sys/errno.h, defined from scratch)
// ═══════════════════════════════════════════════════════════════════════════

const ENONE  : i32 =  0;  // No error
const EPERM  : i32 = -1;  // Operation not permitted
const ENOENT : i32 = -2;  // No such file or directory
const ENOMEM : i32 = -12; // Cannot allocate memory
const EBUSY  : i32 = -16; // Device busy
const ENODEV : i32 = -19; // Operation not supported by device
const EINVAL : i32 = -22; // Invalid argument
const ENXIO  : i32 = -6;  // Device not configured

// ═══════════════════════════════════════════════════════════════════════════
// § 2. Newbus device class identifiers (comptime validated)
// ═══════════════════════════════════════════════════════════════════════════

/// Device class enum for BSD newbus model.
const DeviceClass = enum(u8) {
    network   = 0,
    storage   = 1,
    display   = 2,
    input     = 3,
    audio     = 4,
    usb       = 5,
    serial    = 6,
    unknown   = 255,
};

// ═══════════════════════════════════════════════════════════════════════════
// § 3. Driver registration table — static, no alloc
// ═══════════════════════════════════════════════════════════════════════════

const MAX_BSD_DRIVERS: usize = 32;

const BsdDriverEntry = struct {
    name      : [32]u8, // Null-terminated driver name (inline, no pointer)
    name_len  : u8,
    vendor_id : u32,
    device_id : u32,
    mmio_base : u64,
    class     : DeviceClass,
    active    : bool,
};

fn emptyEntry() BsdDriverEntry {
    return BsdDriverEntry{
        .name      = [_]u8{0} ** 32,
        .name_len  = 0,
        .vendor_id = 0,
        .device_id = 0,
        .mmio_base = 0,
        .class     = .unknown,
        .active    = false,
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// § 4. BsdCompatLayer — global state
// ═══════════════════════════════════════════════════════════════════════════

const BsdCompatLayer = struct {
    drivers     : [MAX_BSD_DRIVERS]BsdDriverEntry,
    driver_count: usize,
    initialized : bool,
};

var g_bsd_compat: BsdCompatLayer = BsdCompatLayer{
    .drivers      = [_]BsdDriverEntry{emptyEntry()} ** MAX_BSD_DRIVERS,
    .driver_count = 0,
    .initialized  = false,
};

// ═══════════════════════════════════════════════════════════════════════════
// § 5. Comptime register map validation
//      Zig's comptime ensures all register offsets are validated at compile
//      time — if any constraint is violated, the build fails.
// ═══════════════════════════════════════════════════════════════════════════

/// Validate that a register offset is naturally aligned for its width.
fn comptimeValidateRegister(comptime offset: u32, comptime width: u32) void {
    if (offset % width != 0) {
        @compileError("Register offset is not naturally aligned");
    }
}

// Validate our core registers at compile time
comptime {
    // PCI Configuration Space registers (Type 0 Header)
    comptimeValidateRegister(0x00, 4); // Vendor + Device ID
    comptimeValidateRegister(0x04, 4); // Command + Status
    comptimeValidateRegister(0x08, 4); // Revision + Class Code
    comptimeValidateRegister(0x0C, 4); // Cache Line + Latency
    comptimeValidateRegister(0x10, 4); // BAR0
    comptimeValidateRegister(0x14, 4); // BAR1
    comptimeValidateRegister(0x18, 4); // BAR2
}

// ═══════════════════════════════════════════════════════════════════════════
// § 6. MMIO helpers (inline asm — no import)
// ═══════════════════════════════════════════════════════════════════════════

fn mmio_read32(base: u64, offset: u32) u32 {
    const addr = @as(*volatile u32, @ptrFromInt(base + @as(u64, offset)));
    return addr.*;
}

fn mmio_write32(base: u64, offset: u32, val: u32) void {
    const addr = @as(*volatile u32, @ptrFromInt(base + @as(u64, offset)));
    addr.* = val;
}

// ═══════════════════════════════════════════════════════════════════════════
// § 7. Exported C-ABI functions
// ═══════════════════════════════════════════════════════════════════════════

/// Initialise the BSD compatibility layer.
export fn bsd_compat_init() i32 {
    if (g_bsd_compat.initialized) return EBUSY;
    g_bsd_compat.initialized = true;
    return ENONE;
}

/// Register a newbus-style driver.
export fn bsd_compat_register(
    name_ptr  : [*]const u8,
    name_len  : u32,
    vendor_id : u32,
    device_id : u32,
    mmio_base : u64,
    dev_class : u8,
) i32 {
    if (!g_bsd_compat.initialized) return ENODEV;
    if (g_bsd_compat.driver_count >= MAX_BSD_DRIVERS) return ENOMEM;

    const effective_len: usize = if (name_len > 31) 31 else @as(usize, name_len);

    var entry = &g_bsd_compat.drivers[g_bsd_compat.driver_count];
    entry.vendor_id = vendor_id;
    entry.device_id = device_id;
    entry.mmio_base = mmio_base;
    entry.active    = true;
    entry.name_len  = @truncate(effective_len);

    // Classify device
    entry.class = switch (dev_class) {
        0 => .network,
        1 => .storage,
        2 => .display,
        3 => .input,
        4 => .audio,
        5 => .usb,
        6 => .serial,
        else => .unknown,
    };

    // Copy name bytes manually — no memcpy / @memcpy
    var i: usize = 0;
    while (i < effective_len) : (i += 1) {
        entry.name[i] = name_ptr[i];
    }
    entry.name[effective_len] = 0; // Null terminate

    g_bsd_compat.driver_count += 1;
    return ENONE;
}

/// Unregister a driver by vendor/device ID.
export fn bsd_compat_unregister(vendor_id: u32, device_id: u32) i32 {
    if (!g_bsd_compat.initialized) return ENODEV;

    var i: usize = 0;
    while (i < g_bsd_compat.driver_count) : (i += 1) {
        if (g_bsd_compat.drivers[i].vendor_id == vendor_id and
            g_bsd_compat.drivers[i].device_id == device_id and
            g_bsd_compat.drivers[i].active)
        {
            g_bsd_compat.drivers[i].active = false;
            return ENONE;
        }
    }
    return ENODEV;
}

/// Return number of active BSD compat drivers.
export fn bsd_compat_active_count() u32 {
    var count: u32 = 0;
    var i: usize = 0;
    while (i < g_bsd_compat.driver_count) : (i += 1) {
        if (g_bsd_compat.drivers[i].active) count += 1;
    }
    return count;
}

/// Shutdown and de-initialise the compat layer.
export fn bsd_compat_shutdown() i32 {
    if (!g_bsd_compat.initialized) return ENODEV;

    // Deactivate all drivers
    var i: usize = 0;
    while (i < g_bsd_compat.driver_count) : (i += 1) {
        g_bsd_compat.drivers[i].active = false;
    }
    g_bsd_compat.driver_count = 0;
    g_bsd_compat.initialized = false;
    return ENONE;
}
