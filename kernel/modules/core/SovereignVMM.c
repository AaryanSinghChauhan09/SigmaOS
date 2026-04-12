/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUAL MEMORY MANAGER (v2.0 — MODULAR)
 * =========================================================================
 * Refactored into sub-shards: PMM, VMA, AddrSpace.
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "SovereignPMM.h"
#include "SovereignVMA.h"
#include "SovereignAddrSpace.h"

#define PAGE_SHIFT 12

/* -----------------------------------------------------------------------
 * ░░ PAGE FAULT HANDLER
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_page_fault(sigma_u32 pid, sigma_u64 fault_addr,
                              sigma_bool write_fault) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return SIGMA_ESRCH;

    SigmaVMA_t *vma = vma_find(as, fault_addr);
    if (!vma) {
        sigma_printf("Σ [VMM]: SEGFAULT pid=%u addr=0x%llx — no VMA\n",
                     pid, (unsigned long long)fault_addr);
        return SIGMA_EACCES;
    }

    if (write_fault && !(vma->flags & VM_WRITE)) {
        if (vma->flags & VM_COW) {
            sigma_u64 new_phys = pmm_alloc_frame();
            if (!new_phys) return SIGMA_ENOMEM;
            sigma_printf("Σ [VMM]: CoW break pid=%u addr=0x%llx → phys=0x%llx\n",
                         pid, (unsigned long long)(fault_addr & ~(PAGE_SIZE-1)),
                         (unsigned long long)new_phys);
            as->cow_breaks++;
            return SIGMA_OK;
        }
        sigma_printf("Σ [VMM]: SIGSEGV pid=%u write to read-only addr=0x%llx\n",
                     pid, (unsigned long long)fault_addr);
        return SIGMA_EACCES;
    }

    sigma_u64 phys = pmm_alloc_frame();
    if (!phys) {
        sigma_printf("Σ [VMM]: OOM — no free frames (pid=%u)\n", pid);
        return SIGMA_ENOMEM;
    }

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
 * ░░ MMAP / MUNMAP / MPROTECT / BRK wrappers
 * ----------------------------------------------------------------------- */

sigma_u64 sigma_mmap_va(sigma_u32 pid, sigma_u64 hint, sigma_size_t length, sigma_u32 prot, const char *name) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return 0;
    length = (length + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1);
    sigma_u64 base = (hint && hint >= 0x10000ULL) ? (hint & ~(PAGE_SIZE - 1)) : as->mmap_base;
    if (base == as->mmap_base) as->mmap_base += length + PAGE_SIZE;

    SigmaVMA_t *v = vma_insert(as, base, base + length, prot | VM_ANON, name);
    if (!v) return 0;

    sigma_printf("Σ [VMM]: mmap pid=%u [%s] %s%s%s 0x%llx–0x%llx (%lu KB)\n",
                 pid, name, (prot & VM_READ) ? "r" : "-", (prot & VM_WRITE) ? "w" : "-", (prot & VM_EXEC) ? "x" : "-",
                 (unsigned long long)base, (unsigned long long)(base + length), (unsigned long)(length / 1024));
    return base;
}

sigma_err_t sigma_munmap(sigma_u32 pid, sigma_u64 addr, sigma_size_t length) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return SIGMA_ESRCH;
    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        SigmaVMA_t *v = &as->vmas[i];
        if (addr >= v->start && addr + length <= v->end) {
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
            sigma_printf("Σ [VMM]: munmap pid=%u 0x%llx–0x%llx\n", pid, (unsigned long long)addr, (unsigned long long)(addr + length));
            for (sigma_u32 j = i; j + 1 < as->vma_count; j++) as->vmas[j] = as->vmas[j+1];
            as->vma_count--;
            return SIGMA_OK;
        }
    }
    return SIGMA_EINVAL;
}

sigma_err_t sigma_mprotect(sigma_u32 pid, sigma_u64 addr, sigma_size_t length, sigma_u32 prot) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return SIGMA_ESRCH;
    SigmaVMA_t *v = vma_find(as, addr);
    if (!v) return SIGMA_EINVAL;
    sigma_u32 old = v->flags;
    v->flags = (v->flags & ~(VM_READ|VM_WRITE|VM_EXEC)) | prot;
    sigma_printf("Σ [VMM]: mprotect pid=%u 0x%llx (was %x, now %x)\n", pid, (unsigned long long)addr, old, v->flags);
    return SIGMA_OK;
}

sigma_u64 sigma_brk(sigma_u32 pid, sigma_u64 new_brk) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return 0;
    if (new_brk == 0) return as->brk;
    if (new_brk < as->brk) { as->brk = new_brk; return as->brk; }
    sigma_u64 old_brk = as->brk;
    vma_insert(as, old_brk, new_brk, VM_READ | VM_WRITE | VM_ANON | VM_HEAP, "[heap]");
    as->brk = new_brk;
    sigma_printf("Σ [VMM]: brk pid=%u 0x%llx → 0x%llx\n", pid, (unsigned long long)old_brk, (unsigned long long)new_brk);
    return as->brk;
}

sigma_err_t sigma_vmm_fork(sigma_u32 parent_pid, sigma_u32 child_pid) {
    SigmaAddressSpace_t *parent = vmm_get_space(parent_pid);
    if (!parent) return SIGMA_ESRCH;
    SigmaAddressSpace_t *child  = vmm_create_space(child_pid);
    if (!child)  return SIGMA_ENOMEM;
    child->vma_count = parent->vma_count; child->brk = parent->brk;
    child->mmap_base = parent->mmap_base; child->stack_top = parent->stack_top;
    for (sigma_u32 i = 0; i < parent->vma_count; i++) {
        child->vmas[i] = parent->vmas[i];
        if (parent->vmas[i].flags & VM_WRITE) {
            child->vmas[i].flags |= VM_COW; parent->vmas[i].flags |= VM_COW;
        }
    }
    sigma_printf("Σ [VMM]: fork() CoW setup: pid=%u → pid=%u\n", parent_pid, child_pid);
    return SIGMA_OK;
}

void sigma_vmm_print_maps(sigma_u32 pid) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return;
    sigma_printf("Σ [VMM]: /proc/%u/maps ──────────────────────────\n", pid);
    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        SigmaVMA_t *v = &as->vmas[i];
        sigma_printf("  %016llx-%016llx  %s\n", (unsigned long long)v->start, (unsigned long long)v->end, v->name);
    }
    sigma_printf("Σ [VMM]: faults=%u cow_breaks=%u free_frames=%u\n", as->page_faults, as->cow_breaks, pmm_get_free_count());
}

/* -----------------------------------------------------------------------
 * ░░ Public init
 * ----------------------------------------------------------------------- */
void SovereignVMM_Init(void) {
    sigma_printf("Σ [VMM]: Initialising Sovereign Virtual Memory Manager (Modular v2.0)...\n");
    pmm_init();
    vmm_as_init();

    /* Create address space for init process (pid=1) */
    SigmaAddressSpace_t *as = vmm_create_space(1);
    vma_insert(as, 0x0000000000400000ULL, 0x0000000000401000ULL, VM_READ | VM_EXEC, "[text]");
    sigma_brk(1, as->brk + 4 * PAGE_SIZE);
    sigma_mmap_va(1, 0, 64 * 1024, VM_READ | VM_WRITE, "[anon]");
    vma_insert(as, as->stack_top - 8 * PAGE_SIZE, as->stack_top, VM_READ | VM_WRITE | VM_STACK, "[stack]");

    sigma_printf("Σ [VMM]: Virtual Memory Manager online.\n");
}


void SovereignVMM_Register(void) {
    static SovereignModule_t s_vmm_module = {
        .name = "SovereignVMM",
        .type = MODULE_TYPE_CORE,
        .Init = SovereignVMM_Init,
    };
    sigma_module_register(&s_vmm_module);
}
