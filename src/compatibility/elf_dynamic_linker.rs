// ELF Dynamic Linker (ld-linux.so equivalent)
// Implements runtime ELF dynamic linking, symbol resolution, GOT/PLT relocation

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::vec::Vec;

/// ELF class (32-bit or 64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfClass {
    Elf32,
    Elf64,
}

/// ELF data encoding (little-endian or big-endian)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfData {
    LittleEndian,
    BigEndian,
}

/// ELF machine architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ElfMachine {
    EM_NONE = 0,
    EM_X86_64 = 62,
    EM_AARCH64 = 183,
    EM_RISCV64 = 243,
}

/// ELF file type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ElfType {
    ET_NONE = 0,
    ET_REL = 1,      // Relocatable file
    ET_EXEC = 2,     // Executable file
    ET_DYN = 3,      // Shared object file
    ET_CORE = 4,     // Core file
}

/// ELF program header type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PhType {
    PT_NULL = 0,
    PT_LOAD = 1,      // Loadable segment
    PT_DYNAMIC = 2,   // Dynamic linking information
    PT_INTERP = 3,    // Interpreter path
    PT_NOTE = 4,      // Auxiliary information
    PT_SHLIB = 5,     // Reserved
    PT_PHDR = 6,      // Program header table
    PT_TLS = 7,       // Thread-local storage
    PT_GNU_EH_FRAME = 0x6474e550,
    PT_GNU_STACK = 0x6474e551,
    PT_GNU_RELRO = 0x6474e552,
}

/// ELF dynamic array entry types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DynTag {
    DT_NULL = 0,
    DT_NEEDED = 1,      // Name of needed library
    DT_PLTRELSZ = 2,    // Size of PLT relocation entries
    DT_PLTGOT = 3,      // Address of PLT GOT
    DT_HASH = 4,        // Address of symbol hash table
    DT_STRTAB = 5,      // Address of string table
    DT_SYMTAB = 6,      // Address of symbol table
    DT_RELA = 7,        // Address of relocation entries
    DT_RELASZ = 8,      // Size of relocation entries
    DT_RELAENT = 9,     // Size of relocation entry
    DT_STRSZ = 10,      // Size of string table
    DT_SYMENT = 11,     // Size of symbol entry
    DT_INIT = 12,       // Address of initialization function
    DT_FINI = 13,       // Address of termination function
    DT_SONAME = 14,     // Shared object name
    DT_RPATH = 15,      // Library search path
    DT_SYMBOLIC = 16,   // Start symbol search here
    DT_REL = 17,        // Address of relocation entries
    DT_RELSZ = 18,      // Size of relocation entries
    DT_RELENT = 19,     // Size of relocation entry
    DT_PLTREL = 20,     // Type of PLT relocation
    DT_DEBUG = 21,      // Debug information
    DT_TEXTREL = 22,    // Relocation may modify text
    DT_JMPREL = 23,     // Address of PLT relocations
    DT_BIND_NOW = 24,   // Process relocations now
    DT_INIT_ARRAY = 25, // Initialization function array
    DT_FINI_ARRAY = 26, // Termination function array
    DT_INIT_ARRAYSZ = 27,
    DT_FINI_ARRAYSZ = 28,
    DT_RUNPATH = 29,    // Library search path
    DT_FLAGS = 30,      // Dynamic flags
    DT_PREINIT_ARRAY = 32,
    DT_PREINIT_ARRAYSZ = 33,
}

/// ELF symbol binding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SymBind {
    STB_LOCAL = 0,
    STB_GLOBAL = 1,
    STB_WEAK = 2,
}

/// ELF symbol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SymType {
    STT_NOTYPE = 0,
    STT_OBJECT = 1,
    STT_FUNC = 2,
    STT_SECTION = 3,
    STT_FILE = 4,
    STT_COMMON = 5,
    STT_TLS = 6,
}

/// ELF relocation type (x86_64)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum RelType {
    R_X86_64_NONE = 0,
    R_X86_64_64 = 1,          // Direct 64-bit
    R_X86_64_PC32 = 2,        // PC-relative 32-bit
    R_X86_64_GOT32 = 3,       // 32-bit GOT entry
    R_X86_64_PLT32 = 4,       // 32-bit PLT entry
    R_X86_64_COPY = 5,        // Copy from shared object
    R_X86_64_GLOB_DAT = 6,    // GOT entry for data
    R_X86_64_JUMP_SLOT = 7,   // PLT entry for function
    R_X86_64_RELATIVE = 8,    // Relative relocation
    R_X86_64_GOTPCREL = 9,    // 32-bit PC-relative GOT
    R_X86_64_32 = 10,         // Direct 32-bit
    R_X86_64_32S = 11,        // Direct 32-bit with sign extension
    R_X86_64_16 = 12,         // Direct 16-bit
    R_X86_64_PC16 = 13,       // PC-relative 16-bit
    R_X86_64_8 = 14,          // Direct 8-bit
    R_X86_64_PC8 = 15,        // PC-relative 8-bit
    R_X86_64_IRELATIVE = 37,  // Indirect relative
}

/// ELF symbol
#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub bind: SymBind,
    pub sym_type: SymType,
    pub shndx: u16,
}

/// ELF relocation entry
#[derive(Debug, Clone)]
pub struct ElfRelocation {
    pub offset: u64,
    pub info: u64,
    pub addend: i64,
    pub rel_type: RelType,
    pub symbol_index: u32,
}

/// Shared library object
#[derive(Debug, Clone)]
pub struct SharedLibrary {
    pub name: String,
    pub base_address: u64,
    pub symbols: BTreeMap<String, ElfSymbol>,
    pub plt_got: u64,
    pub dynamic: Vec<(DynTag, u64)>,
}

/// Global Offset Table (GOT) entry
#[derive(Debug, Clone)]
pub struct GotEntry {
    pub address: u64,
    pub value: u64,
    pub relocated: bool,
}

/// Procedure Linkage Table (PLT) entry
#[derive(Debug, Clone)]
pub struct PltEntry {
    pub address: u64,
    pub got_entry: u64,
    pub symbol_name: String,
    pub resolved: bool,
}

/// ELF dynamic linker
#[derive(Debug)]
pub struct ElfDynamicLinker {
    loaded_libraries: BTreeMap<String, SharedLibrary>,
    global_symbol_table: BTreeMap<String, (u64, String)>, // symbol -> (address, library_name)
    got: BTreeMap<u64, GotEntry>,
    plt: BTreeMap<u64, PltEntry>,
    total_relocations: AtomicU64,
    total_symbols_resolved: AtomicU64,
}

impl ElfDynamicLinker {
    pub fn new() -> Self {
        ElfDynamicLinker {
            loaded_libraries: BTreeMap::new(),
            global_symbol_table: BTreeMap::new(),
            got: BTreeMap::new(),
            plt: BTreeMap::new(),
            total_relocations: AtomicU64::new(0),
            total_symbols_resolved: AtomicU64::new(0),
        }
    }

    /// Load shared library
    pub fn load_library(&mut self, name: String, base_address: u64) -> Result<(), &'static str> {
        if self.loaded_libraries.contains_key(&name) {
            return Err("Library already loaded");
        }

        let library = SharedLibrary {
            name: name.clone(),
            base_address,
            symbols: BTreeMap::new(),
            plt_got: base_address + 0x2000, // Placeholder PLT/GOT offset
            dynamic: Vec::new(),
        };

        self.loaded_libraries.insert(name.clone(), library);
        Ok(())
    }

    /// Add symbol to library
    pub fn add_symbol(&mut self, library_name: &str, symbol: ElfSymbol) {
        if let Some(library) = self.loaded_libraries.get_mut(library_name) {
            library.symbols.insert(symbol.name.clone(), symbol.clone());
            self.global_symbol_table.insert(
                symbol.name.clone(),
                (symbol.value, library_name.to_string()),
            );
        }
    }

    /// Add dynamic entry
    pub fn add_dynamic_entry(&mut self, library_name: &str, tag: DynTag, value: u64) {
        if let Some(library) = self.loaded_libraries.get_mut(library_name) {
            library.dynamic.push((tag, value));
        }
    }

    /// Resolve symbol (dlsym equivalent)
    pub fn resolve_symbol(&self, symbol_name: &str) -> Option<u64> {
        if let Some((address, _)) = self.global_symbol_table.get(symbol_name) {
            self.total_symbols_resolved.fetch_add(1, Ordering::SeqCst);
            Some(*address)
        } else {
            None
        }
    }

    /// Create GOT entry
    pub fn create_got_entry(&mut self, address: u64) {
        self.got.insert(
            address,
            GotEntry {
                address,
                value: 0,
                relocated: false,
            },
        );
    }

    /// Create PLT entry
    pub fn create_plt_entry(&mut self, plt_address: u64, got_address: u64, symbol_name: String) {
        self.plt.insert(
            plt_address,
            PltEntry {
                address: plt_address,
                got_entry: got_address,
                symbol_name: symbol_name.clone(),
                resolved: false,
            },
        );
    }

    /// Perform relocation
    pub fn perform_relocation(&mut self, reloc: &ElfRelocation) -> Result<(), &'static str> {
        self.total_relocations.fetch_add(1, Ordering::SeqCst);

        match reloc.rel_type {
            RelType::R_X86_64_RELATIVE => {
                // Relative relocation: addend + base address
                if let Some(got_entry) = self.got.get_mut(&reloc.offset) {
                    got_entry.value = reloc.addend as u64 + 0x1000; // Placeholder base
                    got_entry.relocated = true;
                }
            }
            RelType::R_X86_64_GLOB_DAT | RelType::R_X86_64_JUMP_SLOT => {
                // Global data or PLT jump slot: resolve symbol
                if let Some(got_entry) = self.got.get_mut(&reloc.offset) {
                    // Look up symbol by index (simplified)
                    if let Some((address, _)) = self.global_symbol_table.values().nth(reloc.symbol_index as usize) {
                        got_entry.value = *address;
                        got_entry.relocated = true;
                    }
                }
            }
            RelType::R_X86_64_64 => {
                // Direct 64-bit relocation
                if let Some(got_entry) = self.got.get_mut(&reloc.offset) {
                    got_entry.value = reloc.addend as u64;
                    got_entry.relocated = true;
                }
            }
            RelType::R_X86_64_COPY => {
                // Copy relocation from shared object
                if let Some(got_entry) = self.got.get_mut(&reloc.offset) {
                    got_entry.value = reloc.addend as u64;
                    got_entry.relocated = true;
                }
            }
            _ => {
                // Other relocation types not implemented
            }
        }

        Ok(())
    }

    /// Process PLT relocations (lazy binding)
    pub fn process_plt_relocations(&mut self) {
        let mut to_resolve: Vec<(u64, String)> = Vec::new();

        // Collect symbols to resolve
        for plt_entry in self.plt.values() {
            if !plt_entry.resolved {
                to_resolve.push((plt_entry.got_entry, plt_entry.symbol_name.clone()));
            }
        }

        // Resolve symbols
        for (got_address, symbol_name) in to_resolve {
            if let Some(address) = self.resolve_symbol(&symbol_name) {
                if let Some(got_entry) = self.got.get_mut(&got_address) {
                    got_entry.value = address;
                    got_entry.relocated = true;
                }
                // Mark PLT entry as resolved
                for plt_entry in self.plt.values_mut() {
                    if plt_entry.got_entry == got_address {
                        plt_entry.resolved = true;
                        break;
                    }
                }
            }
        }
    }

    /// Get loaded library
    pub fn get_library(&self, name: &str) -> Option<&SharedLibrary> {
        self.loaded_libraries.get(name)
    }

    /// Get statistics
    pub fn get_stats(&self) -> LinkerStats {
        LinkerStats {
            libraries_loaded: self.loaded_libraries.len() as u64,
            total_symbols: self.global_symbol_table.len() as u64,
            total_relocations: self.total_relocations.load(Ordering::SeqCst),
            symbols_resolved: self.total_symbols_resolved.load(Ordering::SeqCst),
            got_entries: self.got.len() as u64,
            plt_entries: self.plt.len() as u64,
        }
    }
}

impl Default for ElfDynamicLinker {
    fn default() -> Self {
        Self::new()
    }
}

/// Linker statistics
#[derive(Debug, Clone)]
pub struct LinkerStats {
    pub libraries_loaded: u64,
    pub total_symbols: u64,
    pub total_relocations: u64,
    pub symbols_resolved: u64,
    pub got_entries: u64,
    pub plt_entries: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_library() {
        let mut linker = ElfDynamicLinker::new();
        assert!(linker.load_library("libc.so.6".to_string(), 0x7f0000000000).is_ok());
        assert!(linker.load_library("libc.so.6".to_string(), 0x7f0000000000).is_err());
    }

    #[test]
    fn test_add_symbol() {
        let mut linker = ElfDynamicLinker::new();
        linker.load_library("libc.so.6".to_string(), 0x7f0000000000).unwrap();

        let symbol = ElfSymbol {
            name: "printf".to_string(),
            value: 0x7f0000001000,
            size: 64,
            bind: SymBind::STB_GLOBAL,
            sym_type: SymType::STT_FUNC,
            shndx: 0,
        };

        linker.add_symbol("libc.so.6", symbol);
        assert!(linker.resolve_symbol("printf").is_some());
        assert!(linker.resolve_symbol("nonexistent").is_none());
    }

    #[test]
    fn test_got_plt_entries() {
        let mut linker = ElfDynamicLinker::new();
        linker.create_got_entry(0x7f0000002000);
        linker.create_plt_entry(0x7f0000003000, 0x7f0000002000, "malloc".to_string());

        let stats = linker.get_stats();
        assert_eq!(stats.got_entries, 1);
        assert_eq!(stats.plt_entries, 1);
    }

    #[test]
    fn test_relocation() {
        let mut linker = ElfDynamicLinker::new();
        linker.load_library("libc.so.6".to_string(), 0x7f0000000000).unwrap();

        let symbol = ElfSymbol {
            name: "malloc".to_string(),
            value: 0x7f0000001000,
            size: 64,
            bind: SymBind::STB_GLOBAL,
            sym_type: SymType::STT_FUNC,
            shndx: 0,
        };
        linker.add_symbol("libc.so.6", symbol);

        linker.create_got_entry(0x7f0000002000);

        let reloc = ElfRelocation {
            offset: 0x7f0000002000,
            info: 0,
            addend: 0,
            rel_type: RelType::R_X86_64_GLOB_DAT,
            symbol_index: 0,
        };

        assert!(linker.perform_relocation(&reloc).is_ok());
        let stats = linker.get_stats();
        assert_eq!(stats.total_relocations, 1);
    }

    #[test]
    fn test_plt_resolution() {
        let mut linker = ElfDynamicLinker::new();
        linker.load_library("libc.so.6".to_string(), 0x7f0000000000).unwrap();

        let symbol = ElfSymbol {
            name: "printf".to_string(),
            value: 0x7f0000001000,
            size: 64,
            bind: SymBind::STB_GLOBAL,
            sym_type: SymType::STT_FUNC,
            shndx: 0,
        };
        linker.add_symbol("libc.so.6", symbol);

        linker.create_got_entry(0x7f0000002000);
        linker.create_plt_entry(0x7f0000003000, 0x7f0000002000, "printf".to_string());

        linker.process_plt_relocations();

        let stats = linker.get_stats();
        assert_eq!(stats.symbols_resolved, 1);
    }
}
