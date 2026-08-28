extern crate alloc;
// Dynamic ELF Relocation Primitives for SigmaOS
// Zero-dependency, #![no_std] compliant ELF dynamic symbol relocation resolution (x86_64 ABI parity)

use alloc::vec::Vec;

/// Standard x86_64 ELF Relocation Types
pub const R_X86_64_NONE: u32 = 0;
pub const R_X86_64_64: u32 = 1;
pub const R_X86_64_PC32: u32 = 2;
pub const R_X86_64_GOT32: u32 = 3;
pub const R_X86_64_PLT32: u32 = 4;
pub const R_X86_64_GLOB_DAT: u32 = 6;
pub const R_X86_64_JUMP_SLOT: u32 = 7;
pub const R_X86_64_RELATIVE: u32 = 8;
pub const R_X86_64_GOTPCREL: u32 = 9;

/// ELF Symbol Entry Representation
#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: Vec<u8>,
    pub value: u64,
    pub size: u64,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
}

impl ElfSymbol {
    pub fn new(name: &[u8], value: u64, size: u64) -> Self {
        Self {
            name: Vec::from(name),
            value,
            size,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
        }
    }
}

/// ELF Relocation Entry with Addend (Elf64_Rela)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElfRelaEntry {
    pub offset: u64,
    pub info: u64,
    pub addend: i64,
}

impl ElfRelaEntry {
    pub const fn new(offset: u64, reloc_type: u32, symbol_idx: u32, addend: i64) -> Self {
        let info = ((symbol_idx as u64) << 32) | (reloc_type as u64);
        Self {
            offset,
            info,
            addend,
        }
    }

    pub const fn reloc_type(&self) -> u32 {
        (self.info & 0xFFFF_FFFF) as u32
    }

    pub const fn symbol_index(&self) -> u32 {
        (self.info >> 32) as u32
    }
}

/// Dynamic Linker & Symbol Relocator
#[derive(Debug, Clone)]
pub struct ElfRelocator {
    pub base_address: u64,
    pub symbols: Vec<ElfSymbol>,
}

impl ElfRelocator {
    pub fn new(base_address: u64) -> Self {
        Self {
            base_address,
            symbols: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: ElfSymbol) {
        self.symbols.push(symbol);
    }

    pub fn lookup_symbol(&self, name: &[u8]) -> Option<&ElfSymbol> {
        for sym in self.symbols.iter() {
            if sym.name.as_slice() == name {
                return Some(sym);
            }
        }
        None
    }

    /// Resolves an ELF relocation entry and computes target relocation value
    pub fn resolve_relocation(
        &self,
        entry: &ElfRelaEntry,
        symbol_name: Option<&[u8]>,
    ) -> Result<u64, &'static str> {
        let reloc_type = entry.reloc_type();
        let addend = entry.addend as u64;

        match reloc_type {
            R_X86_64_NONE => Ok(0),
            R_X86_64_RELATIVE => {
                // Base + Addend B + A
                Ok(self.base_address.wrapping_add(addend))
            }
            R_X86_64_64 | R_X86_64_GLOB_DAT | R_X86_64_JUMP_SLOT => {
                // Symbol Value + Addend S + A
                let s_name = symbol_name.ok_or("Relocation requires symbol name")?;
                let sym = self
                    .lookup_symbol(s_name)
                    .ok_or("Undefined symbol for relocation")?;
                Ok(sym.value.wrapping_add(addend))
            }
            R_X86_64_PC32 | R_X86_64_PLT32 => {
                // S + A - P (Relative to place)
                let s_name = symbol_name.ok_or("Relocation requires symbol name")?;
                let sym = self
                    .lookup_symbol(s_name)
                    .ok_or("Undefined symbol for relocation")?;
                let p = self.base_address.wrapping_add(entry.offset);
                Ok(sym.value.wrapping_add(addend).wrapping_sub(p))
            }
            _ => Err("Unsupported ELF relocation type"),
        }
    }
}
