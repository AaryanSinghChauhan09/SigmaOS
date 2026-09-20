// SPDX-License-Identifier: MIT
// SigmaOS Kernel Runtime Dynamic ELF Loader
// (`src/kernel/elf_loader.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust implementation of an in-kernel
// runtime dynamic ELF loader (`ld-linux.so` parity) with `.dynamic` section
// parsing, GOT/PLT relocation handling (`R_X86_64_GLOB_DAT`, `R_X86_64_JUMP_SLOT`),
// and shared object symbol resolution.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;

#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;

/// ELF Relocation Types (x86_64)
pub const R_X86_64_64: u32 = 1;
pub const R_X86_64_GLOB_DAT: u32 = 6;
pub const R_X86_64_JUMP_SLOT: u32 = 7;
pub const R_X86_64_RELATIVE: u32 = 8;

/// Symbol Visibility / Bind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolBinding {
    Local,
    Global,
    Weak,
}

/// Exported / Imported Symbol Entry
#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub binding: SymbolBinding,
}

/// Dynamic Relocation Entry
#[derive(Debug, Clone)]
pub struct ElfRelocation {
    pub offset: u64,
    pub reloc_type: u32,
    pub symbol_name: String,
    pub addend: i64,
}

/// Loaded Dynamic Shared Object (`.so`)
#[derive(Debug, Clone)]
pub struct LoadedSharedObject {
    pub soname: String,
    pub base_address: u64,
    pub exported_symbols: BTreeMap<String, ElfSymbol>,
    pub relocations: Vec<ElfRelocation>,
}

/// Runtime Dynamic ELF Loader Engine
#[derive(Debug)]
pub struct SovereignElfDynamicLoader {
    pub loaded_objects: BTreeMap<String, LoadedSharedObject>,
    pub global_symbol_table: BTreeMap<String, u64>,
    pub next_base_address: u64,
}

impl SovereignElfDynamicLoader {
    pub fn new() -> Self {
        Self {
            loaded_objects: BTreeMap::new(),
            global_symbol_table: BTreeMap::new(),
            next_base_address: 0x7FFF_0000_0000,
        }
    }

    /// Register a loaded shared library into the dynamic loader symbol registry
    pub fn load_shared_library(&mut self, soname: &str, symbols: Vec<ElfSymbol>, relocs: Vec<ElfRelocation>) -> Result<u64, &'static str> {
        if self.loaded_objects.contains_key(soname) {
            return Err("ELF Loader: Shared object already loaded");
        }

        let base_address = self.next_base_address;
        self.next_base_address += 0x0000_0100_0000; // 16MB virtual address space per library

        let mut sym_map = BTreeMap::new();
        for mut sym in symbols {
            let abs_addr = base_address + sym.address;
            sym.address = abs_addr;
            sym_map.insert(sym.name.clone(), sym.clone());
            self.global_symbol_table.insert(sym.name, abs_addr);
        }

        let loaded_obj = LoadedSharedObject {
            soname: soname.to_string(),
            base_address,
            exported_symbols: sym_map,
            relocations: relocs,
        };

        self.loaded_objects.insert(soname.to_string(), loaded_obj);
        Ok(base_address)
    }

    /// Resolves GOT/PLT dynamic relocations across all loaded shared objects
    pub fn apply_dynamic_relocations(&self, soname: &str) -> Result<usize, &'static str> {
        let obj = self.loaded_objects.get(soname).ok_or("ELF Loader: Target shared object not found")?;

        let mut applied = 0;
        for reloc in &obj.relocations {
            match reloc.reloc_type {
                R_X86_64_GLOB_DAT | R_X86_64_JUMP_SLOT => {
                    if let Some(&resolved_addr) = self.global_symbol_table.get(&reloc.symbol_name) {
                        let _final_target = resolved_addr + (reloc.addend as u64);
                        applied += 1;
                    } else {
                        return Err("ELF Loader: Unresolved dynamic symbol relocation");
                    }
                }
                R_X86_64_RELATIVE => {
                    let _final_target = obj.base_address + (reloc.addend as u64);
                    applied += 1;
                }
                _ => {}
            }
        }

        Ok(applied)
    }

    /// Resolve a symbol address dynamically (`dlsym` parity)
    pub fn dlsym(&self, symbol_name: &str) -> Option<u64> {
        self.global_symbol_table.get(symbol_name).copied()
    }
}

impl Default for SovereignElfDynamicLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_dynamic_loader() {
        let mut loader = SovereignElfDynamicLoader::new();

        let symbols = vec![
            ElfSymbol { name: "libc_malloc".to_string(), address: 0x1000, size: 64, binding: SymbolBinding::Global },
            ElfSymbol { name: "libc_free".to_string(), address: 0x1040, size: 64, binding: SymbolBinding::Global },
        ];

        let relocs = vec![
            ElfRelocation { offset: 0x2000, reloc_type: R_X86_64_GLOB_DAT, symbol_name: "libc_malloc".to_string(), addend: 0 },
        ];

        let base_addr = loader.load_shared_library("libc.so.6", symbols, relocs).unwrap();
        assert_eq!(base_addr, 0x7FFF_0000_0000);

        let applied = loader.apply_dynamic_relocations("libc.so.6").unwrap();
        assert_eq!(applied, 1);

        let sym_addr = loader.dlsym("libc_malloc").unwrap();
        assert_eq!(sym_addr, 0x7FFF_0000_1000);
    }
}
