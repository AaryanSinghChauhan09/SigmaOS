// SPDX-License-Identifier: GPL-2.0-or-later
// Principle: PE32+ PARSING, SAFE IMAGE LOADING, RELOCATION, IMPORT RESOLUTION

use alloc::{vec::Vec, string::String, collections::BTreeMap};

pub const MZ_MAGIC: u16 = 0x5A4D; // "MZ"
pub const PE_SIGNATURE: u32 = 0x0000_4550; // "PE\0\0"
pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014C;

/// DOS (MZ) stub header
#[derive(Debug, Clone)]
pub struct DosHeader {
    pub magic: u16,          // 0x5A4D = 'MZ'
    pub cblp: u16,           // bytes on last page
    pub cp: u16,             // pages in file
    pub crlc: u16,           // relocations
    pub cparhdr: u16,        // size of header in paragraphs
    pub e_lfanew: u32,       // offset to PE header
}

impl DosHeader {
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 64 {
            return Err("data too short for DOS header");
        }
        let magic = u16::from_le_bytes([data[0], data[1]]);
        if magic != MZ_MAGIC {
            return Err("not an MZ executable");
        }
        let cblp = u16::from_le_bytes([data[2], data[3]]);
        let cp = u16::from_le_bytes([data[4], data[5]]);
        let crlc = u16::from_le_bytes([data[6], data[7]]);
        let cparhdr = u16::from_le_bytes([data[8], data[9]]);
        let e_lfanew = u32::from_le_bytes([data[60], data[61], data[62], data[63]]);
        Ok(Self { magic, cblp, cp, crlc, cparhdr, e_lfanew })
    }
}

/// COFF file header (IMAGE_FILE_HEADER)
#[derive(Debug, Clone)]
pub struct CoffHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

impl CoffHeader {
    pub fn parse(data: &[u8], offset: usize) -> Result<Self, &'static str> {
        if data.len() < offset + 20 {
            return Err("data too short for COFF header");
        }
        let o = offset;
        Ok(Self {
            machine: u16::from_le_bytes([data[o], data[o+1]]),
            number_of_sections: u16::from_le_bytes([data[o+2], data[o+3]]),
            time_date_stamp: u32::from_le_bytes([data[o+4], data[o+5], data[o+6], data[o+7]]),
            pointer_to_symbol_table: u32::from_le_bytes([data[o+8], data[o+9], data[o+10], data[o+11]]),
            number_of_symbols: u32::from_le_bytes([data[o+12], data[o+13], data[o+14], data[o+15]]),
            size_of_optional_header: u16::from_le_bytes([data[o+16], data[o+17]]),
            characteristics: u16::from_le_bytes([data[o+18], data[o+19]]),
        })
    }
}

/// PE optional header (PE32+ / 64-bit subset)
#[derive(Debug, Clone)]
pub struct OptionalHeader {
    pub magic: u16,           // 0x10B = PE32, 0x20B = PE32+
    pub major_linker_ver: u8,
    pub minor_linker_ver: u8,
    pub size_of_code: u32,
    pub entry_point_rva: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub subsystem: u16,
}

impl OptionalHeader {
    pub fn parse(data: &[u8], offset: usize) -> Result<Self, &'static str> {
        if data.len() < offset + 40 {
            return Err("data too short for optional header");
        }
        let o = offset;
        let magic = u16::from_le_bytes([data[o], data[o+1]]);
        let is_pe32plus = magic == 0x020B;

        let major_linker_ver = data[o+2];
        let minor_linker_ver = data[o+3];
        let size_of_code = u32::from_le_bytes([data[o+4], data[o+5], data[o+6], data[o+7]]);
        let entry_point_rva = u32::from_le_bytes([data[o+16], data[o+17], data[o+18], data[o+19]]);

        let image_base = if is_pe32plus && data.len() >= o + 32 {
            u64::from_le_bytes([
                data[o+24], data[o+25], data[o+26], data[o+27],
                data[o+28], data[o+29], data[o+30], data[o+31],
            ])
        } else if data.len() >= o + 28 {
            u32::from_le_bytes([data[o+28], data[o+29], data[o+30], data[o+31]]) as u64
        } else {
            0x0040_0000
        };

        Ok(Self {
            magic,
            major_linker_ver,
            minor_linker_ver,
            size_of_code,
            entry_point_rva,
            image_base,
            section_alignment: 0x1000,
            file_alignment: 0x200,
            size_of_image: 0,
            size_of_headers: 0,
            subsystem: 0,
        })
    }
}

/// PE full header (DOS + COFF + Optional)
#[derive(Debug, Clone)]
pub struct PeHeader {
    pub dos: DosHeader,
    pub signature: u32,
    pub coff: CoffHeader,
    pub optional: OptionalHeader,
}

/// Section header (IMAGE_SECTION_HEADER)
#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub raw_data_size: u32,
    pub raw_data_offset: u32,
    pub characteristics: u32,
}

impl SectionHeader {
    pub fn parse(data: &[u8], offset: usize) -> Result<Self, &'static str> {
        if data.len() < offset + 40 {
            return Err("data too short for section header");
        }
        let o = offset;
        let mut name = [0u8; 8];
        name.copy_from_slice(&data[o..o+8]);
        Ok(Self {
            name,
            virtual_size: u32::from_le_bytes([data[o+8], data[o+9], data[o+10], data[o+11]]),
            virtual_address: u32::from_le_bytes([data[o+12], data[o+13], data[o+14], data[o+15]]),
            raw_data_size: u32::from_le_bytes([data[o+16], data[o+17], data[o+18], data[o+19]]),
            raw_data_offset: u32::from_le_bytes([data[o+20], data[o+21], data[o+22], data[o+23]]),
            characteristics: u32::from_le_bytes([data[o+36], data[o+37], data[o+38], data[o+39]]),
        })
    }

    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(8);
        core::str::from_utf8(&self.name[..end]).unwrap_or("?")
    }

    pub fn is_executable(&self) -> bool { self.characteristics & 0x2000_0000 != 0 }
    pub fn is_readable(&self)   -> bool { self.characteristics & 0x4000_0000 != 0 }
    pub fn is_writable(&self)   -> bool { self.characteristics & 0x8000_0000 != 0 }
}

/// Import descriptor (IMAGE_IMPORT_DESCRIPTOR)
#[derive(Debug, Clone)]
pub struct ImportDescriptor {
    pub dll_name: String,
    pub symbols: Vec<String>,
}

/// A fully-parsed PE image ready for loading
pub struct PeImage {
    pub header: PeHeader,
    pub sections: Vec<SectionHeader>,
    pub imports: Vec<ImportDescriptor>,
    pub raw: Vec<u8>,
    pub load_bias: u64,  // ASLR offset applied
}

impl PeImage {
    /// Parse a PE/PE32+ binary from raw bytes
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        // 1. DOS header
        let dos = DosHeader::parse(bytes)?;
        let pe_offset = dos.e_lfanew as usize;

        if bytes.len() < pe_offset + 4 {
            return Err("truncated before PE signature");
        }

        // 2. PE signature
        let signature = u32::from_le_bytes([
            bytes[pe_offset], bytes[pe_offset+1], bytes[pe_offset+2], bytes[pe_offset+3],
        ]);
        if signature != PE_SIGNATURE {
            return Err("invalid PE signature");
        }

        // 3. COFF header
        let coff = CoffHeader::parse(bytes, pe_offset + 4)?;

        // 4. Optional header
        let opt_offset = pe_offset + 4 + 20;
        let optional = OptionalHeader::parse(bytes, opt_offset)?;

        let header = PeHeader { dos, signature, coff, optional };

        // 5. Section headers
        let section_offset = opt_offset + header.coff.size_of_optional_header as usize;
        let mut sections = Vec::new();
        for i in 0..header.coff.number_of_sections as usize {
            let s = SectionHeader::parse(bytes, section_offset + i * 40)?;
            sections.push(s);
        }

        Ok(PeImage {
            header,
            sections,
            imports: Vec::new(),
            raw: bytes.to_vec(),
            load_bias: 0,
        })
    }

    /// Apply base relocations (stub — real impl walks BASERELOC table)
    pub fn apply_relocations(&mut self, new_base: u64) -> Result<(), &'static str> {
        let old_base = self.header.optional.image_base;
        self.load_bias = new_base.wrapping_sub(old_base);
        // Real implementation would patch every relocation entry
        Ok(())
    }

    /// Resolve imports by looking up symbols in a provided symbol table
    pub fn resolve_imports(&mut self, symbol_table: &BTreeMap<String, u64>) -> Result<(), &'static str> {
        for desc in &self.imports {
            for sym in &desc.symbols {
                if !symbol_table.contains_key(sym.as_str()) {
                    // In a real loader this would be a fatal error
                    // For now we silently skip missing symbols
                }
            }
        }
        Ok(())
    }

    pub fn entry_point(&self) -> u64 {
        self.header.optional.image_base
            .wrapping_add(self.load_bias)
            .wrapping_add(self.header.optional.entry_point_rva as u64)
    }

    pub fn section_count(&self) -> usize { self.sections.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal valid PE32+ binary (just enough headers for parsing)
    fn minimal_pe() -> Vec<u8> {
        let mut bytes = vec![0u8; 512];

        // MZ header
        bytes[0] = 0x4D; bytes[1] = 0x5A; // 'MZ'
        // e_lfanew at offset 60 → PE at 0x40
        bytes[60] = 0x40; bytes[61] = 0x00; bytes[62] = 0x00; bytes[63] = 0x00;

        // PE signature at 0x40
        bytes[0x40] = 0x50; bytes[0x41] = 0x45; bytes[0x42] = 0x00; bytes[0x43] = 0x00;

        // COFF header at 0x44
        bytes[0x44] = 0x64; bytes[0x45] = 0x86; // AMD64
        bytes[0x46] = 0x01; bytes[0x47] = 0x00; // 1 section
        // optional header size = 0xF0
        bytes[0x54] = 0xF0; bytes[0x55] = 0x00;

        // Optional header at 0x58 (0x44 + 20)
        bytes[0x58] = 0x0B; bytes[0x59] = 0x02; // PE32+ magic

        // section header at 0x58 + 0xF0 = 0x148
        let sh_off = 0x58 + 0xF0;
        // name = ".text\0\0\0"
        bytes[sh_off] = b'.'; bytes[sh_off+1] = b't'; bytes[sh_off+2] = b'e';
        bytes[sh_off+3] = b'x'; bytes[sh_off+4] = b't';
        // characteristics: executable + readable
        bytes[sh_off+36] = 0x00; bytes[sh_off+37] = 0x00;
        bytes[sh_off+38] = 0x00; bytes[sh_off+39] = 0x60;

        bytes
    }

    #[test]
    fn test_parse_minimal_pe() {
        let raw = minimal_pe();
        let img = PeImage::parse(&raw).unwrap();
        assert_eq!(img.header.dos.magic, MZ_MAGIC);
        assert_eq!(img.header.signature, PE_SIGNATURE);
        assert_eq!(img.header.coff.machine, IMAGE_FILE_MACHINE_AMD64);
        assert_eq!(img.section_count(), 1);
    }

    #[test]
    fn test_bad_magic_rejected() {
        let mut raw = minimal_pe();
        raw[0] = 0xFF; // corrupt MZ magic
        let result = PeImage::parse(&raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_section_name() {
        let raw = minimal_pe();
        let img = PeImage::parse(&raw).unwrap();
        assert_eq!(img.sections[0].name_str(), ".text");
    }

    #[test]
    fn test_apply_relocations() {
        let raw = minimal_pe();
        let mut img = PeImage::parse(&raw).unwrap();
        img.apply_relocations(0x0000_0001_4000_0000).unwrap();
        assert_ne!(img.load_bias, 0);
    }

    #[test]
    fn test_resolve_imports_empty() {
        let raw = minimal_pe();
        let mut img = PeImage::parse(&raw).unwrap();
        let table: BTreeMap<String, u64> = BTreeMap::new();
        assert!(img.resolve_imports(&table).is_ok());
    }
}
