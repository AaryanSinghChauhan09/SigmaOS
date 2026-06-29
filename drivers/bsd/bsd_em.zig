// SPDX-License-Identifier: GPL-2.0-or-later
//
// =========================================================================
// SIGMAOS: BSD em(4) INTEL ETHERNET DRIVER (Zig — freestanding, no std)
// =========================================================================
//
// Replaces: drivers/bsd/bsd_em.cpp
// Language: Zig (freestanding)
//
// Intel 82540EM (E1000) Gigabit Ethernet driver for the BSD target.
// Implements the FreeBSD em(4) interface semantics using SigmaOS MMIO.
// ZERO standard library. ZERO @import("std"). ZERO predefined functions.
//
// Reference: Intel 82540EM Software Developer's Manual
//            FreeBSD em(4) man page — driver behaviour contract
//
// Compile with:
//   zig build-obj -target x86_64-freestanding-none -O ReleaseFast \
//                 drivers/bsd/bsd_em.zig -femit-bin=build/bsd_em.o
//
// Selected at build time with: TARGET_OS=bsd
// =========================================================================

// No imports — entirely self-contained.

// ═══════════════════════════════════════════════════════════════════════════
// § 1. Intel E1000 Register Map (defined from SDM, not from any header)
// ═══════════════════════════════════════════════════════════════════════════

const E1000_CTRL    : u32 = 0x0000;
const E1000_STATUS  : u32 = 0x0008;
const E1000_EECD    : u32 = 0x0010;
const E1000_EERD    : u32 = 0x0014;
const E1000_ICR     : u32 = 0x00C0;
const E1000_IMS     : u32 = 0x00D0;
const E1000_IMC     : u32 = 0x00D8;
const E1000_RCTL    : u32 = 0x0100;
const E1000_TCTL    : u32 = 0x0400;
const E1000_RDBAL   : u32 = 0x2800;
const E1000_RDBAH   : u32 = 0x2804;
const E1000_RDLEN   : u32 = 0x2808;
const E1000_RDH     : u32 = 0x2810;
const E1000_RDT     : u32 = 0x2818;
const E1000_TDBAL   : u32 = 0x3800;
const E1000_TDBAH   : u32 = 0x3804;
const E1000_TDLEN   : u32 = 0x3808;
const E1000_TDH     : u32 = 0x3810;
const E1000_TDT     : u32 = 0x3818;
const E1000_RAL0    : u32 = 0x5400;
const E1000_RAH0    : u32 = 0x5404;
const E1000_MTA_BASE: u32 = 0x5200;

// CTRL bits
const CTRL_RST  : u32 = 1 << 26;
const CTRL_SLU  : u32 = 1 << 6;
const CTRL_ASDE : u32 = 1 << 5;

// RCTL bits
const RCTL_EN   : u32 = 1 << 1;
const RCTL_BAM  : u32 = 1 << 15;
const RCTL_SECRC: u32 = 1 << 26;

// TCTL bits
const TCTL_EN   : u32 = 1 << 1;
const TCTL_PSP  : u32 = 1 << 3;
const TCTL_CT   : u32 = 0x10 << 4;
const TCTL_COLD : u32 = 0x40 << 12;

// STATUS bits
const STATUS_LU : u32 = 1 << 1;

// PCI IDs
const VENDOR_INTEL : u32 = 0x8086;
const DEVICE_82540 : u32 = 0x100E;

const POLL_MAX     : u32 = 200_000;

// ═══════════════════════════════════════════════════════════════════════════
// § 2. TX/RX Descriptor structures (16 bytes each)
// ═══════════════════════════════════════════════════════════════════════════

const TxDesc = packed struct {
    buffer_addr: u64,
    length     : u16,
    cso        : u8,
    cmd        : u8,
    status     : u8,
    css        : u8,
    special    : u16,
};

const RxDesc = packed struct {
    buffer_addr: u64,
    length     : u16,
    checksum   : u16,
    status     : u8,
    errors     : u8,
    special    : u16,
};

// Comptime size assertions
comptime {
    if (@sizeOf(TxDesc) != 16) @compileError("TxDesc must be 16 bytes");
    if (@sizeOf(RxDesc) != 16) @compileError("RxDesc must be 16 bytes");
}

// TX CMD bits
const TX_CMD_EOP  : u8 = 1 << 0;
const TX_CMD_IFCS : u8 = 1 << 1;
const TX_CMD_RS   : u8 = 1 << 3;

// RX STATUS bits
const RX_DD  : u8 = 1 << 0;
const RX_EOP : u8 = 1 << 1;

const TX_RING_SIZE : usize = 128;
const RX_RING_SIZE : usize = 128;
const RX_BUF_SIZE  : usize = 2048;

// ═══════════════════════════════════════════════════════════════════════════
// § 3. MMIO helpers (volatile, no import)
// ═══════════════════════════════════════════════════════════════════════════

fn mmio_read32(base: u64, offset: u32) u32 {
    const addr = @as(*volatile u32, @ptrFromInt(base + @as(u64, offset)));
    return addr.*;
}

fn mmio_write32(base: u64, offset: u32, val: u32) void {
    const addr = @as(*volatile u32, @ptrFromInt(base + @as(u64, offset)));
    addr.* = val;
}

fn spin_pause() void {
    asm volatile ("pause" ::: "memory");
}

fn poll32(base: u64, offset: u32, mask: u32, expected: u32) bool {
    var i: u32 = 0;
    while (i < POLL_MAX) : (i += 1) {
        if ((mmio_read32(base, offset) & mask) == expected) return true;
        spin_pause();
    }
    return false;
}

// ═══════════════════════════════════════════════════════════════════════════
// § 4. BSD em(4) Driver State
// ═══════════════════════════════════════════════════════════════════════════

fn zeroTxDesc() TxDesc {
    return TxDesc{
        .buffer_addr = 0, .length = 0, .cso = 0,
        .cmd = 0, .status = 0, .css = 0, .special = 0,
    };
}

fn zeroRxDesc() RxDesc {
    return RxDesc{
        .buffer_addr = 0, .length = 0, .checksum = 0,
        .status = 0, .errors = 0, .special = 0,
    };
}

var g_tx_descs: [TX_RING_SIZE]TxDesc align(4096) =
    [_]TxDesc{zeroTxDesc()} ** TX_RING_SIZE;

var g_rx_descs: [RX_RING_SIZE]RxDesc align(4096) =
    [_]RxDesc{zeroRxDesc()} ** RX_RING_SIZE;

var g_rx_buffers: [RX_RING_SIZE][RX_BUF_SIZE]u8 align(4096) =
    [_][RX_BUF_SIZE]u8{[_]u8{0} ** RX_BUF_SIZE} ** RX_RING_SIZE;

var g_mmio_base   : u64   = 0;
var g_tx_tail     : usize = 0;
var g_rx_head     : usize = 0;
var g_tx_count    : u64   = 0;
var g_rx_count    : u64   = 0;
var g_initialized : bool  = false;

// ═══════════════════════════════════════════════════════════════════════════
// § 5. Exported BSD em(4) interface
// ═══════════════════════════════════════════════════════════════════════════

/// bsd_em_attach — called by FreeBSD newbus probe/attach sequence.
/// Initialises the Intel 82540EM NIC.
///
/// Sequence mirrors FreeBSD em_attach():
///   1. Device reset
///   2. Link up + auto-speed
///   3. Clear interrupts + MTA
///   4. RX ring init
///   5. TX ring init
///   6. Enable RX/TX
export fn bsd_em_attach(mmio_base: u64) i32 {
    g_mmio_base = mmio_base;

    // Step 1: Reset
    const ctrl = mmio_read32(mmio_base, E1000_CTRL);
    mmio_write32(mmio_base, E1000_CTRL, ctrl | CTRL_RST);
    // Spin-wait for reset
    var spin: u32 = 0;
    while (spin < 100_000) : (spin += 1) { spin_pause(); }

    // Step 2: Set Link Up
    mmio_write32(mmio_base, E1000_CTRL, CTRL_SLU | CTRL_ASDE);

    // Step 3: Clear interrupts
    _ = mmio_read32(mmio_base, E1000_ICR);
    mmio_write32(mmio_base, E1000_IMC, 0xFFFF_FFFF);

    // Clear MTA (128 entries)
    var m: u32 = 0;
    while (m < 128) : (m += 1) {
        mmio_write32(mmio_base, E1000_MTA_BASE + m * 4, 0);
    }

    // Step 4: RX ring
    var i: usize = 0;
    while (i < RX_RING_SIZE) : (i += 1) {
        g_rx_descs[i].buffer_addr = @intFromPtr(&g_rx_buffers[i]);
        g_rx_descs[i].status = 0;
    }
    const rdba: u64 = @intFromPtr(&g_rx_descs);
    mmio_write32(mmio_base, E1000_RDBAL, @truncate(rdba));
    mmio_write32(mmio_base, E1000_RDBAH, @truncate(rdba >> 32));
    mmio_write32(mmio_base, E1000_RDLEN, @as(u32, RX_RING_SIZE * 16));
    mmio_write32(mmio_base, E1000_RDH, 0);
    mmio_write32(mmio_base, E1000_RDT, @as(u32, RX_RING_SIZE - 1));

    // Step 5: TX ring
    const tdba: u64 = @intFromPtr(&g_tx_descs);
    mmio_write32(mmio_base, E1000_TDBAL, @truncate(tdba));
    mmio_write32(mmio_base, E1000_TDBAH, @truncate(tdba >> 32));
    mmio_write32(mmio_base, E1000_TDLEN, @as(u32, TX_RING_SIZE * 16));
    mmio_write32(mmio_base, E1000_TDH, 0);
    mmio_write32(mmio_base, E1000_TDT, 0);

    // Step 6: Enable
    mmio_write32(mmio_base, E1000_RCTL, RCTL_EN | RCTL_BAM | RCTL_SECRC);
    mmio_write32(mmio_base, E1000_TCTL, TCTL_EN | TCTL_PSP | TCTL_CT | TCTL_COLD);

    // Check link
    _ = poll32(mmio_base, E1000_STATUS, STATUS_LU, STATUS_LU);

    g_initialized = true;
    return 0;
}

/// bsd_em_detach — tear down the NIC (FreeBSD em_detach equivalent).
export fn bsd_em_detach() i32 {
    if (!g_initialized) return -19; // ENODEV

    // Disable RX/TX
    mmio_write32(g_mmio_base, E1000_RCTL, 0);
    mmio_write32(g_mmio_base, E1000_TCTL, 0);

    // Mask all interrupts
    mmio_write32(g_mmio_base, E1000_IMC, 0xFFFF_FFFF);

    g_initialized = false;
    return 0;
}

/// bsd_em_transmit — send a single frame.
export fn bsd_em_transmit(data_phys: u64, len: u32) i32 {
    if (!g_initialized or data_phys == 0 or len == 0) return -22; // EINVAL

    const idx = g_tx_tail;
    g_tx_descs[idx] = TxDesc{
        .buffer_addr = data_phys,
        .length      = @truncate(len),
        .cso         = 0,
        .cmd         = TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS,
        .status      = 0,
        .css         = 0,
        .special     = 0,
    };

    g_tx_tail = (idx + 1) % TX_RING_SIZE;
    g_tx_count += 1;

    // Ring doorbell
    mmio_write32(g_mmio_base, E1000_TDT, @truncate(g_tx_tail));
    return 0;
}

/// bsd_em_poll_rx — poll for a received frame.
/// Returns received length (> 0) or 0 if no frame.
export fn bsd_em_poll_rx(out_buf_phys: *u64) i32 {
    if (!g_initialized) return -19;

    const idx = g_rx_head;
    const desc = &g_rx_descs[idx];

    if ((desc.status & RX_DD) == 0) return 0; // No frame

    const len = desc.length;
    out_buf_phys.* = desc.buffer_addr;

    // Reset for reuse
    g_rx_descs[idx].status = 0;
    g_rx_head = (idx + 1) % RX_RING_SIZE;
    g_rx_count += 1;

    // Advance RDT
    mmio_write32(g_mmio_base, E1000_RDT, @truncate(idx));

    return @as(i32, len);
}

/// Return TX frame count.
export fn bsd_em_tx_count() u64 {
    return g_tx_count;
}

/// Return RX frame count.
export fn bsd_em_rx_count() u64 {
    return g_rx_count;
}
