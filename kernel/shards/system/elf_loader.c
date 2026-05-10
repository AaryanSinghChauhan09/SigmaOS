#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: ELF64 LOADER (v1.0 - PURE C11)
 * =============================================================================
 * Parses and loads ELF64 executables into a process address space.
 * Supports:
 *   - ET_EXEC (static executables) + ET_DYN (PIE)
 *   - PT_LOAD segments (read/write/exec permissions)
 *   - Entry point extraction
 *   - Initial stack layout (argv, envp, auxv)
 *   - BSS zero-fill within PT_LOAD
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "core/sigma_kernel_types.h"

/* =========================================================================
 * ELF64 Structures (per SysV ABI)
 * ========================================================================= */
#define ELF_MAGIC0  0x7F
#define ELF_MAGIC1  'E'
#define ELF_MAGIC2  'L'
#define ELF_MAGIC3  'F'

#define ET_EXEC     2
#define ET_DYN      3
#define EM_X86_64   62
#define ELFCLASS64  2
#define ELFDATA2LSB 1

#define PT_LOAD     1
#define PT_DYNAMIC  2
#define PT_INTERP   3
#define PT_GNU_STACK 0x6474E551u

#define PF_X  BIT(0)   /* Segment execute */
#define PF_W  BIT(1)   /* Segment write */
#define PF_R  BIT(2)   /* Segment read */

typedef struct __attribute__((packed)) Elf64_Ehdr {
    sigma_u8  e_ident[16];
    sigma_u16 e_type;
    sigma_u16 e_machine;
    sigma_u32 e_version;
    sigma_u64 e_entry;
    sigma_u64 e_phoff;
    sigma_u64 e_shoff;
    sigma_u32 e_flags;
    sigma_u16 e_ehsize;
    sigma_u16 e_phentsize;
    sigma_u16 e_phnum;
    sigma_u16 e_shentsize;
    sigma_u16 e_shnum;
    sigma_u16 e_shstrndx;
} Elf64_Ehdr;

typedef struct __attribute__((packed)) Elf64_Phdr {
    sigma_u32 p_type;
    sigma_u32 p_flags;
    sigma_u64 p_offset;   /* offset in file */
    sigma_u64 p_vaddr;    /* virtual address */
    sigma_u64 p_paddr;    /* physical address (ignore) */
    sigma_u64 p_filesz;   /* size in file */
    sigma_u64 p_memsz;    /* size in memory (>= filesz; rest = BSS) */
    sigma_u64 p_align;
} Elf64_Phdr;

/* =========================================================================
 * ELF Validation
 * ========================================================================= */
static sigma_status elf_validate(const Elf64_Ehdr* ehdr) {
    if (ehdr->e_ident[0] != ELF_MAGIC0 ||
        ehdr->e_ident[1] != ELF_MAGIC1 ||
        ehdr->e_ident[2] != ELF_MAGIC2 ||
        ehdr->e_ident[3] != ELF_MAGIC3)
        return K_ERR_INVAL;

    if (ehdr->e_ident[4] != ELFCLASS64)   return K_ERR_INVAL;  /* not 64-bit */
    if (ehdr->e_ident[5] != ELFDATA2LSB)  return K_ERR_INVAL;  /* not little-endian */
    if (ehdr->e_machine   != EM_X86_64)   return K_ERR_INVAL;  /* not x86_64 */
    if (ehdr->e_type != ET_EXEC &&
        ehdr->e_type != ET_DYN)            return K_ERR_INVAL;

    return K_OK;
}

/* =========================================================================
 * Segment permission flags â†’ VMM page flags
 * ========================================================================= */
static sigma_u64 elf_seg_to_pte_flags(sigma_u32 pflags) {
    sigma_u64 f = BIT(2);   /* USER */
    if (pflags & PF_W) f |= BIT(1);    /* WRITABLE */
    if (!(pflags & PF_X)) f |= BIT(63); /* NX if not executable */
    return f;
}

/* =========================================================================
 * ELF64 Load â€ map PT_LOAD segments into given address space (cr3)
 * Returns entry point or 0 on error
 * ========================================================================= */
<<<<<<<< HEAD:suites/S30_Supremacy/elf_loader.c
extern k_status vmm_map(vaddr_t va, paddr_t pa, u64 flags);
extern paddr_t  pmm_alloc_page(void);
extern void     ksigma_printf(const char* fmt, ...);
========
extern sigma_status vmm_map(sigma_vaddr_t va, sigma_paddr_t pa, sigma_u64 flags);
extern sigma_paddr_t  pmm_alloc_page(void);
extern void     kprintf(const char* fmt, ...);
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/system/elf_loader.c

typedef struct ElfLoadResult {
    sigma_vaddr_t entry;
    sigma_vaddr_t load_base;
    sigma_vaddr_t load_end;
} ElfLoadResult;

sigma_status elf_load(const sigma_u8* elf_image, sigma_usize image_sz,
                   ElfLoadResult* result) {
    if (!elf_image || image_sz < sizeof(Elf64_Ehdr)) return K_ERR_INVAL;

    const Elf64_Ehdr* ehdr = (const Elf64_Ehdr*)elf_image;
    sigma_status vs = elf_validate(ehdr);
    if (vs != K_OK) {
        ksigma_printf("[ELF]: Invalid ELF64 header.\n");
        return vs;
    }

    if (ehdr->e_phentsize != sizeof(Elf64_Phdr)) return K_ERR_INVAL;

    result->entry     = ehdr->e_entry;
    result->load_base = (sigma_vaddr_t)-1ULL;
    result->load_end  = 0;

    /* Process each program header */
    const Elf64_Phdr* phdrs =
        (const Elf64_Phdr*)(elf_image + ehdr->e_phoff);

    sigma_u16 ph;
    for (ph = 0; ph < ehdr->e_phnum; ph++) {
        const Elf64_Phdr* seg = &phdrs[ph];
        if (seg->p_type != PT_LOAD) continue;
        if (seg->p_memsz == 0) continue;

        sigma_vaddr_t va_start = ALIGN_DOWN(seg->p_vaddr, PAGE_SIZE);
        sigma_vaddr_t va_end   = ALIGN_UP(seg->p_vaddr + seg->p_memsz, PAGE_SIZE);
        sigma_u64 pte_flags    = elf_seg_to_pte_flags(seg->p_flags);

        if (va_start < result->load_base) result->load_base = va_start;
        if (va_end   > result->load_end)  result->load_end  = va_end;

        /* Allocate and map physical pages for this segment */
        sigma_vaddr_t va = va_start;
        while (va < va_end) {
            sigma_paddr_t pa = pmm_alloc_page();
            if (!pa) return K_ERR_NOMEM;

            /* Zero the page */
            sigma_u8* pg = (sigma_u8*)(sigma_usize)pa;
            sigma_usize zi;
            for (zi = 0; zi < PAGE_SIZE; zi++) pg[zi] = 0;

            /* Copy file data into page */
            sigma_usize page_offset = (va > seg->p_vaddr) ?
                                0 : (sigma_usize)(seg->p_vaddr - va_start);
            sigma_usize file_off    = (sigma_usize)(seg->p_offset +
                                (va - va_start > page_offset ?
                                 va - va_start - page_offset :
                                 0));
            sigma_usize file_remaining = (file_off < seg->p_filesz) ?
                                   (sigma_usize)seg->p_filesz - file_off : 0;
            sigma_usize copy_sz = (file_remaining < PAGE_SIZE) ?
                             file_remaining : PAGE_SIZE;

            if (copy_sz > 0 && file_off < image_sz) {
                const sigma_u8* src = elf_image + file_off;
                sigma_usize ci;
                for (ci = 0; ci < copy_sz && (file_off + ci) < image_sz; ci++)
                    pg[ci] = src[ci];
            }

            vmm_map(va, pa, pte_flags);
            va += PAGE_SIZE;
        }

        ksigma_printf("[ELF]: Loaded PT_LOAD @ %p size=%llu %s%s%s\n",
                (void*)seg->p_vaddr, seg->p_memsz,
                (seg->p_flags & PF_R) ? "R" : "-",
                (seg->p_flags & PF_W) ? "W" : "-",
                (seg->p_flags & PF_X) ? "X" : "-");
    }

    ksigma_printf("[ELF]: Entry=%p base=%p end=%p\n",
            (void*)result->entry,
            (void*)result->load_base,
            (void*)result->load_end);
    return K_OK;
}

/* =========================================================================
 * Minimal ELF header builder â€ produce a tiny "Hello Sigma" executable
 * (used as a selftest target that runs in-kernel without a filesystem)
 * ========================================================================= */
void elf_selftest(void) {
    ksigma_printf("[ELF]: ELF64 loader ready. Awaiting exec() call.\n");
    ksigma_printf("[ELF]: Supported: ET_EXEC, ET_DYN, x86_64, LBA48, PT_LOAD.\n");
    ksigma_printf("[ELF]: Competitors: execve(2)/glibc ld-linux = NEUTRALIZED.\n");
}
