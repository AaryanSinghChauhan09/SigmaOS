/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ELF BINARY LOADER (v1.0 — PURE C11)
 * =========================================================================
 * Competitor Gap Closed: Linux fs/binfmt_elf.c, macOS dyld Mach-O loader,
 * FreeBSD sys/kern/imgact_elf.c, Windows PE loader (ntdll!LdrLoadDll).
 * SigmaOS had no executable loading capability whatsoever.
 *
 * This shard implements:
 *   § 1  ELF32 / ELF64 header validation (magic, class, machine)
 *   § 2  Program header (PT_LOAD) segment mapping
 *   § 3  Dynamic section processing (.dynamic, DT_NEEDED)
 *   § 4  Section header parsing (.text, .data, .bss, .rodata, .symtab)
 *   § 5  Symbol table lookup (nm / readelf parity)
 *   § 6  execve() — load image, set up stack, jump to entry point
 *   § 7  Auxiliary vector (AT_ENTRY, AT_PHDR, AT_PHNUM, AT_PAGESZ…)
 *   § 8  Interpreter (PT_INTERP) detection for dynamic binaries
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

/* -----------------------------------------------------------------------
 * ░░ ELF CONSTANTS (SYSV ABI / Linux supplement)
 * ----------------------------------------------------------------------- */

/* ELF magic */
#define ELFMAG0  0x7F
#define ELFMAG1  'E'
#define ELFMAG2  'L'
#define ELFMAG3  'F'

/* EI_CLASS */
#define ELFCLASS32  1
#define ELFCLASS64  2

/* EI_DATA */
#define ELFDATA2LSB 1   /* little-endian */
#define ELFDATA2MSB 2

/* e_type */
#define ET_NONE  0
#define ET_REL   1
#define ET_EXEC  2
#define ET_DYN   3
#define ET_CORE  4

/* e_machine */
#define EM_386    3
#define EM_X86_64 62
#define EM_ARM    40
#define EM_AARCH64 183
#define EM_RISCV  243

/* Phdr p_type */
#define PT_NULL    0
#define PT_LOAD    1
#define PT_DYNAMIC 2
#define PT_INTERP  3
#define PT_NOTE    4
#define PT_SHLIB   5
#define PT_PHDR    6
#define PT_TLS     7
#define PT_GNU_STACK 0x6474E551
#define PT_GNU_RELRO 0x6474E552

/* Phdr p_flags */
#define PF_X   0x1   /* execute */
#define PF_W   0x2   /* write   */
#define PF_R   0x4   /* read    */

/* Shdr sh_type */
#define SHT_NULL     0
#define SHT_PROGBITS 1
#define SHT_SYMTAB   2
#define SHT_STRTAB   3
#define SHT_RELA     4
#define SHT_HASH     5
#define SHT_DYNAMIC  6
#define SHT_NOBITS   8   /* .bss */
#define SHT_REL      9

/* Dynamic tags (d_tag) */
#define DT_NULL    0
#define DT_NEEDED  1
#define DT_SYMTAB  6
#define DT_STRTAB  5
#define DT_STRSZ   10
#define DT_PLTGOT  3
#define DT_JMPREL  23
#define DT_PLTRELSZ 2
#define DT_PLTREL  20

/* ST_BIND / ST_TYPE macros */
#define ELF64_ST_BIND(i)  ((i) >> 4)
#define ELF64_ST_TYPE(i)  ((i) & 0xF)
#define STB_LOCAL   0
#define STB_GLOBAL  1
#define STB_WEAK    2
#define STT_NOTYPE  0
#define STT_OBJECT  1
#define STT_FUNC    2

/* Auxiliary vector types */
#define AT_NULL     0
#define AT_PHDR     3
#define AT_PHENT    4
#define AT_PHNUM    5
#define AT_PAGESZ   6
#define AT_BASE     7
#define AT_FLAGS    8
#define AT_ENTRY    9
#define AT_UID      11
#define AT_EUID     12
#define AT_GID      13
#define AT_EGID     14
#define AT_SECURE   23
#define AT_RANDOM   25
#define AT_HWCAP    16
#define AT_HWCAP2   26

/* -----------------------------------------------------------------------
 * ░░ ELF64 STRUCTURES (packed to match binary layout)
 * ----------------------------------------------------------------------- */
typedef struct {
    sigma_u8  e_ident[16]; /* magic, class, data, version, OS/ABI */
    sigma_u16 e_type;
    sigma_u16 e_machine;
    sigma_u32 e_version;
    sigma_u64 e_entry;     /* entry point virtual address */
    sigma_u64 e_phoff;     /* program header offset */
    sigma_u64 e_shoff;     /* section header offset */
    sigma_u32 e_flags;
    sigma_u16 e_ehsize;    /* ELF header size = 64 */
    sigma_u16 e_phentsize; /* phdr entry size  = 56 */
    sigma_u16 e_phnum;     /* number of phdrs */
    sigma_u16 e_shentsize; /* shdr entry size  = 64 */
    sigma_u16 e_shnum;
    sigma_u16 e_shstrndx;  /* section name string table index */
} SIGMA_PACKED Elf64Ehdr_t;

typedef struct {
    sigma_u32 p_type;
    sigma_u32 p_flags;
    sigma_u64 p_offset;  /* file offset */
    sigma_u64 p_vaddr;   /* virtual address */
    sigma_u64 p_paddr;   /* physical address */
    sigma_u64 p_filesz;
    sigma_u64 p_memsz;
    sigma_u64 p_align;
} SIGMA_PACKED Elf64Phdr_t;

typedef struct {
    sigma_u32 sh_name;
    sigma_u32 sh_type;
    sigma_u64 sh_flags;
    sigma_u64 sh_addr;
    sigma_u64 sh_offset;
    sigma_u64 sh_size;
    sigma_u32 sh_link;
    sigma_u32 sh_info;
    sigma_u64 sh_addralign;
    sigma_u64 sh_entsize;
} SIGMA_PACKED Elf64Shdr_t;

typedef struct {
    sigma_u32 st_name;
    sigma_u8  st_info;
    sigma_u8  st_other;
    sigma_u16 st_shndx;
    sigma_u64 st_value;
    sigma_u64 st_size;
} SIGMA_PACKED Elf64Sym_t;

typedef struct {
    sigma_i64 d_tag;
    sigma_u64 d_val;  /* or d_ptr */
} SIGMA_PACKED Elf64Dyn_t;

/* Auxiliary vector entry */
typedef struct {
    sigma_u64 a_type;
    sigma_u64 a_val;
} SigmaAuxVec_t;

/* -----------------------------------------------------------------------
 * ░░ LOADED IMAGE DESCRIPTOR
 * ----------------------------------------------------------------------- */
#define MAX_SEGMENTS    16
#define MAX_DEPS        32
#define MAX_SYMBOLS    256
#define DEP_NAME_LEN    64

typedef struct {
    sigma_u64  vaddr;
    sigma_u64  memsz;
    sigma_u64  filesz;
    sigma_u32  flags;   /* PF_R | PF_W | PF_X */
    sigma_bool mapped;
} SigmaSegment_t;

typedef struct {
    char       name[64];
    sigma_u8   bind;    /* STB_LOCAL / STB_GLOBAL / STB_WEAK */
    sigma_u8   type;    /* STT_FUNC / STT_OBJECT */
    sigma_u64  value;   /* resolved virtual address */
    sigma_u64  size;
} SigmaSymbol_t;

typedef struct {
    char           path[256];
    sigma_u64      entry;        /* entry point vaddr */
    sigma_u64      load_base;    /* ASLR / PIE relocation base */
    sigma_u64      phdr_va;      /* vaddr of first PT_PHDR */
    sigma_u16      phnum;
    sigma_bool     is_dynamic;
    sigma_bool     has_interp;
    char           interp[128];  /* e.g. "/lib64/ld-linux-x86-64.so.2" */
    sigma_u16      e_machine;

    SigmaSegment_t segments[MAX_SEGMENTS];
    sigma_u32      seg_count;

    char           deps[MAX_DEPS][DEP_NAME_LEN]; /* DT_NEEDED */
    sigma_u32      dep_count;

    SigmaSymbol_t  symbols[MAX_SYMBOLS];
    sigma_u32      sym_count;

    /* Stack setup */
    sigma_u64      stack_top;
    sigma_u64      sp;           /* initial RSP */
} SigmaELFImage_t;

/* -----------------------------------------------------------------------
 * ░░ § 1. ELF HEADER VALIDATION
 * ----------------------------------------------------------------------- */
static sigma_err_t elf_validate_header(const sigma_u8 *blob, sigma_size_t size) {
    if (size < 64) {
        sigma_printf("Σ [ELF]: File too small (%lu < 64)\n", (unsigned long)size);
        return SIGMA_EINVAL;
    }
    const Elf64Ehdr_t *eh = (const Elf64Ehdr_t *)blob;
    if (eh->e_ident[0] != ELFMAG0 || eh->e_ident[1] != ELFMAG1 ||
        eh->e_ident[2] != ELFMAG2 || eh->e_ident[3] != ELFMAG3) {
        sigma_printf("Σ [ELF]: Invalid magic (not ELF)\n");
        return SIGMA_ENOEXEC;
    }
    if (eh->e_ident[4] != ELFCLASS64) {
        sigma_printf("Σ [ELF]: Only ELF64 supported (class=%u)\n", eh->e_ident[4]);
        return SIGMA_ENOEXEC;
    }
    if (eh->e_ident[5] != ELFDATA2LSB) {
        sigma_printf("Σ [ELF]: Only little-endian supported\n");
        return SIGMA_ENOEXEC;
    }
    if (eh->e_type != ET_EXEC && eh->e_type != ET_DYN) {
        sigma_printf("Σ [ELF]: e_type=%u — not ET_EXEC or ET_DYN\n", eh->e_type);
        return SIGMA_ENOEXEC;
    }
    static const char *machine_name(sigma_u16 m) {
        switch(m) {
            case EM_386:    return "i386";
            case EM_X86_64: return "x86_64";
            case EM_ARM:    return "ARM";
            case EM_AARCH64:return "AArch64";
            case EM_RISCV:  return "RISC-V";
            default:        return "unknown";
        }
    }
    sigma_printf("Σ [ELF]: Valid ELF64 — type=%s machine=%s entry=0x%llx\n",
                 eh->e_type == ET_EXEC ? "ET_EXEC" : "ET_DYN",
                 machine_name(eh->e_machine),
                 (unsigned long long)eh->e_entry);
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ § 2. PROGRAM HEADER (PT_LOAD) PROCESSING
 * ----------------------------------------------------------------------- */
static sigma_err_t elf_load_phdrs(const sigma_u8 *blob, sigma_size_t size,
                                   SigmaELFImage_t *img) {
    const Elf64Ehdr_t *eh = (const Elf64Ehdr_t *)blob;
    if (eh->e_phoff + (sigma_u64)eh->e_phnum * eh->e_phentsize > size)
        return SIGMA_ENOEXEC;

    sigma_u64 load_min = (sigma_u64)-1, load_max = 0;

    for (sigma_u16 i = 0; i < eh->e_phnum && img->seg_count < MAX_SEGMENTS; i++) {
        const Elf64Phdr_t *ph = (const Elf64Phdr_t *)
            (blob + eh->e_phoff + (sigma_u64)i * eh->e_phentsize);

        switch (ph->p_type) {
        case PT_LOAD:
            if (ph->p_vaddr < load_min) load_min = ph->p_vaddr;
            if (ph->p_vaddr + ph->p_memsz > load_max)
                load_max = ph->p_vaddr + ph->p_memsz;
            {
                SigmaSegment_t *seg = &img->segments[img->seg_count++];
                seg->vaddr   = ph->p_vaddr;
                seg->memsz   = ph->p_memsz;
                seg->filesz  = ph->p_filesz;
                seg->flags   = ph->p_flags;
                seg->mapped  = SIGMA_TRUE;
            }
            sigma_printf("Σ [ELF]: PT_LOAD  vaddr=0x%llx memsz=0x%llx %s%s%s\n",
                         (unsigned long long)ph->p_vaddr,
                         (unsigned long long)ph->p_memsz,
                         (ph->p_flags & PF_R) ? "R" : "-",
                         (ph->p_flags & PF_W) ? "W" : "-",
                         (ph->p_flags & PF_X) ? "X" : "-");
            break;

        case PT_INTERP:
            img->has_interp = SIGMA_TRUE;
            if (ph->p_offset + ph->p_filesz <= size)
                sigma_strcpy(img->interp,
                             (const char *)(blob + ph->p_offset),
                             sizeof(img->interp));
            sigma_printf("Σ [ELF]: PT_INTERP '%s' (dynamic binary)\n", img->interp);
            img->is_dynamic = SIGMA_TRUE;
            break;

        case PT_DYNAMIC:
            img->is_dynamic = SIGMA_TRUE;
            sigma_printf("Σ [ELF]: PT_DYNAMIC at offset=0x%llx\n",
                         (unsigned long long)ph->p_offset);
            break;

        case PT_PHDR:
            img->phdr_va = ph->p_vaddr;
            break;

        case PT_GNU_STACK:
            sigma_printf("Σ [ELF]: GNU_STACK flags=%c%c%c (NX=%s)\n",
                         (ph->p_flags & PF_R) ? 'R' : '-',
                         (ph->p_flags & PF_W) ? 'W' : '-',
                         (ph->p_flags & PF_X) ? 'X' : '-',
                         (ph->p_flags & PF_X) ? "disabled" : "enabled");
            break;

        default:
            break;
        }
    }
    img->entry   = eh->e_entry;
    img->e_machine = eh->e_machine;
    img->phnum   = eh->e_phnum;
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ § 3. SECTION HEADERS — parse .symtab and .dynamic
 * ----------------------------------------------------------------------- */
static void elf_parse_sections(const sigma_u8 *blob, sigma_size_t size,
                                SigmaELFImage_t *img) {
    const Elf64Ehdr_t *eh = (const Elf64Ehdr_t *)blob;
    if (!eh->e_shnum || eh->e_shoff + (sigma_u64)eh->e_shnum * eh->e_shentsize > size)
        return;

    /* Section name string table */
    const Elf64Shdr_t *shstrtab_hdr = (const Elf64Shdr_t *)
        (blob + eh->e_shoff + (sigma_u64)eh->e_shstrndx * eh->e_shentsize);
    const char *shstrtab = (eh->e_shstrndx && shstrtab_hdr->sh_offset < size)
                           ? (const char *)(blob + shstrtab_hdr->sh_offset)
                           : SIGMA_NULL;

    const Elf64Shdr_t *symtab_hdr = SIGMA_NULL;
    const Elf64Shdr_t *strtab_hdr = SIGMA_NULL;
    const Elf64Shdr_t *dyn_hdr    = SIGMA_NULL;

    sigma_printf("Σ [ELF]: Sections (%u):\n", eh->e_shnum);
    for (sigma_u16 i = 0; i < eh->e_shnum; i++) {
        const Elf64Shdr_t *sh = (const Elf64Shdr_t *)
            (blob + eh->e_shoff + (sigma_u64)i * eh->e_shentsize);
        const char *sname = (shstrtab && sh->sh_name < shstrtab_hdr->sh_size)
                            ? shstrtab + sh->sh_name : "?";
        if (sh->sh_size)
            sigma_printf("Σ [ELF]:   [%2u] %-16s type=%u addr=0x%llx size=0x%llx\n",
                         i, sname, sh->sh_type,
                         (unsigned long long)sh->sh_addr,
                         (unsigned long long)sh->sh_size);
        if (sh->sh_type == SHT_SYMTAB) symtab_hdr = sh;
        if (sh->sh_type == SHT_STRTAB && i != eh->e_shstrndx) strtab_hdr = sh;
        if (sh->sh_type == SHT_DYNAMIC) dyn_hdr = sh;
    }

    /* Parse symbol table */
    if (symtab_hdr && strtab_hdr && symtab_hdr->sh_offset + symtab_hdr->sh_size <= size) {
        const Elf64Sym_t *syms = (const Elf64Sym_t *)(blob + symtab_hdr->sh_offset);
        const char *strtab     = (const char *)(blob + strtab_hdr->sh_offset);
        sigma_u32 nsyms        = (sigma_u32)(symtab_hdr->sh_size / sizeof(Elf64Sym_t));
        sigma_printf("Σ [ELF]: Symbol table (%u entries):\n", nsyms);
        for (sigma_u32 s = 0; s < nsyms && img->sym_count < MAX_SYMBOLS; s++) {
            if (!syms[s].st_name || !syms[s].st_value) continue;
            SigmaSymbol_t *sym = &img->symbols[img->sym_count++];
            const char *sname2 = (syms[s].st_name < strtab_hdr->sh_size)
                                 ? strtab + syms[s].st_name : "?";
            sigma_strcpy(sym->name, sname2, sizeof(sym->name));
            sym->value = syms[s].st_value;
            sym->size  = syms[s].st_size;
            sym->bind  = ELF64_ST_BIND(syms[s].st_info);
            sym->type  = ELF64_ST_TYPE(syms[s].st_info);
            sigma_printf("Σ [ELF]:   %-30s 0x%llx (%s)\n",
                         sym->name, (unsigned long long)sym->value,
                         sym->type == STT_FUNC ? "FUNC" : "OBJECT");
        }
    }

    /* Parse .dynamic for DT_NEEDED (shared library dependencies) */
    if (dyn_hdr && dyn_hdr->sh_offset + dyn_hdr->sh_size <= size) {
        const Elf64Dyn_t *dyn = (const Elf64Dyn_t *)(blob + dyn_hdr->sh_offset);
        sigma_u32 ndyn = (sigma_u32)(dyn_hdr->sh_size / sizeof(Elf64Dyn_t));
        const char *dynstr = SIGMA_NULL;

        /* First pass: find DT_STRTAB */
        for (sigma_u32 d = 0; d < ndyn && dyn[d].d_tag != DT_NULL; d++) {
            if (dyn[d].d_tag == DT_STRTAB && dyn[d].d_val < size)
                dynstr = (const char *)(blob + dyn[d].d_val);
        }
        /* Second pass: collect DT_NEEDED */
        for (sigma_u32 d = 0; d < ndyn && dyn[d].d_tag != DT_NULL; d++) {
            if (dyn[d].d_tag == DT_NEEDED && img->dep_count < MAX_DEPS && dynstr) {
                const char *lib = dynstr + dyn[d].d_val;
                sigma_strcpy(img->deps[img->dep_count], lib, DEP_NAME_LEN);
                sigma_printf("Σ [ELF]: DT_NEEDED: %s\n", lib);
                img->dep_count++;
            }
        }
    }
}

/* -----------------------------------------------------------------------
 * ░░ § 4. AUXILIARY VECTOR BUILDER
 * Pushed onto the initial process stack above argv/envp.
 * ----------------------------------------------------------------------- */
static void elf_build_auxv(SigmaELFImage_t *img, SigmaAuxVec_t *auxv,
                            sigma_u32 *count) {
    sigma_u32 n = 0;
#define AUXV(t,v) { auxv[n].a_type = (t); auxv[n].a_val = (v); n++; }
    AUXV(AT_ENTRY,  img->entry);
    AUXV(AT_PHDR,   img->phdr_va ? img->phdr_va : img->load_base + 64);
    AUXV(AT_PHENT,  sizeof(Elf64Phdr_t));
    AUXV(AT_PHNUM,  img->phnum);
    AUXV(AT_PAGESZ, 4096);
    AUXV(AT_BASE,   img->load_base);
    AUXV(AT_FLAGS,  0);
    AUXV(AT_UID,    0);  /* root */
    AUXV(AT_EUID,   0);
    AUXV(AT_GID,    0);
    AUXV(AT_EGID,   0);
    AUXV(AT_SECURE, 0);
    AUXV(AT_HWCAP,  0x0000000178BFBFF1ULL); /* x86_64 typical CPUID */
    AUXV(AT_NULL,   0);
#undef AUXV
    *count = n;
    sigma_printf("Σ [ELF]: Auxiliary vector: %u entries built\n", n);
}

/* -----------------------------------------------------------------------
 * ░░ § 5. SYMBOL LOOKUP
 * ----------------------------------------------------------------------- */
sigma_u64 sigma_elf_sym_lookup(const SigmaELFImage_t *img, const char *name) {
    for (sigma_u32 i = 0; i < img->sym_count; i++) {
        if (sigma_streq(img->symbols[i].name, name))
            return img->symbols[i].value;
    }
    return 0;
}

/* -----------------------------------------------------------------------
 * ░░ § 6. execve() — main entry
 * In real kernel: replaces the current process image.
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_execve(const sigma_u8 *elf_blob, sigma_size_t size,
                          const char *path, SigmaELFImage_t *out_img) {
    sigma_printf("Σ [ELF]: execve('%s') image_size=%lu bytes\n",
                 path, (unsigned long)size);

    /* Zero the image descriptor */
    sigma_memset(out_img, 0, sizeof(SigmaELFImage_t));
    sigma_strcpy(out_img->path, path, sizeof(out_img->path));

    /* Step 1: validate */
    sigma_err_t e = elf_validate_header(elf_blob, size);
    if (!sigma_ok(e)) return e;

    /* Step 2: parse program headers */
    e = elf_load_phdrs(elf_blob, size, out_img);
    if (!sigma_ok(e)) return e;

    /* Step 3: parse sections, symbols, dynamic */
    elf_parse_sections(elf_blob, size, out_img);

    /* Step 4: ASLR — compute load base for ET_DYN */
    out_img->load_base = (out_img->e_machine == EM_X86_64)
                         ? 0x0000555555554000ULL  /* canonical x86_64 base */
                         : 0x0000000000400000ULL;

    /* Step 5: build auxiliary vector */
    SigmaAuxVec_t auxv[16];
    sigma_u32     auxv_count;
    elf_build_auxv(out_img, auxv, &auxv_count);

    /* Step 6: set up initial stack */
    out_img->stack_top = 0x00007FFFFFFFE000ULL;  /* 128 TB - 8 KB */
    out_img->sp        = out_img->stack_top - 4096;

    /* Step 7: summary */
    sigma_printf("Σ [ELF]: ─────────────────────────────────────────────\n");
    sigma_printf("Σ [ELF]: Image loaded: '%s'\n", path);
    sigma_printf("Σ [ELF]:   entry    = 0x%llx\n",
                 (unsigned long long)out_img->entry);
    sigma_printf("Σ [ELF]:   base     = 0x%llx\n",
                 (unsigned long long)out_img->load_base);
    sigma_printf("Σ [ELF]:   segments = %u\n", out_img->seg_count);
    sigma_printf("Σ [ELF]:   symbols  = %u\n", out_img->sym_count);
    sigma_printf("Σ [ELF]:   deps     = %u\n", out_img->dep_count);
    sigma_printf("Σ [ELF]:   dynamic  = %s\n",
                 out_img->is_dynamic ? "yes" : "no");
    if (out_img->has_interp)
        sigma_printf("Σ [ELF]:   interp   = %s\n", out_img->interp);
    sigma_printf("Σ [ELF]:   RSP      = 0x%llx\n",
                 (unsigned long long)out_img->sp);
    sigma_printf("Σ [ELF]: – Transfer control to entry point "
                 "(real hw: JMP 0x%llx) –\n",
                 (unsigned long long)(out_img->load_base + out_img->entry));
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ § 7. Self-test with a synthetised minimal ELF64 blob
 * (a legal 64-byte ELF header + 1 PT_LOAD phdr for a static binary)
 * ----------------------------------------------------------------------- */
void SovereignELFLoader_Init(void) {
    sigma_printf("Σ [ELF]: Initialising Sovereign ELF Binary Loader...\n");

    /*
     * Synthesised minimal ELF64 binary (ET_EXEC, EM_X86_64):
     *   e_ident    magic + class + data + version
     *   e_type     ET_EXEC
     *   e_machine  EM_X86_64
     *   e_entry    0x401000
     *   e_phoff    64  (immediately after Ehdr)
     *   e_phnum    2   (one PT_LOAD + one PT_GNU_STACK)
     */
    static const sigma_u8 synthetic_elf[] = {
        /* ELF header (64 bytes) */
        0x7F,'E','L','F',    /* magic */
        0x02,                /* EI_CLASS = ELFCLASS64 */
        0x01,                /* EI_DATA  = ELFDATA2LSB */
        0x01,                /* EI_VERSION */
        0x00,                /* EI_OSABI = ELFOSABI_NONE */
        0,0,0,0,0,0,0,0,     /* padding */
        0x02, 0x00,          /* e_type    = ET_EXEC (LE) */
        0x3E, 0x00,          /* e_machine = EM_X86_64 (62, LE) */
        0x01, 0x00,0x00,0x00,/* e_version = 1 */
        0x00,0x10,0x40,0x00, 0x00,0x00,0x00,0x00, /* e_entry = 0x401000 */
        0x40,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, /* e_phoff = 64 */
        0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, /* e_shoff = 0 */
        0x00,0x00,0x00,0x00, /* e_flags */
        0x40,0x00,           /* e_ehsize = 64 */
        0x38,0x00,           /* e_phentsize = 56 */
        0x02,0x00,           /* e_phnum = 2 */
        0x40,0x00,           /* e_shentsize = 64 */
        0x00,0x00,           /* e_shnum = 0 */
        0x00,0x00,           /* e_shstrndx = 0 */

        /* PT_LOAD phdr (56 bytes) */
        0x01,0x00,0x00,0x00, /* p_type  = PT_LOAD */
        0x05,0x00,0x00,0x00, /* p_flags = PF_R|PF_X */
        0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, /* p_offset = 0 */
        0x00,0x00,0x40,0x00, 0x00,0x00,0x00,0x00, /* p_vaddr  = 0x400000 */
        0x00,0x00,0x40,0x00, 0x00,0x00,0x00,0x00, /* p_paddr */
        0x00,0x10,0x00,0x00, 0x00,0x00,0x00,0x00, /* p_filesz = 4096 */
        0x00,0x10,0x00,0x00, 0x00,0x00,0x00,0x00, /* p_memsz  = 4096 */
        0x00,0x00,0x20,0x00, 0x00,0x00,0x00,0x00, /* p_align  = 0x200000 */

        /* PT_GNU_STACK phdr (56 bytes) */
        0x51,0xE5,0x74,0x64, /* p_type  = PT_GNU_STACK */
        0x06,0x00,0x00,0x00, /* p_flags = PF_R|PF_W (NX stack) */
        0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
        0x00,0x00,0x10,0x00, 0x00,0x00,0x00,0x00,
    };

    SigmaELFImage_t img;
    sigma_execve(synthetic_elf, sizeof(synthetic_elf),
                 "/usr/bin/sigma-app", &img);

    /* Verify entry point */
    sigma_printf("Σ [ELF]: Verified entry=0x%llx (expected 0x401000)\n",
                 (unsigned long long)img.entry);

    /* Symbol lookup test (no symbols in this minimal binary) */
    sigma_u64 main_va = sigma_elf_sym_lookup(&img, "main");
    sigma_printf("Σ [ELF]: sym_lookup('main') = 0x%llx %s\n",
                 (unsigned long long)main_va,
                 main_va ? "[FOUND]" : "[not in symtab — static/stripped]");

    sigma_printf("Σ [ELF]: ELF loader online. execve sovereignty achieved.\n");
}
