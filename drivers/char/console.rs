// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/char/console.rs — Console, serial, and character device drivers
//
// Provides: /dev/console, /dev/null, /dev/zero, /dev/random, /dev/tty
// Also implements VGA text-mode console + serial debug output.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, AtomicU64, Ordering};

// ── VGA text-mode constants ───────────────────────────────────────────────
const VGA_BASE:   u64 = 0xB8000;
const VGA_COLS:   usize = 80;
const VGA_ROWS:   usize = 25;

// Colour byte: bg[7:4] | fg[3:0]
const COLOR_WHITE_ON_BLACK: u8 = 0x07;
const COLOR_GREEN_ON_BLACK: u8 = 0x0A;
const COLOR_RED_ON_BLACK:   u8 = 0x0C;
const COLOR_CYAN_ON_BLACK:  u8 = 0x0B;

// ── VGA console ───────────────────────────────────────────────────────────
static VGA_COL: AtomicUsize = AtomicUsize::new(0);
static VGA_ROW: AtomicUsize = AtomicUsize::new(0);

unsafe fn vga_char(col: usize, row: usize, c: u8, color: u8) {
    let offset = (row * VGA_COLS + col) * 2;
    let ptr = (VGA_BASE + offset as u64) as *mut u8;
    *ptr = c;
    *ptr.add(1) = color;
}

unsafe fn vga_scroll() {
    let base = VGA_BASE as *mut u8;
    // Move rows 1..24 up by one row
    core::ptr::copy(
        base.add(VGA_COLS * 2),
        base,
        (VGA_ROWS - 1) * VGA_COLS * 2,
    );
    // Clear last row
    let last_row = base.add((VGA_ROWS - 1) * VGA_COLS * 2);
    for i in 0..VGA_COLS * 2 {
        *last_row.add(i) = if i % 2 == 0 { b' ' } else { COLOR_WHITE_ON_BLACK };
    }
}

unsafe fn vga_putc_color(c: u8, color: u8) {
    let mut col = VGA_COL.load(Ordering::Relaxed);
    let mut row = VGA_ROW.load(Ordering::Relaxed);

    match c {
        b'\n' => {
            col = 0;
            row += 1;
        }
        b'\r' => {
            col = 0;
        }
        b'\x08' => { // backspace
            if col > 0 { col -= 1; }
            vga_char(col, row, b' ', color);
        }
        _ => {
            vga_char(col, row, c, color);
            col += 1;
            if col >= VGA_COLS {
                col = 0;
                row += 1;
            }
        }
    }

    if row >= VGA_ROWS {
        vga_scroll();
        row = VGA_ROWS - 1;
    }

    VGA_COL.store(col, Ordering::Relaxed);
    VGA_ROW.store(row, Ordering::Relaxed);

    // Update VGA cursor via I/O ports
    let pos = row * VGA_COLS + col;
    outb(0x3D4, 0x0F);
    outb(0x3D5, (pos & 0xFF) as u8);
    outb(0x3D4, 0x0E);
    outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
}

unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
}

// ── Serial port (COM1) ────────────────────────────────────────────────────
unsafe fn serial_putc(c: u8) {
    // Wait for transmit-hold-register empty
    let mut status: u8;
    loop {
        core::arch::asm!("in al, dx", out("al") status, in("dx") 0x3FDu16, options(nomem, nostack));
        if status & 0x20 != 0 { break; }
        core::arch::asm!("pause", options(nomem, nostack));
    }
    core::arch::asm!("out dx, al", in("dx") 0x3F8u16, in("al") c, options(nomem, nostack));
}

unsafe fn serial_puts_raw(s: &[u8]) {
    for &b in s {
        if b == b'\n' { serial_putc(b'\r'); }
        serial_putc(b);
    }
}

// ── PRNG for /dev/random ──────────────────────────────────────────────────
static RAND_STATE: AtomicU64 = AtomicU64::new(0xDEAD_BEEF_CAFE_1234);

fn rand_byte() -> u8 {
    let mut x = RAND_STATE.load(Ordering::Relaxed);
    x ^= x << 13; x ^= x >> 7; x ^= x << 17;
    RAND_STATE.store(x, Ordering::Relaxed);
    (x & 0xFF) as u8
}

// ── Console device ops ────────────────────────────────────────────────────

/// Write to VGA + serial simultaneously
pub unsafe fn console_write(buf: *const u8, len: usize, color: u8) -> i64 {
    if buf.is_null() { return -14; }
    let data = core::slice::from_raw_parts(buf, len);
    for &b in data {
        vga_putc_color(b, color);
        serial_putc(b);
    }
    len as i64
}

/// Read from keyboard buffer (non-blocking, returns 0 if empty)
pub unsafe fn console_read(buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    // Read from PS/2 keyboard port
    let status: u8;
    core::arch::asm!("in al, dx", out("al") status, in("dx") 0x64u16, options(nomem, nostack));
    if status & 0x01 == 0 { return 0; } // no data

    let scancode: u8;
    core::arch::asm!("in al, dx", out("al") scancode, in("dx") 0x60u16, options(nomem, nostack));

    // Minimal scancode → ASCII (US QWERTY, no shift)
    let ascii = scancode_to_ascii(scancode);
    if ascii != 0 && len > 0 {
        *buf = ascii;
        return 1;
    }
    0
}

fn scancode_to_ascii(sc: u8) -> u8 {
    const TABLE: [u8; 58] = [
        0, 0, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0',
        b'-', b'=', b'\x08', b'\t',
        b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
        0, // left ctrl
        b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',
        0, // left shift
        b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, 0, 0, b' ',
    ];
    if sc < 58 { TABLE[sc as usize] } else { 0 }
}

// ── /dev/null, /dev/zero, /dev/random ─────────────────────────────────────

pub unsafe fn devnull_read(_buf: *mut u8, _len: usize) -> i64 { 0 }
pub unsafe fn devnull_write(_buf: *const u8, len: usize) -> i64 { len as i64 }

pub unsafe fn devzero_read(buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    core::ptr::write_bytes(buf, 0, len);
    len as i64
}

pub unsafe fn devrandom_read(buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    for i in 0..len { *buf.add(i) = rand_byte(); }
    len as i64
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_console_write(buf: *const u8, len: usize) -> i64 {
    console_write(buf, len, COLOR_WHITE_ON_BLACK)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_console_read(buf: *mut u8, len: usize) -> i64 {
    console_read(buf, len)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_log(msg: *const u8, len: usize) {
    if msg.is_null() { return; }
    let data = core::slice::from_raw_parts(msg, len);
    serial_puts_raw(data);
    // Also to VGA in cyan for kernel messages
    for &b in data {
        vga_putc_color(b, COLOR_CYAN_ON_BLACK);
    }
}

/// vga_putc — called from sovereign_kernel_main
#[no_mangle]
pub unsafe extern "C" fn vga_putc(c: u8, color: u8) {
    vga_putc_color(c, color);
}

#[no_mangle]
pub unsafe extern "C" fn serial_init() {
    outb(0x3F9, 0x00); // disable interrupts
    outb(0x3FB, 0x80); // DLAB=1
    outb(0x3F8, 0x01); // 115200 baud (divisor lo)
    outb(0x3F9, 0x00); // divisor hi
    outb(0x3FB, 0x03); // 8N1
    outb(0x3FA, 0xC7); // FIFO
    outb(0x3FC, 0x0B); // RTS/DSR
}

#[no_mangle]
pub unsafe extern "C" fn serial_puts(s: *const u8) {
    if s.is_null() { return; }
    let mut p = s;
    while *p != 0 {
        let c = *p;
        if c == b'\n' { serial_putc(b'\r'); }
        serial_putc(c);
        p = p.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn console_init() {
    serial_init();
    // Clear VGA screen
    let base = VGA_BASE as *mut u8;
    for i in 0..VGA_ROWS * VGA_COLS * 2 {
        *base.add(i) = if i % 2 == 0 { b' ' } else { COLOR_WHITE_ON_BLACK };
    }
    VGA_COL.store(0, Ordering::Relaxed);
    VGA_ROW.store(0, Ordering::Relaxed);
}
