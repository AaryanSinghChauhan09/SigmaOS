// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_kpatch.h — Live kernel function patching (kpatch-inspired)
 *
 * Allows security fixes to be applied to a running kernel without reboot.
 * Uses ftrace to redirect calls from the original function to a patched version.
 *
 * Workflow:
 *   1. sigma-kpatch-build generates a .kpatch module from a diff
 *   2. sigma_kpatch_load() installs the module
 *   3. ftrace redirects old function → new function
 *   4. Original function is still in memory (easy rollback)
 *   5. sigma_kpatch_unload() restores original function
 *
 * Safety constraints (matches kpatch safety model):
 *   - Patch can only redirect a function that is NOT currently on any call stack
 *   - Atomic: either ALL functions in a .kpatch are redirected or NONE are
 *   - Patched modules are cryptographically signed (sigma_module_sign)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_KPATCH_MAGIC    0x53474B50  /* "SGKP" */
#define SIGMA_KPATCH_MAX_FUNCS 64         /* max functions per patch module */

typedef struct {
    sigma_u64 old_addr;      /* address of original function in kernel      */
    sigma_u64 new_addr;      /* address of replacement function             */
    char      name[128];     /* function name (for audit log)               */
    bool      applied;       /* true after ftrace redirect is installed     */
} sigma_kpatch_func_t;

typedef struct {
    sigma_u32          magic;          /* SIGMA_KPATCH_MAGIC                */
    sigma_u32          version;        /* patch format version              */
    char               patch_id[64];  /* e.g. "CVE-2026-1234-fix"          */
    char               description[256];
    sigma_kpatch_func_t funcs[SIGMA_KPATCH_MAX_FUNCS];
    int                func_count;
    sigma_u8           signature[4595]; /* Dilithium3 signature             */
    bool               loaded;
} sigma_kpatch_module_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/*
 * Load a kpatch module from a .kpatch file.
 * Verifies the Dilithium3 signature before applying.
 * Atomic: all-or-nothing application.
 */
int sigma_kpatch_load(const char* kpatch_path);

/*
 * Unload a patch — restores original function pointers.
 * Safe to call even while the patched functions are being executed
 * (waits for all CPUs to exit the patched functions first).
 */
int sigma_kpatch_unload(const char* patch_id);

/* List all currently loaded patches to the audit log */
void sigma_kpatch_list(void);

/* Check if a specific function has been patched */
bool sigma_kpatch_is_patched(const char* func_name);

/*
 * sigma-kpatch-build: generates a .kpatch module from a unified diff.
 * Called offline (not at runtime) — output is signed and deployed.
 * See: scripts/sigma-kpatch-build.sh
 */
