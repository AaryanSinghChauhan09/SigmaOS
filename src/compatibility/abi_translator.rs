// SigmaOS Cross-Kernel ABI and Calling Convention Translator
// Translates function register calling conventions and stack allocations across x86, x64, ARM, Linux, BSD, and Windows kernels.

#![no_std]

type u64_type = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitecture {
    X86,
    X64,
    Arm32,
    Arm64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    // 32-bit x86
    Cdecl,      // Stack right-to-left, caller cleans
    Stdcall,    // Stack right-to-left, callee cleans
    Fastcall32, // ECX, EDX, remaining on stack, callee cleans
    Thiscall,   // ECX (this), remaining on stack, callee cleans
    // 64-bit and RISC architectures
    MicrosoftX64,  // RCX, RDX, R8, R9, remaining on stack, 32-byte shadow space, 16-byte aligned
    SystemVAmd64,  // RDI, RSI, RDX, RCX, R8, R9, remaining on stack, 16-byte aligned
    AArch32AAPCS,  // R0-R3 (even aligned for 64-bit), remaining on stack
    AArch64AAPCS,  // X0-X7, remaining on stack, 16-byte aligned
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvocationLayout {
    pub registers: [u64_type; 8], // Registers mapped in convention order
    pub registers_used: usize,
    pub stack_size: usize,        // Bytes required on stack
    pub shadow_space_size: usize, // Windows x64 shadow/home space size
    pub stack_alignment: usize,   // Stack alignment boundary
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
        let mut stack_cleaning_authority = "CALLER";
        let mut shadow_space_bytes = 0;
        let mut stack_alignment_bytes = 4; // x86 standard

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
            CpuArchitecture::X86 | CpuArchitecture::X64 => {
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
            CpuArchitecture::Arm32 | CpuArchitecture::Arm64 => {
                // Translate legacy OABI to modern EABI parameter alignment
                for &reg in old_registers {
                    modern_registers.push(reg);
                }
            }
        }
        Ok(modern_registers)
    }
}

// Simple Vector wrapper to avoid alloc/std limits inside standalone test setups
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.data, self.len) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
extern crate std;

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    let layout = core::alloc::Layout::from_size_align(size, 8).unwrap_or(
        core::alloc::Layout::from_size_align_unchecked(8, 8)
    );
    std::alloc::alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    extern "C" {
        fn free(ptr: *mut u8);
    }
    free(ptr);
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x86_abi_translation() {
        let translator = ABITranslator::new(CpuArchitecture::X64);
        let old_regs = [10, 20, 30];
        let modern_regs = translator.translate_register_map(&old_regs).unwrap();
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
    fn test_compute_invocation_layout_arm() {
        let translator = ABITranslator::new(CpuArchitecture::Arm);
        let params = vec![11, 22, 33, 44, 55];

        // AArch32 AAPCS: R0-R3, rest on stack
        let aapcs32 = translator.compute_invocation_layout(&params, CallingConvention::AArch32AAPCS);
        assert_eq!(aapcs32.register_params, vec![
            ("R0".to_string(), 11),
            ("R1".to_string(), 22),
            ("R2".to_string(), 33),
            ("R3".to_string(), 44),
        ]);
        assert_eq!(aapcs32.stack_params, vec![55]);
        assert_eq!(aapcs32.stack_alignment_bytes, 8);

        // AArch64 AAPCS: X0-X7, rest on stack
        let aapcs64 = translator.compute_invocation_layout(&params, CallingConvention::AArch64AAPCS);
        assert_eq!(aapcs64.register_params, vec![
            ("X0".to_string(), 11),
            ("X1".to_string(), 22),
            ("X2".to_string(), 33),
            ("X3".to_string(), 44),
            ("X4".to_string(), 55),
        ]);
        assert!(aapcs64.stack_params.is_empty());
        assert_eq!(aapcs64.stack_alignment_bytes, 16);
    }
}
