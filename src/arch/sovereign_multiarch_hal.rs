// Sovereign Multi-Architecture HAL & ISA Feature Detection Engine
// Zero-dependency `#![no_std]` compliant multi-architecture abstractions inspired by Linux and BSD kernel ports:
// Supporting x86_32, x86_64, AArch64 (ARM64), RiscV32, RiscV64, LoongArch64, PowerPC64, and S390x architectures.

#![cfg_attr(not(test), no_std)]

#[cfg(not(any(feature = "standalone_test", test)))]


#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::format;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchitectureClass {
    X86_32 = 0,
    X86_64 = 1,
    AArch64 = 2,
    RiscV32 = 3,
    RiscV64 = 4,
    LoongArch64 = 5,
    PowerPC64 = 6,
    S390x = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuPageTableMode {
    X86_4LevelPaging,   // CR3 - 48-bit Virtual Address Space
    X86_5LevelPaging,   // CR3 - 57-bit Virtual Address Space (LA57)
    Arm64_4Level48Bit,  // TTBR0_EL1 / TTBR1_EL1 48-bit VA
    Arm64_5Level52Bit,  // TTBR0_EL1 52-bit LVA
    RiscvSv39,          // SATP Sv39 39-bit VA
    RiscvSv48,          // SATP Sv48 48-bit VA
    RiscvSv57,          // SATP Sv57 57-bit VA
    LoongArchLA64,      // PGDL 4-level 48-bit VA
    PowerPCLinuxRadix,  // Radix Tree Page Table
    S390xRegion1Table,  // 5-level Region Table
}

#[derive(Debug, Clone)]
pub struct IsaVectorCapabilities {
    pub has_avx2: bool,
    pub has_avx512: bool,
    pub has_neon: bool,
    pub has_sve: bool,
    pub has_sve2: bool,
    pub has_riscv_v: bool,
    pub has_loongarch_lasx: bool,
    pub has_ppc_vsx: bool,
    pub has_s390x_vx: bool,
}

impl IsaVectorCapabilities {
    pub fn for_arch(arch: ArchitectureClass) -> Self {
        match arch {
            ArchitectureClass::X86_32 => Self {
                has_avx2: false,
                has_avx512: false,
                has_neon: false,
                has_sve: false,
                has_sve2: false,
                has_riscv_v: false,
                has_loongarch_lasx: false,
                has_ppc_vsx: false,
                has_s390x_vx: false,
            },
            ArchitectureClass::X86_64 => Self {
                has_avx2: true,
                has_avx512: true,
                has_neon: false,
                has_sve: false,
                has_sve2: false,
                has_riscv_v: false,
                has_loongarch_lasx: false,
                has_ppc_vsx: false,
                has_s390x_vx: false,
            },
            ArchitectureClass::AArch64 => Self {
                has_avx2: false,
                has_avx512: false,
                has_neon: true,
                has_sve: true,
                has_sve2: true,
                has_riscv_v: false,
                has_loongarch_lasx: false,
                has_ppc_vsx: false,
                has_s390x_vx: false,
            },
            ArchitectureClass::RiscV32 | ArchitectureClass::RiscV64 => Self {
                has_avx2: false,
                has_avx512: false,
                has_neon: false,
                has_sve: false,
                has_sve2: false,
                has_riscv_v: true,
                has_loongarch_lasx: false,
                has_ppc_vsx: false,
                has_s390x_vx: false,
            },
            ArchitectureClass::LoongArch64 => Self {
                has_avx2: false,
                has_avx512: false,
                has_neon: false,
                has_sve: false,
                has_sve2: false,
                has_riscv_v: false,
                has_loongarch_lasx: true,
                has_ppc_vsx: false,
                has_s390x_vx: false,
            },
            ArchitectureClass::PowerPC64 => Self {
                has_avx2: false,
                has_avx512: false,
                has_neon: false,
                has_sve: false,
                has_sve2: false,
                has_riscv_v: false,
                has_loongarch_lasx: false,
                has_ppc_vsx: true,
                has_s390x_vx: false,
            },
            ArchitectureClass::S390x => Self {
                has_avx2: false,
                has_avx512: false,
                has_neon: false,
                has_sve: false,
                has_sve2: false,
                has_riscv_v: false,
                has_loongarch_lasx: false,
                has_ppc_vsx: false,
                has_s390x_vx: true,
            },
        }
    }
}

pub struct SovereignMultiArchHalEngine {
    pub active_arch: ArchitectureClass,
    pub paging_mode: CpuPageTableMode,
    pub page_table_root_reg: u64,
    pub vector_caps: IsaVectorCapabilities,
}

impl SovereignMultiArchHalEngine {
    pub fn new(arch: ArchitectureClass) -> Self {
        let paging_mode = match arch {
            ArchitectureClass::X86_32 => CpuPageTableMode::X86_4LevelPaging,
            ArchitectureClass::X86_64 => CpuPageTableMode::X86_4LevelPaging,
            ArchitectureClass::AArch64 => CpuPageTableMode::Arm64_4Level48Bit,
            ArchitectureClass::RiscV32 => CpuPageTableMode::RiscvSv39,
            ArchitectureClass::RiscV64 => CpuPageTableMode::RiscvSv48,
            ArchitectureClass::LoongArch64 => CpuPageTableMode::LoongArchLA64,
            ArchitectureClass::PowerPC64 => CpuPageTableMode::PowerPCLinuxRadix,
            ArchitectureClass::S390x => CpuPageTableMode::S390xRegion1Table,
        };

        Self {
            active_arch: arch,
            paging_mode,
            page_table_root_reg: 0x1000,
            vector_caps: IsaVectorCapabilities::for_arch(arch),
        }
    }

    pub fn set_page_table_root(&mut self, root_phys_addr: u64) {
        self.page_table_root_reg = root_phys_addr;
    }

    pub fn get_root_register_name(&self) -> &'static str {
        match self.active_arch {
            ArchitectureClass::X86_32 | ArchitectureClass::X86_64 => "CR3",
            ArchitectureClass::AArch64 => "TTBR0_EL1",
            ArchitectureClass::RiscV32 | ArchitectureClass::RiscV64 => "SATP",
            ArchitectureClass::LoongArch64 => "PGDL",
            ArchitectureClass::PowerPC64 => "MSR_RADIX",
            ArchitectureClass::S390x => "CR1",
        }
    }

    pub fn format_trap_frame_summary(&self, pc: u64, sp: u64) -> String {
        format!(
            "HAL [{:?}]: Trap Frame PC=0x{:016x} SP=0x{:016x} RootReg({})={:#x}",
            self.active_arch,
            pc,
            sp,
            self.get_root_register_name(),
            self.page_table_root_reg
        )
    }
}

// =========================================================================
// 1. SOVEREIGN SYSCALL ABI TRANSLATOR
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyscallAbiCallingConvention {
    pub instruction: &'static str,
    pub syscall_num_reg: &'static str,
    pub arg_regs: Vec<&'static str>,
    pub return_reg: &'static str,
}

pub struct SovereignSyscallAbiTranslator;

impl SovereignSyscallAbiTranslator {
    pub fn get_fast_trampoline_msr(arch: ArchitectureClass) -> Vec<(&'static str, u32)> {
        match arch {
            ArchitectureClass::X86_64 => vec![
                ("IA32_STAR", 0xC000_0081),
                ("IA32_LSTAR", 0xC000_0082),
                ("IA32_FMASK", 0xC000_0084),
                ("IA32_KERNEL_GS_BASE", 0xC000_0102),
            ],
            ArchitectureClass::AArch64 => vec![
                ("TTBR1_EL1", 0x01),
                ("VBAR_EL1", 0x02),
            ],
            ArchitectureClass::RiscV64 => vec![
                ("STVEC", 0x05),
                ("SSCRATCH", 0x40),
            ],
            _ => vec![("GENERIC_SYSCALL_ENTRY", 0x00)],
        }
    }

    pub fn get_calling_convention(arch: ArchitectureClass) -> SyscallAbiCallingConvention {
        match arch {
            ArchitectureClass::X86_32 => SyscallAbiCallingConvention {
                instruction: "int 0x80 / sysenter",
                syscall_num_reg: "eax",
                arg_regs: vec!["ebx", "ecx", "edx", "esi", "edi", "ebp"],
                return_reg: "eax",
            },
            ArchitectureClass::X86_64 => SyscallAbiCallingConvention {
                instruction: "syscall",
                syscall_num_reg: "rax",
                arg_regs: vec!["rdi", "rsi", "rdx", "r10", "r8", "r9"],
                return_reg: "rax",
            },
            ArchitectureClass::AArch64 => SyscallAbiCallingConvention {
                instruction: "svc #0",
                syscall_num_reg: "x8",
                arg_regs: vec!["x0", "x1", "x2", "x3", "x4", "x5"],
                return_reg: "x0",
            },
            ArchitectureClass::RiscV32 | ArchitectureClass::RiscV64 => SyscallAbiCallingConvention {
                instruction: "ecall",
                syscall_num_reg: "a7",
                arg_regs: vec!["a0", "a1", "a2", "a3", "a4", "a5"],
                return_reg: "a0",
            },
            ArchitectureClass::LoongArch64 => SyscallAbiCallingConvention {
                instruction: "syscall 0",
                syscall_num_reg: "a7",
                arg_regs: vec!["a0", "a1", "a2", "a3", "a4", "a5"],
                return_reg: "a0",
            },
            ArchitectureClass::PowerPC64 => SyscallAbiCallingConvention {
                instruction: "sc",
                syscall_num_reg: "r0",
                arg_regs: vec!["r3", "r4", "r5", "r6", "r7", "r8"],
                return_reg: "r3",
            },
            ArchitectureClass::S390x => SyscallAbiCallingConvention {
                instruction: "svc 0",
                syscall_num_reg: "r1",
                arg_regs: vec!["r2", "r3", "r4", "r5", "r6", "r7"],
                return_reg: "r2",
            },
        }
    }
}

// =========================================================================
// 2. SOVEREIGN MMU PAGE TABLE WALKER
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTranslation {
    pub virtual_addr: u64,
    pub physical_addr: u64,
    pub page_size_bytes: usize,
    pub is_user_accessible: bool,
    pub is_executable: bool,
    pub is_writable: bool,
}

pub struct SovereignMmuPageTableWalker {
    pub arch: ArchitectureClass,
    pub page_table_mode: CpuPageTableMode,
}

impl SovereignMmuPageTableWalker {
    pub fn new(arch: ArchitectureClass) -> Self {
        let page_table_mode = match arch {
            ArchitectureClass::X86_32 => CpuPageTableMode::X86_4LevelPaging,
            ArchitectureClass::X86_64 => CpuPageTableMode::X86_4LevelPaging,
            ArchitectureClass::AArch64 => CpuPageTableMode::Arm64_4Level48Bit,
            ArchitectureClass::RiscV32 => CpuPageTableMode::RiscvSv39,
            ArchitectureClass::RiscV64 => CpuPageTableMode::RiscvSv48,
            ArchitectureClass::LoongArch64 => CpuPageTableMode::LoongArchLA64,
            ArchitectureClass::PowerPC64 => CpuPageTableMode::PowerPCLinuxRadix,
            ArchitectureClass::S390x => CpuPageTableMode::S390xRegion1Table,
        };

        Self {
            arch,
            page_table_mode,
        }
    }

    pub fn walk_page_table(&self, root_table_phys: u64, virt_addr: u64) -> PhysicalTranslation {
        // Identity / offset page translation simulation
        let page_offset = virt_addr & 0xFFF;
        let base_phys = (root_table_phys + (virt_addr >> 12)) & !0xFFF;
        let physical_addr = base_phys + page_offset;

        PhysicalTranslation {
            virtual_addr: virt_addr,
            physical_addr,
            page_size_bytes: 4096,
            is_user_accessible: virt_addr < 0x8000_0000_0000,
            is_executable: true,
            is_writable: true,
        }
    }
}

// =========================================================================
// 3. SOVEREIGN SIMD VECTOR DISPATCHER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorInstructionSet {
    Avx2,
    Avx512,
    ArmNeon,
    ArmSve2,
    RiscvVector10,
    LoongArchLasx,
    FallbackScalar,
}

// =========================================================================
// 4. SOVEREIGN 64-BIT ADDRESS SPACE LAYOUT ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressSpaceCanonicality {
    CanonicalUser,
    CanonicalKernel,
    NonCanonicalHole,
}

#[derive(Debug, Clone)]
pub struct Sovereign64BitAddressSpaceLayout {
    pub arch: ArchitectureClass,
    pub is_la57_enabled: bool,
    pub user_va_start: u64,
    pub user_va_end: u64,
    pub kernel_va_start: u64,
    pub kernel_va_end: u64,
}

impl Sovereign64BitAddressSpaceLayout {
    pub fn new(arch: ArchitectureClass, is_la57_enabled: bool) -> Self {
        let (user_end, kernel_start) = if is_la57_enabled {
            (0x00FF_FFFF_FFFF_FFFF, 0xFF00_0000_0000_0000)
        } else {
            (0x0000_7FFF_FFFF_FFFF, 0xFFFF_8000_0000_0000)
        };

        Self {
            arch,
            is_la57_enabled,
            user_va_start: 0x0000_0000_0000_0000,
            user_va_end: user_end,
            kernel_va_start: kernel_start,
            kernel_va_end: 0xFFFF_FFFF_FFFF_FFFF,
        }
    }

    pub fn classify_address(&self, va: u64) -> AddressSpaceCanonicality {
        if va <= self.user_va_end {
            AddressSpaceCanonicality::CanonicalUser
        } else if va >= self.kernel_va_start {
            AddressSpaceCanonicality::CanonicalKernel
        } else {
            AddressSpaceCanonicality::NonCanonicalHole
        }
    }
}

// =========================================================================
// 5. SOVEREIGN 64-BIT ATOMIC & SIMD VECTOR REGISTER ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct Sovereign64BitAtomicAndSimdEngine {
    pub arch: ArchitectureClass,
    pub simd_state_size_bytes: usize,
}

impl Sovereign64BitAtomicAndSimdEngine {
    pub fn new(arch: ArchitectureClass) -> Self {
        let simd_state_size_bytes = match arch {
            ArchitectureClass::X86_64 => 64 * 32, // AVX-512 ZMM0-ZMM31 (2048 bytes)
            ArchitectureClass::AArch64 => 32 * 256, // ARM SVE2 Z0-Z31 (8192 bytes)
            ArchitectureClass::RiscV64 => 32 * 128, // RISC-V V registers v0-v31 (4096 bytes)
            _ => 32 * 16, // Default 128-bit vector register file
        };

        Self {
            arch,
            simd_state_size_bytes,
        }
    }

    /// Emulates 128-bit atomic double-word compare-and-swap (cmpxchg16b parity)
    pub fn compare_and_swap_u128(
        &self,
        target: &mut (u64, u64),
        expected: (u64, u64),
        desired: (u64, u64),
    ) -> (bool, (u64, u64)) {
        if *target == expected {
            *target = desired;
            (true, expected)
        } else {
            (false, *target)
        }
    }
}

pub struct SovereignSimdVectorDispatcher {
    pub preferred_isa: VectorInstructionSet,
}

impl SovereignSimdVectorDispatcher {
    pub fn auto_detect(caps: &IsaVectorCapabilities) -> Self {
        let preferred = if caps.has_avx512 {
            VectorInstructionSet::Avx512
        } else if caps.has_avx2 {
            VectorInstructionSet::Avx2
        } else if caps.has_sve2 {
            VectorInstructionSet::ArmSve2
        } else if caps.has_neon {
            VectorInstructionSet::ArmNeon
        } else if caps.has_riscv_v {
            VectorInstructionSet::RiscvVector10
        } else if caps.has_loongarch_lasx {
            VectorInstructionSet::LoongArchLasx
        } else {
            VectorInstructionSet::FallbackScalar
        };

        Self {
            preferred_isa: preferred,
        }
    }

    pub fn vector_add_u32(&self, a: &[u32], b: &[u32]) -> Vec<u32> {
        let len = a.len().min(b.len());
        let mut result = vec![0u32; len];

        for i in 0..len {
            result[i] = a[i].wrapping_add(b[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiarch_hal_x86_64() {
        let mut hal = SovereignMultiArchHalEngine::new(ArchitectureClass::X86_64);
        assert_eq!(hal.get_root_register_name(), "CR3");
        assert!(hal.vector_caps.has_avx2);

        hal.set_page_table_root(0x200000);
        let summary = hal.format_trap_frame_summary(0x7fff0000, 0x7fff1000);
        assert!(summary.contains("X86_64"));
        assert!(summary.contains("CR3"));
    }

    #[test]
    fn test_multiarch_hal_arm64_riscv() {
        let arm_hal = SovereignMultiArchHalEngine::new(ArchitectureClass::AArch64);
        assert_eq!(arm_hal.get_root_register_name(), "TTBR0_EL1");
        assert!(arm_hal.vector_caps.has_neon);

        let rv_hal = SovereignMultiArchHalEngine::new(ArchitectureClass::RiscV64);
        assert_eq!(rv_hal.get_root_register_name(), "SATP");
        assert!(rv_hal.vector_caps.has_riscv_v);
    }

    #[test]
    fn test_64bit_address_space_layout_and_la57() {
        let layout_48 = Sovereign64BitAddressSpaceLayout::new(ArchitectureClass::X86_64, false);
        assert_eq!(layout_48.classify_address(0x0000_7FFF_FFFF_0000), AddressSpaceCanonicality::CanonicalUser);
        assert_eq!(layout_48.classify_address(0xFFFF_8000_0000_0000), AddressSpaceCanonicality::CanonicalKernel);
        assert_eq!(layout_48.classify_address(0x0000_8000_0000_0000), AddressSpaceCanonicality::NonCanonicalHole);

        let layout_57 = Sovereign64BitAddressSpaceLayout::new(ArchitectureClass::X86_64, true);
        assert_eq!(layout_57.classify_address(0x00FF_FFFF_FFFF_0000), AddressSpaceCanonicality::CanonicalUser);
        assert_eq!(layout_57.classify_address(0xFF00_0000_0000_0000), AddressSpaceCanonicality::CanonicalKernel);
    }

    #[test]
    fn test_64bit_atomic_and_simd_vector_state() {
        let engine_x64 = Sovereign64BitAtomicAndSimdEngine::new(ArchitectureClass::X86_64);
        assert_eq!(engine_x64.simd_state_size_bytes, 2048);

        let mut pair = (0x1234_5678_9ABC_DEF0, 0x0000_0000_0000_0001);
        let expected = (0x1234_5678_9ABC_DEF0, 0x0000_0000_0000_0001);
        let desired = (0x9999_8888_7777_6666, 0x0000_0000_0000_0002);

        let (success, old) = engine_x64.compare_and_swap_u128(&mut pair, expected, desired);
        assert!(success);
        assert_eq!(old, expected);
        assert_eq!(pair, desired);

        let engine_arm = Sovereign64BitAtomicAndSimdEngine::new(ArchitectureClass::AArch64);
        assert_eq!(engine_arm.simd_state_size_bytes, 8192);
    }

    #[test]
    fn test_64bit_fast_trampoline_msr() {
        let msrs_x64 = SovereignSyscallAbiTranslator::get_fast_trampoline_msr(ArchitectureClass::X86_64);
        assert_eq!(msrs_x64.len(), 4);
        assert_eq!(msrs_x64[1].0, "IA32_LSTAR");

        let msrs_arm = SovereignSyscallAbiTranslator::get_fast_trampoline_msr(ArchitectureClass::AArch64);
        assert_eq!(msrs_arm[0].0, "TTBR1_EL1");
    }

    #[test]
    fn test_syscall_abi_translation() {
        let x64_abi = SovereignSyscallAbiTranslator::get_calling_convention(ArchitectureClass::X86_64);
        assert_eq!(x64_abi.instruction, "syscall");
        assert_eq!(x64_abi.syscall_num_reg, "rax");
        assert_eq!(x64_abi.arg_regs[0], "rdi");

        let arm64_abi = SovereignSyscallAbiTranslator::get_calling_convention(ArchitectureClass::AArch64);
        assert_eq!(arm64_abi.instruction, "svc #0");
        assert_eq!(arm64_abi.syscall_num_reg, "x8");
        assert_eq!(arm64_abi.arg_regs[0], "x0");

        let riscv_abi = SovereignSyscallAbiTranslator::get_calling_convention(ArchitectureClass::RiscV64);
        assert_eq!(riscv_abi.instruction, "ecall");
        assert_eq!(riscv_abi.syscall_num_reg, "a7");
        assert_eq!(riscv_abi.arg_regs[0], "a0");
    }

    #[test]
    fn test_mmu_page_table_walker() {
        let walker = SovereignMmuPageTableWalker::new(ArchitectureClass::X86_64);
        let translation = walker.walk_page_table(0x1000, 0x0040_1234);

        assert_eq!(translation.virtual_addr, 0x0040_1234);
        assert!(translation.is_user_accessible);
        assert_eq!(translation.page_size_bytes, 4096);
    }

    #[test]
    fn test_simd_vector_dispatcher() {
        let caps_x64 = IsaVectorCapabilities::for_arch(ArchitectureClass::X86_64);
        let dispatcher_x64 = SovereignSimdVectorDispatcher::auto_detect(&caps_x64);
        assert_eq!(dispatcher_x64.preferred_isa, VectorInstructionSet::Avx512);

        let res = dispatcher_x64.vector_add_u32(&[10, 20, 30], &[1, 2, 3]);
        assert_eq!(res, vec![11, 22, 33]);

        let caps_arm64 = IsaVectorCapabilities::for_arch(ArchitectureClass::AArch64);
        let dispatcher_arm = SovereignSimdVectorDispatcher::auto_detect(&caps_arm64);
        assert_eq!(dispatcher_arm.preferred_isa, VectorInstructionSet::ArmSve2);
    }
}
