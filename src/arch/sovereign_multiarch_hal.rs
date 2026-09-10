// Sovereign Multi-Architecture HAL & ISA Feature Detection Engine
// Zero-dependency `#![no_std]` compliant multi-architecture abstractions inspired by Linux and BSD kernel ports:
// Supporting x86_32, x86_64, AArch64 (ARM64), RiscV32, RiscV64, LoongArch64, PowerPC64, and S390x architectures.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

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
        let mut arm_hal = SovereignMultiArchHalEngine::new(ArchitectureClass::AArch64);
        assert_eq!(arm_hal.get_root_register_name(), "TTBR0_EL1");
        assert!(arm_hal.vector_caps.has_neon);

        let mut rv_hal = SovereignMultiArchHalEngine::new(ArchitectureClass::RiscV64);
        assert_eq!(rv_hal.get_root_register_name(), "SATP");
        assert!(rv_hal.vector_caps.has_riscv_v);
    }
}
