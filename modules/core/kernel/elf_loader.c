#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS ELF Binary Loader Prototype
// ---------------------------------------------------------

#define EI_NIDENT 16

typedef struct {
    unsigned char e_ident[EI_NIDENT];
    uint16_t      e_type;
    uint16_t      e_machine;
    uint32_t      e_version;
    uint64_t      e_entry;
    uint64_t      e_phoff;
    uint64_t      e_shoff;
    uint32_t      e_flags;
    uint16_t      e_ehsize;
    uint16_t      e_phentsize;
    uint16_t      e_phnum;
    uint16_t      e_shentsize;
    uint16_t      e_shnum;
    uint16_t      e_shstrndx;
} elf64_ehdr_t;

typedef struct {
    uint32_t   p_type;
    uint32_t   p_flags;
    uint64_t   p_offset;
    uint64_t   p_vaddr;
    uint64_t   p_paddr;
    uint64_t   p_filesz;
    uint64_t   p_memsz;
    uint64_t   p_align;
} elf64_phdr_t;

#define PT_LOAD 1

// Validates ELF Magic Number
int elf_check_supported(elf64_ehdr_t *hdr) {
    if (hdr->e_ident[0] != 0x7f || hdr->e_ident[1] != 'E' ||
        hdr->e_ident[2] != 'L'  || hdr->e_ident[3] != 'F') {
        return 0; // Not an ELF file
    }
    if (hdr->e_ident[4] != 2) return 0; // Not 64-bit
    return 1;
}

// Loads an ELF binary into memory (Mock)
int load_elf_binary(const char* filepath) {
    // 1. Read ELF header from filesystem
    elf64_ehdr_t header; // Mock reading from filepath
    
    // 2. Validate format
    if (!elf_check_supported(&header)) {
        return -1; // Error
    }
    
    // 3. Create address space
    // address_space_t* as = create_address_space();
    
    // 4. Iterate Program Headers and load PT_LOAD segments
    // for each segment:
    //   vmm_allocate_page(as, phdr.p_vaddr, phdr.p_flags);
    //   copy_data(file_buffer + phdr.p_offset, phdr.p_vaddr, phdr.p_filesz);
    
    // 5. Setup stack and execute
    // jump_to(header.e_entry);
    
    return 0;
}
