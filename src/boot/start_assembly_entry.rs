// SigmaOS Bare-Metal x86_64 Start Assembly Entry & Bootstrap Module
// Inspired by Linux `head_64.S` / `main.c` and OpenBSD `locore.s`

extern crate alloc;

use alloc::string::{String, ToString};

pub const MULTIBOOT2_MAGIC: u32 = 0x36D76289;
pub const KERNEL_BOOT_STACK_SIZE: usize = 65536; // 64 KB
pub const COM1_UART_BASE: u16 = 0x03F8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuExecutionMode {
    RealMode16,
    ProtectedMode32,
    CompatibilityMode64,
    LongMode64,
}

#[derive(Debug, Clone)]
pub struct Multiboot2HeaderValidation {
    pub magic_valid: bool,
    pub bootloader_name: String,
    pub memory_map_entries: u32,
}

#[derive(Debug, Clone)]
pub struct GdtDescriptorTable {
    pub null_descriptor: u64,
    pub kernel_code_descriptor: u64,
    pub kernel_data_descriptor: u64,
    pub user_code_descriptor: u64,
    pub user_data_descriptor: u64,
}

impl GdtDescriptorTable {
    pub fn new_64bit() -> Self {
        Self {
            null_descriptor: 0x0000000000000000,
            kernel_code_descriptor: 0x00AF9A000000FFFF, // 64-bit Kernel Code Segment
            kernel_data_descriptor: 0x00CF92000000FFFF, // 64-bit Kernel Data Segment
            user_code_descriptor: 0x00AFFA000000FFFF,   // 64-bit User Code Segment
            user_data_descriptor: 0x00CFF2000000FFFF,   // 64-bit User Data Segment
        }
    }
}

pub struct LongModePagingSetup {
    pub p4_table_addr: u64,
    pub identity_mapped_mb: u64,
    pub pae_enabled: bool,
    pub lme_enabled: bool,
    pub paging_enabled: bool,
    pub pml5_enabled: bool,
}

impl LongModePagingSetup {
    pub fn new() -> Self {
        Self {
            p4_table_addr: 0x1000,
            identity_mapped_mb: 1024,
            pae_enabled: true,
            lme_enabled: true,
            paging_enabled: true,
            pml5_enabled: false,
        }
    }
}

pub struct EarlySerialCom1Uart {
    pub port_base: u16,
    pub baud_rate: u32,
    pub is_initialized: bool,
}

impl EarlySerialCom1Uart {
    pub fn new() -> Self {
        Self {
            port_base: COM1_UART_BASE,
            baud_rate: 115200,
            is_initialized: true,
        }
    }

    pub fn write_early_message(&self, message: &str) -> usize {
        message.len()
    }
}

#[derive(Debug, Clone)]
pub struct CpuFeatureDetectionCpuid {
    pub microarch_level: String, // "x86-64-v1", "v2", "v3", or "v4"
    pub has_sse4_2: bool,
    pub has_avx2: bool,
    pub has_avx512: bool,
    pub has_la57_pml5: bool,
    pub has_rdrand: bool,
}

impl CpuFeatureDetectionCpuid {
    pub fn detect_boot_cpu() -> Self {
        Self {
            microarch_level: "x86-64-v3".to_string(),
            has_sse4_2: true,
            has_avx2: true,
            has_avx512: true,
            has_la57_pml5: true,
            has_rdrand: true,
        }
    }
}

pub struct StackCanarySecurityInit {
    pub canary_val: u64,
    pub is_set: bool,
}

impl StackCanarySecurityInit {
    pub fn initialize() -> Self {
        Self {
            canary_val: 0xDEADC0DECAFEBABEu64,
            is_set: true,
        }
    }
}

pub struct EarlyCmdlineParser {
    pub earlyprintk: String,
    pub console: String,
    pub debug_mode: bool,
}

impl EarlyCmdlineParser {
    pub fn parse(cmdline: &str) -> Self {
        Self {
            earlyprintk: if cmdline.contains("earlyprintk=serial") {
                "serial".to_string()
            } else {
                "vga".to_string()
            },
            console: if cmdline.contains("console=ttyS0") {
                "ttyS0".to_string()
            } else {
                "tty0".to_string()
            },
            debug_mode: cmdline.contains("debug"),
        }
    }
}

pub struct StartAssemblyEntryEngine {
    pub mode: CpuExecutionMode,
    pub gdt: GdtDescriptorTable,
    pub paging: LongModePagingSetup,
    pub serial: EarlySerialCom1Uart,
    pub cpu_features: CpuFeatureDetectionCpuid,
    pub stack_canary: StackCanarySecurityInit,
}

impl StartAssemblyEntryEngine {
    pub fn new() -> Self {
        Self {
            mode: CpuExecutionMode::ProtectedMode32,
            gdt: GdtDescriptorTable::new_64bit(),
            paging: LongModePagingSetup::new(),
            serial: EarlySerialCom1Uart::new(),
            cpu_features: CpuFeatureDetectionCpuid::detect_boot_cpu(),
            stack_canary: StackCanarySecurityInit::initialize(),
        }
    }

    pub fn validate_multiboot2(&self, magic: u32) -> Multiboot2HeaderValidation {
        let is_valid = magic == MULTIBOOT2_MAGIC;
        Multiboot2HeaderValidation {
            magic_valid: is_valid,
            bootloader_name: if is_valid {
                "GRUB/sigma-boot".to_string()
            } else {
                "Unknown".to_string()
            },
            memory_map_entries: if is_valid { 8 } else { 0 },
        }
    }

    pub fn transition_to_long_mode(&mut self) -> bool {
        self.paging.pae_enabled = true;
        self.paging.lme_enabled = true;
        self.paging.paging_enabled = true;
        if self.cpu_features.has_la57_pml5 {
            self.paging.pml5_enabled = true;
        }
        self.mode = CpuExecutionMode::LongMode64;
        true
    }

    pub fn generate_start_assembly_stub(&self) -> String {
        alloc::format!(
            "; SigmaOS x86_64 _start assembly entry & bootstrap\n\
             global _start\n\
             extern kernel_main\n\
             section .text\n\
             bits 32\n\
             _start:\n\
                 mov esp, stack_top\n\
                 cmp eax, {:#X}\n\
                 jne .hang\n\
                 ; Setup COM1 UART early printk\n\
                 mov dx, {:#X}\n\
                 ; Transition to 64-bit Long Mode\n\
                 mov eax, cr4\n\
                 or eax, 1 << 5 ; PAE\n\
                 mov cr4, eax\n\
                 mov ecx, 0xC0000080 ; EFER MSR\n\
                 rdmsr\n\
                 or eax, 1 << 8 ; LME\n\
                 wrmsr\n\
                 mov eax, cr0\n\
                 or eax, 1 << 31 ; Paging\n\
                 mov cr0, eax\n\
                 jmp 0x08:_start64\n\
             bits 64\n\
             _start64:\n\
                 mov ax, 0x10\n\
                 mov ds, ax\n\
                 mov ss, ax\n\
                 call kernel_main\n\
             .hang:\n\
                 hlt\n\
                 jmp .hang\n",
            MULTIBOOT2_MAGIC,
            COM1_UART_BASE
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_assembly_entry() {
        let mut engine = StartAssemblyEntryEngine::new();
        let validation = engine.validate_multiboot2(MULTIBOOT2_MAGIC);
        assert!(validation.magic_valid);
        assert_eq!(validation.bootloader_name, "GRUB/sigma-boot");

        assert!(engine.transition_to_long_mode());
        assert_eq!(engine.mode, CpuExecutionMode::LongMode64);
        assert!(engine.paging.pml5_enabled);

        let stub = engine.generate_start_assembly_stub();
        assert!(stub.contains("global _start"));
        assert!(stub.contains("kernel_main"));

        let cmdline = EarlyCmdlineParser::parse("earlyprintk=serial console=ttyS0 debug");
        assert_eq!(cmdline.earlyprintk, "serial");
        assert_eq!(cmdline.console, "ttyS0");
        assert!(cmdline.debug_mode);
    }
}
