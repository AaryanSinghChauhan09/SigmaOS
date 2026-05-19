/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: IOMMU (Input-Output Memory Management Unit)
 * =============================================================================
 * Inspired by: Linux kernel drivers/iommu/iommu.c
 *              FreeBSD sys/x86/iommu/intel_drv.c
 * =============================================================================
 * DMA remapping and isolation, preventing devices from accessing unauthorized RAM.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define IOMMU_DOMAIN_UNMANAGED 0
#define IOMMU_DOMAIN_DMA       1
#define IOMMU_DOMAIN_IDENTITY  2

#define IOMMU_READ  (1 << 0)
#define IOMMU_WRITE (1 << 1)
#define IOMMU_CACHE (1 << 2)

#define MAX_IOMMU_DOMAINS 16
#define MAX_IOMMU_MAPPINGS 128

typedef struct {
    sigma_u64 iova; /* IO Virtual Address */
    sigma_u64 paddr; /* Physical Address */
    sigma_u32 size;
    sigma_u32 prot;
    sigma_bool active;
} iommu_mapping_t;

typedef struct {
    sigma_u32 type;
    iommu_mapping_t mappings[MAX_IOMMU_MAPPINGS];
    sigma_bool active;
} iommu_domain_t;

static iommu_domain_t domains[MAX_IOMMU_DOMAINS];

void iommu_init(void) {
    sigma_memset(domains, 0, sizeof(domains));
    sigma_printf("[iommu] IOMMU Subsystem initialized\n");
}

int iommu_domain_alloc(sigma_u32 type) {
    for (sigma_u32 i = 0; i < MAX_IOMMU_DOMAINS; i++) {
        if (!domains[i].active) {
            domains[i].type = type;
            domains[i].active = SIGMA_TRUE;
            sigma_printf("[iommu] Allocated IOMMU Domain %u (Type: %u)\n", i, type);
            return (int)i;
        }
    }
    return -1;
}

void iommu_domain_free(sigma_u32 domain_id) {
    if (domain_id < MAX_IOMMU_DOMAINS && domains[domain_id].active) {
        domains[domain_id].active = SIGMA_FALSE;
        sigma_printf("[iommu] Freed IOMMU Domain %u\n", domain_id);
    }
}

int iommu_map(sigma_u32 domain_id, sigma_u64 iova, sigma_u64 paddr, sigma_u32 size, sigma_u32 prot) {
    if (domain_id >= MAX_IOMMU_DOMAINS || !domains[domain_id].active) return -1;
    
    iommu_domain_t* dom = &domains[domain_id];
    
    for (sigma_u32 i = 0; i < MAX_IOMMU_MAPPINGS; i++) {
        if (!dom->mappings[i].active) {
            dom->mappings[i].iova = iova;
            dom->mappings[i].paddr = paddr;
            dom->mappings[i].size = size;
            dom->mappings[i].prot = prot;
            dom->mappings[i].active = SIGMA_TRUE;
            
            sigma_printf("[iommu] Mapped IOVA 0x%llx -> Phys 0x%llx (Size: %u, Prot: %u)\n", 
                         iova, paddr, size, prot);
                         
            /* In a real kernel, this writes to the IOMMU page tables (e.g. VT-d Context/Extended Page Tables) */
            return 0;
        }
    }
    sigma_printf("[iommu] ERR: Domain %u mapping table full\n", domain_id);
    return -1;
}

int iommu_unmap(sigma_u32 domain_id, sigma_u64 iova, sigma_u32 size) {
    if (domain_id >= MAX_IOMMU_DOMAINS || !domains[domain_id].active) return -1;
    
    iommu_domain_t* dom = &domains[domain_id];
    
    for (sigma_u32 i = 0; i < MAX_IOMMU_MAPPINGS; i++) {
        if (dom->mappings[i].active && dom->mappings[i].iova == iova && dom->mappings[i].size == size) {
            dom->mappings[i].active = SIGMA_FALSE;
            sigma_printf("[iommu] Unmapped IOVA 0x%llx\n", iova);
            
            /* In a real kernel, invalidate IOMMU TLB here */
            return 0;
        }
    }
    return -1;
}
