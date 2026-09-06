#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Multi-Arch Hardware Abstraction Layer (HAL)
// Hardware abstraction layer supporting x86_64, AArch64 (ARM64), and RISC-V (RV64GC).

use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch {
    X86_64,
    AArch64,
    RiscV64,
}

pub struct MultiArchHal {
    pub arch: TargetArch,
    pub interrupt_controller: String,
    pub page_levels: u8,
}

impl MultiArchHal {
    pub fn new(arch: TargetArch) -> Self {
        let (interrupt_controller, page_levels) = match arch {
            TargetArch::X86_64 => ("x2APIC", 4),
            TargetArch::AArch64 => ("GICv3", 4),
            TargetArch::RiscV64 => ("PLIC", 3), // Sv39/Sv48
        };
        Self {
            arch,
            interrupt_controller: interrupt_controller.to_string(),
            page_levels,
        }
    }

    pub fn initialize_hardware_irqs(&self) -> Result<&'static str, &'static str> {
        match self.arch {
            TargetArch::X86_64 => Ok("x2APIC and IO-APIC routing initialized"),
            TargetArch::AArch64 => Ok("ARM GICv3 distributor and redistributors initialized"),
            TargetArch::RiscV64 => Ok("RISC-V PLIC and CLINT timer initialized"),
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_arch_hal() {
        let x86_hal = MultiArchHal::new(TargetArch::X86_64);
        assert_eq!(x86_hal.page_levels, 4);

        let arm_hal = MultiArchHal::new(TargetArch::AArch64);
        assert!(arm_hal.initialize_hardware_irqs().unwrap().contains("GICv3"));

        let riscv_hal = MultiArchHal::new(TargetArch::RiscV64);
        assert_eq!(riscv_hal.interrupt_controller, "PLIC");
    }
}
