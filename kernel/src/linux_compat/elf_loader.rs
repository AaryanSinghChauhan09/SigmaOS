/// Error types for zero-dependency ELF parser
#[derive(Debug)]
pub enum ElfError {
    InvalidMagic,
    Not64Bit,
    NotLittleEndian,
    InvalidClass,
}

#[repr(C)]
struct Elf64Header {
    magic: [u8; 4],
    class: u8,
    data: u8,
    version: u8,
    osabi: u8,
    abiversion: u8,
    pad: [u8; 7],
    e_type: u16,
    machine: u16,
    version2: u32,
    entry: u64,
    phoff: u64,
    shoff: u64,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

pub struct ElfLoader;

impl ElfLoader {
    pub fn load(data: &[u8]) -> Result<u64, ElfError> {
        if data.len() < core::mem::size_of::<Elf64Header>() {
            return Err(ElfError::InvalidMagic);
        }

        let header = unsafe { &*(data.as_ptr() as *const Elf64Header) };

        // Verify ELF Magic
        if header.magic != [0x7f, b'E', b'L', b'F'] {
            return Err(ElfError::InvalidMagic);
        }

        // Class 2 is 64-bit ELF
        if header.class != 2 {
            return Err(ElfError::Not64Bit);
        }

        // Data 1 is Little Endian
        if header.data != 1 {
            return Err(ElfError::NotLittleEndian);
        }

        // Return entry point of static Linux binary
        Ok(header.entry)
    }
}
