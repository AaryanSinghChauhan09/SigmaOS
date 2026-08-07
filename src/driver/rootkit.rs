// SigmaOS Kernel-Mode Reverse Engineering & Hooking / Rootkit Subsystem
// Zero-dependency, #![no_std] compliant kernel structures.

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

// =========================================================================
// 1. Syscall Stub Disassembler & Opcode Parser
// =========================================================================

/// Simulates programmatically parsing and disassembling user-mode system call stubs (e.g. NTDLL stubs)
/// to extract the system call index directly from the machine instructions.
/// Format of a typical x64 syscall stub:
///   mov r10, rcx
///   mov eax, index  <- opcode: 0xB8 followed by 32-bit index
///   syscall
///   ret
pub struct SyscallStubDisassembler;

impl SyscallStubDisassembler {
    /// Disassembles the raw machine code bytes of a system call stub to extract the system call index
    pub fn extract_index_from_stub(machine_code: &[u8]) -> Result<u32, &'static str> {
        let mut i = 0;
        while i < machine_code.len() {
            // Look for 'mov eax, imm32' opcode: 0xB8
            if machine_code[i] == 0xB8 {
                if i + 4 < machine_code.len() {
                    // Extract 32-bit immediate value (little-endian)
                    let index = (machine_code[i + 1] as u32)
                        | ((machine_code[i + 2] as u32) << 8)
                        | ((machine_code[i + 3] as u32) << 16)
                        | ((machine_code[i + 4] as u32) << 24);
                    return Ok(index);
                }
            }
            // Also handle 'mov r8d, imm32' or similar if necessary, but 0xB8 is standard for eax/rax
            i += 1;
        }
        Err("Could not find system call index in stub bytecode")
    }
}

// =========================================================================
// 2. Sections, Objects, and Mapped Views (Windows Section Object Parity)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionBackingType {
    NormalFileBacked, // Disk file backed section
    PageFileBacked,   // Page file / RAM backed section (shared memory)
}

pub struct SectionObject {
    pub backing_type: SectionBackingType,
    pub size_bytes: usize,
    pub physical_pages: Vec<u64>,
    pub is_read_only: bool,
}

impl SectionObject {
    pub fn new(
        backing_type: SectionBackingType,
        size_bytes: usize,
        page_allocator: &mut dyn FnMut() -> u64,
    ) -> Self {
        let page_count = (size_bytes + 4095) / 4096;
        let mut pages = Vec::new();
        for _ in 0..page_count {
            pages.push(page_allocator());
        }

        Self {
            backing_type,
            size_bytes,
            physical_pages: pages,
            is_read_only: false,
        }
    }

    /// Creates a virtual address mapping view of this section (MapViewOfSection parity).
    /// Multiple views can map to the exact same physical pages (enabling shared memory).
    pub fn create_mapped_view(&self, start_virtual_address: u64) -> MappedView {
        MappedView {
            virtual_address_range_start: start_virtual_address,
            size_bytes: self.size_bytes,
            physical_pages: self.physical_pages.clone(),
        }
    }
}

pub struct MappedView {
    pub virtual_address_range_start: u64,
    pub size_bytes: usize,
    pub physical_pages: Vec<u64>,
}

impl MappedView {
    pub fn read_offset(&self, offset: usize) -> Result<u64, &'static str> {
        if offset >= self.size_bytes {
            return Err("Access violation: offset out of bounds of mapped view");
        }
        let page_idx = offset / 4096;
        let page_offset = offset % 4096;
        let phys_page = self.physical_pages[page_idx];
        Ok(phys_page + page_offset as u64)
    }
}

// =========================================================================
// 3. Rootkit Stealth & File Hiding by I/O Interception
// =========================================================================

#[derive(Debug, Clone)]
pub struct FileDirectoryEntry {
    pub filename: String,
    pub size_bytes: usize,
}

pub struct StealthFilterDriver {
    pub hidden_filename: String,
}

impl StealthFilterDriver {
    pub fn new(filename_to_hide: &str) -> Self {
        Self {
            hidden_filename: String::from(filename_to_hide),
        }
    }

    /// Intercepts the directory listing responses (IRP_MJ_DIRECTORY_CONTROL) and filters out hidden files
    pub fn filter_directory_response(
        &self,
        directory_entries: &mut Vec<FileDirectoryEntry>,
    ) -> usize {
        let initial_len = directory_entries.len();
        directory_entries.retain(|entry| entry.filename != self.hidden_filename);
        initial_len - directory_entries.len() // Return number of hidden files filtered out
    }
}

// =========================================================================
// Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_stub_disassembler() {
        // Mock bytecode for standard Windows x64 syscall stub:
        // mov r10, rcx
        // mov eax, 0x00000109  <- opcode: 0xB8, followed by little-endian 32-bit index (09 01 00 00)
        // syscall
        // ret
        let mock_machine_code = [
            0x4C, 0x8B, 0xD1, // mov r10, rcx
            0xB8, 0x09, 0x01, 0x00, 0x00, // mov eax, 0x109
            0x0F, 0x05, // syscall
            0xC3, // ret
        ];

        let index = SyscallStubDisassembler::extract_index_from_stub(&mock_machine_code).unwrap();
        assert_eq!(index, 0x109);
    }

    #[test]
    fn test_section_objects_and_mapped_views() {
        let mut page_counter = 0x5000_0000;
        let mut mock_allocator = || {
            let addr = page_counter;
            page_counter += 4096;
            addr
        };

        // Create a page-file backed shared memory section of 8192 bytes (2 pages)
        let section = SectionObject::new(
            SectionBackingType::PageFileBacked,
            8192,
            &mut mock_allocator,
        );

        assert_eq!(section.backing_type, SectionBackingType::PageFileBacked);
        assert_eq!(section.physical_pages.len(), 2);
        assert_eq!(section.physical_pages[0], 0x5000_0000);
        assert_eq!(section.physical_pages[1], 0x5000_1000);

        // Map multiple views of the same section at different virtual address ranges (shared memory!)
        let view_user = section.create_mapped_view(0x0000_7FFF_0000_0000);
        let view_kernel = section.create_mapped_view(0xFFFF_8000_0000_0000);

        // Both views resolve to the exact same physical frames!
        assert_eq!(view_user.read_offset(500).unwrap(), 0x5000_0000 + 500);
        assert_eq!(view_kernel.read_offset(500).unwrap(), 0x5000_0000 + 500);

        assert_eq!(view_user.read_offset(5000).unwrap(), 0x5000_1000 + 904);
        assert_eq!(view_kernel.read_offset(5000).unwrap(), 0x5000_1000 + 904);
    }

    #[test]
    fn test_stealth_rootkit_file_hiding() {
        let rootkit = StealthFilterDriver::new("stealth_payload.sys");

        let mut directory_listing = Vec::new();
        directory_listing.push(FileDirectoryEntry {
            filename: String::from("explorer.exe"),
            size_bytes: 520000,
        });
        directory_listing.push(FileDirectoryEntry {
            filename: String::from("stealth_payload.sys"),
            size_bytes: 4096,
        });
        directory_listing.push(FileDirectoryEntry {
            filename: String::from("ntoskrnl.exe"),
            size_bytes: 8192000,
        });

        assert_eq!(directory_listing.len(), 3);

        // Intercept directory query and hide our rootkit file!
        let hidden_count = rootkit.filter_directory_response(&mut directory_listing);
        assert_eq!(hidden_count, 1);
        assert_eq!(directory_listing.len(), 2);
        assert_eq!(directory_listing[0].filename, "explorer.exe");
        assert_eq!(directory_listing[1].filename, "ntoskrnl.exe");
    }
}
