/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S05_Memory/shards/sigma_vmm.c
 * =========================================================================
 */

#include "sigma_vmm.h"
#include "sigma_libc.h"

/* ── Global address space table ────────────────────────────────────────── */
#define SIGMA_MAX_PROCS 512
static sigma_addrspace_t s_spaces[SIGMA_MAX_PROCS];
static vmm_u32           s_space_count = 0;

/* ── Physical memory pool (simplified bump allocator) ───────────────────── */
static vmm_u64 s_phys_pool = 0x100000000ULL;  /* start at 4GB physical   */
static vmm_u64 s_phys_free = (1024ULL * 1024 * 1024); /* 1GB pool        */

static vmm_u64 phys_alloc(vmm_u64 size) {
    vmm_u64 pages = (size + SIGMA_PAGE_SIZE - 1) / SIGMA_PAGE_SIZE;
    vmm_u64 bytes = pages * SIGMA_PAGE_SIZE;
    if (bytes > s_phys_free) return 0;
    vmm_u64 addr = s_phys_pool;
    s_phys_pool += bytes;
    s_phys_free -= bytes;
    return addr;
}

/* ── Helpers ─────────────────────────────────────────────────────────────── */
static sigma_addrspace_t *get_as(vmm_u32 pid) {
    for (vmm_u32 i = 0; i < s_space_count; i++)
        if (s_spaces[i].base.id == pid) return &s_spaces[i];
    return VMM_NULL;
}

static vmm_u64 aslr_slide(vmm_u64 seed) {
    /* Simple ASLR: hash the seed into a page-aligned offset in [1MB, 128MB] */
    vmm_u64 range = 127ULL * 1024 * 1024;
    vmm_u64 slide = (seed ^ (seed >> 17) ^ (seed << 5)) % range;
    return (slide / SIGMA_PAGE_SIZE) * SIGMA_PAGE_SIZE + (1ULL << 20);
}

/* ── Init ────────────────────────────────────────────────────────────────── */
void sigma_vmm_init(void) {
    sigma_sigma_sigma_sigma_memset(s_spaces, 0, sizeof(s_spaces));
    sigma_sigma_sigma_sigma_printf("S [VMM] Initialized. Physical pool: 1GB @ 0x%llx\n",
                 (unsigned long long)s_phys_pool);
}

/* ── Address space lifecycle ─────────────────────────────────────────────── */
vmm_i32 sigma_vmm_addrspace_create(vmm_u32 pid, vmm_u64 aslr_seed) {
    if (s_space_count >= SIGMA_MAX_PROCS) return VMM_ERR;
    sigma_addrspace_t *as = &s_spaces[s_space_count++];
    sigma_sigma_sigma_sigma_memset(as, 0, sizeof(*as));
    as->base.id     = pid;
    as->base.name   = "sigma_address_space";
    as->aslr_offset = aslr_slide(aslr_seed);
    as->brk         = 0x400000ULL + as->aslr_offset;
    sigma_sigma_sigma_sigma_printf("S [VMM] AS created: pid=%u aslr=0x%llx\n",
                 as->base.id, (unsigned long long)as->aslr_offset);
    return VMM_OK;
}

void sigma_vmm_addrspace_destroy(vmm_u32 pid) {
    sigma_addrspace_t *as = get_as(pid);
    if (!as) return;
    sigma_sigma_sigma_sigma_printf("S [VMM] AS destroyed: pid=%u vmas=%u total_vm=%llu KB\n",
                 as->base.id, as->vma_count, (unsigned long long)as->total_vm_kb);
    sigma_sigma_sigma_sigma_memset(as, 0, sizeof(*as));
}

/* ── mmap ────────────────────────────────────────────────────────────────── */
vmm_u64 sigma_mmap(vmm_u32 pid, vmm_u64 hint, vmm_u64 length,
                    vmm_u32 prot, vmm_u32 flags) {
    sigma_addrspace_t *as = get_as(pid);
    if (!as || as->vma_count >= SIGMA_VMM_MAX_VMAS) return 0;

    vmm_u64 pages  = (length + SIGMA_PAGE_SIZE - 1) / SIGMA_PAGE_SIZE;
    vmm_u64 sz     = pages * SIGMA_PAGE_SIZE;
    vmm_u64 phys   = phys_alloc(sz);
    if (!phys) return 0;

    /* Place VMA above existing or at hint */
    vmm_u64 vaddr = hint ? hint : (as->brk + as->aslr_offset);
    if (flags & MAP_HUGE) vaddr = (vaddr + SIGMA_PAGE_HUGE - 1)
                                  & ~(SIGMA_PAGE_HUGE - 1);

    sigma_vma_t *vma = &as->vmas[as->vma_count++];
    vma->start     = vaddr;
    vma->end       = vaddr + sz;
    vma->prot      = prot;
    vma->flags     = flags;
    vma->owner_pid = pid;
    vma->is_huge   = (flags & MAP_HUGE) ? VMM_TRUE : VMM_FALSE;
    vma->is_locked = (flags & MAP_LOCKED) ? VMM_TRUE : VMM_FALSE;
    vma->phys_base = phys;

    as->total_vm_kb += sz / 1024;
    as->rss_kb      += sz / 1024;
    as->brk          = vma->end;

    sigma_sigma_sigma_sigma_printf("S [VMM] MMAP: pid=%u vaddr=0x%llx len=%llupg prot=%u%s\n",
                 pid, (unsigned long long)vaddr,
                 (unsigned long long)pages, prot,
                 vma->is_huge ? " [HUGE]" : "");
    return vaddr;
}

/* ── munmap ──────────────────────────────────────────────────────────────── */
vmm_i32 sigma_munmap(vmm_u32 pid, vmm_u64 addr, vmm_u64 length) {
    sigma_addrspace_t *as = get_as(pid);
    if (!as) return VMM_ERR;
    for (vmm_u32 i = 0; i < as->vma_count; i++) {
        if (as->vmas[i].start == addr) {
            vmm_u64 sz = as->vmas[i].end - as->vmas[i].start;
            as->total_vm_kb -= sz / 1024;
            as->rss_kb      -= sz / 1024;
            /* Remove by shifting */
            for (vmm_u32 j = i; j < as->vma_count - 1; j++)
                as->vmas[j] = as->vmas[j + 1];
            as->vma_count--;
            (void)length;
            return VMM_OK;
        }
    }
    return VMM_ERR;
}

/* ── mprotect ────────────────────────────────────────────────────────────── */
vmm_i32 sigma_mprotect(vmm_u32 pid, vmm_u64 addr, vmm_u64 length, vmm_u32 new_prot) {
    sigma_addrspace_t *as = get_as(pid);
    if (!as) return VMM_ERR;
    for (vmm_u32 i = 0; i < as->vma_count; i++) {
        if (addr >= as->vmas[i].start && addr < as->vmas[i].end) {
            as->vmas[i].prot = new_prot;
            sigma_sigma_sigma_sigma_printf("S [VMM] MPROTECT: pid=%u addr=0x%llx prot=%u\n",
                         pid, (unsigned long long)addr, new_prot);
            (void)length;
            return VMM_OK;
        }
    }
    return VMM_ERR;
}

/* ── brk ─────────────────────────────────────────────────────────────────── */
vmm_u64 sigma_brk(vmm_u32 pid, vmm_u64 new_brk) {
    sigma_addrspace_t *as = get_as(pid);
    if (!as) return 0;
    if (new_brk > as->brk) {
        vmm_u64 inc = new_brk - as->brk;
        phys_alloc(inc);
        as->total_vm_kb += inc / 1024;
        as->rss_kb      += inc / 1024;
    }
    as->brk = new_brk;
    return as->brk;
}

/* ── THP / KSM hints ─────────────────────────────────────────────────────── */
vmm_i32 sigma_madvise_huge(vmm_u32 pid, vmm_u64 addr, vmm_u64 len) {
    sigma_sigma_sigma_sigma_printf("S [VMM] THP ADVISE: pid=%u addr=0x%llx len=%llu\n",
                 pid, (unsigned long long)addr, (unsigned long long)len);
    return VMM_OK;
}

vmm_i32 sigma_ksm_merge(vmm_u32 pid) {
    sigma_sigma_sigma_sigma_printf("S [VMM] KSM: page deduplication scan for pid=%u\n", pid);
    return VMM_OK;
}

/* ── OOM killer ──────────────────────────────────────────────────────────── */
vmm_u32 sigma_oom_select_victim(void) {
    vmm_u32 worst_pid = 0;
    vmm_u64 worst_rss = 0;
    for (vmm_u32 i = 0; i < s_space_count; i++) {
        if (s_spaces[i].rss_kb > worst_rss) {
            worst_rss = s_spaces[i].rss_kb;
            worst_pid = s_spaces[i].base.id;
        }
    }
    sigma_sigma_sigma_sigma_printf("S [OOM] Selected victim: pid=%u rss=%llu KB\n",
                 worst_pid, (unsigned long long)worst_rss);
    return worst_pid;
}

void sigma_oom_kill(vmm_u32 pid) {
    sigma_sigma_sigma_sigma_printf("S [OOM] KILL: pid=%u\n", pid);
    sigma_vmm_addrspace_destroy(pid);
}

/* ── /proc/pid/maps equivalent ───────────────────────────────────────────── */
void sigma_vmm_dump(vmm_u32 pid) {
    sigma_addrspace_t *as = get_as(pid);
    if (!as) return;
    sigma_sigma_sigma_sigma_printf("\nS VMM MAPS pid=%u (ASLR=0x%llx)\n",
                 as->base.id, (unsigned long long)as->aslr_offset);
    sigma_sigma_sigma_sigma_printf("%-18s %-18s %s\n", "START", "END", "PROT");
    for (vmm_u32 i = 0; i < as->vma_count; i++) {
        sigma_vma_t *v = &as->vmas[i];
        sigma_sigma_sigma_sigma_printf("  0x%014llx 0x%014llx %c%c%c%s\n",
                     (unsigned long long)v->start,
                     (unsigned long long)v->end,
                     (v->prot & PROT_READ)  ? 'r' : '-',
                     (v->prot & PROT_WRITE) ? 'w' : '-',
                     (v->prot & PROT_EXEC)  ? 'x' : '-',
                     v->is_huge ? " [HUGE]" : "");
    }
}
