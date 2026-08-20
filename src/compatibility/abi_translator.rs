// SigmaOS Cross-Kernel ABI Translator
// Designed to translate function register calling conventions, C struct layouts, and packet alignments across x86, ARM, RISC-V, and BSD ABIs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitecture {
    X86,
    Arm,
    Mips,
    Riscv64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    Cdecl,
    Stdcall,
    Fastcall32,
    Thiscall,
    MicrosoftX64,
    SystemVAmd64,
    AArch32AAPCS,
    AArch64AAPCS,
    RiscvLP64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationStackLayout {
    pub register_params: Vec<(String, u64)>,
    pub stack_params: Vec<u64>,
    pub stack_cleaning_authority: &'static str,
    pub shadow_space_bytes: usize,
    pub stack_alignment_bytes: usize,
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

    /// Computes register and stack layouts for a function invocation based on chosen ABI / calling convention
    pub fn compute_invocation_layout(
        &self,
        params: &[u64],
        convention: CallingConvention,
    ) -> InvocationStackLayout {
        let mut register_params = Vec::new();
        let mut stack_params = Vec::new();
        let stack_cleaning_authority;
        let mut shadow_space_bytes = 0;
        let stack_alignment_bytes;

        match convention {
            CallingConvention::Cdecl => {
                stack_params.extend_from_slice(params);
                stack_cleaning_authority = "CALLER";
                stack_alignment_bytes = 4;
            }
            CallingConvention::Stdcall => {
                stack_params.extend_from_slice(params);
                stack_cleaning_authority = "CALLEE";
                stack_alignment_bytes = 4;
            }
            CallingConvention::Fastcall32 => {
                if params.len() >= 1 {
                    register_params.push(("ECX".to_string(), params[0]));
                }
                if params.len() >= 2 {
                    register_params.push(("EDX".to_string(), params[1]));
                }
                if params.len() > 2 {
                    stack_params.extend_from_slice(&params[2..]);
                }
                stack_cleaning_authority = "CALLEE";
                stack_alignment_bytes = 4;
            }
            CallingConvention::Thiscall => {
                if params.len() >= 1 {
                    register_params.push(("ECX".to_string(), params[0]));
                }
                if params.len() > 1 {
                    stack_params.extend_from_slice(&params[1..]);
                }
                stack_cleaning_authority = "CALLEE";
                stack_alignment_bytes = 4;
            }
            CallingConvention::MicrosoftX64 => {
                let gprs = ["RCX", "RDX", "R8", "R9"];
                for i in 0..params.len() {
                    if i < 4 {
                        register_params.push((gprs[i].to_string(), params[i]));
                    } else {
                        stack_params.push(params[i]);
                    }
                }
                shadow_space_bytes = 32;
                stack_cleaning_authority = "CALLER";
                stack_alignment_bytes = 16;
            }
            CallingConvention::SystemVAmd64 => {
                let gprs = ["RDI", "RSI", "RDX", "RCX", "R8", "R9"];
                for i in 0..params.len() {
                    if i < 6 {
                        register_params.push((gprs[i].to_string(), params[i]));
                    } else {
                        stack_params.push(params[i]);
                    }
                }
                stack_cleaning_authority = "CALLER";
                stack_alignment_bytes = 16;
            }
            CallingConvention::AArch32AAPCS => {
                let gprs = ["R0", "R1", "R2", "R3"];
                for i in 0..params.len() {
                    if i < 4 {
                        register_params.push((gprs[i].to_string(), params[i]));
                    } else {
                        stack_params.push(params[i]);
                    }
                }
                stack_cleaning_authority = "CALLER";
                stack_alignment_bytes = 8;
            }
            CallingConvention::AArch64AAPCS => {
                let gprs = ["X0", "X1", "X2", "X3", "X4", "X5", "X6", "X7"];
                for i in 0..params.len() {
                    if i < 8 {
                        register_params.push((gprs[i].to_string(), params[i]));
                    } else {
                        stack_params.push(params[i]);
                    }
                }
                stack_cleaning_authority = "CALLER";
                stack_alignment_bytes = 16;
            }
            CallingConvention::RiscvLP64 => {
                let gprs = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"];
                for i in 0..params.len() {
                    if i < 8 {
                        register_params.push((gprs[i].to_string(), params[i]));
                    } else {
                        stack_params.push(params[i]);
                    }
                }
                stack_cleaning_authority = "CALLER";
                stack_alignment_bytes = 16;
            }
        }

        InvocationStackLayout {
            register_params,
            stack_params,
            stack_cleaning_authority,
            shadow_space_bytes,
            stack_alignment_bytes,
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
            CpuArchitecture::Arm | CpuArchitecture::Mips | CpuArchitecture::Riscv64 => {
                for &reg in old_registers {
                    modern_registers.push(reg);
                }
            }
        }
        Ok(modern_registers)
    }
}

// =========================================================================
// POSIX / BSD System Call & C Struct Layout Alignment Subsystem
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsAbiStandard {
    LinuxGlibc,
    FreeBsd,
    OpenBsd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructFieldOffset {
    pub name: &'static str,
    pub offset_bytes: usize,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyscallStructLayout {
    pub struct_name: &'static str,
    pub total_size_bytes: usize,
    pub fields: Vec<StructFieldOffset>,
}

impl SyscallStructLayout {
    pub fn compute_stat_layout(abi: OsAbiStandard) -> Self {
        let mut fields = Vec::new();
        let total_size_bytes;

        match abi {
            OsAbiStandard::LinuxGlibc => {
                // Linux x86_64 struct stat layout (144 bytes)
                fields.push(StructFieldOffset { name: "st_dev", offset_bytes: 0, size_bytes: 8 });
                fields.push(StructFieldOffset { name: "st_ino", offset_bytes: 8, size_bytes: 8 });
                fields.push(StructFieldOffset { name: "st_nlink", offset_bytes: 16, size_bytes: 8 });
                fields.push(StructFieldOffset { name: "st_mode", offset_bytes: 24, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_uid", offset_bytes: 28, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_gid", offset_bytes: 32, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_size", offset_bytes: 48, size_bytes: 8 });
                total_size_bytes = 144;
            }
            OsAbiStandard::FreeBsd => {
                // FreeBSD struct stat layout (120 bytes)
                fields.push(StructFieldOffset { name: "st_dev", offset_bytes: 0, size_bytes: 8 });
                fields.push(StructFieldOffset { name: "st_ino", offset_bytes: 8, size_bytes: 8 });
                fields.push(StructFieldOffset { name: "st_nlink", offset_bytes: 16, size_bytes: 8 });
                fields.push(StructFieldOffset { name: "st_mode", offset_bytes: 24, size_bytes: 2 });
                fields.push(StructFieldOffset { name: "st_uid", offset_bytes: 28, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_gid", offset_bytes: 32, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_size", offset_bytes: 48, size_bytes: 8 });
                total_size_bytes = 120;
            }
            OsAbiStandard::OpenBsd => {
                // OpenBSD struct stat layout (128 bytes)
                fields.push(StructFieldOffset { name: "st_dev", offset_bytes: 0, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_ino", offset_bytes: 8, size_bytes: 8 });
                fields.push(StructFieldOffset { name: "st_mode", offset_bytes: 16, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_nlink", offset_bytes: 20, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_uid", offset_bytes: 24, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_gid", offset_bytes: 28, size_bytes: 4 });
                fields.push(StructFieldOffset { name: "st_size", offset_bytes: 48, size_bytes: 8 });
                total_size_bytes = 128;
            }
        }

        SyscallStructLayout {
            struct_name: "stat",
            total_size_bytes,
            fields,
        }
    }

    pub fn compute_timespec_layout() -> Self {
        let mut fields = Vec::new();
        fields.push(StructFieldOffset { name: "tv_sec", offset_bytes: 0, size_bytes: 8 });
        fields.push(StructFieldOffset { name: "tv_nsec", offset_bytes: 8, size_bytes: 8 });

        SyscallStructLayout {
            struct_name: "timespec",
            total_size_bytes: 16,
            fields,
        }
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

    #[test]
    fn test_compute_invocation_layout_x86() {
        let translator = ABITranslator::new(CpuArchitecture::X86);
        let params = vec![100, 200, 300, 400];

        // cdecl: all on stack
        let cdecl_layout = translator.compute_invocation_layout(&params, CallingConvention::Cdecl);
        assert_eq!(cdecl_layout.stack_params, vec![100, 200, 300, 400]);
        assert!(cdecl_layout.register_params.is_empty());
        assert_eq!(cdecl_layout.stack_cleaning_authority, "CALLER");
        assert_eq!(cdecl_layout.stack_alignment_bytes, 4);

        // stdcall: all on stack, callee cleans
        let stdcall_layout = translator.compute_invocation_layout(&params, CallingConvention::Stdcall);
        assert_eq!(stdcall_layout.stack_params, vec![100, 200, 300, 400]);
        assert_eq!(stdcall_layout.stack_cleaning_authority, "CALLEE");

        // fastcall: ECX, EDX, rest on stack
        let fastcall_layout = translator.compute_invocation_layout(&params, CallingConvention::Fastcall32);
        assert_eq!(fastcall_layout.register_params, vec![
            ("ECX".to_string(), 100),
            ("EDX".to_string(), 200),
        ]);
        assert_eq!(fastcall_layout.stack_params, vec![300, 400]);
        assert_eq!(fastcall_layout.stack_cleaning_authority, "CALLEE");
    }

    #[test]
    fn test_compute_invocation_layout_x64() {
        let translator = ABITranslator::new(CpuArchitecture::X86);
        let params = vec![1, 2, 3, 4, 5, 6, 7];

        // Microsoft x64: RCX, RDX, R8, R9, rest on stack, 32-byte shadow
        let ms_layout = translator.compute_invocation_layout(&params, CallingConvention::MicrosoftX64);
        assert_eq!(ms_layout.register_params, vec![
            ("RCX".to_string(), 1),
            ("RDX".to_string(), 2),
            ("R8".to_string(), 3),
            ("R9".to_string(), 4),
        ]);
        assert_eq!(ms_layout.stack_params, vec![5, 6, 7]);
        assert_eq!(ms_layout.shadow_space_bytes, 32);
        assert_eq!(ms_layout.stack_alignment_bytes, 16);

        // System V AMD64: RDI, RSI, RDX, RCX, R8, R9, rest on stack
        let sysv_layout = translator.compute_invocation_layout(&params, CallingConvention::SystemVAmd64);
        assert_eq!(sysv_layout.register_params, vec![
            ("RDI".to_string(), 1),
            ("RSI".to_string(), 2),
            ("RDX".to_string(), 3),
            ("RCX".to_string(), 4),
            ("R8".to_string(), 5),
            ("R9".to_string(), 6),
        ]);
        assert_eq!(sysv_layout.stack_params, vec![7]);
        assert_eq!(sysv_layout.shadow_space_bytes, 0);
        assert_eq!(sysv_layout.stack_alignment_bytes, 16);
    }

    #[test]
    fn test_compute_invocation_layout_riscv() {
        let translator = ABITranslator::new(CpuArchitecture::Riscv64);
        let params = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];

        let riscv_layout = translator.compute_invocation_layout(&params, CallingConvention::RiscvLP64);
        assert_eq!(riscv_layout.register_params, vec![
            ("a0".to_string(), 10),
            ("a1".to_string(), 20),
            ("a2".to_string(), 30),
            ("a3".to_string(), 40),
            ("a4".to_string(), 50),
            ("a5".to_string(), 60),
            ("a6".to_string(), 70),
            ("a7".to_string(), 80),
        ]);
        assert_eq!(riscv_layout.stack_params, vec![90]);
        assert_eq!(riscv_layout.stack_alignment_bytes, 16);
    }

    #[test]
    fn test_syscall_struct_layouts() {
        let linux_stat = SyscallStructLayout::compute_stat_layout(OsAbiStandard::LinuxGlibc);
        assert_eq!(linux_stat.total_size_bytes, 144);

        let freebsd_stat = SyscallStructLayout::compute_stat_layout(OsAbiStandard::FreeBsd);
        assert_eq!(freebsd_stat.total_size_bytes, 120);

        let timespec = SyscallStructLayout::compute_timespec_layout();
        assert_eq!(timespec.total_size_bytes, 16);
    }
}
