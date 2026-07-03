// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/keyboard.rs — PS/2 keyboard + USB HID keyboard driver
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

// ── PS/2 keyboard ports ───────────────────────────────────────────────────
const PS2_DATA:   u16 = 0x60;
const PS2_STATUS: u16 = 0x64;
const PS2_CMD:    u16 = 0x64;

unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    core::arch::asm!("in al, dx", out("al") v, in("dx") port, options(nomem, nostack));
    v
}
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
}

// ── Key ring buffer ───────────────────────────────────────────────────────
const KEY_BUF_SIZE: usize = 256;

static mut KEY_BUF: [u8; KEY_BUF_SIZE] = [0u8; KEY_BUF_SIZE];
static mut KEY_HEAD: usize = 0;
static mut KEY_TAIL: usize = 0;
static SHIFT_HELD: AtomicBool = AtomicBool::new(false);
static CTRL_HELD:  AtomicBool = AtomicBool::new(false);
static CAPS_LOCK:  AtomicBool = AtomicBool::new(false);

unsafe fn key_push(c: u8) {
    let next = (KEY_TAIL + 1) % KEY_BUF_SIZE;
    if next != KEY_HEAD {
        KEY_BUF[KEY_TAIL] = c;
        KEY_TAIL = next;
    }
}

pub unsafe fn key_pop() -> Option<u8> {
    if KEY_HEAD == KEY_TAIL { return None; }
    let c = KEY_BUF[KEY_HEAD];
    KEY_HEAD = (KEY_HEAD + 1) % KEY_BUF_SIZE;
    Some(c)
}

// ── Full US QWERTY scancode set 1 tables ─────────────────────────────────
const SCANCODE_NORMAL: [u8; 128] = [
    0,   27,  b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=',
    b'\x08', b'\t',
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
    0,   // left ctrl
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',
    0,   // left shift
    b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, b'*',
    0,   // left alt
    b' ',
    0,   // caps lock
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,   // F1-F10
    0,   // num lock
    0,   // scroll lock
    b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1', b'2', b'3', b'0', b'.',
    0, 0, 0,
    0, 0,   // F11, F12
    0, 0, 0, 0, 0, 0, 0,
];

const SCANCODE_SHIFTED: [u8; 128] = [
    0,   27,  b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+',
    b'\x08', b'\t',
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', b'\n',
    0,
    b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~',
    0,
    b'|', b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b'<', b'>', b'?', 0, b'*',
    0,
    b' ',
    0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
    b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1', b'2', b'3', b'0', b'.',
    0, 0, 0,
    0, 0,
    0, 0, 0, 0, 0, 0, 0,
];

// ── Scancode handler (called from IRQ1) ───────────────────────────────────
pub unsafe fn handle_scancode(sc: u8) {
    let is_break = sc & 0x80 != 0;
    let sc_make  = sc & 0x7F;

    match sc_make {
        0x2A | 0x36 => { SHIFT_HELD.store(!is_break, Ordering::Relaxed); return; } // L/R shift
        0x1D        => { CTRL_HELD.store(!is_break, Ordering::Relaxed);  return; } // Ctrl
        0x3A if !is_break => { // Caps Lock toggle
            let cur = CAPS_LOCK.load(Ordering::Relaxed);
            CAPS_LOCK.store(!cur, Ordering::Relaxed);
            return;
        }
        _ => {}
    }

    if is_break || sc_make >= 128 { return; }

    let shift = SHIFT_HELD.load(Ordering::Relaxed);
    let caps  = CAPS_LOCK.load(Ordering::Relaxed);
    let ctrl  = CTRL_HELD.load(Ordering::Relaxed);

    let mut c = if shift { SCANCODE_SHIFTED[sc_make as usize] }
                else     { SCANCODE_NORMAL [sc_make as usize] };

    if c == 0 { return; }

    // Caps lock: swap case for letters
    if caps && !shift {
        if c >= b'a' && c <= b'z' { c -= 32; }
    } else if caps && shift {
        if c >= b'A' && c <= b'Z' { c += 32; }
    }

    // Ctrl key: convert to control characters (Ctrl-C = 3, Ctrl-D = 4, etc.)
    if ctrl && c >= b'a' && c <= b'z' {
        c = c - b'a' + 1;
    } else if ctrl && c >= b'A' && c <= b'Z' {
        c = c - b'A' + 1;
    }

    key_push(c);
}

// ── PS/2 keyboard IRQ handler ─────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_keyboard_irq() -> bool {
    let status = inb(PS2_STATUS);
    if status & 0x01 == 0 { return false; } // no data
    let sc = inb(PS2_DATA);
    handle_scancode(sc);
    true
}

// ── Init ──────────────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_keyboard_init() {
    // Drain any pending PS/2 data
    while inb(PS2_STATUS) & 0x01 != 0 { let _ = inb(PS2_DATA); }

    // Enable PS/2 keyboard interrupt (IRQ 1 = vector 33)
    extern "C" { fn sigma_request_irq(irq: u8, handler: unsafe extern "C" fn() -> bool); }
    sigma_request_irq(1, sigma_keyboard_irq_wrapper);
}

unsafe extern "C" fn sigma_keyboard_irq_wrapper() -> bool {
    sigma_keyboard_irq()
}

// ── Blocking/non-blocking read ────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_keyboard_read() -> u8 {
    // Non-blocking: return 0 if no key available
    key_pop().unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_keyboard_read_blocking() -> u8 {
    loop {
        if let Some(c) = key_pop() { return c; }
        core::arch::asm!("hlt", options(nomem, nostack));
    }
}
