// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_universal_loader.h — Universal Binary Loader
 *
 * Detects the format of an executable and dispatches to the appropriate
 * loader/compatibility layer. This is how SigmaOS runs Linux ELF binaries,
 * Flatpak bundles, and OCI images without containers.
 *
 * Supported formats (in detection priority order):
 *   SIGMA_BINFMT_NATIVE_ELF  — SigmaOS native ELF64 (no shim needed)
 *   SIGMA_BINFMT_LINUX_ELF   — Standard Linux ELF64 (POSIX shim via SovereignCompat)
 *   SIGMA_BINFMT_OCI_BUNDLE  — OCI Runtime bundle (config.json + rootfs/)
 *   SIGMA_BINFMT_FLATPAK     — Flatpak .flatpak bundle
 *   SIGMA_BINFMT_WASM        — WebAssembly module (via sigma WASM runtime)
 *   SIGMA_BINFMT_SCRIPT      — #! shebang script
 */
#include <sigma_kernel_types.h>

typedef enum {
    SIGMA_BINFMT_UNKNOWN     = 0,
    SIGMA_BINFMT_NATIVE_ELF  = 1,  /* SigmaOS native — run directly           */
    SIGMA_BINFMT_LINUX_ELF   = 2,  /* Linux ELF — route through POSIX shim    */
    SIGMA_BINFMT_OCI_BUNDLE  = 3,  /* OCI bundle — parse config.json          */
    SIGMA_BINFMT_FLATPAK     = 4,  /* .flatpak bundle                         */
    SIGMA_BINFMT_WASM        = 5,  /* WebAssembly — sigma WASM runtime        */
    SIGMA_BINFMT_SCRIPT      = 6,  /* #! shebang                              */
} sigma_binfmt_t;

/* Loader descriptor — registered by each compatibility layer */
typedef struct sigma_loader {
    sigma_binfmt_t format;
    const char*    name;          /* e.g. "linux-elf", "oci-bundle"           */

    /* Probe: return true if this loader handles the given file */
    bool (*probe)(const sigma_u8* header, sigma_size_t hdr_len);

    /* Load: set up address space, apply pledge/unveil, exec */
    int (*load)(const char* path, const char** argv, const char** envp);

    struct sigma_loader* next;
} sigma_loader_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Register a binary format loader (called by each compat module at init) */
void sigma_binfmt_register(sigma_loader_t* loader);

/* Detect binary format from the first 16 bytes of the file */
sigma_binfmt_t sigma_binfmt_detect(const char* path);

/*
 * Universal exec — detects format, selects loader, runs binary.
 * Called by sys_execve. Replaces per-format special-casing.
 */
int sigma_exec_universal(const char* path, const char** argv, const char** envp);

void sigma_universal_loader_init(void);

/* ── Magic bytes used for detection ──────────────────────────────────────── */
#define ELF_MAGIC           "\x7fELF"
#define WASM_MAGIC          "\x00asm"
#define OCI_BUNDLE_MARKER   "config.json"   /* directory with this file       */
#define FLATPAK_MAGIC       "var/lib/flatpak"
