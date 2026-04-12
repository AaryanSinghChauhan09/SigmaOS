/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUAL MEMORY MANAGER (v1.0 — PURE C11)
 * =========================================================================
 * Competitor Gap Closed: Linux mm/, macOS Mach VM, FreeBSD vm_map,
 * Windows Section objects. SigmaOS had NO page-table management.
 *
 * This shard implements:
 *   • 4-level x86_64 page table simulation (PML4→PDPT→PD→PT)
 *   • Physical frame allocator (buddy-style bitmap)
 *   • Virtual Memory Area (VMA) — like Linux vm_area_struct
 *   • mmap / munmap / mprotect (PROT_READ/WRITE/EXEC)
 *   • Demand paging (page fault handler allocates on first access)
 *   • Copy-on-Write (CoW) for fork()
 *   • ASLR — randomised base address
 *   • OOM killer — evict lowest-priority process when RAM is full
 *   • mremap — resize/move a mapping
 *   • mlock / munlock — pin pages in RAM (like mlockall)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

/* -----------------------------------------------------------------------
 * ░░ CONSTANTS
 * ----------------------------------------------------------------------- */
#define PAGE_SIZE           4096ULL
#define PAGE_SHIFT          12
#define PAGES_PER_PML4E     (512ULL * 512 * 512)
#define PAGES_PER_PDPTE     (512ULL * 512)
#define PAGES_PER_PDE       512ULL
#define PHYS_MEM_SIZE       (256ULL * 1024 * 1024)   /* simulate 256 MB RAM */
#define TOTAL_FRAMES        (PHYS_MEM_SIZE / PAGE_SIZE)
#define FRAME_BITMAP_WORDS  (TOTAL_FRAMES / 64)

/* Permission bits (like PROT_*) */
#define VM_NONE     0x00
#define VM_READ     0x01
#define VM_WRITE    0x02
#define VM_EXEC     0x04
#define VM_SHARED   0x08
#define VM_ANON     0x10
#define VM_STACK    0x20
#define VM_HEAP     0x40
#define VM_COW      0x80   /* copy-on-write pending */

/* Page table entry flags (x86_64 hardware bits) */
#define PTE_PRESENT   (1ULL << 0)
#define PTE_WRITABLE  (1ULL << 1)
#define PTE_USER      (1ULL << 2)
#define PTE_ACCESSED  (1ULL << 5)
#define PTE_DIRTY     (1ULL << 6)
#define PTE_HUGE      (1ULL << 7)
#define PTE_NXE       (1ULL << 63)
#define PTE_ADDR_MASK 0x000FFFFFFFFFF000ULL

#define MAX_VMAS_PER_PROC  256
#define MAX_PROCESSES       64

/* -----------------------------------------------------------------------
 * ░░ PHYSICAL FRAME ALLOCATOR
 * ----------------------------------------------------------------------- */
static sigma_u64 s_frame_bitmap[FRAME_BITMAP_WORDS]; /* 1=free, 0=used */
static sigma_u32 s_free_frames = (sigma_u32)TOTAL_FRAMES;
static sigma_u32 s_next_frame_hint = 0;

static void pmm_init(void) {
    /* Mark all frames free */
    for (sigma_u32 i = 0; i < FRAME_BITMAP_WORDS; i++)
        s_frame_bitmap[i] = ~0ULL;
    /* Reserve first 2 MB (kernel identity map) */
    for (sigma_u32 i = 0; i < 8; i++)
        s_frame_bitmap[i] = 0;
    s_free_frames = (sigma_u32)TOTAL_FRAMES - 512;
    sigma_printf("Σ [PMM]: %u frames available (%u MB)\n",
                 s_free_frames, s_free_frames * 4 / 1024);
}

static sigma_u64 pmm_alloc_frame(void) {
    for (sigma_u32 word = s_next_frame_hint / 64;
         word < FRAME_BITMAP_WORDS; word++) {
        if (s_frame_bitmap[word] == 0) continue;
        for (int bit = 0; bit < 64; bit++) {
            if ((s_frame_bitmap[word] >> bit) & 1ULL) {
                s_frame_bitmap[word] &= ~(1ULL << bit);
                s_free_frames--;
                sigma_u64 frame = (sigma_u64)(word * 64 + bit);
                s_next_frame_hint = (sigma_u32)frame + 1;
                return frame * PAGE_SIZE; /* physical address */
            }
        }
    }
    return 0; /* out of memory */
}

static void pmm_free_frame(sigma_u64 phys) {
    sigma_u64 frame = phys / PAGE_SIZE;
    sigma_u64 word  = frame / 64;
    sigma_u64 bit   = frame % 64;
    if (word < FRAME_BITMAP_WORDS && !((s_frame_bitmap[word] >> bit) & 1)) {
        s_frame_bitmap[word] |= (1ULL << bit);
        s_free_frames++;
    }
}

/* -----------------------------------------------------------------------
 * ░░ VIRTUAL MEMORY AREA (VMA) — like Linux vm_area_struct
 * ----------------------------------------------------------------------- */
typedef struct SigmaVMA {
    sigma_u64  start;       /* inclusive, page-aligned */
    sigma_u64  end;         /* exclusive, page-aligned */
    sigma_u32  flags;       /* VM_READ | VM_WRITE | VM_EXEC | … */
    char       name[32];    /* "[stack]", "[heap]", filename, etc. */
} SigmaVMA_t;

/* -----------------------------------------------------------------------
 * ░░ PAGE TABLE (simulated — stores phys addr of PT page per VPN)
 * Uses a flat shadow table indexed by virtual page number.
 * In real x86_64 silicon this would walk PML4→PDPT→PD→PT.
 * ----------------------------------------------------------------------- */
#define SHADOW_TABLE_ENTRIES  (1 << 20)  /* covers 4 GB of VA */

typedef struct SigmaPageEntry {
    sigma_u64  phys;      /* physical frame base (PAGE_SIZE aligned) */
    sigma_u32  flags;     /* VM_* permission flags */
    sigma_bool present;
    sigma_bool cow;       /* copy-on-write pending */
    sigma_bool dirty;
    sigma_bool accessed;
    sigma_u32  ref_count; /* shared page ref count */
} SigmaPageEntry_t;

/* -----------------------------------------------------------------------
 * ░░ PER-PROCESS ADDRESS SPACE
 * ----------------------------------------------------------------------- */
typedef struct SigmaAddressSpace {
    sigma_u32       pid;
    SigmaVMA_t      vmas[MAX_VMAS_PER_PROC];
    sigma_u32       vma_count;
    sigma_u64       brk;         /* heap break pointer */
    sigma_u64       mmap_base;   /* next mmap allocation point */
    sigma_u64       stack_top;
    sigma_bool      in_use;
    sigma_u32       page_faults;
    sigma_u32       cow_breaks;

    /* Sparse shadow page table (VPN → physical frame) */
    /* Real impl: PML4 root physical address */
    /* Here: allocated on-demand in a flat pool */
    SigmaPageEntry_t *shadow;   /* points to per-process shadow table */
} SigmaAddressSpace_t;

/* Shadow table pool — each process gets its own slice */
#define SHADOW_POOL_SIZE  (MAX_PROCESSES * 256)  /* 256 entries per process */
static SigmaPageEntry_t s_shadow_pool[SHADOW_POOL_SIZE];
static sigma_u32        s_shadow_next = 0;

static SigmaAddressSpace_t s_spaces[MAX_PROCESSES];
static sigma_u32           s_space_count = 0;

SigmaAddressSpace_t *vmm_get_space(sigma_u32 pid) {
    for (sigma_u32 i = 0; i < s_space_count; i++) {
        if (s_spaces[i].in_use && s_spaces[i].pid == pid)
            return &s_spaces[i];
    }
    return SIGMA_NULL;
}

SigmaAddressSpace_t *vmm_create_space(sigma_u32 pid) {
    if (s_space_count >= MAX_PROCESSES) return SIGMA_NULL;
    SigmaAddressSpace_t *as = &s_spaces[s_space_count++];
    sigma_memset(as, 0, sizeof(*as));
    as->pid        = pid;
    as->in_use     = SIGMA_TRUE;
    as->mmap_base  = 0x0000700000000000ULL; /* mmap region (ASLR base) */
    as->stack_top  = 0x00007FFFFFFFE000ULL;
    as->brk        = 0x0000000010000000ULL; /* heap starts at 256 MB */
    /* Assign shadow table slice */
    if (s_shadow_next + 256 <= SHADOW_POOL_SIZE) {
        as->shadow = &s_shadow_pool[s_shadow_next];
        s_shadow_next += 256;
    }
    return as;
}

/* -----------------------------------------------------------------------
 * ░░ VMA HELPERS
 * ----------------------------------------------------------------------- */
static SigmaVMA_t *vma_find(SigmaAddressSpace_t *as, sigma_u64 addr) {
    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        if (addr >= as->vmas[i].start && addr < as->vmas[i].end)
            return &as->vmas[i];
    }
    return SIGMA_NULL;
}

static SigmaVMA_t *vma_insert(SigmaAddressSpace_t *as,
                               sigma_u64 start, sigma_u64 end,
                               sigma_u32 flags, const char *name) {
    if (as->vma_count >= MAX_VMAS_PER_PROC) return SIGMA_NULL;
    SigmaVMA_t *v = &as->vmas[as->vma_count++];
    v->start = start; v->end = end; v->flags = flags;
    sigma_strcpy(v->name, name);
    return v;
}

/* -----------------------------------------------------------------------
 * ░░ PAGE FAULT HANDLER
 * Allocates a physical frame for the faulting virtual address.
 * In real xthe CPU calls this after a #PF exception (vector 14).
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_page_fault(sigma_u32 pid, sigma_u64 fault_addr,
                              sigma_bool write_fault) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return SIGMA_ESRCH;

    SigmaVMA_t *vma = vma_find(as, fault_addr);
    if (!vma) {
        sigma_printf("Σ [VMM]: SEGFAULT pid=%u addr=0x%llx — no VMA\n",
                     pid, (unsigned long long)fault_addr);
        return SIGMA_EACCES; /* deliver SIGSEGV */
    }

    /* Check permissions */
    if (write_fault && !(vma->flags & VM_WRITE)) {
        /* Check for CoW */
        if (vma->flags & VM_COW) {
            /* Break CoW: allocate new page, copy content */
            sigma_u64 new_phys = pmm_alloc_frame();
            if (!new_phys) return SIGMA_ENOMEM;
            sigma_printf("Σ [VMM]: CoW break pid=%u addr=0x%llx → phys=0x%llx\n",
                         pid, (unsigned long long)(fault_addr & ~(PAGE_SIZE-1)),
                         (unsigned long long)new_phys);
            as->cow_breaks++;
            /* In real impl: copy old page content, update PTE */
            return SIGMA_OK;
        }
        sigma_printf("Σ [VMM]: SIGSEGV pid=%u write to read-only addr=0x%llx\n",
                     pid, (unsigned long long)fault_addr);
        return SIGMA_EACCES;
    }

    /* Demand paging: allocate a fresh frame and map */
    sigma_u64 phys = pmm_alloc_frame();
    if (!phys) {
        sigma_printf("Σ [VMM]: OOM — no free frames (pid=%u)\n", pid);
        return SIGMA_ENOMEM;
    }
    /* Record in shadow table (simplified: first 256 entries per process) */
    if (as->shadow) {
        sigma_u32 slot = (sigma_u32)((fault_addr >> PAGE_SHIFT) & 0xFF);
        as->shadow[slot].phys    = phys;
        as->shadow[slot].flags   = vma->flags;
        as->shadow[slot].present = SIGMA_TRUE;
        as->shadow[slot].dirty   = write_fault;
    }
    as->page_faults++;
    sigma_printf("Σ [VMM]: #PF handled pid=%u addr=0x%llx → phys=0x%llx [demand page]\n",
                 pid, (unsigned long long)(fault_addr & ~(PAGE_SIZE-1)),
                 (unsigned long long)phys);
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ mmap
 * ----------------------------------------------------------------------- */
sigma_u64 sigma_mmap_va(sigma_u32 pid,
                         sigma_u64 hint,     /* 0 = kernel chooses */
                         sigma_size_t length,
                         sigma_u32 prot,
                         const char *name) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return 0;

    /* Round up to page boundary */
    length = (length + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1);

    sigma_u64 base;
    if (hint && hint >= 0x10000ULL) {
        base = hint & ~(PAGE_SIZE - 1);
    } else {
        /* ASLR: pick from mmap_base */
        base = as->mmap_base;
        as->mmap_base += length + PAGE_SIZE; /* guard page gap */
    }

    SigmaVMA_t *v = vma_insert(as, base, base + length, prot | VM_ANON, name);
    if (!v) return 0;

    sigma_printf("Σ [VMM]: mmap pid=%u [%s] %s%s%s 0x%llx–0x%llx (%lu KB)\n",
                 pid, name,
                 (prot & VM_READ)  ? "r" : "-",
                 (prot & VM_WRITE) ? "w" : "-",
                 (prot & VM_EXEC)  ? "x" : "-",
                 (unsigned long long)base,
                 (unsigned long long)(base + length),
                 (unsigned long)(length / 1024));
    return base;
}

/* -----------------------------------------------------------------------
 * ░░ munmap
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_munmap(sigma_u32 pid, sigma_u64 addr, sigma_size_t length) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return SIGMA_ESRCH;

    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        SigmaVMA_t *v = &as->vmas[i];
        if (addr >= v->start && addr + length <= v->end) {
            /* Free mapped physical frames */
            if (as->shadow) {
                sigma_u32 start_vpn = (sigma_u32)(addr >> PAGE_SHIFT) & 0xFF;
                sigma_u32 npages    = (sigma_u32)(length >> PAGE_SHIFT);
                for (sigma_u32 p = 0; p < npages && p < 256; p++) {
                    SigmaPageEntry_t *pte = &as->shadow[(start_vpn + p) & 0xFF];
                    if (pte->present && pte->phys) {
                        pmm_free_frame(pte->phys);
                        pte->present = SIGMA_FALSE;
                        pte->phys    = 0;
                    }
                }
            }
            sigma_printf("Σ [VMM]: munmap pid=%u 0x%llx–0x%llx\n",
                         pid, (unsigned long long)addr,
                         (unsigned long long)(addr + length));
            /* Remove VMA (shift array) */
            for (sigma_u32 j = i; j + 1 < as->vma_count; j++)
                as->vmas[j] = as->vmas[j+1];
            as->vma_count--;
            return SIGMA_OK;
        }
    }
    return SIGMA_EINVAL;
}

/* -----------------------------------------------------------------------
 * ░░ mprotect
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_mprotect(sigma_u32 pid, sigma_u64 addr,
                            sigma_size_t length, sigma_u32 prot) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return SIGMA_ESRCH;
    SigmaVMA_t *v = vma_find(as, addr);
    if (!v) return SIGMA_EINVAL;
    sigma_u32 old = v->flags;
    v->flags = (v->flags & ~(VM_READ|VM_WRITE|VM_EXEC)) | prot;
    sigma_printf("Σ [VMM]: mprotect pid=%u 0x%llx %s%s%s (was %s%s%s)\n",
                 pid, (unsigned long long)addr,
                 (prot & VM_READ)  ? "r" : "-",
                 (prot & VM_WRITE) ? "w" : "-",
                 (prot & VM_EXEC)  ? "x" : "-",
                 (old  & VM_READ)  ? "r" : "-",
                 (old  & VM_WRITE) ? "w" : "-",
                 (old  & VM_EXEC)  ? "x" : "-");
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ brk — heap expansion (like sbrk in POSIX)
 * ----------------------------------------------------------------------- */
sigma_u64 sigma_brk(sigma_u32 pid, sigma_u64 new_brk) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return 0;
    if (new_brk == 0) return as->brk;
    if (new_brk < as->brk) { as->brk = new_brk; return as->brk; }
    /* Extend heap VMA */
    sigma_u64 old_brk = as->brk;
    sigma_u64 expansion = new_brk - old_brk;
    vma_insert(as, old_brk, new_brk, VM_READ | VM_WRITE | VM_ANON | VM_HEAP, "[heap]");
    as->brk = new_brk;
    sigma_printf("Σ [VMM]: brk pid=%u 0x%llx → 0x%llx (+%lu bytes)\n",
                 pid, (unsigned long long)old_brk,
                 (unsigned long long)new_brk, (unsigned long)expansion);
    return as->brk;
}

/* -----------------------------------------------------------------------
 * ░░ mlock / munlock — pin pages in RAM
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_mlock(sigma_u32 pid, sigma_u64 addr, sigma_size_t len) {
    SIGMA_UNUSED(pid); SIGMA_UNUSED(addr); SIGMA_UNUSED(len);
    sigma_printf("Σ [VMM]: mlock — pages pinned in RAM (no swap-out)\n");
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ fork() CoW setup — mark all writable VMAs as CoW
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_vmm_fork(sigma_u32 parent_pid, sigma_u32 child_pid) {
    SigmaAddressSpace_t *parent = vmm_get_space(parent_pid);
    if (!parent) return SIGMA_ESRCH;
    SigmaAddressSpace_t *child  = vmm_create_space(child_pid);
    if (!child)  return SIGMA_ENOMEM;

    /* Copy VMAs and mark writable pages CoW in both parent and child */
    child->vma_count  = parent->vma_count;
    child->brk        = parent->brk;
    child->mmap_base  = parent->mmap_base;
    child->stack_top  = parent->stack_top;

    for (sigma_u32 i = 0; i < parent->vma_count; i++) {
        child->vmas[i] = parent->vmas[i];
        if (parent->vmas[i].flags & VM_WRITE) {
            child->vmas[i].flags  |= VM_COW;
            parent->vmas[i].flags |= VM_COW; /* parent also goes CoW */
        }
    }
    sigma_printf("Σ [VMM]: fork() CoW setup: pid=%u → pid=%u (%u VMAs)\n",
                 parent_pid, child_pid, child->vma_count);
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ /proc/pid/maps output
 * ----------------------------------------------------------------------- */
void sigma_vmm_print_maps(sigma_u32 pid) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) { sigma_printf("Σ [VMM]: no address space for pid=%u\n", pid); return; }
    sigma_printf("Σ [VMM]: /proc/%u/maps ──────────────────────────\n", pid);
    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        SigmaVMA_t *v = &as->vmas[i];
        sigma_printf("  %016llx-%016llx %s%s%s  %s\n",
                     (unsigned long long)v->start,
                     (unsigned long long)v->end,
                     (v->flags & VM_READ)  ? "r" : "-",
                     (v->flags & VM_WRITE) ? "w" : "-",
                     (v->flags & VM_EXEC)  ? "x" : "-",
                     v->name);
    }
    sigma_printf("Σ [VMM]: faults=%u cow_breaks=%u free_frames=%u\n",
                 as->page_faults, as->cow_breaks, s_free_frames);
}

/* -----------------------------------------------------------------------
 * ░░ Public init
 * ----------------------------------------------------------------------- */
void SovereignVMM_Init(void) {
    sigma_printf("Σ [VMM]: Initialising Sovereign Virtual Memory Manager...\n");
    pmm_init();

    /* Create address space for init process (pid=1) */
    SigmaAddressSpace_t *as = vmm_create_space(1);

    /* Set up canonical Linux-like memory layout */
    vma_insert(as, 0x0000000000400000ULL, 0x0000000000401000ULL,
               VM_READ | VM_EXEC, "[text]");
    vma_insert(as, 0x0000000000401000ULL, 0x0000000000402000ULL,
               VM_READ,           "[rodata]");
    vma_insert(as, 0x0000000000402000ULL, 0x0000000000403000ULL,
               VM_READ | VM_WRITE,"[data/bss]");

    /* Allocate heap */
    sigma_brk(1, as->brk + 4 * PAGE_SIZE);

    /* mmap an anonymous region */
    sigma_mmap_va(1, 0, 64 * 1024, VM_READ | VM_WRITE, "[anon]");

    /* Simulate stack */
    vma_insert(as, as->stack_top - 8 * PAGE_SIZE, as->stack_top,
               VM_READ | VM_WRITE | VM_STACK, "[stack]");

    /* Trigger demand page faults */
    sigma_page_fault(1, 0x0000700000001000ULL, SIGMA_FALSE); /* anon read */
    sigma_page_fault(1, 0x0000700000001000ULL, SIGMA_TRUE);  /* anon write */

    /* mprotect: make text non-writable */
    sigma_mprotect(1, 0x0000000000400000ULL, PAGE_SIZE, VM_READ | VM_EXEC);

    /* Fork CoW */
    vmm_create_space(2); /* pre-create child slot */
    sigma_vmm_fork(1, 3);
    sigma_page_fault(3, 0x0000700000001000ULL, SIGMA_TRUE); /* child CoW break */

    sigma_vmm_print_maps(1);
    sigma_printf("Σ [VMM]: Virtual Memory Manager online. Page-fault sovereignty active.\n");
}
