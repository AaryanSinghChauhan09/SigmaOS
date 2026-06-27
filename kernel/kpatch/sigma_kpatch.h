// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_kpatch.h — Live kernel patching without reboot
 *
 * Inspired by kpatch (Red Hat) and Linux Kernel Live Patching (klp).
 *
 * Mechanism:
 *   1. sigma-kpatch-build diffs baseline vs patched vmlinuz-sigma
 *   2. Extracts changed functions → links into a .spatch module
 *   3. Module is Ed25519-signed by the build key
 *   4. sigma_kpatch_apply() quiesces all CPUs (stop_machine equivalent)
 *   5. ftrace redirects old function → new function in the patch module
 *   6. CPUs resume; old code is unreachable but still in memory
 *
 * Practical use: a zerotrust bypass CVE can be patched in seconds on a
 * live cluster with zero VM downtime.
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_KPATCH_MAGIC       0x5369674B50617463ULL  /* "SigKPatc" */
#define SIGMA_KPATCH_VERSION     1
#define SIGMA_KPATCH_MAX_FUNCS   128

/* ── A single function replacement ──────────────────────────────────────── */
typedef struct {
    uintptr_t old_addr;        /* VA of function to replace in live kernel  */
    uintptr_t new_addr;        /* VA of replacement function in patch module */
    char      func_name[128];  /* Symbol name — for audit log + revert      */
    sigma_u32 old_size;        /* Size of old function (for overlap check)  */
    bool      active;          /* true once ftrace redirect is live         */
} sigma_kpatch_func_t;

/* ── Loadable patch module ───────────────────────────────────────────────── */
typedef struct {
    sigma_u64          magic;                  /* SIGMA_KPATCH_MAGIC         */
    sigma_u32          version;                /* SIGMA_KPATCH_VERSION       */
    char               patch_id[64];           /* "sigma-kpatch-CVE-2026-42" */
    char               target_kernel[32];      /* kernel build ID this fits  */
    char               description[256];       /* Human-readable description */
    sigma_u64          timestamp;              /* Unix timestamp of build    */
    sigma_u8           signature[64];          /* Ed25519 sig over all above */
    sigma_u32          n_funcs;
    sigma_kpatch_func_t funcs[SIGMA_KPATCH_MAX_FUNCS];
} sigma_kpatch_module_t;

/* ── Apply/revert API ────────────────────────────────────────────────────── */

/*
 * sigma_kpatch_apply — atomically redirect all functions in the patch.
 * Uses a stop_machine equivalent: all CPUs quiesced, no CPU inside any
 * patched function during the redirect.
 * Returns 0 on success, -EBUSY if a CPU is inside a patched function,
 * -EBADMSG if signature verification fails.
 */
int sigma_kpatch_apply(const sigma_kpatch_module_t *patch);

/* List all active patches. cb is called once per active patch_id. */
void sigma_kpatch_list(void (*cb)(const char *patch_id,
                                   const char *description,
                                   sigma_u64   timestamp,
                                   void       *userdata),
                       void *userdata);

/*
 * Revert a live patch — restores original function pointer.
 * The same stop_machine quiesce applies.
 */
int sigma_kpatch_revert(const char *patch_id);

/* Query whether a specific patch is currently active. */
bool sigma_kpatch_is_active(const char *patch_id);

/* Verify Ed25519 signature on a patch module without applying it. */
int sigma_kpatch_verify(const sigma_kpatch_module_t *patch,
                         const sigma_u8 *pubkey);
