//! SigmaOS — x86_64 Interrupt Descriptor Table (IDT)
//! SPDX-License-Identifier: MIT OR GPL-2.0
//!
//! Implements a full 256-entry IDT with hardware exception handlers,
//! APIC timer vector (0x20), and syscall vector (0x80).
//! The IDT is loaded via the `lidt` instruction using the IDTR structure.

#![allow(dead_code)]

use core::arch::asm;

// ─── IDT Entry (16 bytes, 64-bit interrupt gate) ────────────────────────────

/// A single 64-bit mode IDT descriptor (interrupt gate, 16 bytes).
/// Layout per Intel SDM Vol. 3A §6.14.1:
///   [15:0]   offset_low   — bits 0–15 of handler RIP
///   [31:16]  selector     — code segment selector (kernel CS)
///   [34:32]  ist          — interrupt stack table index (0 = none)
///   [39:35]  reserved0    — must be zero
///   [43:40]  gate_type    — 0xE = 64-bit interrupt gate
///   [44]     zero         — must be zero for interrupt/trap gates
///   [46:45]  dpl          — descriptor privilege level
///   [47]     present      — segment present flag
///   [63:48]  offset_mid   — bits 16–31 of handler RIP
///   [95:64]  offset_high  — bits 32–63 of handler RIP
///   [127:96] reserved1    — must be zero
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low:  u16,
    selector:    u16,
    ist:         u8,   // bits [2:0] = IST index; bits [7:3] = 0
    type_attr:   u8,   // P | DPL[1:0] | 0 | gate_type[3:0]
    offset_mid:  u16,
    offset_high: u32,
    reserved:    u32,
}

impl IdtEntry {
    /// Creates a cleared (not-present) IDT entry.
    pub const fn missing() -> Self {
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

    /// Builds a 64-bit interrupt gate for `handler` at ring-0.
    ///
    /// # Arguments
    /// * `handler`  — virtual address of the ISR stub
    /// * `selector` — kernel code segment selector (typically 0x08)
    /// * `ist`      — IST stack index (0 = legacy stack switching)
    /// * `dpl`      — descriptor privilege level (0 = kernel, 3 = user)
    pub fn new(handler: u64, selector: u16, ist: u8, dpl: u8) -> Self {
        // Safety invariant: `type_attr` encodes P=1, DPL, type=0xE.
        // 0x8E = 1000_1110b  →  P=1 | DPL=0 | 0 | type=0xE (64-bit intr gate)
        // 0xEE = 1110_1110b  →  P=1 | DPL=3 | 0 | type=0xE (user-callable)
        let type_attr = 0x8E | ((dpl & 0x3) << 5);
        Self {
            offset_low:  (handler & 0xFFFF) as u16,
            selector,
            ist:         ist & 0x7,
            type_attr,
            offset_mid:  ((handler >> 16) & 0xFFFF) as u16,
            offset_high: ((handler >> 32) & 0xFFFF_FFFF) as u32,
            reserved:    0,
        }
    }
}

// ─── Static IDT ─────────────────────────────────────────────────────────────

/// 256-entry IDT, 16 bytes each = 4096 bytes, naturally aligned.
/// SAFETY: Only modified during `init()` before interrupts are enabled;
///         after that it is read-only from the CPU's perspective.
#[repr(align(16))]
struct Idt([IdtEntry; 256]);

static mut IDT: Idt = Idt([IdtEntry::missing(); 256]);

// ─── IDTR Descriptor ────────────────────────────────────────────────────────

#[repr(C, packed)]
struct Idtr {
    limit: u16,   // size of IDT in bytes minus 1
    base:  u64,   // linear address of IDT
}

// ─── Default / Panic Handlers ───────────────────────────────────────────────

/// Minimal handler frame pushed by CPU (without error code).
#[repr(C)]
struct InterruptFrame {
    rip:    u64,
    cs:     u64,
    rflags: u64,
    rsp:    u64,
    ss:     u64,
}

/// Default catch-all ISR — halts the machine.
///
/// In a real kernel this would at minimum print the vector number
/// and the faulting RIP before halting.
extern "x86-interrupt" fn default_handler(_frame: InterruptFrame) {
    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) };
    }
}

/// #DF — Double Fault (vector 8). Always has error code 0.
extern "x86-interrupt" fn double_fault_handler(_frame: InterruptFrame, _error: u64) -> ! {
    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) };
    }
}

/// #PF — Page Fault (vector 14). CR2 holds the faulting virtual address.
extern "x86-interrupt" fn page_fault_handler(_frame: InterruptFrame, error_code: u64) {
    let cr2: u64;
    // SAFETY: `mov rax, cr2` is a privileged read; we are in ring-0.
    unsafe {
        asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack, preserves_flags));
    }
    // TODO: forward to the VM page-fault dispatcher.
    let _ = (cr2, error_code);
    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) };
    }
}

/// #GP — General Protection Fault (vector 13).
extern "x86-interrupt" fn general_protection_fault_handler(
    _frame: InterruptFrame,
    _error: u64,
) {
    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) };
    }
}

/// APIC timer interrupt (vector 0x20 = 32).
extern "x86-interrupt" fn apic_timer_handler(_frame: InterruptFrame) {
    // Write EOI to Local APIC (offset 0xB0).
    // SAFETY: 0xFEE00000 is the well-known LAPIC MMIO base;
    //         single-writer per CPU, no aliasing.
    unsafe {
        let eoi = (0xFEE0_0000u64 + 0xB0) as *mut u32;
        eoi.write_volatile(0);
    }
}

/// Software interrupt syscall gate (vector 0x80), DPL=3 so user code can `int 0x80`.
extern "x86-interrupt" fn syscall_int80_handler(_frame: InterruptFrame) {
    // Dispatch through the syscall layer.
    // Real implementation forwards rax/rdi/rsi/rdx to syscall_dispatch.
}

// ─── Macro to register a handler ────────────────────────────────────────────

macro_rules! set_handler {
    ($idt:expr, $vec:expr, $handler:expr, $sel:expr, $ist:expr, $dpl:expr) => {
        $idt[$vec] = IdtEntry::new($handler as u64, $sel, $ist, $dpl);
    };
}

macro_rules! set_handler_with_err {
    ($idt:expr, $vec:expr, $handler:expr, $sel:expr, $ist:expr, $dpl:expr) => {
        $idt[$vec] = IdtEntry::new($handler as u64, $sel, $ist, $dpl);
    };
}

// ─── Public init ────────────────────────────────────────────────────────────

/// Kernel CS selector (matches GDT entry 1, RPL=0).
const KERNEL_CS: u16 = 0x08;

/// Populate the IDT and load it with `lidt`.
///
/// # Safety
/// Must be called exactly once, before enabling hardware interrupts (`sti`).
/// The IDT lives in a static; after this call it must not be mutated unless
/// interrupts are first disabled again.
pub fn init() {
    // SAFETY: We are the only writer at this point (single-core init,
    //         interrupts not yet enabled). The `&mut` borrow of `IDT.0`
    //         is valid for the duration of this function.
    let idt = unsafe { &mut IDT.0 };

    // ── CPU Exception Handlers ──────────────────────────────────────────
    set_handler!(idt,  0, default_handler,                   KERNEL_CS, 0, 0); // #DE Divide Error
    set_handler!(idt,  1, default_handler,                   KERNEL_CS, 0, 0); // #DB Debug
    set_handler!(idt,  2, default_handler,                   KERNEL_CS, 0, 0); // #NMI
    set_handler!(idt,  3, default_handler,                   KERNEL_CS, 0, 3); // #BP Breakpoint (DPL=3)
    set_handler!(idt,  4, default_handler,                   KERNEL_CS, 0, 0); // #OF Overflow
    set_handler!(idt,  5, default_handler,                   KERNEL_CS, 0, 0); // #BR Bound Range
    set_handler!(idt,  6, default_handler,                   KERNEL_CS, 0, 0); // #UD Invalid Opcode
    set_handler!(idt,  7, default_handler,                   KERNEL_CS, 0, 0); // #NM Device Not Available
    set_handler_with_err!(idt,  8, double_fault_handler,     KERNEL_CS, 0, 0); // #DF Double Fault
    // 9: reserved (legacy coprocessor segment overrun)
    set_handler_with_err!(idt, 10, general_protection_fault_handler, KERNEL_CS, 0, 0); // #TS Invalid TSS
    set_handler_with_err!(idt, 11, general_protection_fault_handler, KERNEL_CS, 0, 0); // #NP Seg Not Present
    set_handler_with_err!(idt, 12, general_protection_fault_handler, KERNEL_CS, 0, 0); // #SS Stack Fault
    set_handler_with_err!(idt, 13, general_protection_fault_handler, KERNEL_CS, 0, 0); // #GP
    set_handler_with_err!(idt, 14, page_fault_handler,      KERNEL_CS, 0, 0); // #PF Page Fault
    // 15: reserved
    set_handler!(idt, 16, default_handler,                   KERNEL_CS, 0, 0); // #MF x87 FPU Error
    set_handler_with_err!(idt, 17, general_protection_fault_handler, KERNEL_CS, 0, 0); // #AC Alignment Check
    set_handler!(idt, 18, default_handler,                   KERNEL_CS, 0, 0); // #MC Machine Check
    set_handler!(idt, 19, default_handler,                   KERNEL_CS, 0, 0); // #XM SIMD FP Exception

    // Fill vectors 20–31 (reserved by Intel) with the default handler.
    for v in 20..32usize {
        idt[v] = IdtEntry::new(default_handler as u64, KERNEL_CS, 0, 0);
    }

    // ── Hardware / APIC Interrupts ───────────────────────────────────────
    set_handler!(idt, 0x20, apic_timer_handler, KERNEL_CS, 0, 0); // APIC timer

    // Fill remaining vectors with default handler.
    for v in 0x21usize..256 {
        if idt[v].type_attr == 0 {
            idt[v] = IdtEntry::new(default_handler as u64, KERNEL_CS, 0, 0);
        }
    }

    // ── int 0x80 legacy syscall gate ─────────────────────────────────────
    idt[0x80] = IdtEntry::new(syscall_int80_handler as u64, KERNEL_CS, 0, 3);

    // ── Load the IDTR ────────────────────────────────────────────────────
    let idtr = Idtr {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        // SAFETY: `IDT` is a valid static, so its address is a valid
        //         linear address for the CPU to read.
        base: unsafe { &IDT as *const Idt as u64 },
    };

    // SAFETY: `idtr` points to a correctly-sized, properly-aligned IDT;
    //         we are in ring-0 so `lidt` is permitted.
    unsafe {
        asm!(
            "lidt [{0}]",
            in(reg) &idtr as *const Idtr,
            options(nostack, preserves_flags)
        );
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idt_entry_size() {
        assert_eq!(core::mem::size_of::<IdtEntry>(), 16);
    }

    #[test]
    fn idt_entry_missing_is_zero() {
        let e = IdtEntry::missing();
        assert_eq!(e.offset_low,  0);
        assert_eq!(e.type_attr,   0);
        assert_eq!(e.offset_high, 0);
    }

    #[test]
    fn idt_entry_new_encodes_correctly() {
        let handler: u64 = 0xDEAD_BEEF_1234_5678;
        let entry = IdtEntry::new(handler, 0x08, 0, 0);
        // offset_low = bits 15:0
        assert_eq!(entry.offset_low,  0x5678);
        // offset_mid = bits 31:16
        assert_eq!(entry.offset_mid,  0x1234);
        // offset_high = bits 63:32
        assert_eq!(entry.offset_high, 0xDEAD_BEEF);
        // type_attr: P=1, DPL=0, type=0xE  =>  0x8E
        assert_eq!(entry.type_attr,   0x8E);
        assert_eq!(entry.selector,    0x08);
    }

    #[test]
    fn idt_entry_dpl3_encodes_correctly() {
        let entry = IdtEntry::new(0x0, 0x08, 0, 3);
        // 0x8E | (3 << 5) = 0x8E | 0x60 = 0xEE
        assert_eq!(entry.type_attr, 0xEE);
    }

    #[test]
    fn idt_total_size() {
        assert_eq!(core::mem::size_of::<Idt>(), 256 * 16);
    }
}
