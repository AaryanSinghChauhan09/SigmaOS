// Extended Application Binary Interface (ABI) Calling Conventions & Relocation for SigmaOS

/// Target Application Binary Interface Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetAbiConvention {
    SystemVAmd64,
    WindowsX64,
    Arm64Aapcs,
    Riscv64G,
}

/// System V AMD64 ABI Execution Frame Layout
#[derive(Debug, Clone)]
pub struct SystemVAbiFrame {
    pub arg_registers: [u64; 6],   // RDI, RSI, RDX, RCX, R8, R9
    pub red_zone_bytes: [u8; 128], // Red Zone below stack pointer
    pub rsp_aligned_16: bool,
}

impl SystemVAbiFrame {
    pub fn new(args: &[u64]) -> Self {
        let mut reg_args = [0u64; 6];
        for (i, &arg) in args.iter().take(6).enumerate() {
            reg_args[i] = arg;
        }
        SystemVAbiFrame {
            arg_registers: reg_args,
            red_zone_bytes: [0u8; 128],
            rsp_aligned_16: true,
        }
    }
}

/// Windows x64 ABI Execution Frame Layout
#[derive(Debug, Clone)]
pub struct WindowsX64AbiFrame {
    pub shadow_space_bytes: [u8; 32], // 32-byte shadow space on stack
    pub arg_registers: [u64; 4],      // RCX, RDX, R8, R9
}

impl WindowsX64AbiFrame {
    pub fn new(args: &[u64]) -> Self {
        let mut reg_args = [0u64; 4];
        for (i, &arg) in args.iter().take(4).enumerate() {
            reg_args[i] = arg;
        }
        WindowsX64AbiFrame {
            shadow_space_bytes: [0u8; 32],
            arg_registers: reg_args,
        }
    }
}

/// Dynamic Symbol Relocation Entry
#[derive(Debug, Clone)]
pub struct SymbolRelocationEntry {
    pub symbol_name: String,
    pub target_virtual_address: u64,
    pub relocation_type: u32,
    pub addend: i64,
}

/// Dynamic ABI Relocation Table
pub struct DynamicAbiRelocationTable {
    pub relocations: Vec<SymbolRelocationEntry>,
}

impl DynamicAbiRelocationTable {
    pub fn new() -> Self {
        DynamicAbiRelocationTable {
            relocations: Vec::new(),
        }
    }

    pub fn add_relocation(&mut self, name: &str, vaddr: u64, rel_type: u32, addend: i64) {
        self.relocations.push(SymbolRelocationEntry {
            symbol_name: String::from(name),
            target_virtual_address: vaddr,
            relocation_type: rel_type,
            addend,
        });
    }

    pub fn resolve_symbol(&self, name: &str) -> Option<u64> {
        self.relocations
            .iter()
            .find(|r| r.symbol_name == name)
            .map(|r| (r.target_virtual_address as i64 + r.addend) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abi_frames_and_relocations() {
        let sysv = SystemVAbiFrame::new(&[10, 20, 30, 40, 50, 60]);
        assert_eq!(sysv.arg_registers[0], 10);
        assert_eq!(sysv.arg_registers[5], 60);

        let win64 = WindowsX64AbiFrame::new(&[100, 200, 300, 400]);
        assert_eq!(win64.arg_registers[0], 100);
        assert_eq!(win64.shadow_space_bytes.len(), 32);

        let mut rel = DynamicAbiRelocationTable::new();
        rel.add_relocation("open", 0x4000, 7, 0x10);
        assert_eq!(rel.resolve_symbol("open"), Some(0x4010));
    }
}
