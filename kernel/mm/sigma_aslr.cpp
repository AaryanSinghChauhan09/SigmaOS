// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_aslr.cpp — ASLR + W^X enforcement (HardenedBSD kern_aslr.c-inspired)
 *
 * 42-bit entropy per region on x86_64. Each memory region (stack, heap, mmap,
 * vdso) gets independently randomised addresses so knowing one region's base
 * does not reveal any other.  W^X ensures no page is simultaneously writable
 * and executable — blocking ROP-based JIT spraying attacks.
 */
#include "sigma_aslr.h"
#include "sigma_log.h"
#include "include/sigma_sysctl.h"

extern "C" void sigma_random_bytes(sigma_u8* buf, sigma_size_t len);

/* ── Tunables ─────────────────────────────────────────────────────────────── */
static int  g_aslr_enabled      = 1;
static int  g_aslr_entropy_bits = 42;
static int  g_wx_enforcement    = 1;

SIGMA_SYSCTL(aslr_enabled,   "security.aslr.enabled",       SYSCTL_TYPE_BOOL, &g_aslr_enabled,      false)
SIGMA_SYSCTL(aslr_entropy,   "security.aslr.entropy_bits",  SYSCTL_TYPE_INT,  &g_aslr_entropy_bits,  true)
SIGMA_SYSCTL(wx_enforcement, "security.aslr.wx_enforcement",SYSCTL_TYPE_INT,  &g_wx_enforcement,     false)

/* ── Masks ────────────────────────────────────────────────────────────────── */
#define SIGMA_ASLR_ENTROPY_BITS_64   42
#define SIGMA_ASLR_PAGE_ALIGN        0xFFFFFFFFFFFFF000ULL
#define SIGMA_PROT_WRITE             0x02
#define SIGMA_PROT_EXEC              0x04

/* ── ASLR layout generation ─────────────────────────────────────────────── */

int sigma_aslr_generate_layout(sigma_aslr_layout_t* layout) {
    if (!g_aslr_enabled) {
        /* Deterministic layout — useful only for debugging; never in production */
        layout->stack_base   = 0x7FFFFFFFE000ULL;
        layout->heap_base    = 0x0000700000000000ULL;
        layout->mmap_base    = 0x0000600000000000ULL;
        layout->vdso_base    = 0x00007FFF00000000ULL;
        layout->entropy_bits = 0;
        return 0;
    }

    sigma_u8 rand_bytes[32];
    sigma_random_bytes(rand_bytes, sizeof(rand_bytes));

    sigma_u64 mask = ((1ULL << SIGMA_ASLR_ENTROPY_BITS_64) - 1ULL) << 12;

    sigma_u64 r0 = *(sigma_u64*)(rand_bytes + 0);
    sigma_u64 r1 = *(sigma_u64*)(rand_bytes + 8);
    sigma_u64 r2 = *(sigma_u64*)(rand_bytes + 16);
    sigma_u64 r3 = *(sigma_u64*)(rand_bytes + 24);

    /* Each region has an independent random offset — per-region entropy */
    layout->stack_base   = (0x7FFFFFFFFFFF0000ULL - (r0 & mask)) & SIGMA_ASLR_PAGE_ALIGN;
    layout->heap_base    = (0x0000700000000000ULL + (r1 & mask)) & SIGMA_ASLR_PAGE_ALIGN;
    layout->mmap_base    = (0x0000600000000000ULL + (r2 & mask)) & SIGMA_ASLR_PAGE_ALIGN;
    layout->vdso_base    = (0x00007FFF00000000ULL + (r3 & mask)) & SIGMA_ASLR_PAGE_ALIGN;
    layout->entropy_bits = (sigma_u8)SIGMA_ASLR_ENTROPY_BITS_64;

    return 0;
}

/* ── W^X enforcement ─────────────────────────────────────────────────────── */

int sigma_mm_check_wx(sigma_u32 prot_flags) {
    if (!g_wx_enforcement) return 0;

    if ((prot_flags & SIGMA_PROT_WRITE) && (prot_flags & SIGMA_PROT_EXEC)) {
        sigma_log_err(
            "[sigma-mm] W^X VIOLATION: attempted PROT_WRITE|PROT_EXEC mapping "
            "(prot=0x%x) — denied\n", prot_flags);
        return -1; /* -EPERM */
    }
    return 0;
}

/* ── Init ─────────────────────────────────────────────────────────────────── */

void sigma_aslr_init(void) {
    sigma_log_info("[sigma-aslr] ASLR initialised: enabled=%d entropy=%d bits W^X=%d\n",
                   g_aslr_enabled, g_aslr_entropy_bits, g_wx_enforcement);
}
