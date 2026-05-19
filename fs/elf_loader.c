/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: ELF EXECUTABLE BINARY LOADER
 * =============================================================================
 * Inspired by: Linux kernel fs/binfmt_elf.c
 *              FreeBSD sys/kern/imgact_elf.c
 * =============================================================================
 * Parses and validates Executable and Linkable Format (ELF) binaries.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define ELF_MAGIC 0x464C457F /* '\x7f', 'E', 'L', 'F' */

#define ELFCLASS64 2
#define ELFDATA2LSB 1
#define EV_CURRENT 1

#define ET_EXEC 2
#define ET_DYN  3

#define EM_X86_64 62

#define PT_LOAD 1

typedef struct {
    sigma_u32  magic;
    sigma_u8   elf_class;
    sigma_u8   data_encoding;
    sigma_u8   version;
    sigma_u8   os_abi;
    sigma_u8   abi_version;
    sigma_u8   pad[7];
    sigma_u16  type;
    sigma_u16  machine;
    sigma_u32  elf_version;
    sigma_u64  entry_point;
    sigma_u64  program_header_offset;
    sigma_u64  section_header_offset;
    sigma_u32  flags;
    sigma_u16  ehsize;
    sigma_u16  phentsize;
    sigma_u16  phnum;
    sigma_u16  shentsize;
    sigma_u16  shnum;
    sigma_u16  shstrndx;
} __attribute__((packed)) elf64_ehdr_t;

typedef struct {
    sigma_u32  type;
    sigma_u32  flags;
    sigma_u64  offset;
    sigma_u64  vaddr;
    sigma_u64  paddr;
    sigma_u64  filesz;
    sigma_u64  memsz;
    sigma_u64  align;
} __attribute__((packed)) elf64_phdr_t;

int elf_load_binary(const void* file_data, sigma_u32 file_size, sigma_u64* out_entry) {
    if (file_size < sizeof(elf64_ehdr_t)) {
        sigma_printf("[elf] ERR: File too small to be an ELF\n");
        return -1;
    }

    const elf64_ehdr_t* ehdr = (const elf64_ehdr_t*)file_data;

    /* Validate Magic */
    if (ehdr->magic != ELF_MAGIC) {
        sigma_printf("[elf] ERR: Invalid ELF magic (found 0x%X)\n", ehdr->magic);
        return -1;
    }

    /* Validate architecture constraints */
    if (ehdr->elf_class != ELFCLASS64 || ehdr->data_encoding != ELFDATA2LSB) {
        sigma_printf("[elf] ERR: Only 64-bit Little-Endian ELF supported\n");
        return -1;
    }

    if (ehdr->machine != EM_X86_64) {
        sigma_printf("[elf] ERR: Unsupported machine type (expected x86_64)\n");
        return -1;
    }

    if (ehdr->type != ET_EXEC && ehdr->type != ET_DYN) {
        sigma_printf("[elf] ERR: Not an executable or PIE binary\n");
        return -1;
    }

    sigma_printf("[elf] Loading ELF64 executable (Entry: 0x%llx, %u Program Headers)\n", 
                 ehdr->entry_point, ehdr->phnum);

    /* Parse Program Headers */
    const sigma_u8* phdr_base = (const sigma_u8*)file_data + ehdr->program_header_offset;
    
    for (sigma_u16 i = 0; i < ehdr->phnum; i++) {
        const elf64_phdr_t* phdr = (const elf64_phdr_t*)(phdr_base + (i * ehdr->phentsize));
        
        if (phdr->type == PT_LOAD) {
            sigma_printf("[elf]  -> PT_LOAD: vaddr=0x%llx, memsz=%llu bytes, flags=0x%X\n",
                         phdr->vaddr, phdr->memsz, phdr->flags);
            
            /* Real kernel: Allocate virtual memory here and map it to physical frames.
               Then memcpy(phdr->vaddr, file_data + phdr->offset, phdr->filesz)
               and memset(phdr->vaddr + phdr->filesz, 0, phdr->memsz - phdr->filesz) for BSS */
        }
    }

    *out_entry = ehdr->entry_point;
    return 0;
}
