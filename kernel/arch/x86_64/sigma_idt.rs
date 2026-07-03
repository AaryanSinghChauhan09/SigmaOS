// SPDX-License-Identifier: MIT
// kernel/arch/x86_64/sigma_idt.rs
//
// SigmaOS IDT initialization — #![no_std], C-ABI exports for linking.
// Defines IDTGate (16-byte packed), populates 32 exception entries, and
// loads the table via LIDT.
//
// ISR stubs live in arch/x86_64/isr_stubs.asm (vectors 0-31).
// C-ABI exports: sigma_idt_init, sigma_idt_load

#![no_std]
#![allow(dead_code)]

use core::arch::asm;

// ── IDT gate — exactly 16 bytes, matches x86_64 hardware spec ─────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IDTGate {
    pub offset_low:  u16,  // handler addr [15:0]
    pub selector:    u16,  // kernel code segment selector
    pub ist:         u8,   // interrupt stack table index (bits 2:0)
    pub type_attr:   u8,   // P | DPL[1:0] | 0 | gate_type[3:0]
    pub offset_mid:  u16,  // handler addr [31:16]
    pub offset_high: u32,  // handler addr [63:32]
    pub reserved:    u32,  // must be zero
}

const _: () = assert!(core::mem::size_of::<IDTGate>() == 16);

impl IDTGate {
    /// Return a zero-initialised (not-present) gate.
    const fn zero() -> Self {
        Self {
            offset_low:  0,
            selector:    0,
            ist:         0,
            type_attr:   0,
            offset_mid:  0,
            offset_high: 0,
            reserved:    0,
        }
    }

    /// Encode a 64-bit handler address, selector, and type flags.
    #[inline]
    fn set(&mut self, handler: u64, selector: u16, type_attr: u8) {
        self.offset_low  = (handler & 0xFFFF) as u16;
        self.offset_mid  = ((handler >> 16) & 0xFFFF) as u16;
        self.offset_high = ((handler >> 32) & 0xFFFF_FFFF) as u32;
        self.selector    = selector;
        self.ist         = 0;
        self.type_attr   = type_attr;
        self.reserved    = 0;
    }
}

// ── IDT pointer (operand for LIDT) ────────────────────────────────────────
#[repr(C, packed)]
struct IDTPointer {
    limit: u16,
    base:  u64,
}

// ── Gate type / attribute constants ──────────────────────────────────────
/// Present, DPL=0, 64-bit Interrupt Gate (0x8E)
const GATE_INTERRUPT_RING0: u8 = 0x8E;
/// Present, DPL=3, 64-bit Interrupt Gate (0xEE) — for user-callable vectors
const GATE_INTERRUPT_RING3: u8 = 0xEE;

/// Kernel code segment selector (GDT slot 1, RPL=0).
const KERNEL_CS: u16 = 0x08;

/// Total IDT entries supported by x86_64.
const IDT_SIZE: usize = 256;

// ── Static IDT table (256 × 16 bytes = 4096 bytes) ────────────────────────
#[used]
static mut IDT: [IDTGate; IDT_SIZE] = [IDTGate::zero(); IDT_SIZE];

#[used]
static mut IDT_PTR: IDTPointer = IDTPointer { limit: 0, base: 0 };

// ── External ISR stub symbols (defined in arch/x86_64/isr_stubs.asm) ──────
// The isr_stubs.asm exports sigma_isr_0 … sigma_isr_31 (and beyond).
// We declare them here for address resolution.
extern "C" {
    fn sigma_isr_0();  fn sigma_isr_1();  fn sigma_isr_2();  fn sigma_isr_3();
    fn sigma_isr_4();  fn sigma_isr_5();  fn sigma_isr_6();  fn sigma_isr_7();
    fn sigma_isr_8();  fn sigma_isr_9();  fn sigma_isr_10(); fn sigma_isr_11();
    fn sigma_isr_12(); fn sigma_isr_13(); fn sigma_isr_14(); fn sigma_isr_15();
    fn sigma_isr_16(); fn sigma_isr_17(); fn sigma_isr_18(); fn sigma_isr_19();
    fn sigma_isr_20(); fn sigma_isr_21(); fn sigma_isr_22(); fn sigma_isr_23();
    fn sigma_isr_24(); fn sigma_isr_25(); fn sigma_isr_26(); fn sigma_isr_27();
    fn sigma_isr_28(); fn sigma_isr_29(); fn sigma_isr_30(); fn sigma_isr_31();
}

/// Array of ISR stub function pointers (vectors 0-31).
static ISR_STUBS: [unsafe extern "C" fn(); 32] = [
    sigma_isr_0,  sigma_isr_1,  sigma_isr_2,  sigma_isr_3,
    sigma_isr_4,  sigma_isr_5,  sigma_isr_6,  sigma_isr_7,
    sigma_isr_8,  sigma_isr_9,  sigma_isr_10, sigma_isr_11,
    sigma_isr_12, sigma_isr_13, sigma_isr_14, sigma_isr_15,
    sigma_isr_16, sigma_isr_17, sigma_isr_18, sigma_isr_19,
    sigma_isr_20, sigma_isr_21, sigma_isr_22, sigma_isr_23,
    sigma_isr_24, sigma_isr_25, sigma_isr_26, sigma_isr_27,
    sigma_isr_28, sigma_isr_29, sigma_isr_30, sigma_isr_31,
];

// ── sigma_idt_init ─────────────────────────────────────────────────────────
/// Populate IDT entries 0-31 with the ISR stubs from isr_stubs.asm,
/// then set up the IDT pointer for sigma_idt_load().
///
/// Must be called once at kernel init before enabling interrupts.
/// Exported as C ABI for cross-language linking.
#[no_mangle]
pub unsafe extern "C" fn sigma_idt_init() {
    // Clear all 256 gates.
    for gate in IDT.iter_mut() {
        *gate = IDTGate::zero();
    }

    // Install CPU exception stubs for vectors 0-31.
    for (vector, &stub) in ISR_STUBS.iter().enumerate() {
        let handler_addr = stub as u64;
        IDT[vector].set(handler_addr, KERNEL_CS, GATE_INTERRUPT_RING0);
    }

    // Build the IDTR pointer.
    let base = IDT.as_ptr() as u64;
    let limit = (IDT_SIZE * core::mem::size_of::<IDTGate>() - 1) as u16;
    IDT_PTR = IDTPointer { limit, base };
}

// ── sigma_idt_load ─────────────────────────────────────────────────────────
/// Load the IDT into the CPU via LIDT.
/// Call after sigma_idt_init(). Must be called on every CPU core.
#[no_mangle]
pub unsafe extern "C" fn sigma_idt_load() {
    asm!(
        "lidt [{ptr}]",
        ptr = in(reg) &IDT_PTR,
        options(preserves_flags, nostack, readonly),
    );
}

// ── sigma_idt_set_handler (optional runtime hook) ─────────────────────────
/// Install or replace a single IDT gate at runtime.
///
/// # Safety
/// `handler` must be a valid, non-null, canonical kernel virtual address.
#[no_mangle]
pub unsafe extern "C" fn sigma_idt_set_handler(
    vector:    u8,
    handler:   u64,
    selector:  u16,
    type_attr: u8,
) {
    IDT[vector as usize].set(handler, selector, type_attr);
}
