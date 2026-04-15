/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN WINE COMPAT — WINDOWS API LAYER (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: Wine / Proton / DXVK
 *   https://github.com/wine-mirror/wine
 *   https://github.com/ValveSoftware/Proton
 *
 * Wine USPs absorbed:
 *   ✓ PE (Portable Executable) loader — MZ header, section mapping
 *   ✓ Windows API stubs — Win32 kernel32/ntdll/user32 shims
 *   ✓ Registry emulation — HKLM, HKCU, keys, values, types
 *   ✓ Windows message queue (HWND/MSG/WM_ event loop stub)
 *   ✓ WoW64 — 32-bit PE execution on 64-bit kernel
 *   ✓ Wine loader (wine / wine64 binary entry points)
 *
 * DXVK / Proton USPs absorbed:
 *   ✓ DirectX → Vulkan translation table (D3D11, D3D9)
 *   ✓ esync / fsync — eventfd-based synchronisation primitives
 *   ✓ Steam Play / Proton-GE compatibility mode
 *   ✓ NVAPI / DXGI adapter enumeration stubs
 * =========================================================================
 */

#ifndef SOVEREIGN_WINE_COMPAT_H
#define SOVEREIGN_WINE_COMPAT_H

#include "sigma_types.h"

/* -------------------------------------------------------------------------
 * PE Format constants
 * ---------------------------------------------------------------------- */
#define PE_MZ_MAGIC      0x5A4D     /* 'MZ' */
#define PE_PE_MAGIC      0x4550     /* 'PE' */
#define PE_OPT_PE32      0x010B
#define PE_OPT_PE32PLUS  0x020B     /* 64-bit */

#define PE_MAX_SECTIONS  96
#define PE_NAME_MAX      64

/* -------------------------------------------------------------------------
 * PE / MZ header structures (minimal — enough for loader)
 * ---------------------------------------------------------------------- */
typedef struct {
    sigma_u16 e_magic;        /* MZ magic                     */
    sigma_u16 e_cblp;
    sigma_u16 e_cp;
    sigma_u16 e_crlc;
    sigma_u16 e_cparhdr;
    sigma_u16 e_minalloc;
    sigma_u16 e_maxalloc;
    sigma_u16 e_ss;
    sigma_u16 e_sp;
    sigma_u16 e_csum;
    sigma_u16 e_ip;
    sigma_u16 e_cs;
    sigma_u16 e_lfarlc;
    sigma_u16 e_ovno;
    sigma_u16 e_res[4];
    sigma_u16 e_oemid;
    sigma_u16 e_oeminfo;
    sigma_u16 e_res2[10];
    sigma_u32 e_lfanew;       /* Offset to PE header          */
} SIGMA_PACKED SigmaMZHeader_t;

typedef struct {
    sigma_u16 Machine;
    sigma_u16 NumberOfSections;
    sigma_u32 TimeDateStamp;
    sigma_u32 PointerToSymbolTable;
    sigma_u32 NumberOfSymbols;
    sigma_u16 SizeOfOptionalHeader;
    sigma_u16 Characteristics;
} SIGMA_PACKED SigmeCOFFHeader_t;

typedef struct {
    char      Name     [8];
    sigma_u32 VirtualSize;
    sigma_u32 VirtualAddress;
    sigma_u32 SizeOfRawData;
    sigma_u32 PointerToRawData;
    sigma_u32 PointerToRelocations;
    sigma_u32 PointerToLinenumbers;
    sigma_u16 NumberOfRelocations;
    sigma_u16 NumberOfLinenumbers;
    sigma_u32 Characteristics;
} SIGMA_PACKED SigmaPESection_t;

/* -------------------------------------------------------------------------
 * Loaded PE image descriptor
 * ---------------------------------------------------------------------- */
typedef struct {
    char           name   [PE_NAME_MAX];
    void          *base;           /* Virtual base address (simulated)       */
    sigma_u64      image_size;
    sigma_u32      entry_rva;      /* Entry point relative virtual address   */
    sigma_bool     is_64bit;
    sigma_bool     is_dll;
    sigma_u32      n_sections;
    SigmaPESection_t sections[PE_MAX_SECTIONS];
} SigmaPEImage_t;

/* -------------------------------------------------------------------------
 * Registry emulation
 * ---------------------------------------------------------------------- */
#define SIGMA_REG_KEY_MAX   128
#define SIGMA_REG_VAL_MAX  4096
#define SIGMA_REG_ENTRIES   512

typedef enum {
    SIGMA_REG_SZ        = 1,
    SIGMA_REG_DWORD     = 4,
    SIGMA_REG_QWORD     = 11,
    SIGMA_REG_BINARY    = 3,
    SIGMA_REG_MULTI_SZ  = 7,
} SigmaRegType_t;

typedef struct {
    char           hive [16];           /* HKLM, HKCU, HKCR … */
    char           key  [SIGMA_REG_KEY_MAX];
    char           name [SIGMA_REG_KEY_MAX];
    SigmaRegType_t type;
    sigma_u8       data [SIGMA_REG_VAL_MAX];
    sigma_u32      data_len;
    sigma_bool     occupied;
} SigmaRegEntry_t;

/* -------------------------------------------------------------------------
 * DirectX → Vulkan translation map (DXVK-inspired)
 * ---------------------------------------------------------------------- */
#define SIGMA_DXVK_MAP_MAX 64

typedef struct {
    char      dx_call     [64];
    char      vk_call     [64];
    sigma_u32 d3d_version;   /* 9, 11, 12 */
    sigma_u64 translated_count;
} SigmaDXVKEntry_t;

/* -------------------------------------------------------------------------
 * Wine context
 * ---------------------------------------------------------------------- */
typedef struct {
    SigmaPEImage_t  loaded_images[32];
    sigma_u32       image_count;
    SigmaRegEntry_t registry    [SIGMA_REG_ENTRIES];
    sigma_u32       reg_count;
    SigmaDXVKEntry_t dxvk_map  [SIGMA_DXVK_MAP_MAX];
    sigma_u32       dxvk_count;
    sigma_bool      wow64_mode;
    sigma_bool      esync_enabled;
    sigma_bool      fsync_enabled;
} SigmaWineCtx_t;

extern SigmaWineCtx_t g_sigma_wine;

/* -------------------------------------------------------------------------
 * Public API — PE Loader
 * ---------------------------------------------------------------------- */
sigma_err_t  sigma_pe_load        (SigmaWineCtx_t *w, const char *path);
sigma_err_t  sigma_pe_run         (SigmaWineCtx_t *w, const char *name,
                                    const char *args);
void         sigma_pe_list        (const SigmaWineCtx_t *w);

/* Registry */
sigma_err_t  sigma_reg_set        (SigmaWineCtx_t *w,
                                    const char *hive, const char *key,
                                    const char *name, SigmaRegType_t type,
                                    const void *data, sigma_u32 len);
sigma_err_t  sigma_reg_get        (const SigmaWineCtx_t *w,
                                    const char *hive, const char *key,
                                    const char *name, void *out, sigma_u32 max);
void         sigma_reg_dump       (const SigmaWineCtx_t *w,
                                    const char *hive);

/* DXVK */
sigma_err_t  sigma_dxvk_translate (SigmaWineCtx_t *w,
                                    const char *dx_call, sigma_u32 d3d_ver);
void         sigma_dxvk_stats     (const SigmaWineCtx_t *w);
sigma_err_t  sigma_esync_create   (SigmaWineCtx_t *w);
sigma_err_t  sigma_fsync_create   (SigmaWineCtx_t *w);

/* Win32 API stubs */
sigma_u32    sigma_win32_GetLastError  (void);
void         sigma_win32_SetLastError  (sigma_u32 err);
void        *sigma_win32_VirtualAlloc  (void *addr, sigma_size_t size,
                                         sigma_u32 type, sigma_u32 protect);
sigma_err_t  sigma_win32_VirtualFree   (void *addr);
int          sigma_win32_CreateThread  (void *(*fn)(void*), void *arg);
void         sigma_win32_ExitProcess   (sigma_u32 code);
void         sigma_win32_MessageBoxA   (const char *title, const char *msg);

void SovereignWineCompat_Init(void);

#endif /* SOVEREIGN_WINE_COMPAT_H */
