/// SigmaOS Sovereign Interrupt Descriptor Table (IDT)
/// Migrated from C++ to Rust — no_std, no alloc, no external crates.
/// Provides a 256-entry IDT for CPU exceptions and hardware IRQs.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;

// ─── IDT Entry (packed, matches x86_64 hardware spec) ───────────────────────

/// A single 16-byte entry in the 64-bit Interrupt Descriptor Table.
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    base_low:  SigmaU16,   // Handler address bits [15:0]
    selector:  SigmaU16,   // Kernel code segment selector
    ist:       SigmaU8,    // Interrupt Stack Table offset
    flags:     SigmaU8,    // Type and attributes (P, DPL, Gate Type)
    base_mid:  SigmaU16,   // Handler address bits [31:16]
    base_high: SigmaU32,   // Handler address bits [63:32]
    reserved:  SigmaU32,   // Must be zero
}

impl IdtEntry {
    /// A zeroed IDT entry (not-present).
    const fn zero() -> Self {
        Self {
            base_low:  0,
            selector:  0,
            ist:       0,
            flags:     0,
            base_mid:  0,
            base_high: 0,
            reserved:  0,
        }
    }

    /// Encode a handler address, selector, and flags into this entry.
    fn set(&mut self, handler: SigmaU64, selector: SigmaU16, flags: SigmaU8) {
        self.base_low  = (handler & 0xFFFF) as SigmaU16;
        self.base_mid  = ((handler >> 16) & 0xFFFF) as SigmaU16;
        self.base_high = ((handler >> 32) & 0xFFFFFFFF) as SigmaU32;
        self.selector  = selector;
        self.flags     = flags;
        self.ist       = 0;
        self.reserved  = 0;
    }
}

// ─── IDT Pointer (loaded by LIDT instruction) ──────────────────────────────

/// Pointer structure passed to the `lidt` instruction.
#[repr(C, packed)]
struct IdtPointer {
    limit: SigmaU16,
    base:  SigmaU64,
}

// ─── Gate Type Constants ────────────────────────────────────────────────────

/// IDT gate type/attribute constants.
struct GateFlags;
impl GateFlags {
    /// Present, Ring 0, 64-bit Interrupt Gate
    const INTERRUPT_GATE_RING0: SigmaU8 = 0x8E;
    /// Present, Ring 0, 64-bit Trap Gate
    const TRAP_GATE_RING0: SigmaU8 = 0x8F;
    /// Present, Ring 3, 64-bit Interrupt Gate (for syscalls)
    const INTERRUPT_GATE_RING3: SigmaU8 = 0xEE;
}

/// Number of IDT entries (CPU supports 256 vectors).
const IDT_ENTRY_COUNT: usize = 256;

/// Kernel code segment selector (GDT entry 1 × 8).
const KERNEL_CS: SigmaU16 = 0x08;

// ─── SovereignIDT ───────────────────────────────────────────────────────────

/// The Sovereign Interrupt Descriptor Table.
/// Maps 256 interrupt vectors to handler addresses.
struct SovereignIDT {
    entries: [IdtEntry; IDT_ENTRY_COUNT],
    pointer: IdtPointer,
}

impl SovereignIDT {
    /// Construct a new IDT with all entries zeroed (not-present).
    const fn new() -> Self {
        Self {
            entries: [IdtEntry::zero(); IDT_ENTRY_COUNT],
            pointer: IdtPointer { limit: 0, base: 0 },
        }
    }

    /// Set the handler for a specific interrupt vector.
    fn set_entry(&mut self, vector: SigmaU8, handler: SigmaU64, selector: SigmaU16, flags: SigmaU8) {
        self.entries[vector as usize].set(handler, selector, flags);
    }

    /// Initialize the IDT.
    /// Zeros all entries, sets up the first 32 CPU exception vectors as
    /// Ring 0 Interrupt Gates, then loads the IDT via `lidt`.
    fn init(&mut self) {
        // Zero-fill all entries (defensive)
        let mut i: usize = 0;
        while i < IDT_ENTRY_COUNT {
            self.entries[i] = IdtEntry::zero();
            i += 1;
        }

        // Map CPU exception vectors 0..31 as Interrupt Gates
        // Handler address is 0 (stubs — to be wired to actual ISR trampolines)
        let mut vector: SigmaU8 = 0;
        while vector < 32 {
            self.set_entry(vector, 0, KERNEL_CS, GateFlags::INTERRUPT_GATE_RING0);
            vector += 1;
        }

        // Set the IDT pointer for lidt
        let idt_size = core::mem::size_of::<[IdtEntry; IDT_ENTRY_COUNT]>() as SigmaU16;
        self.pointer.limit = idt_size - 1;
        self.pointer.base = &self.entries as *const _ as SigmaU64;

        self.load();
    }

    /// Load the IDT into the CPU via the `lidt` instruction.
    #[inline(never)]
    fn load(&self) {
        unsafe {
            core::arch::asm!(
                "lidt [{idt_ptr}]",
                idt_ptr = in(reg) &self.pointer,
                options(preserves_flags, nostack),
            );
        }
    }
}

// ─── Global Singleton ───────────────────────────────────────────────────────

/// The single IDT instance.
static mut IDT: SovereignIDT = SovereignIDT::new();

// ─── C-ABI Bridge ───────────────────────────────────────────────────────────

/// C-callable entry point for IDT initialization.
/// Replaces the original `extern "C" void idt_init()`.
#[no_mangle]
pub unsafe extern "C" fn idt_init() {
    IDT.init();
}
