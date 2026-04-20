/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S05_Memory/shards/sigma_vmm.h
 * =========================================================================
 * Virtual Memory Manager — gap-closes:
 *   Linux  : mmap, brk, mremap, mprotect, KSM, THP, ASLR, OOM killer
 *   macOS  : vm_allocate, Mach VM regions, Guard pages
 *   Windows: VirtualAlloc, AWE, Large Pages, Working Set Trimming
 *   BSD    : vmspace, pmap, wire/unwire
 * =========================================================================
 */

#ifndef SIGMA_VMM_H
#define SIGMA_VMM_H

/* Self-contained primitives */
#include "suites/S01_Genesis/shards/SovereignCommon.h"

typedef sigma_sz_t  vmm_u64;
typedef sigma_u32   vmm_u32;
typedef sigma_i32   vmm_i32;
typedef sigma_u8    vmm_u8;
typedef sigma_bool  vmm_bool;

#define VMM_TRUE  SIGMA_TRUE
#define VMM_FALSE SIGMA_FALSE
#define VMM_NULL  SIGMA_NULL
#define VMM_OK    SIGMA_OK
#define VMM_ERR   -1

/* ── Page size constants ────────────────────────────────────────────────── */
#define SIGMA_PAGE_SIZE      4096ULL
#define SIGMA_PAGE_HUGE      (2ULL * 1024 * 1024)   /* 2MB THP          */
#define SIGMA_PAGE_GIANT     (1ULL * 1024 * 1024 * 1024) /* 1GB          */

/* ── Memory protection flags (POSIX mprotect + PKEY parity) ────────────── */
#define PROT_NONE   0x00
#define PROT_READ   0x01
#define PROT_WRITE  0x02
#define PROT_EXEC   0x04
#define PROT_GUARD  0x08   /* guard page — macOS/Windows parity         */

/* ── mmap flags ─────────────────────────────────────────────────────────── */
#define MAP_PRIVATE   0x01
#define MAP_SHARED    0x02
#define MAP_ANONYMOUS 0x04
#define MAP_FIXED     0x08
#define MAP_HUGE      0x10   /* THP hint                                 */
#define MAP_LOCKED    0x20   /* mlock equivalent                         */

/* ── VMA (Virtual Memory Area) — Linux vm_area_struct equivalent ────────── */
typedef struct {
    vmm_u64 start;       /* inclusive start virtual address            */
    vmm_u64 end;         /* exclusive end virtual address              */
    vmm_u32 prot;        /* protection flags                           */
    vmm_u32 flags;       /* mmap flags                                 */
    vmm_u32 owner_pid;
    vmm_bool is_huge;    /* backed by huge pages (THP)                 */
    vmm_bool is_locked;  /* mlock'd, never swapped                     */
    vmm_u64 phys_base;   /* mapped physical frame (simplified)         */
} sigma_vma_t;

#define SIGMA_VMM_MAX_VMAS 4096

/* ── Address Space (Object-Oriented) ────────────────────────────────────── */
typedef struct {
    sigma_obj_t  base;         /* Inheritance from SovereignObject */
    sigma_vma_t  vmas[SIGMA_VMM_MAX_VMAS];
    vmm_u32      vma_count;
    vmm_u64      total_vm_kb;
    vmm_u64      rss_kb;
    vmm_u64      brk;          /* current program break                 */
    vmm_u64      aslr_offset;  /* ASLR slide                            */
} sigma_addrspace_t;

/* ── OOM scoring (Linux /proc/pid/oom_score_adj parity) ─────────────────── */
typedef struct {
    vmm_u32 pid;
    vmm_i32 oom_adj;    /* -1000 (never kill) to +1000 (kill first)   */
    vmm_u64 rss_kb;
    vmm_u64 cpu_ticks;
} sigma_oom_candidate_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void          sigma_vmm_init(void);
vmm_i32       sigma_vmm_addrspace_create(vmm_u32 pid, vmm_u64 aslr_seed);
void          sigma_vmm_addrspace_destroy(vmm_u32 pid);

vmm_u64       sigma_mmap(vmm_u32 pid, vmm_u64 hint, vmm_u64 length,
                          vmm_u32 prot, vmm_u32 flags);
vmm_i32       sigma_munmap(vmm_u32 pid, vmm_u64 addr, vmm_u64 length);
vmm_i32       sigma_mprotect(vmm_u32 pid, vmm_u64 addr,
                              vmm_u64 length, vmm_u32 new_prot);
vmm_u64       sigma_brk(vmm_u32 pid, vmm_u64 new_brk);

/* THP & memory optimization */
vmm_i32       sigma_madvise_huge(vmm_u32 pid, vmm_u64 addr, vmm_u64 len);
vmm_i32       sigma_ksm_merge(vmm_u32 pid);   /* Kernel Same-page Merging */

/* OOM killer */
vmm_u32       sigma_oom_select_victim(void);
void          sigma_oom_kill(vmm_u32 pid);

void          sigma_vmm_dump(vmm_u32 pid);   /* /proc/pid/maps equivalent */

#endif /* SIGMA_VMM_H */
