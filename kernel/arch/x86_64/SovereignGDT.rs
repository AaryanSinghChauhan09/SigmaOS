/// SigmaOS Sovereign Global Descriptor Table (GDT)
/// Migrated from C++ to Rust — no_std, no alloc, no external crates.
/// OOP via struct + impl pattern with singleton access.

#![no_std]
#![allow(dead_code)]

/// Custom kernel types — hand-defined, no stdlib dependency.
type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;

// ─── GDT Entry (packed, C-repr for hardware compatibility) ──────────────────

/// A single entry in the Global Descriptor Table.
/// Layout matches the x86_64 hardware specification exactly.
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GdtEntry {
    limit_low:   SigmaU16,
    base_low:    SigmaU16,
    base_mid:    SigmaU8,
    access:      SigmaU8,
    granularity: SigmaU8,
    base_high:   SigmaU8,
}

impl GdtEntry {
    /// Create a zeroed GDT entry.
    const fn zero() -> Self {
        Self {
            limit_low:   0,
            base_low:    0,
            base_mid:    0,
            access:      0,
            granularity: 0,
            base_high:   0,
        }
    }
}

// ─── GDT Pointer (loaded by LGDT instruction) ──────────────────────────────

/// Pointer structure passed to the `lgdt` instruction.
#[repr(C, packed)]
struct GdtPointer {
    limit: SigmaU16,
    base:  SigmaU64,
}

// ─── Segment Constants ──────────────────────────────────────────────────────

/// Access byte constants for GDT entries.
struct SegmentAccess;
impl SegmentAccess {
    const KERNEL_CODE: SigmaU8 = 0x9A;  // Execute/Read, Ring 0
    const KERNEL_DATA: SigmaU8 = 0x92;  // Read/Write, Ring 0
    const USER_CODE:   SigmaU8 = 0xFA;  // Execute/Read, Ring 3
    const USER_DATA:   SigmaU8 = 0xF2;  // Read/Write, Ring 3
}

/// Granularity byte constants.
struct SegmentGranularity;
impl SegmentGranularity {
    const CODE_64BIT: SigmaU8 = 0xAF;   // 64-bit code segment, 4K pages
    const DATA:       SigmaU8 = 0xCF;   // 32-bit compatible data, 4K pages
}

// ─── SovereignGDT (Singleton via static mut) ────────────────────────────────

/// The Sovereign Global Descriptor Table.
/// Standardizes segments for Ring 0 (kernel) and Ring 3 (user) operation.
/// Provides a singleton interface matching the original C++ design.
struct SovereignGDT {
    entries: [GdtEntry; 5],
    pointer: GdtPointer,
}

impl SovereignGDT {
    /// Construct a new GDT with all entries zeroed.
    const fn new() -> Self {
        Self {
            entries: [GdtEntry::zero(); 5],
            pointer: GdtPointer { limit: 0, base: 0 },
        }
    }

    /// Set a GDT entry at the given index.
    /// Encodes base, limit, access, and granularity into the packed format.
    fn set_entry(
        &mut self,
        index:       usize,
        base:        SigmaU32,
        limit:       SigmaU32,
        access:      SigmaU8,
        granularity: SigmaU8,
    ) {
        self.entries[index].base_low    = (base & 0xFFFF) as SigmaU16;
        self.entries[index].base_mid    = ((base >> 16) & 0xFF) as SigmaU8;
        self.entries[index].base_high   = ((base >> 24) & 0xFF) as SigmaU8;
        self.entries[index].limit_low   = (limit & 0xFFFF) as SigmaU16;
        self.entries[index].granularity = ((limit >> 16) & 0x0F) as SigmaU8;
        self.entries[index].granularity |= granularity & 0xF0;
        self.entries[index].access      = access;
    }

    /// Initialize the GDT with the standard 5-entry layout.
    /// Entry 0: Null segment (required by CPU)
    /// Entry 1: Kernel code (64-bit, Ring 0)
    /// Entry 2: Kernel data (Ring 0)
    /// Entry 3: User code (64-bit, Ring 3)
    /// Entry 4: User data (Ring 3)
    fn init(&mut self) {
        // Null segment
        self.set_entry(0, 0, 0, 0, 0);
        // Kernel Code (64-bit)
        self.set_entry(1, 0, 0xFFFFFFFF, SegmentAccess::KERNEL_CODE, SegmentGranularity::CODE_64BIT);
        // Kernel Data
        self.set_entry(2, 0, 0xFFFFFFFF, SegmentAccess::KERNEL_DATA, SegmentGranularity::DATA);
        // User Code (64-bit)
        self.set_entry(3, 0, 0xFFFFFFFF, SegmentAccess::USER_CODE, SegmentGranularity::CODE_64BIT);
        // User Data
        self.set_entry(4, 0, 0xFFFFFFFF, SegmentAccess::USER_DATA, SegmentGranularity::DATA);

        // Set the GDT pointer for lgdt
        let gdt_size = core::mem::size_of::<[GdtEntry; 5]>() as SigmaU16;
        self.pointer.limit = gdt_size - 1;
        self.pointer.base = &self.entries as *const _ as SigmaU64;

        self.load();
    }

    /// Load the GDT via the `lgdt` instruction, then reload all segment registers.
    /// Performs a far return to reload CS with the kernel code segment selector (0x08).
    /// Sets DS, ES, FS, GS, SS to the kernel data segment selector (0x10).
    #[inline(never)]
    fn load(&self) {
        unsafe {
            core::arch::asm!(
                "lgdt [{gdt_ptr}]",
                "push 0x08",
                "lea rax, [rip + 2f]",
                "push rax",
                "retfq",
                "2:",
                "mov ax, 0x10",
                "mov ds, ax",
                "mov es, ax",
                "mov fs, ax",
                "mov gs, ax",
                "mov ss, ax",
                gdt_ptr = in(reg) &self.pointer,
                out("rax") _,
                options(preserves_flags),
            );
        }
    }
}

// ─── Global Singleton ───────────────────────────────────────────────────────

/// The single GDT instance for the BSP (bootstrap processor).
static mut GDT: SovereignGDT = SovereignGDT::new();

// ─── C-ABI Bridge ───────────────────────────────────────────────────────────

/// C-callable entry point for GDT initialization.
/// Replaces the original `extern "C" void gdt_init()`.
#[no_mangle]
pub unsafe extern "C" fn gdt_init() {
    GDT.init();
}
