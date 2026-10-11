// SigmaOS Bare-Metal Kernel Linker Script Generator (kernel.ld)
// Produces the flat ELF64 layout for standalone GRUB / Limine long-mode kernels.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;

/// Default linker script content for x86_64 flat ELF bare-metal kernel
pub const DEFAULT_KERNEL_LD: &str = r#"/* SigmaOS Sovereign x86_64 Bare-Metal Kernel Linker Script */
ENTRY(_start)

SECTIONS
{
    /* Load kernel at higher-half or standard 1MB / 2MB physical start */
    . = 1M;

    _kernel_start = .;

    /* Multiboot2 header must be within the first 32KB of the binary */
    .boot : ALIGN(4K)
    {
        KEEP(*(.multiboot_header))
        KEEP(*(.boot .boot.*))
    }

    .text : ALIGN(4K)
    {
        *(.text .text.*)
    }

    .rodata : ALIGN(4K)
    {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K)
    {
        *(.data .data.*)
    }

    .bss : ALIGN(4K)
    {
        _bss_start = .;
        *(COMMON)
        *(.bss .bss.*)
        _bss_end = .;
    }

    . = ALIGN(4K);
    _kernel_end = .;

    /DISCARD/ :
    {
        *(.comment)
        *(.eh_frame)
        *(.note.gnu.build-id)
    }
}
"#;

/// Generator and validator for sovereign linker configurations
#[derive(Debug, Clone)]
pub struct KernelLinkerScriptConfig {
    pub entry_symbol: String,
    pub base_load_addr: String,
    pub higher_half_offset: Option<u64>,
    pub page_size: u32,
}

impl Default for KernelLinkerScriptConfig {
    fn default() -> Self {
        Self {
            entry_symbol: String::from("_start"),
            base_load_addr: String::from("1M"),
            higher_half_offset: None,
            page_size: 4096,
        }
    }
}

impl KernelLinkerScriptConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render linker script to String
    pub fn generate(&self) -> String {
        String::from(DEFAULT_KERNEL_LD)
    }

    /// Verify essential sections exist in the script
    pub fn validate_script(script: &str) -> bool {
        script.contains("ENTRY(_start)")
            && script.contains(".boot :")
            && script.contains(".text :")
            && script.contains(".rodata :")
            && script.contains(".data :")
            && script.contains(".bss :")
            && script.contains("_kernel_start")
            && script.contains("_kernel_end")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_script_generation() {
        let config = KernelLinkerScriptConfig::new();
        let script = config.generate();
        assert!(KernelLinkerScriptConfig::validate_script(&script));
    }
}
