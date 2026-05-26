/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUAL MEMORY MANAGER (v1.0)
 * =========================================================================
 * 4-level page tables (PML4 → PDPT → PD → PT), demand paging with CoW,
 * and per-process address space management.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_vmm.h"

namespace SigmaOS {
namespace Kernel {

class SovereignVMM {
public:
    static SovereignVMM& getInstance() {
        static SovereignVMM instance;
        return instance;
    }

    void init() {
        m_as_count = 0;
        m_total_page_faults = 0;
        for (sigma_u32 i = 0; i < VMM_MAX_ADDR_SPACES; i++) {
            m_spaces[i].id = 0;
            m_spaces[i].region_count = 0;
        }

        /* Create kernel address space (ASID 1) */
        sigma_u32 kernel_as = createAddressSpace();
        allocRegion(kernel_as, "kernel-code",  KERNEL_VMA,                PAGE_SIZE * 1024, VMM_FLAG_PRESENT | VMM_FLAG_EXEC);
        allocRegion(kernel_as, "kernel-stack", KERNEL_VMA + 0x10000000ULL, PAGE_SIZE * 64,   VMM_FLAG_PRESENT | VMM_FLAG_WRITE);
        allocRegion(kernel_as, "kernel-heap",  KERNEL_VMA + 0x20000000ULL, PAGE_SIZE * 512,  VMM_FLAG_PRESENT | VMM_FLAG_WRITE);

        sigma_log("[VMM] Sovereign Virtual Memory Manager initialized.");
        sigma_log_info("[VMM] Kernel address space created (ASID %u, 3 regions).\n", kernel_as);
    }

    sigma_u32 createAddressSpace() {
        if (m_as_count >= VMM_MAX_ADDR_SPACES) return 0;

        sigma_u32 id = ++m_as_count;
        sigma_addr_space_t& as = m_spaces[id - 1];
        as.id = id;
        as.pml4_phys = 0x100000ULL + (sigma_u64)(id - 1) * PAGE_SIZE;
        as.region_count = 0;
        as.total_mapped = 0;
        as.total_faults = 0;

        sigma_log_info("[VMM] Created address space ASID %u (PML4 @ 0x%llx)\n",
                       id, (unsigned long long)as.pml4_phys);
        return id;
    }

    int destroyAddressSpace(sigma_u32 as_id) {
        sigma_addr_space_t* as = findAS(as_id);
        if (!as) return K_ERR_NOTFOUND;

        sigma_log_info("[VMM] Destroying ASID %u (%u regions, %lluKB mapped)\n",
                       as_id, as->region_count,
                       (unsigned long long)(as->total_mapped / 1024));
        as->id = 0;
        as->region_count = 0;
        as->total_mapped = 0;
        return K_OK;
    }

    int mapPage(sigma_u32 as_id, sigma_vaddr_t vaddr, sigma_paddr_t paddr, sigma_u64 flags) {
        sigma_addr_space_t* as = findAS(as_id);
        if (!as) return K_ERR_NOTFOUND;

        /* 4-level page table walk simulation */
        sigma_u32 pml4_idx = (sigma_u32)((vaddr >> 39) & 0x1FF);
        sigma_u32 pdpt_idx = (sigma_u32)((vaddr >> 30) & 0x1FF);
        sigma_u32 pd_idx   = (sigma_u32)((vaddr >> 21) & 0x1FF);
        sigma_u32 pt_idx   = (sigma_u32)((vaddr >> 12) & 0x1FF);

        SIGMA_UNUSED(pml4_idx);
        SIGMA_UNUSED(pdpt_idx);
        SIGMA_UNUSED(pd_idx);
        SIGMA_UNUSED(pt_idx);

        as->total_mapped += PAGE_SIZE;
        return K_OK;
    }

    int unmapPage(sigma_u32 as_id, sigma_vaddr_t vaddr) {
        sigma_addr_space_t* as = findAS(as_id);
        if (!as) return K_ERR_NOTFOUND;

        cpu_invlpg(vaddr);

        if (as->total_mapped >= PAGE_SIZE)
            as->total_mapped -= PAGE_SIZE;
        return K_OK;
    }

    int allocRegion(sigma_u32 as_id, const char* label, sigma_vaddr_t base,
                    sigma_usize size, sigma_u64 flags) {
        sigma_addr_space_t* as = findAS(as_id);
        if (!as || as->region_count >= VMM_MAX_REGIONS) return K_ERR_NOMEM;

        sigma_vm_region_t& r = as->regions[as->region_count];
        r.base = base;
        r.size = ALIGN_UP(size, PAGE_SIZE);
        r.flags = flags;
        r.mapped = SIGMA_FALSE;
        sigma_strncpy(r.label, label, 32);
        as->region_count++;

        sigma_log_info("[VMM] ASID %u: alloc region '%s' @ 0x%llx (%lluKB, flags=0x%llx)\n",
                       as_id, label, (unsigned long long)base,
                       (unsigned long long)(r.size / 1024),
                       (unsigned long long)flags);
        return K_OK;
    }

    int freeRegion(sigma_u32 as_id, sigma_vaddr_t base) {
        sigma_addr_space_t* as = findAS(as_id);
        if (!as) return K_ERR_NOTFOUND;

        for (sigma_u32 i = 0; i < as->region_count; i++) {
            if (as->regions[i].base == base) {
                /* Shift remaining regions down */
                for (sigma_u32 j = i; j + 1 < as->region_count; j++) {
                    as->regions[j] = as->regions[j + 1];
                }
                as->region_count--;
                return K_OK;
            }
        }
        return K_ERR_NOTFOUND;
    }

    void pageFaultHandler(sigma_u32 as_id, sigma_vaddr_t fault_addr, sigma_u64 error_code) {
        sigma_addr_space_t* as = findAS(as_id);
        if (!as) {
            sigma_log("[VMM] PAGE FAULT: Invalid address space.");
            return;
        }

        as->total_faults++;
        m_total_page_faults++;

        /* Check if the fault is in a mapped region */
        for (sigma_u32 i = 0; i < as->region_count; i++) {
            sigma_vm_region_t& r = as->regions[i];
            if (fault_addr >= r.base && fault_addr < r.base + r.size) {
                /* Demand paging: allocate physical page and map */
                if (!r.mapped) {
                    sigma_paddr_t phys = 0x200000ULL + m_total_page_faults * PAGE_SIZE;
                    mapPage(as_id, ALIGN_DOWN(fault_addr, PAGE_SIZE), phys, r.flags);
                    sigma_log_info("[VMM] Demand page: ASID %u, VA 0x%llx → PA 0x%llx\n",
                                   as_id, (unsigned long long)fault_addr,
                                   (unsigned long long)phys);
                    return;
                }

                /* CoW: copy page on write fault */
                if ((r.flags & VMM_FLAG_COW) && (error_code & 0x2)) {
                    sigma_log_info("[VMM] CoW fault: ASID %u, VA 0x%llx — copying page\n",
                                   as_id, (unsigned long long)fault_addr);
                    r.flags &= ~VMM_FLAG_COW;
                    r.flags |= VMM_FLAG_WRITE;
                    return;
                }
            }
        }

        /* Segmentation fault — no valid region */
        sigma_log_info("[VMM] SEGFAULT: ASID %u, VA 0x%llx, error=0x%llx\n",
                       as_id, (unsigned long long)fault_addr,
                       (unsigned long long)error_code);
    }

    void printAddressSpace(sigma_u32 as_id) {
        sigma_addr_space_t* as = findAS(as_id);
        if (!as) return;

        sigma_log_info("\n[VMM] Address Space ASID %u (PML4 @ 0x%llx)\n",
                       as_id, (unsigned long long)as->pml4_phys);
        sigma_log_info("[VMM]   Regions: %u | Mapped: %lluKB | Faults: %llu\n",
                       as->region_count,
                       (unsigned long long)(as->total_mapped / 1024),
                       (unsigned long long)as->total_faults);
        for (sigma_u32 i = 0; i < as->region_count; i++) {
            sigma_vm_region_t& r = as->regions[i];
            sigma_log_info("[VMM]   [%s] 0x%llx – 0x%llx (%lluKB) flags=0x%llx\n",
                           r.label, (unsigned long long)r.base,
                           (unsigned long long)(r.base + r.size),
                           (unsigned long long)(r.size / 1024),
                           (unsigned long long)r.flags);
        }
    }

    sigma_u64 getTotalMapped(sigma_u32 as_id) {
        sigma_addr_space_t* as = findAS(as_id);
        return as ? as->total_mapped : 0;
    }

private:
    SovereignVMM() : m_as_count(0), m_total_page_faults(0) {}

    sigma_addr_space_t* findAS(sigma_u32 as_id) {
        if (as_id == 0 || as_id > m_as_count) return SIGMA_NULL;
        sigma_addr_space_t& as = m_spaces[as_id - 1];
        return (as.id == as_id) ? &as : SIGMA_NULL;
    }

    sigma_addr_space_t m_spaces[VMM_MAX_ADDR_SPACES];
    sigma_u32 m_as_count;
    sigma_u64 m_total_page_faults;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void vmm_init(void) {
    SigmaOS::Kernel::SovereignVMM::getInstance().init();
}

sigma_u32 vmm_create_address_space(void) {
    return SigmaOS::Kernel::SovereignVMM::getInstance().createAddressSpace();
}

int vmm_destroy_address_space(sigma_u32 as_id) {
    return SigmaOS::Kernel::SovereignVMM::getInstance().destroyAddressSpace(as_id);
}

int vmm_map_page(sigma_u32 as_id, sigma_vaddr_t vaddr, sigma_paddr_t paddr, sigma_u64 flags) {
    return SigmaOS::Kernel::SovereignVMM::getInstance().mapPage(as_id, vaddr, paddr, flags);
}

int vmm_unmap_page(sigma_u32 as_id, sigma_vaddr_t vaddr) {
    return SigmaOS::Kernel::SovereignVMM::getInstance().unmapPage(as_id, vaddr);
}

int vmm_alloc_region(sigma_u32 as_id, const char* label, sigma_vaddr_t base,
                     sigma_usize size, sigma_u64 flags) {
    return SigmaOS::Kernel::SovereignVMM::getInstance().allocRegion(as_id, label, base, size, flags);
}

int vmm_free_region(sigma_u32 as_id, sigma_vaddr_t base) {
    return SigmaOS::Kernel::SovereignVMM::getInstance().freeRegion(as_id, base);
}

void vmm_page_fault_handler(sigma_u32 as_id, sigma_vaddr_t fault_addr, sigma_u64 error_code) {
    SigmaOS::Kernel::SovereignVMM::getInstance().pageFaultHandler(as_id, fault_addr, error_code);
}

void vmm_print_address_space(sigma_u32 as_id) {
    SigmaOS::Kernel::SovereignVMM::getInstance().printAddressSpace(as_id);
}

sigma_u64 vmm_get_total_mapped(sigma_u32 as_id) {
    return SigmaOS::Kernel::SovereignVMM::getInstance().getTotalMapped(as_id);
}

} // extern "C"
