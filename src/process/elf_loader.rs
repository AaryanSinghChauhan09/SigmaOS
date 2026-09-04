// SPDX-License-Identifier: MIT
/// SigmaOS: ELF Binary Loader
/// Loads and parses ELF (Executable and Linkable Format) binaries for execution

use std::string::{String, ToString};
use std::vec::Vec;
use core::fmt;
use core::mem;

/// ELF Header Magic Number
const ELF_MAGIC: &[u8; 4] = b"\x7FELF";

/// ELF Class (32-bit vs 64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfClass {
    Bits32 = 1,
    Bits64 = 2,
}

/// ELF Data Encoding (endianness)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfEncoding {
    Little = 1,
    Big = 2,
}

/// ELF OS ABI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfOsAbi {
    Unix = 0,
    Linux = 3,
    FreeBsd = 9,
    OpenBsd = 12,
}

/// ELF Type (executable, shared object, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfType {
    Relocatable = 1,
    Executable = 2,
    Shared = 3,
    Core = 4,
}

/// ELF Machine Type (architecture)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfMachine {
    X86 = 3,
    Mips = 8,
    PowerPC = 20,
    Arm = 40,
    X86_64 = 62,
    AArch64 = 183,
    RiscV = 243,
}

/// Program Header Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramHeaderType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    Shlib = 5,
    Phdr = 6,
}

/// Section Header Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionHeaderType {
    Null = 0,
    Progbits = 1,
    Symtab = 2,
    Strtab = 3,
    Rela = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    Nobits = 8,
    Rel = 9,
    Shlib = 10,
    Dynsym = 11,
}

/// ELF Header (64-bit)
#[derive(Debug, Clone)]
pub struct ElfHeader {
    pub class: ElfClass,
    pub encoding: ElfEncoding,
    pub os_abi: ElfOsAbi,
    pub elf_type: ElfType,
    pub machine: ElfMachine,
    pub version: u32,
    pub entry_point: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub header_size: u16,
    pub program_header_size: u16,
    pub program_header_count: u16,
    pub section_header_size: u16,
    pub section_header_count: u16,
    pub string_section_index: u16,
}

/// Program Header (64-bit)
#[derive(Debug, Clone)]
pub struct ProgramHeader {
    pub header_type: ProgramHeaderType,
    pub flags: u32,
    pub offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub alignment: u64,
}

/// Section Header (64-bit)
#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name_offset: u32,
    pub header_type: SectionHeaderType,
    pub flags: u64,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub address_align: u64,
    pub entity_size: u64,
}

/// Loadable Segment
#[derive(Debug, Clone)]
pub struct LoadableSegment {
    pub virtual_address: u64,
    pub file_offset: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub executable: bool,
    pub writable: bool,
    pub readable: bool,
}

/// ELF Loading Error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElfError {
    InvalidMagic,
    InvalidClass,
    InvalidEncoding,
    InvalidType,
    InvalidMachine,
    InvalidFormat,
    NotExecutable,
    CorruptedHeader,
    CorruptedProgramHeader,
    CorruptedSectionHeader,
    NoLoadableSegments,
}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic => write!(f, "Invalid ELF magic number"),
            Self::InvalidClass => write!(f, "Invalid ELF class"),
            Self::InvalidEncoding => write!(f, "Invalid ELF encoding"),
            Self::InvalidType => write!(f, "Invalid ELF type"),
            Self::InvalidMachine => write!(f, "Invalid machine type"),
            Self::InvalidFormat => write!(f, "Invalid ELF format"),
            Self::NotExecutable => write!(f, "Not an executable"),
            Self::CorruptedHeader => write!(f, "Corrupted ELF header"),
            Self::CorruptedProgramHeader => write!(f, "Corrupted program header"),
            Self::CorruptedSectionHeader => write!(f, "Corrupted section header"),
            Self::NoLoadableSegments => write!(f, "No loadable segments found"),
        }
    }
}

/// ELF Loader
pub struct ElfLoader;

impl ElfLoader {
    /// Parse ELF header from binary data
    pub fn parse_header(data: &[u8]) -> Result<ElfHeader, ElfError> {
        if data.len() < 64 {
            return Err(ElfError::InvalidFormat);
        }

        // Check magic number
        if &data[0..4] != ELF_MAGIC {
            return Err(ElfError::InvalidMagic);
        }

        let class = match data[4] {
            1 => ElfClass::Bits32,
            2 => ElfClass::Bits64,
            _ => return Err(ElfError::InvalidClass),
        };

        let encoding = match data[5] {
            1 => ElfEncoding::Little,
            2 => ElfEncoding::Big,
            _ => return Err(ElfError::InvalidEncoding),
        };

        let os_abi = match data[7] {
            0 => ElfOsAbi::Unix,
            3 => ElfOsAbi::Linux,
            9 => ElfOsAbi::FreeBsd,
            12 => ElfOsAbi::OpenBsd,
            _ => ElfOsAbi::Unix,
        };

        // For 64-bit little-endian (most common case)
        if class != ElfClass::Bits64 || encoding != ElfEncoding::Little {
            return Err(ElfError::InvalidFormat);
        }

        let elf_type = match u16::from_le_bytes([data[16], data[17]]) {
            1 => ElfType::Relocatable,
            2 => ElfType::Executable,
            3 => ElfType::Shared,
            4 => ElfType::Core,
            _ => return Err(ElfError::InvalidType),
        };

        let machine = match u16::from_le_bytes([data[18], data[19]]) {
            3 => ElfMachine::X86,
            8 => ElfMachine::Mips,
            20 => ElfMachine::PowerPC,
            40 => ElfMachine::Arm,
            62 => ElfMachine::X86_64,
            183 => ElfMachine::AArch64,
            243 => ElfMachine::RiscV,
            _ => return Err(ElfError::InvalidMachine),
        };

        let version = u32::from_le_bytes([data[20], data[21], data[22], data[23]]);

        // Read 64-bit addresses and offsets
        let entry_point = u64::from_le_bytes([
            data[32], data[33], data[34], data[35], data[36], data[37], data[38], data[39],
        ]);

        let program_header_offset = u64::from_le_bytes([
            data[40], data[41], data[42], data[43], data[44], data[45], data[46], data[47],
        ]);

        let section_header_offset = u64::from_le_bytes([
            data[48], data[49], data[50], data[51], data[52], data[53], data[54], data[55],
        ]);

        let flags = u32::from_le_bytes([data[56], data[57], data[58], data[59]]);
        let header_size = u16::from_le_bytes([data[60], data[61]]);
        let program_header_size = u16::from_le_bytes([data[62], data[63]]);
        let program_header_count = u16::from_le_bytes([data[64], data[65]]);
        let section_header_size = u16::from_le_bytes([data[66], data[67]]);
        let section_header_count = u16::from_le_bytes([data[68], data[69]]);
        let string_section_index = u16::from_le_bytes([data[70], data[71]]);

        Ok(ElfHeader {
            class,
            encoding,
            os_abi,
            elf_type,
            machine,
            version,
            entry_point,
            program_header_offset,
            section_header_offset,
            flags,
            header_size,
            program_header_size,
            program_header_count,
            section_header_size,
            section_header_count,
            string_section_index,
        })
    }

    /// Get all loadable segments
    pub fn get_loadable_segments(
        data: &[u8],
        header: &ElfHeader,
    ) -> Result<Vec<LoadableSegment>, ElfError> {
        let mut segments = Vec::new();

        for i in 0..header.program_header_count {
            let offset = (header.program_header_offset
                + (i as u64 * header.program_header_size as u64)) as usize;

            if offset + header.program_header_size as usize > data.len() {
                return Err(ElfError::CorruptedProgramHeader);
            }

            let segment_data = &data[offset..offset + header.program_header_size as usize];

            // Parse program header (32 bytes for type and flags, followed by offsets and sizes)
            let segment_type = u32::from_le_bytes([
                segment_data[0],
                segment_data[1],
                segment_data[2],
                segment_data[3],
            ]);

            if segment_type != 1 {
                // Only interested in LOAD segments (type 1)
                continue;
            }

            let flags = u32::from_le_bytes([
                segment_data[4],
                segment_data[5],
                segment_data[6],
                segment_data[7],
            ]);

            let file_offset = u64::from_le_bytes([
                segment_data[8], segment_data[9], segment_data[10], segment_data[11],
                segment_data[12], segment_data[13], segment_data[14], segment_data[15],
            ]);

            let vaddr = u64::from_le_bytes([
                segment_data[16], segment_data[17], segment_data[18], segment_data[19],
                segment_data[20], segment_data[21], segment_data[22], segment_data[23],
            ]);

            let file_size = u64::from_le_bytes([
                segment_data[32], segment_data[33], segment_data[34], segment_data[35],
                segment_data[36], segment_data[37], segment_data[38], segment_data[39],
            ]);

            let memory_size = u64::from_le_bytes([
                segment_data[40], segment_data[41], segment_data[42], segment_data[43],
                segment_data[44], segment_data[45], segment_data[46], segment_data[47],
            ]);

            segments.push(LoadableSegment {
                virtual_address: vaddr,
                file_offset,
                file_size,
                memory_size,
                executable: (flags & 0x1) != 0,
                writable: (flags & 0x2) != 0,
                readable: (flags & 0x4) != 0,
            });
        }

        if segments.is_empty() {
            return Err(ElfError::NoLoadableSegments);
        }

        Ok(segments)
    }

    /// Validate ELF executable
    pub fn validate(data: &[u8]) -> Result<(), ElfError> {
        let header = Self::parse_header(data)?;

        if header.elf_type != ElfType::Executable && header.elf_type != ElfType::Shared {
            return Err(ElfError::NotExecutable);
        }

        // Verify we have loadable segments
        Self::get_loadable_segments(data, &header)?;

        Ok(())
    }

    /// Get entry point of executable
    pub fn get_entry_point(data: &[u8]) -> Result<u64, ElfError> {
        let header = Self::parse_header(data)?;
        Ok(header.entry_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_minimal_elf() -> Vec<u8> {
        let mut data = vec![0u8; 512];

        // ELF magic
        data[0..4].copy_from_slice(b"\x7FELF");

        // Class (64-bit)
        data[4] = 2;

        // Encoding (little-endian)
        data[5] = 1;

        // OS ABI
        data[7] = 0; // Unix

        // Type (executable)
        data[16..18].copy_from_slice(&(2u16).to_le_bytes());

        // Machine (x86-64)
        data[18..20].copy_from_slice(&(62u16).to_le_bytes());

        // Version
        data[20..24].copy_from_slice(&(1u32).to_le_bytes());

        // Entry point
        data[32..40].copy_from_slice(&(0x400000u64).to_le_bytes());

        // Program header offset
        data[40..48].copy_from_slice(&(64u64).to_le_bytes());

        // Section header offset (unused for now)
        data[48..56].copy_from_slice(&(0u64).to_le_bytes());

        data
    }

    #[test]
    fn test_parse_header() {
        let data = create_minimal_elf();
        let header = ElfLoader::parse_header(&data).unwrap();

        assert_eq!(header.class, ElfClass::Bits64);
        assert_eq!(header.encoding, ElfEncoding::Little);
        assert_eq!(header.elf_type, ElfType::Executable);
        assert_eq!(header.machine, ElfMachine::X86_64);
        assert_eq!(header.entry_point, 0x400000);
    }

    #[test]
    fn test_invalid_magic() {
        let data = vec![0u8; 64];
        let result = ElfLoader::parse_header(&data);
        assert_eq!(result, Err(ElfError::InvalidMagic));
    }

    #[test]
    fn test_invalid_class() {
        let mut data = vec![0u8; 64];
        data[0..4].copy_from_slice(b"\x7FELF");
        data[4] = 99; // Invalid class

        let result = ElfLoader::parse_header(&data);
        assert_eq!(result, Err(ElfError::InvalidClass));
    }
}
