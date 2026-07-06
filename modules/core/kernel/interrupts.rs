/// SigmaOS — modules/core/kernel/interrupts.rs
/// x86_64 Interrupt Descriptor Table: 256 entries, IRQ registration, default handler.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU64  = u64;
type SigmaU32  = u32;
type SigmaU16  = u16;
type SigmaU8   = u8;
type SigmaI32  = i32;

// ─── Constants ────────────────────────────────────────────────────────────────

pub const IDT_ENTRIES:     usize = 256;
pub const IRQ_BASE:        SigmaU8 = 32;    // PIC remapped to 0x20
pub const IRQ_TIMER:       SigmaU8 = 32;    // PIT / LAPIC timer
pub const IRQ_KEYBOARD:    SigmaU8 = 33;
pub const IRQ_CASCADE:     SigmaU8 = 34;
pub const IRQ_COM2:        SigmaU8 = 35;
pub const IRQ_COM1:        SigmaU8 = 36;
pub const IRQ_LPT2:        SigmaU8 = 37;
pub const IRQ_FLOPPY:      SigmaU8 = 38;
pub const IRQ_SPURIOUS:    SigmaU8 = 39;
pub const IRQ_RTC:         SigmaU8 = 40;
pub const IRQ_MOUSE:       SigmaU8 = 44;
pub const IRQ_PRIMARY_ATA: SigmaU8 = 46;

pub const GATE_TYPE_INT:   SigmaU8 = 0x8E;  // 64-bit interrupt gate, DPL=0
pub const GATE_TYPE_TRAP:  SigmaU8 = 0x8F;  // 64-bit trap gate, DPL=0
pub const GATE_TYPE_USER:  SigmaU8 = 0xEE;  // 64-bit interrupt gate, DPL=3

// ─── IDT Gate Descriptor ──────────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtGate {
    pub offset_low:  SigmaU16,   // bits 0..15  of handler address
    pub selector:    SigmaU16,   // kernel code segment selector (0x08)
    pub ist:         SigmaU8,    // Interrupt Stack Table index (0 = none)
    pub type_attr:   SigmaU8,    // gate type + DPL + present bit
    pub offset_mid:  SigmaU16,   // bits 16..31 of handler address
    pub offset_high: SigmaU32,   // bits 32..63 of handler address
    pub reserved:    SigmaU32,   // must be zero
}

impl IdtGate {
    pub const fn zero() -> Self {
        IdtGate {
            offset_low: 0, selector: 0, ist: 0,
            type_attr: 0, offset_mid: 0, offset_high: 0, reserved: 0,
        }
    }

    /// Set the gate to point at `handler` function pointer.
    pub fn set_handler(&mut self, handler: u64, type_attr: SigmaU8) {
        self.offset_low  = (handler & 0xFFFF) as SigmaU16;
        self.offset_mid  = ((handler >> 16) & 0xFFFF) as SigmaU16;
        self.offset_high = ((handler >> 32) & 0xFFFF_FFFF) as SigmaU32;
        self.selector    = 0x08;   // kernel CS
        self.ist         = 0;
        self.type_attr   = type_attr;
        self.reserved    = 0;
    }
}

// ─── IDT + IDTR ───────────────────────────────────────────────────────────────

static mut IDT: [IdtGate; IDT_ENTRIES] = [IdtGate::zero(); IDT_ENTRIES];

#[repr(C, packed)]
pub struct Idtr {
    pub limit: SigmaU16,
    pub base:  SigmaU64,
}

static mut IDTR: Idtr = Idtr { limit: 0, base: 0 };

// ─── User-Registered IRQ Handlers ─────────────────────────────────────────────

type IrqHandler = unsafe extern "C" fn(vector: SigmaU8);

static mut IRQ_HANDLERS: [Option<IrqHandler>; IDT_ENTRIES] = [None; IDT_ENTRIES];

// ─── Default Handlers ─────────────────────────────────────────────────────────

extern "C" {
    fn kernel_panic(msg: *const SigmaU8) -> !;
    fn watchdog_tick();
    fn schedule();
}

unsafe extern "C" fn default_fault_handler(vector: SigmaU8) {
    // In production: log the exception frame and trigger self_heal.
    // For now escalate to panic.
    kernel_panic(b"unhandled CPU exception\0".as_ptr());
}

unsafe extern "C" fn timer_irq_handler(vector: SigmaU8) {
    watchdog_tick();
    schedule();
    // Send EOI to LAPIC (write 0 to LAPIC EOI register at 0xFEE000B0)
    #[cfg(target_arch = "x86_64")]
    {
        let lapic_eoi = 0xFEE0_00B0usize as *mut SigmaU32;
        core::ptr::write_volatile(lapic_eoi, 0);
    }
}

// ─── IDT Initialisation ───────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn interrupts_init() -> SigmaI32 {
    // Install default fault handler for all CPU exceptions (0..31)
    for vec in 0..32usize {
        IDT[vec].set_handler(default_fault_handler as u64, GATE_TYPE_INT);
        IRQ_HANDLERS[vec] = Some(default_fault_handler);
    }

    // Install hardware timer on vector 32
    IDT[IRQ_TIMER as usize].set_handler(timer_irq_handler as u64, GATE_TYPE_INT);
    IRQ_HANDLERS[IRQ_TIMER as usize] = Some(timer_irq_handler);

    // All remaining hardware IRQs start with the default (no-op + EOI)
    for vec in 33..IDT_ENTRIES {
        IDT[vec].set_handler(default_fault_handler as u64, GATE_TYPE_INT);
    }

    // Build the IDTR
    IDTR.base  = IDT.as_ptr() as SigmaU64;
    IDTR.limit = (core::mem::size_of::<[IdtGate; IDT_ENTRIES]>() - 1) as SigmaU16;

    // Load the IDT
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("lidt [{}]", in(reg) &IDTR, options(readonly, nostack, preserves_flags));

    // Enable hardware interrupts
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("sti");

    0
}

/// Register a custom IRQ handler for `vector`.
/// Returns 0 on success, -1 if vector is out of range.
#[no_mangle]
pub unsafe extern "C" fn sigma_irq_install(
    vector: SigmaU8,
    handler: IrqHandler,
) -> SigmaI32 {
    let idx = vector as usize;
    if idx >= IDT_ENTRIES { return -1; }
    IDT[idx].set_handler(handler as u64, GATE_TYPE_INT);
    IRQ_HANDLERS[idx] = Some(handler);
    0
}

/// Dispatch an interrupt — called from the low-level ASM stubs.
#[no_mangle]
pub unsafe extern "C" fn irq_dispatch(vector: SigmaU8) {
    let idx = vector as usize;
    if let Some(handler) = IRQ_HANDLERS[idx] {
        handler(vector);
    }
}

/// Disable all hardware interrupts (CLI).
#[no_mangle]
pub unsafe extern "C" fn interrupts_disable() {
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("cli");
}

/// Enable all hardware interrupts (STI).
#[no_mangle]
pub unsafe extern "C" fn interrupts_enable() {
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("sti");
}
