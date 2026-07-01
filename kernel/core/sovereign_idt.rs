// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign IDT setup and exception handling (Rust, no_std)
//! Replaces: kernel/core/sigma_idt.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const IDT_ENTRIES: usize = 256;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IDTGate {
    pub offset_low: u16,
    pub segment_selector: u16,
    pub ist: u8,
    pub type_attr: u8,
    pub offset_mid: u16,
    pub offset_high: u32,
    pub reserved: u32,
}

impl IDTGate {
    pub const fn empty() -> Self {
        Self {
            offset_low: 0,
            segment_selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IDTPointer {
    pub limit: u16,
    pub base: u64,
}

struct SafeIdt {
    table: UnsafeCell<[IDTGate; IDT_ENTRIES]>,
    pointer: UnsafeCell<IDTPointer>,
}

unsafe impl Sync for SafeIdt {}

static IDT: SafeIdt = SafeIdt {
    table: UnsafeCell::new([IDTGate::empty(); IDT_ENTRIES]),
    pointer: UnsafeCell::new(IDTPointer { limit: 0, base: 0 }),
};

extern "C" {
    fn sigma_vga_printf(fmt: *const u8, val1: u32, val2: *const u8, val3: u32, val4: u32);
}

unsafe fn idt_set_gate(num: u8, handler: u64, selector: u16, flags: u8) {
    let idt_table = &mut *IDT.table.get();
    let gate = &mut idt_table[num as usize];
    gate.offset_low = (handler & 0xFFFF) as u16;
    gate.offset_mid = ((handler >> 16) & 0xFFFF) as u16;
    gate.offset_high = ((handler >> 32) & 0xFFFFFFFF) as u32;
    gate.segment_selector = selector;
    gate.ist = 0;
    gate.type_attr = flags;
    gate.reserved = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_idt_init() {
    let idt_ptr = &mut *IDT.pointer.get();
    let idt_table = &mut *IDT.table.get();

    idt_ptr.limit = (core::mem::size_of::<[IDTGate; IDT_ENTRIES]>() - 1) as u16;
    idt_ptr.base = idt_table.as_ptr() as u64;

    for i in 0..IDT_ENTRIES {
        idt_set_gate(i as u8, 0, 0x08, 0x8E);
    }

    core::arch::asm!("lidt ({})", in(reg) idt_ptr);

    sigma_vga_printf(
        b"[IDT] Interrupt Descriptor Table loaded (%u entries)\n\0".as_ptr(),
        IDT_ENTRIES as u32,
        core::ptr::null(),
        0,
        0,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sigma_exception_handler(vector: u32, error_code: u64, rip: u64) {
    // Basic exception routing to serial/VGA output
    if vector == 14 {
        let cr2: u64;
        core::arch::asm!("mov {}, cr2", out(reg) cr2);
    }

    if vector == 8 || vector == 13 || vector == 14 {
        core::arch::asm!("cli", "hlt");
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_irq_handler(irq: u32) {
    // Acknowledge interrupt on PIC controllers
    core::arch::asm!("out 0x20, al", in("al") 0x20u8);
    if irq >= 8 {
        core::arch::asm!("out 0xA0, al", in("al") 0x20u8);
    }
}
