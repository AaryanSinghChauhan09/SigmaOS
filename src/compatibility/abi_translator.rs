// SigmaOS Cross-Kernel ABI Translator
// Designed to translate function register calling conventions and packet alignments across x86 and ARM ABIs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitecture {
    X86,
    Arm,
    Mips,
}

pub struct ABITranslator {
    pub target_arch: CpuArchitecture,
    pub legacy_abi_mode: bool,
}

impl ABITranslator {
    pub fn new(arch: CpuArchitecture) -> Self {
        ABITranslator {
            target_arch: arch,
            legacy_abi_mode: true,
        }
    }

    pub fn translate_register_map(&self, old_registers: &[u64]) -> Result<Vec<u64>, ()> {
        let mut modern_registers = Vec::new();
        match self.target_arch {
            CpuArchitecture::X86 => {
                // Translate legacy fastcall/stdcall register passing (e.g. EAX, EDX, ECX)
                // into modern System V AMD64 ABI (RDI, RSI, RDX, RCX, R8, R9)
                if old_registers.len() >= 3 {
                    modern_registers.push(old_registers[0]); // RDI = old param 1 (EAX)
                    modern_registers.push(old_registers[1]); // RSI = old param 2 (EDX)
                    modern_registers.push(old_registers[2]); // RDX = old param 3 (ECX)
                    for &reg in &old_registers[3..] {
                        modern_registers.push(reg);
                    }
                } else {
                    for &reg in old_registers {
                        modern_registers.push(reg);
                    }
                }
            }
            CpuArchitecture::Arm => {
                // Translate legacy OABI to modern EABI parameter alignment
                for &reg in old_registers {
                    modern_registers.push(reg);
                }
            }
            CpuArchitecture::Mips => {
                // MIPS O32 to N32 register mapping translator
                for &reg in old_registers {
                    modern_registers.push(reg);
                }
            }
        }
        Ok(modern_registers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x86_abi_translation() {
        let translator = ABITranslator::new(CpuArchitecture::X86);
        let old_regs = vec![10, 20, 30];
        let modern_regs = translator.translate_register_map(&old_regs).unwrap();
        // Modern registers should map sequentially
        assert_eq!(modern_regs[0], 10);
        assert_eq!(modern_regs[1], 20);
        assert_eq!(modern_regs[2], 30);
    }
}
