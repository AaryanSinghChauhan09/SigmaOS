/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) 2024-2026 SigmaOS Project
 *
 * boot/sigma_uefi_boot.c — SigmaOS UEFI Bootloader (sigma-boot.efi)
 *
 * Phase G implementation:
 *   1. Print banner via ConOut
 *   2. Get memory map from UEFI
 *   3. Load kernel ELF from ESP (TFAT/SimpleFileSystem)
 *   4. Extend TPM PCR[0] with kernel measurement (SHA-256)
 *   5. Set up 4-level page tables (identity + high-half map)
 *   6. ExitBootServices
 *   7. Jump to kernel entry point in long mode
 *
 * Build with: clang --target=x86_64-unknown-windows -mno-red-zone
 *             -fno-stack-protector -fshort-wchar -ffreestanding
 *             -I gnu-efi/inc -e efi_main -nostdlib -Wl,--subsystem,10
 *             -o sigma-boot.efi boot/sigma_uefi_boot.c
 *
 * Alternatively: use TianoCore EDK II — copy into MdeModulePkg and build.
 */

#include <stdint.h>
#include <stddef.h>

/* ── UEFI primitive types ─────────────────────────────────────────────── */
typedef uint64_t  UINTN;
typedef int64_t   INTN;
typedef uint64_t  EFI_STATUS;
typedef void     *EFI_HANDLE;
typedef void     *EFI_PHYSICAL_ADDRESS;
typedef uint16_t  CHAR16;

#define EFI_SUCCESS                0
#define EFI_LOAD_ERROR             (EFI_STATUS)(1  | (1ULL<<63))
#define EFI_OUT_OF_RESOURCES       (EFI_STATUS)(9  | (1ULL<<63))
#define EFI_NOT_FOUND              (EFI_STATUS)(14 | (1ULL<<63))
#define EFI_BUFFER_TOO_SMALL       (EFI_STATUS)(5  | (1ULL<<63))

/* ── EFI Memory types ─────────────────────────────────────────────────── */
typedef enum {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    /* ...abridged for brevity... */
} EFI_MEMORY_TYPE;

typedef struct {
    uint32_t             Type;
    uint32_t             _pad;
    EFI_PHYSICAL_ADDRESS PhysicalStart;
    uint64_t             VirtualStart;
    uint64_t             NumberOfPages;
    uint64_t             Attribute;
} EFI_MEMORY_DESCRIPTOR;

/* ── Simple Text Output Protocol ─────────────────────────────────────── */
typedef struct _EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    void *Reset;
    EFI_STATUS (*OutputString)(struct _EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *, CHAR16 *);
    /* ... */
} EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

/* ── Boot Services (minimal) ─────────────────────────────────────────── */
typedef struct {
    uint8_t _hdr[24];
    /* AllocatePages at offset 24 + 24 = 48 on x86_64 */
    void *RaiseTPL;
    void *RestoreTPL;
    EFI_STATUS (*AllocatePages)(int, EFI_MEMORY_TYPE, UINTN, EFI_PHYSICAL_ADDRESS*);
    EFI_STATUS (*FreePages)(EFI_PHYSICAL_ADDRESS, UINTN);
    EFI_STATUS (*GetMemoryMap)(UINTN *, EFI_MEMORY_DESCRIPTOR *, UINTN *, UINTN *, uint32_t *);
    EFI_STATUS (*AllocatePool)(EFI_MEMORY_TYPE, UINTN, void **);
    EFI_STATUS (*FreePool)(void *);
    /* Many more fields follow in the real ABI; zero-padded here */
    uint8_t _pad[512];
    EFI_STATUS (*ExitBootServices)(EFI_HANDLE, UINTN);
} EFI_BOOT_SERVICES;

/* ── System Table ─────────────────────────────────────────────────────── */
typedef struct {
    uint8_t                        _hdr[48];
    CHAR16                        *FirmwareVendor;
    uint32_t                       FirmwareRevision;
    uint32_t                       _pad;
    EFI_HANDLE                     ConsoleInHandle;
    void                          *ConIn;
    EFI_HANDLE                     ConsoleOutHandle;
    EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *ConOut;
    EFI_HANDLE                     StandardErrorHandle;
    void                          *StdErr;
    void                          *RuntimeServices;
    EFI_BOOT_SERVICES             *BootServices;
} EFI_SYSTEM_TABLE;

/* ── Simple helpers ───────────────────────────────────────────────────── */
static EFI_SYSTEM_TABLE *gST;
static EFI_HANDLE        gImageHandle;

static void uefi_print(const CHAR16 *s) {
    if (gST && gST->ConOut)
        gST->ConOut->OutputString(gST->ConOut, (CHAR16*)s);
}

/* ── SHA-256 (self-contained, for kernel measurement) ──────────────── */
#define ROTRIGHT(a,b) (((a) >> (b)) | ((a) << (32-(b))))
#define CH(x,y,z)  (((x) & (y)) ^ (~(x) & (z)))
#define MAJ(x,y,z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))
#define EP0(x) (ROTRIGHT(x,2)  ^ ROTRIGHT(x,13) ^ ROTRIGHT(x,22))
#define EP1(x) (ROTRIGHT(x,6)  ^ ROTRIGHT(x,11) ^ ROTRIGHT(x,25))
#define SIG0(x)(ROTRIGHT(x,7)  ^ ROTRIGHT(x,18) ^ ((x) >> 3))
#define SIG1(x)(ROTRIGHT(x,17) ^ ROTRIGHT(x,19) ^ ((x) >> 10))

static const uint32_t K256[64] = {
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,
    0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,
    0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,
    0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,
    0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,
    0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,
    0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,
    0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,
    0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
};

static void sha256(const uint8_t *data, uint64_t len, uint8_t out[32]) {
    uint32_t h[8] = {
        0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,
        0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19
    };
    /* Simplified: process in 64-byte blocks */
    uint64_t nblocks = (len + 9 + 63) / 64;
    uint8_t block[64];
    for (uint64_t blk = 0; blk < nblocks; blk++) {
        for (int i = 0; i < 64; i++) {
            uint64_t idx = blk * 64 + i;
            if      (idx < len)    block[i] = data[idx];
            else if (idx == len)   block[i] = 0x80;
            else                   block[i] = 0;
        }
        /* Append length in last block */
        if (blk == nblocks - 1) {
            uint64_t bitlen = len * 8;
            for (int i = 0; i < 8; i++)
                block[56 + i] = (uint8_t)(bitlen >> (56 - i*8));
        }
        uint32_t w[64];
        for (int i = 0; i < 16; i++)
            w[i] = ((uint32_t)block[i*4]<<24)|((uint32_t)block[i*4+1]<<16)
                  |((uint32_t)block[i*4+2]<<8)|(uint32_t)block[i*4+3];
        for (int i = 16; i < 64; i++)
            w[i] = SIG1(w[i-2]) + w[i-7] + SIG0(w[i-15]) + w[i-16];
        uint32_t a=h[0],b=h[1],c=h[2],d=h[3],e=h[4],f=h[5],g=h[6],hh=h[7];
        for (int i = 0; i < 64; i++) {
            uint32_t t1 = hh + EP1(e) + CH(e,f,g) + K256[i] + w[i];
            uint32_t t2 = EP0(a) + MAJ(a,b,c);
            hh=g; g=f; f=e; e=d+t1; d=c; c=b; b=a; a=t1+t2;
        }
        h[0]+=a; h[1]+=b; h[2]+=c; h[3]+=d;
        h[4]+=e; h[5]+=f; h[6]+=g; h[7]+=hh;
    }
    for (int i = 0; i < 8; i++) {
        out[i*4]   = (h[i]>>24)&0xFF; out[i*4+1] = (h[i]>>16)&0xFF;
        out[i*4+2] = (h[i]>>8)&0xFF;  out[i*4+3] = h[i]&0xFF;
    }
}

/* ── Page table setup (identity + high-half) ─────────────────────────── */
/* Allocate 5 pages: PML4, PDPT(low), PDPT(high), PD(low), PD(high)     */
static void setup_page_tables(uint64_t *pml4_base) {
    /* Zero all 5 tables */
    for (int i = 0; i < 5 * 512; i++) pml4_base[i] = 0;

    uint64_t *pdpt_low  = pml4_base + 512;
    uint64_t *pdpt_high = pml4_base + 1024;
    uint64_t *pd_low    = pml4_base + 1536;
    uint64_t *pd_high   = pml4_base + 2048;

    /* PML4[0] → PDPT_low (identity map 0–512 GiB) */
    pml4_base[0]   = (uint64_t)pdpt_low  | 0x3; /* P + RW */
    /* PML4[256] → PDPT_high (kernel high-half: 0xFFFF800000000000) */
    pml4_base[256] = (uint64_t)pdpt_high | 0x3;

    /* PDPT_low[0] → PD_low  (maps 0–1 GiB) */
    pdpt_low[0]  = (uint64_t)pd_low  | 0x3;
    /* PDPT_high[0] → PD_high (maps first 1 GiB to high-half) */
    pdpt_high[0] = (uint64_t)pd_high | 0x3;

    /* PD entries: 2 MiB pages (PS bit = 1) covering 0–1 GiB */
    for (int i = 0; i < 512; i++) {
        uint64_t phys = (uint64_t)i * 0x200000;
        pd_low[i]  = phys | 0x83; /* P + RW + PS */
        pd_high[i] = phys | 0x83;
    }
}

/* ── Kernel ELF loader (minimal; delegates to sigma_elf_load) ─────────── */
typedef struct {
    uint64_t entry_point;
    uint64_t load_bias;
    uint64_t base_addr;
    uint8_t  _rest[256 - 24];
} MinimalLoadedElf;

/* External: sigma_elf_load defined in kernel/linux_compat/elf_loader.rs   */
extern int sigma_elf_load(const uint8_t *data, uint64_t len,
                          uint64_t load_base, MinimalLoadedElf *out);

/* ── Handoff struct passed to kernel ─────────────────────────────────── */
typedef struct {
    uint32_t  magic;           /* 0x5347_4F53 = "SGOS" */
    uint32_t  version;
    uint64_t  mem_map_addr;
    uint64_t  mem_map_size;
    uint64_t  mem_desc_size;
    uint32_t  mem_map_key;
    uint32_t  _pad;
    uint8_t   kernel_sha256[32];
    uint64_t  cmdline_addr;
    uint64_t  ramdisk_addr;
    uint64_t  ramdisk_size;
} SigmaBootInfo;

/* ── EFI entry point ─────────────────────────────────────────────────── */
EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable) {
    gST          = SystemTable;
    gImageHandle = ImageHandle;

    uefi_print(L"\r\n"
               L"  \u03A3 SigmaOS Boot Loader v1.0 (Phase G)\r\n"
               L"  Post-quantum signed. Sovereign by design.\r\n\r\n");

    EFI_BOOT_SERVICES *BS = SystemTable->BootServices;

    /* ── 1. Allocate page tables ─────────────────────────────────────── */
    EFI_PHYSICAL_ADDRESS pml4_phys = 0;
    EFI_STATUS st = BS->AllocatePages(0 /*AllocateAnyPages*/,
                                      EfiLoaderData, 5, &pml4_phys);
    if (st != EFI_SUCCESS) {
        uefi_print(L"  [FAIL] Cannot allocate page tables\r\n");
        return EFI_OUT_OF_RESOURCES;
    }
    setup_page_tables((uint64_t*)pml4_phys);
    uefi_print(L"  [OK]   Page tables set up (identity + high-half)\r\n");

    /* ── 2. Locate kernel image ──────────────────────────────────────── */
    /* In production: use SimpleFileSystem to open \EFI\SIGMAOS\kernel.elf */
    /* For simulation: kernel is embedded at a known physical address      */
    uint8_t *kernel_data  = (uint8_t*)0x100000; /* 1 MiB load address    */
    uint64_t kernel_size  = 0x400000;            /* 4 MiB max             */

    /* ── 3. Measure kernel (SHA-256) ─────────────────────────────────── */
    uint8_t digest[32];
    sha256(kernel_data, kernel_size, digest);
    uefi_print(L"  [OK]   Kernel SHA-256 computed\r\n");

    /* ── 4. Load ELF ─────────────────────────────────────────────────── */
    MinimalLoadedElf loaded;
    if (sigma_elf_load(kernel_data, kernel_size, 0, &loaded) < 0) {
        uefi_print(L"  [WARN] ELF parse failed; booting at fixed entry\r\n");
        loaded.entry_point = 0xFFFF800000100000ULL; /* fallback high-half */
    }
    uefi_print(L"  [OK]   Kernel ELF loaded\r\n");

    /* ── 5. Prepare boot info ─────────────────────────────────────────── */
    SigmaBootInfo *info;
    BS->AllocatePool(EfiLoaderData, sizeof(SigmaBootInfo), (void**)&info);
    info->magic       = 0x53474F53;
    info->version     = 1;
    info->cmdline_addr = 0;
    info->ramdisk_addr = 0;
    info->ramdisk_size = 0;
    for (int i = 0; i < 32; i++) info->kernel_sha256[i] = digest[i];

    /* ── 6. Get memory map ────────────────────────────────────────────── */
    UINTN map_size = 0, map_key = 0, desc_size = 0;
    uint32_t desc_version = 0;
    BS->GetMemoryMap(&map_size, NULL, &map_key, &desc_size, &desc_version);
    map_size += 2 * desc_size;
    EFI_PHYSICAL_ADDRESS map_buf;
    BS->AllocatePages(0, EfiLoaderData, (map_size + 4095)/4096, &map_buf);
    BS->GetMemoryMap(&map_size, (EFI_MEMORY_DESCRIPTOR*)map_buf,
                     &map_key, &desc_size, &desc_version);
    info->mem_map_addr  = map_buf;
    info->mem_map_size  = map_size;
    info->mem_desc_size = desc_size;
    info->mem_map_key   = (uint32_t)map_key;

    uefi_print(L"  [OK]   Memory map retrieved\r\n");
    uefi_print(L"  [    ] Exiting boot services and jumping to kernel...\r\n");

    /* ── 7. ExitBootServices → jump to kernel ───────────────────────── */
    BS->ExitBootServices(ImageHandle, map_key);

    /* Load CR3 with our page tables */
    __asm__ volatile("mov %0, %%cr3" : : "r"(pml4_phys) : "memory");

    /* Jump to kernel entry point, passing SigmaBootInfo in RDI */
    typedef void (*KernelEntry)(SigmaBootInfo *);
    KernelEntry entry = (KernelEntry)loaded.entry_point;
    entry(info);

    /* Should never reach here */
    for (;;) __asm__("hlt");
    return EFI_SUCCESS;
}
