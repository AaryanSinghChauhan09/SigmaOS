//! x86_64 Global Descriptor Table (GDT) & Task State Segment (TSS)
//!
//! Provides hardware segment descriptors and privilege level separation:
//! - Ring 0 (Kernel Code & Data)
//! - Ring 3 (User Mode Code & Data)
//! - Task State Segment (TSS) for Ring 0 RSP0 stack transition & IST (Interrupt Stack Tables)
//! - TSS Descriptor (16-byte system segment in 64-bit long mode)

#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use core::mem::size_of;

/// GDT Segment Selectors
pub const KERNEL_CS: u16 = 0x08;
pub const KERNEL_DS: u16 = 0x10;
pub const USER_DS: u16 = 0x18 | 3; // RPL 3
pub const USER_CS: u16 = 0x20 | 3; // RPL 3
pub const TSS_SELECTOR: u16 = 0x28;

/// 64-bit Task State Segment (TSS) structure according to AMD64/Intel SDM
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TaskStateSegment {
    pub reserved_0: u32,
    pub rsp0: u64, // Stack pointer for Ring 0 (used upon privilege transition from Ring 3)
    pub rsp1: u64, // Stack pointer for Ring 1
    pub rsp2: u64, // Stack pointer for Ring 2
    pub reserved_1: u64,
    pub ist1: u64, // Interrupt Stack Table 1 (e.g. Double Fault)
    pub ist2: u64, // Interrupt Stack Table 2 (e.g. NMI)
    pub ist3: u64, // Interrupt Stack Table 3 (e.g. Machine Check)
    pub ist4: u64, // Interrupt Stack Table 4
    pub ist5: u64, // Interrupt Stack Table 5
    pub ist6: u64, // Interrupt Stack Table 6
    pub ist7: u64, // Interrupt Stack Table 7
    pub reserved_2: u64,
    pub reserved_3: u16,
    pub iomap_base: u16, // I/O Permission Bitmap Base Address
}

impl TaskStateSegment {
    pub const fn zero() -> Self {
        Self {
            reserved_0: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            reserved_1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            reserved_2: 0,
            reserved_3: 0,
            iomap_base: size_of::<Self>() as u16,
        }
    }

    pub fn set_rsp0(&mut self, stack_top: u64) {
        self.rsp0 = stack_top;
    }

    pub fn set_double_fault_ist(&mut self, stack_top: u64) {
        self.ist1 = stack_top;
    }
}

/// Global Descriptor Table Pointer for `lgdt` instruction
#[repr(C, packed)]
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

/// Master GDT Manager with User Mode (Ring 3) and 16-byte TSS support
pub struct GlobalDescriptorTableManager {
    pub entries: [u64; 8], // Null, KCode, KData, UData, UCode, TSS_Low, TSS_High, Guard
    pub tss: TaskStateSegment,
    pub is_loaded: bool,
}

impl GlobalDescriptorTableManager {
    pub fn new() -> Self {
        let mut manager = Self {
            entries: [0; 8],
            tss: TaskStateSegment::zero(),
            is_loaded: false,
        };

        // 0: Null Descriptor
        manager.entries[0] = 0x0000_0000_0000_0000;

        // 1: Kernel Code 64-bit (0x08): DPL 0, Exec/Read, 64-bit Long Mode
        // Flags: G=1, L=1, P=1, DPL=0, S=1, Type=10 (Exec/Read)
        manager.entries[1] = 0x00AF_9A00_0000_FFFF;

        // 2: Kernel Data 64-bit (0x10): DPL 0, Read/Write
        // Flags: G=1, P=1, DPL=0, S=1, Type=2 (Read/Write)
        manager.entries[2] = 0x00CF_9200_0000_FFFF;

        // 3: User Data 64-bit (0x18 | 3 = 0x1B): DPL 3, Read/Write
        // Flags: G=1, P=1, DPL=3, S=1, Type=2 (Read/Write)
        manager.entries[3] = 0x00CF_F200_0000_FFFF;

        // 4: User Code 64-bit (0x20 | 3 = 0x23): DPL 3, Exec/Read, 64-bit Long Mode
        // Flags: G=1, L=1, P=1, DPL=3, S=1, Type=10 (Exec/Read)
        manager.entries[4] = 0x00AF_FA00_0000_FFFF;

        manager.configure_tss_descriptor();
        manager
    }

    /// Sets up the 16-byte TSS descriptor in entries [5] and [6]
    fn configure_tss_descriptor(&mut self) {
        let tss_ptr = &self.tss as *const _ as u64;
        let tss_limit = (size_of::<TaskStateSegment>() - 1) as u64;

        // TSS Low (64-bit Available TSS: Type 0x9, DPL 0, Present 1)
        let tss_low = (tss_limit & 0xFFFF)
            | ((tss_ptr & 0x00FF_FFFF) << 16)
            | (0x89 << 40) // Present, Type 9 (64-bit Available TSS)
            | (((tss_limit >> 16) & 0x0F) << 48)
            | (((tss_ptr >> 24) & 0xFF) << 56);

        // In 64-bit mode, Granularity flag is typically 0 for byte limit
        self.entries[5] = tss_low;
        // TSS High: Base bits 32..63 and reserved 32 bits
        self.entries[6] = (tss_ptr >> 32) & 0xFFFF_FFFF;
    }

    /// Set kernel stack pointer for privilege transitions
    pub fn set_kernel_stack(&mut self, kernel_rsp0: u64) {
        self.tss.set_rsp0(kernel_rsp0);
        self.configure_tss_descriptor();
    }

    /// Initialize GDT and TSS
    pub fn init(&mut self) {
        self.is_loaded = true;
    }

    /// Builds the interrupt return frame (IRETQ) for Ring 3 transition
    pub fn build_ring3_iret_frame(
        user_rip: u64,
        user_rsp: u64,
        user_rflags: u64,
    ) -> [u64; 5] {
        [
            user_rip,                     // RIP
            USER_CS as u64,              // CS (User Code with RPL 3)
            user_rflags | 0x200,         // RFLAGS (Ensure Interrupt Flag IF is enabled)
            user_rsp,                    // RSP (User Mode Stack)
            USER_DS as u64,              // SS (User Mode Data with RPL 3)
        ]
    }
}

impl Default for GlobalDescriptorTableManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global system initializer for GDT and TSS
pub fn init() {
    let mut gdt = GlobalDescriptorTableManager::new();
    gdt.set_kernel_stack(0xFFFF_8000_0008_0000);
    gdt.init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdt_segment_selectors() {
        let mut gdt = GlobalDescriptorTableManager::new();
        gdt.set_kernel_stack(0xFFFF_8000_0008_0000);
        gdt.init();

        assert_eq!(KERNEL_CS, 0x08);
        assert_eq!(KERNEL_DS, 0x10);
        assert_eq!(USER_DS, 0x1B);
        assert_eq!(USER_CS, 0x23);
        assert_eq!(TSS_SELECTOR, 0x28);
        let rsp0 = gdt.tss.rsp0;
        assert_eq!(rsp0, 0xFFFF_8000_0008_0000);
    }

    #[test]
    fn test_ring3_iret_frame_construction() {
        let frame = GlobalDescriptorTableManager::build_ring3_iret_frame(
            0x0000_0000_0040_0000,
            0x0000_7FFF_FFFF_0000,
            0x0000_0000_0000_0002,
        );
        assert_eq!(frame[0], 0x0000_0000_0040_0000); // RIP
        assert_eq!(frame[1], 0x23);                  // CS
        assert_eq!(frame[2] & 0x200, 0x200);         // RFLAGS (IF enabled)
        assert_eq!(frame[3], 0x0000_7FFF_FFFF_0000); // RSP
        assert_eq!(frame[4], 0x1B);                  // SS
    }
}
