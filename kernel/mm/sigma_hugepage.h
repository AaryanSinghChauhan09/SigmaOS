// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_hugepage.h — Transparent Huge Page (THP) + HugeTLB management
 *
 * Inspired by Linux THP, FreeBSD superpages, and illumos ISM.
 *
 * Features:
 *   - 2 MiB anonymous THP with khugepaged-style background collapse
 *   - 1 GiB static HugeTLB pages for database / AI workloads
 *   - Per-VMA THP policy (always / madvise / never)
 *   - NUMA-aware allocation: prefer local node, fallback with penalty
 *   - THP split on fork (copy-on-write) to avoid excessive sharing
 *   - MADV_HUGEPAGE / MADV_NOHUGEPAGE userspace hints
 *   - khugepaged collapse statistics via /sigma/proc/thp_stats
 */

#include <sigma_kernel_types.h>

/* ── Page size constants ─────────────────────────────────────────────────── */
#define SIGMA_PAGE_SIZE       4096UL
#define SIGMA_HUGEPAGE_2M     (2UL * 1024 * 1024)
#define SIGMA_HUGEPAGE_1G     (1UL * 1024 * 1024 * 1024)
#define SIGMA_HUGEPAGE_2M_SHIFT  21
#define SIGMA_HUGEPAGE_1G_SHIFT  30

/* ── THP policy per VMA ──────────────────────────────────────────────────── */
typedef enum {
    SIGMA_THP_NEVER   = 0,   /* never collapse to huge pages                */
    SIGMA_THP_MADVISE = 1,   /* only collapse if MADV_HUGEPAGE was called    */
    SIGMA_THP_ALWAYS  = 2,   /* collapse opportunistically (default)         */
} sigma_thp_policy_t;

/* ── HugeTLB pool ────────────────────────────────────────────────────────── */
typedef struct {
    sigma_u64  pool_2m_total;    /* 2 MiB pages reserved at boot             */
    sigma_u64  pool_2m_free;
    sigma_u64  pool_1g_total;    /* 1 GiB pages reserved at boot             */
    sigma_u64  pool_1g_free;
    sigma_u64  collapse_success; /* khugepaged successful collapses           */
    sigma_u64  collapse_fail;
    sigma_u64  split_count;      /* huge pages split back to base pages       */
} sigma_thp_stats_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Reserve n_2m × 2 MiB pages and n_1g × 1 GiB pages at boot. */
int sigma_hugepage_pool_init(sigma_u64 n_2m, sigma_u64 n_1g);

/* Allocate one huge page from the static pool (sleeps if pool empty). */
void* sigma_hugepage_alloc(sigma_size_t size); /* size = 2M or 1G          */

/* Return a huge page to the pool. */
void sigma_hugepage_free(void* addr, sigma_size_t size);

/* Set THP policy for a VMA range. */
int sigma_thp_set_policy(void* addr, sigma_size_t len, sigma_thp_policy_t p);

/*
 * sigma_thp_collapse — attempt to collapse a 512-page run into one 2 MiB
 * huge page.  Called by khugepaged background thread.
 * Returns 0 on success, -SIGMA_EAGAIN if pages are pinned/dirty.
 */
int sigma_thp_collapse(void* addr);

/* Split a 2 MiB THP back into 512 base pages (e.g. before fork). */
int sigma_thp_split(void* hugepage_addr);

/* Fill stats snapshot from /sigma/proc/thp_stats. */
int sigma_thp_stats(sigma_thp_stats_t* out);
