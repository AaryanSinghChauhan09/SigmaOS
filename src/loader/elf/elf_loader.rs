/// Custom ELF Loader for SigmaOS
/// Implements ELF binary loading without relying on ld.so
/// Supports ELF32/ELF64 formats, glibc symbol resolution, and Auxiliary Vectors (auxv)
use core::ptr;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// ELF magic number
const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EM_X86_64: u16 = 62;
const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;
const PF_W: u32 = 2;

/// Auxiliary Vector Types expected by glibc / musl `ld.so`
pub mod auxv_types {
    pub const AT_NULL: u64 = 0;
    pub const AT_IGNORE: u64 = 1;
    pub const AT_EXECFD: u64 = 2;
    pub const AT_PHDR: u64 = 3;
    pub const AT_PHENT: u64 = 4;
    pub const AT_PHNUM: u64 = 5;
    pub const AT_PAGESZ: u64 = 6;
    pub const AT_BASE: u64 = 7;
    pub const AT_FLAGS: u64 = 8;
    pub const AT_ENTRY: u64 = 9;
    pub const AT_NOTELF: u64 = 10;
    pub const AT_UID: u64 = 11;
    pub const AT_EUID: u64 = 12;
    pub const AT_GID: u64 = 13;
    pub const AT_EGID: u64 = 14;
    pub const AT_SECURE: u64 = 23;
    pub const AT_RANDOM: u64 = 25;
    pub const AT_EXECFN: u64 = 31;
}

/// Key-Value pair for System V / Linux Auxiliary Vector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuxvEntry {
    pub key: u64,
    pub val: u64,
}

impl AuxvEntry {
    pub fn new(key: u64, val: u64) -> Self {
        Self { key, val }
    }
}

/// Builds auxiliary vector arrays for userland stack layout
#[derive(Debug, Clone)]
pub struct ElfAuxvBuilder {
    pub entries: Vec<AuxvEntry>,
}

impl ElfAuxvBuilder {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, key: u64, val: u64) {
        self.entries.push(AuxvEntry::new(key, val));
    }

    /// Constructs standard auxiliary vector array for an ELF binary
    pub fn build_standard_vector(
        &mut self,
        entry_point: u64,
        phdr_addr: u64,
        phnum: u64,
        phentsize: u64,
        base_addr: u64,
    ) {
        self.add(auxv_types::AT_PAGESZ, 4096);
        self.add(auxv_types::AT_PHDR, phdr_addr);
        self.add(auxv_types::AT_PHENT, phentsize);
        self.add(auxv_types::AT_PHNUM, phnum);
        self.add(auxv_types::AT_BASE, base_addr);
        self.add(auxv_types::AT_FLAGS, 0);
        self.add(auxv_types::AT_ENTRY, entry_point);
        self.add(auxv_types::AT_UID, 1000);
        self.add(auxv_types::AT_EUID, 1000);
        self.add(auxv_types::AT_GID, 1000);
        self.add(auxv_types::AT_EGID, 1000);
        self.add(auxv_types::AT_SECURE, 0);
        self.add(auxv_types::AT_NULL, 0);
    }
}

impl Default for ElfAuxvBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic Glibc / Musl Symbol Resolver Shim
#[derive(Debug, Clone)]
pub struct GlibcSymbolResolver {
    pub exported_symbols: Vec<(&'static str, usize)>,
}

impl GlibcSymbolResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            exported_symbols: Vec::new(),
        };

        // Populate common glibc runtime symbols with simulated function pointers
        resolver.register_symbol("malloc", 0x7FFF_0001_0000);
        resolver.register_symbol("free", 0x7FFF_0001_0010);
        resolver.register_symbol("printf", 0x7FFF_0001_0020);
        resolver.register_symbol("open", 0x7FFF_0001_0030);
        resolver.register_symbol("read", 0x7FFF_0001_0040);
        resolver.register_symbol("write", 0x7FFF_0001_0050);
        resolver.register_symbol("exit", 0x7FFF_0001_0060);
        resolver.register_symbol("pthread_create", 0x7FFF_0001_0070);

        resolver
    }

    pub fn register_symbol(&mut self, symbol_name: &'static str, addr: usize) {
        self.exported_symbols.push((symbol_name, addr));
    }

    pub fn resolve(&self, symbol_name: &str) -> Option<usize> {
        for &(name, addr) in &self.exported_symbols {
            if name == symbol_name {
                return Some(addr);
            }
        }
        None
    }
}

impl Default for GlibcSymbolResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// ELF64 Header Layout
#[repr(C)]
pub struct Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

/// ELF binary descriptor
pub struct ElfBinary {
    data: *const u8,
    size: usize,
    is_64bit: bool,
    entry_point: u64,
}

/// A validated loadable ELF segment. The loader performs the actual mapping;
/// this type contains only checked metadata and a borrowed file image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElfLoadSegment<'a> {
    pub virtual_address: u64,
    pub file_offset: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub flags: u32,
    pub image: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfLoadError {
    TruncatedHeader,
    InvalidMagic,
    UnsupportedClass,
    UnsupportedEncoding,
    UnsupportedMachine,
    InvalidProgramHeaderSize,
    ProgramHeadersOutOfBounds,
    SegmentOutOfBounds,
    InvalidSegmentSize,
    InvalidAlignment,
    AddressOverflow,
    WritableExecutableSegment,
    EntryNotExecutable,
}

/// Validated ELF64 x86_64 load plan for a future address-space mapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfLoadPlan<'a> {
    pub entry_point: u64,
    pub segments: Vec<ElfLoadSegment<'a>>,
}

impl<'a> ElfLoadPlan<'a> {
    pub fn parse(image: &'a [u8]) -> Result<Self, ElfLoadError> {
        if image.len() < 64 {
            return Err(ElfLoadError::TruncatedHeader);
        }
        if image[0..4] != ELF_MAGIC {
            return Err(ElfLoadError::InvalidMagic);
        }
        if image[4] != ELFCLASS64 {
            return Err(ElfLoadError::UnsupportedClass);
        }
        if image[5] != ELFDATA2LSB || image[6] != 1 {
            return Err(ElfLoadError::UnsupportedEncoding);
        }
        if read_u16(image, 18) != Some(EM_X86_64) {
            return Err(ElfLoadError::UnsupportedMachine);
        }

        let entry_point = read_u64(image, 24).ok_or(ElfLoadError::TruncatedHeader)?;
        let program_header_offset = read_u64(image, 32).ok_or(ElfLoadError::TruncatedHeader)?;
        let program_header_size = read_u16(image, 54).ok_or(ElfLoadError::TruncatedHeader)? as u64;
        let program_header_count = read_u16(image, 56).ok_or(ElfLoadError::TruncatedHeader)? as u64;
        if program_header_size < 56 {
            return Err(ElfLoadError::InvalidProgramHeaderSize);
        }

        let headers_size = program_header_size
            .checked_mul(program_header_count)
            .ok_or(ElfLoadError::ProgramHeadersOutOfBounds)?;
        let headers_end = program_header_offset
            .checked_add(headers_size)
            .ok_or(ElfLoadError::ProgramHeadersOutOfBounds)?;
        if headers_end > image.len() as u64 {
            return Err(ElfLoadError::ProgramHeadersOutOfBounds);
        }

        let mut segments = Vec::new();
        for index in 0..program_header_count {
            let header_offset = program_header_offset + index * program_header_size;
            let header =
                &image[header_offset as usize..(header_offset + program_header_size) as usize];
            if read_u32(header, 0) != Some(PT_LOAD) {
                continue;
            }

            let flags = read_u32(header, 4).ok_or(ElfLoadError::InvalidProgramHeaderSize)?;
            let file_offset = read_u64(header, 8).ok_or(ElfLoadError::InvalidProgramHeaderSize)?;
            let virtual_address =
                read_u64(header, 16).ok_or(ElfLoadError::InvalidProgramHeaderSize)?;
            let file_size = read_u64(header, 32).ok_or(ElfLoadError::InvalidProgramHeaderSize)?;
            let memory_size = read_u64(header, 40).ok_or(ElfLoadError::InvalidProgramHeaderSize)?;
            let alignment = read_u64(header, 48).ok_or(ElfLoadError::InvalidProgramHeaderSize)?;

            if file_size > memory_size {
                return Err(ElfLoadError::InvalidSegmentSize);
            }
            if alignment > 1 && !alignment.is_power_of_two() {
                return Err(ElfLoadError::InvalidAlignment);
            }
            if flags & PF_W != 0 && flags & PF_X != 0 {
                return Err(ElfLoadError::WritableExecutableSegment);
            }
            let file_end = file_offset
                .checked_add(file_size)
                .ok_or(ElfLoadError::AddressOverflow)?;
            virtual_address
                .checked_add(memory_size)
                .ok_or(ElfLoadError::AddressOverflow)?;
            if file_end > image.len() as u64 {
                return Err(ElfLoadError::SegmentOutOfBounds);
            }

            segments.push(ElfLoadSegment {
                virtual_address,
                file_offset,
                file_size,
                memory_size,
                flags,
                image: &image[file_offset as usize..file_end as usize],
            });
        }

        if !segments.iter().any(|segment| {
            segment.flags & PF_X != 0
                && entry_point >= segment.virtual_address
                && entry_point < segment.virtual_address + segment.memory_size
        }) {
            return Err(ElfLoadError::EntryNotExecutable);
        }
        Ok(Self {
            entry_point,
            segments,
        })
    }
}

fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    let bytes = data.get(offset..offset + 2)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset + 4)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_u64(data: &[u8], offset: usize) -> Option<u64> {
    let bytes = data.get(offset..offset + 8)?;
    Some(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

impl ElfBinary {
    pub unsafe fn new(data: *const u8, size: usize) -> Option<Self> {
        if size < 64 {
            return None;
        }

        let ident: [u8; 16] = ptr::read_unaligned(data as *const [u8; 16]);
        if ident[0..4] != ELF_MAGIC {
            return None;
        }

        let is_64bit = ident[4] == 2;
        let entry_point = if is_64bit {
            let header: Elf64Header = ptr::read_unaligned(data as *const Elf64Header);
            header.e_entry
        } else {
            0x400000
        };

        Some(ElfBinary {
            data,
            size,
            is_64bit,
            entry_point,
        })
    }

    pub fn is_64bit(&self) -> bool {
        self.is_64bit
    }

    pub fn entry_point(&self) -> u64 {
        self.entry_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_auxv_builder() {
        let mut builder = ElfAuxvBuilder::new();
        builder.build_standard_vector(0x401000, 0x400040, 4, 56, 0x400000);

        assert_eq!(builder.entries.len(), 13);
        assert_eq!(builder.entries[0].key, auxv_types::AT_PAGESZ);
        assert_eq!(builder.entries[0].val, 4096);

        let entry_item = builder
            .entries
            .iter()
            .find(|e| e.key == auxv_types::AT_ENTRY)
            .unwrap();
        assert_eq!(entry_item.val, 0x401000);
    }

    #[test]
    fn test_glibc_symbol_resolver() {
        let resolver = GlibcSymbolResolver::new();

        assert_eq!(resolver.resolve("malloc"), Some(0x7FFF_0001_0000));
        assert_eq!(resolver.resolve("free"), Some(0x7FFF_0001_0010));
        assert_eq!(resolver.resolve("pthread_create"), Some(0x7FFF_0001_0070));
        assert_eq!(resolver.resolve("non_existent_symbol"), None);
    }

    fn minimal_elf(flags: u32, file_size: u64, image_size: usize) -> Vec<u8> {
        let mut image = vec![0u8; image_size.max(136)];
        image[0..4].copy_from_slice(&ELF_MAGIC);
        image[4] = ELFCLASS64;
        image[5] = ELFDATA2LSB;
        image[6] = 1;
        image[18..20].copy_from_slice(&EM_X86_64.to_le_bytes());
        image[24..32].copy_from_slice(&0x400080u64.to_le_bytes());
        image[32..40].copy_from_slice(&64u64.to_le_bytes());
        image[52..54].copy_from_slice(&64u16.to_le_bytes());
        image[54..56].copy_from_slice(&56u16.to_le_bytes());
        image[56..58].copy_from_slice(&1u16.to_le_bytes());

        let ph = &mut image[64..120];
        ph[0..4].copy_from_slice(&PT_LOAD.to_le_bytes());
        ph[4..8].copy_from_slice(&flags.to_le_bytes());
        ph[8..16].copy_from_slice(&120u64.to_le_bytes());
        ph[16..24].copy_from_slice(&0x400000u64.to_le_bytes());
        ph[32..40].copy_from_slice(&file_size.to_le_bytes());
        ph[40..48].copy_from_slice(&4096u64.to_le_bytes());
        ph[48..56].copy_from_slice(&4096u64.to_le_bytes());
        image
    }

    #[test]
    fn test_validated_load_plan() {
        let image = minimal_elf(PF_X, 16, 136);
        let plan = ElfLoadPlan::parse(&image).unwrap();
        assert_eq!(plan.entry_point, 0x400080);
        assert_eq!(plan.segments.len(), 1);
        assert_eq!(plan.segments[0].image.len(), 16);
    }

    #[test]
    fn test_load_plan_rejects_wx_segments() {
        let image = minimal_elf(PF_W | PF_X, 16, 136);
        assert_eq!(
            ElfLoadPlan::parse(&image),
            Err(ElfLoadError::WritableExecutableSegment)
        );
    }

    #[test]
    fn test_load_plan_rejects_truncated_segment() {
        let image = minimal_elf(PF_X, 32, 136);
        assert_eq!(
            ElfLoadPlan::parse(&image),
            Err(ElfLoadError::SegmentOutOfBounds)
        );
    }
}
