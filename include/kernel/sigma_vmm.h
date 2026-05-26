/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUAL MEMORY MANAGER (v1.0)
 * =============================================================================
 * Mission: Page-granular virtual memory with 4-level page tables, demand
 *          paging, Copy-on-Write, and per-process address spaces.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_VMM_H
#define SIGMA_VMM_H

#include "../sigma_kernel_types.h"

#define VMM_MAX_REGIONS       256
#define VMM_MAX_ADDR_SPACES   512

/* Page flags */
#define VMM_FLAG_PRESENT   BIT(0)
#define VMM_FLAG_WRITE     BIT(1)
#define VMM_FLAG_USER      BIT(2)
#define VMM_FLAG_EXEC      BIT(3)
#define VMM_FLAG_COW       BIT(4)   /* Copy-on-Write */
#define VMM_FLAG_NOCACHE   BIT(5)

typedef struct {
    sigma_vaddr_t  base;
    sigma_usize    size;          /* in bytes, must be page-aligned */
    sigma_u64      flags;         /* VMM_FLAG_* bitmask */
    sigma_bool     mapped;
    char           label[32];     /* e.g. "stack", "heap", "code" */
} sigma_vm_region_t;

typedef struct {
    sigma_u32          id;             /* address space ID */
    sigma_paddr_t      pml4_phys;      /* physical address of PML4 table */
    sigma_vm_region_t  regions[VMM_MAX_REGIONS];
    sigma_u32          region_count;
    sigma_u64          total_mapped;   /* total mapped bytes */
    sigma_u64          total_faults;   /* page fault counter */
} sigma_addr_space_t;

#ifdef __cplusplus
extern "C" {
#endif

void              vmm_init(void);
sigma_u32         vmm_create_address_space(void);
int               vmm_destroy_address_space(sigma_u32 as_id);
int               vmm_map_page(sigma_u32 as_id, sigma_vaddr_t vaddr,
                               sigma_paddr_t paddr, sigma_u64 flags);
int               vmm_unmap_page(sigma_u32 as_id, sigma_vaddr_t vaddr);
int               vmm_alloc_region(sigma_u32 as_id, const char* label,
                                   sigma_vaddr_t base, sigma_usize size,
                                   sigma_u64 flags);
int               vmm_free_region(sigma_u32 as_id, sigma_vaddr_t base);
void              vmm_page_fault_handler(sigma_u32 as_id, sigma_vaddr_t fault_addr,
                                         sigma_u64 error_code);
void              vmm_print_address_space(sigma_u32 as_id);
sigma_u64         vmm_get_total_mapped(sigma_u32 as_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VMM_H */
