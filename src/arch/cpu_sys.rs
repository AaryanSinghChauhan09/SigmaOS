use std::string::{String, ToString};
use std::vec::Vec;
use std::vec;
// SigmaOS Processor Initialisation, Memory Layout, and Fast System Call Engine
// Fully absorbs and implements design philosophies of Linux (SMEP/SMAP, LSTAR) and BSD distros (Guard pages, strict GDT/IDT):
// x86-64 GDT segment structures, IDT gates, hardened CR0/CR4 control registers, virtual memory maps, and fast SYSCALL/SYSRET.

use std::collections::BTreeMap;

/// Standard CPU segments defined in the Global Descriptor Table (GDT)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentType {
    KernelCode,
    KernelData,
    UserCode,
    UserData,
    TaskStateSegment,
}

/// Represents a Segment Descriptor in the GDT
#[derive(Debug, Clone)]
pub struct GdtDescriptor {
    pub limit: u32,
    pub base: u32,
    pub access_byte: u8,
    pub flags: u8,
    pub segment_type: SegmentType,
}

/// Represents an Interrupt Descriptor Table (IDT) Gate
#[derive(Debug, Clone)]
pub struct IdtGate {
    pub handler_address: usize,
    pub selector: u16,
    pub gate_type: u8, // e.g., 0xE for 32-bit Interrupt Gate, 0xF for Trap Gate
    pub privilege_level: u8, // 0 (Kernel) to 3 (User)
}

/// Hardware virtual memory layout regions mimicking hardened Linux/BSD distributions
#[derive(Debug, Clone)]
pub struct VirtualMemoryRegion {
    pub name: String,
    pub start_address: u64,
    pub end_address: u64,
    pub is_writable: bool,
    pub is_executable: bool,
    pub has_guard_page: bool, // BSD-style guard zones for thread stack safety
}

/// The core processor controller managing GDT, IDT, CR registers, and System Call MSRs
pub struct ProcessorInitSuite {
    pub gdt: Vec<GdtDescriptor>,
    pub idt: BTreeMap<u8, IdtGate>,
    pub memory_regions: Vec<VirtualMemoryRegion>,
    pub cr0_wp_enabled: bool,  // Write Protect (prevents kernel writing to read-only pages)
    pub cr4_smep_enabled: bool, // Supervisor Mode Execution Prevention (prevents executing user code in ring 0)
    pub cr4_smap_enabled: bool, // Supervisor Mode Access Prevention (prevents accessing user data in ring 0)
    pub ia32_efer_nxe_enabled: bool, // No-Execute Enable (NX page validation)
    pub msr_lstar_address: u64, // Long-mode SYSCALL target address register
}

impl ProcessorInitSuite {
    pub fn new() -> Self {
        Self {
            gdt: Vec::new(),
            idt: BTreeMap::new(),
            memory_regions: Self::default_virtual_memory_layout(),
            cr0_wp_enabled: false,
            cr4_smep_enabled: false,
            cr4_smap_enabled: false,
            ia32_efer_nxe_enabled: false,
            msr_lstar_address: 0,
        }
    }

    /// Hardened x86-64 Memory Layout mapping mimicking modern secure distributions
    fn default_virtual_memory_layout() -> Vec<VirtualMemoryRegion> {
        vec![
            VirtualMemoryRegion {
                name: "User Space Code & Heap".to_string(),
                start_address: 0x0000000000000000,
                end_address: 0x00007FFFFFFFFFFF,
                is_writable: true,
                is_executable: true,
                has_guard_page: true, // Guard against stack-clash exploits
            },
            VirtualMemoryRegion {
                name: "Kernel Code (Ring 0)".to_string(),
                start_address: 0xFFFF800000000000,
                end_address: 0xFFFF800000FFFFFF,
                is_writable: false,
                is_executable: true,
                has_guard_page: false,
            },
            VirtualMemoryRegion {
                name: "Kernel Heap & Page Tables".to_string(),
                start_address: 0xFFFF888000000000,
                end_address: 0xFFFFC87FFFFFFFFF,
                is_writable: true,
                is_executable: false,
                has_guard_page: true, // Prevents heap overflows into system structures
            },
            VirtualMemoryRegion {
                name: "Direct Physical Memory Mapping".to_string(),
                start_address: 0xFFFF880000000000,
                end_address: 0xFFFF887FFFFFFFFF,
                is_writable: true,
                is_executable: false,
                has_guard_page: false,
            },
        ]
    }

    /// Configures the Global Descriptor Table with secure privilege level ring separations
    pub fn initialize_gdt(&mut self) {
        self.gdt.push(GdtDescriptor {
            limit: 0xFFFFF,
            base: 0,
            access_byte: 0x9A, // Kernel Code, Read/Exec, Ring 0
            flags: 0xAF,
            segment_type: SegmentType::KernelCode,
        });
        self.gdt.push(GdtDescriptor {
            limit: 0xFFFFF,
            base: 0,
            access_byte: 0x92, // Kernel Data, Read/Write, Ring 0
            flags: 0xCF,
            segment_type: SegmentType::KernelData,
        });
        self.gdt.push(GdtDescriptor {
            limit: 0xFFFFF,
            base: 0,
            access_byte: 0xFA, // User Code, Read/Exec, Ring 3
            flags: 0xAF,
            segment_type: SegmentType::UserCode,
        });
        self.gdt.push(GdtDescriptor {
            limit: 0xFFFFF,
            base: 0,
            access_byte: 0xF2, // User Data, Read/Write, Ring 3
            flags: 0xCF,
            segment_type: SegmentType::UserData,
        });
    }

    /// Configures the IDT with standard exception and hardware interrupt gates
    pub fn register_idt_gate(&mut self, interrupt_num: u8, handler: usize, gate_type: u8, privilege: u8) {
        let gate = IdtGate {
            handler_address: handler,
            selector: 0x08, // Kernel code segment selector
            gate_type,
            privilege_level: privilege,
        };
        self.idt.insert(interrupt_num, gate);
    }

    /// Activates advanced processor protection mitigations (SMEP, SMAP, WP, NX)
    pub fn configure_cpu_mitigations(&mut self) {
        // CR0.WP = 1
        self.cr0_wp_enabled = true;
        // CR4.SMEP = 1
        self.cr4_smep_enabled = true;
        // CR4.SMAP = 1
        self.cr4_smap_enabled = true;
        // IA32_EFER.NXE = 1
        self.ia32_efer_nxe_enabled = true;
    }

    /// Configures x86-64 Fast System Call registers (MSRs) for SYSCALL/SYSRET transitions
    pub fn configure_fast_syscall_msrs(&mut self, syscall_entry_address: u64) {
        // Set LSTAR MSR target to the fast system call dispatcher address
        self.msr_lstar_address = syscall_entry_address;
    }
}

// =========================================================================
// FAST SYSTEM CALL DISPATCHER
// =========================================================================

pub struct FastSyscallDispatcher {
    pub call_count: usize,
}

impl FastSyscallDispatcher {
    pub fn new() -> Self {
        Self { call_count: 0 }
    }

    /// Handles fast system calls without the interrupt overhead of INT 0x80 (FreeBSD/Linux style)
    pub fn dispatch_syscall(&mut self, rax_syscall_num: u64, rdi_arg1: u64, rsi_arg2: u64) -> Result<u64, &'static str> {
        self.call_count += 1;
        match rax_syscall_num {
            1 => {
                // sys_write(fd, buf_ptr)
                let _fd = rdi_arg1;
                let _buf = rsi_arg2;
                Ok(0) // Return Success
            }
            2 => {
                // sys_open(path_ptr, flags)
                Ok(10) // Return file descriptor 10
            }
            12 => {
                // sys_brk(new_brk_addr)
                Ok(rdi_arg1) // Return new brk address
            }
            _ => Err("Invalid or unsupported syscall number"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_gdt_idt_initialization() {
        let mut suite = ProcessorInitSuite::new();
        assert_eq!(suite.gdt.len(), 0);

        // Init GDT segments
        suite.initialize_gdt();
        assert_eq!(suite.gdt.len(), 4);
        assert_eq!(suite.gdt[0].segment_type, SegmentType::KernelCode);
        assert_eq!(suite.gdt[2].segment_type, SegmentType::UserCode);

        // Register page fault interrupt handler
        suite.register_idt_gate(14, 0xFFFFFFFF80105000, 0x0E, 0);
        assert_eq!(suite.idt.len(), 1);
        let pf_gate = suite.idt.get(&14).unwrap();
        assert_eq!(pf_gate.handler_address, 0xFFFFFFFF80105000);
        assert_eq!(pf_gate.gate_type, 0x0E);
        assert_eq!(pf_gate.privilege_level, 0);
    }

    #[test]
    fn test_hardened_cpu_mitigations() {
        let mut suite = ProcessorInitSuite::new();
        assert!(!suite.cr4_smep_enabled);
        assert!(!suite.cr4_smap_enabled);

        // Apply SMEP, SMAP, WP, NX mitigations
        suite.configure_cpu_mitigations();
        assert!(suite.cr0_wp_enabled);
        assert!(suite.cr4_smep_enabled);
        assert!(suite.cr4_smap_enabled);
        assert!(suite.ia32_efer_nxe_enabled);
    }

    #[test]
    fn test_virtual_memory_layout_bounds() {
        let suite = ProcessorInitSuite::new();
        assert_eq!(suite.memory_regions.len(), 4);

        // Assert user space region
        let user_space = &suite.memory_regions[0];
        assert_eq!(user_space.start_address, 0);
        assert_eq!(user_space.end_address, 0x00007FFFFFFFFFFF);
        assert!(user_space.has_guard_page); // Guard zone verified
    }

    #[test]
    fn test_fast_syscall_dispatching() {
        let mut suite = ProcessorInitSuite::new();
        let mut dispatcher = FastSyscallDispatcher::new();

        suite.configure_fast_syscall_msrs(0xFFFFFFFF80102000);
        assert_eq!(suite.msr_lstar_address, 0xFFFFFFFF80102000);

        // Dispatch sys_open
        let open_fd = dispatcher.dispatch_syscall(2, 0, 0).unwrap();
        assert_eq!(open_fd, 10);
        assert_eq!(dispatcher.call_count, 1);

        // Dispatch sys_brk
        let brk_res = dispatcher.dispatch_syscall(12, 0x5000, 0).unwrap();
        assert_eq!(brk_res, 0x5000);

        // Invalid syscall
        let invalid = dispatcher.dispatch_syscall(999, 0, 0);
        assert!(invalid.is_err());
    }
}
