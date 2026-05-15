/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: VMM PERFORMANCE SHARD (v1.0)
 * =============================================================================
 * Principles: Zero-Wait Memory Duplication.
 * =============================================================================
 */
#include "../../include/core/sigma_kernel_types.h"

extern void vmm_fast_copy(void* dest, void* src);

/* Performance-tier page copy for Forking/COW */
void vmm_copy_page(sigma_u64 dest_phys, sigma_u64 src_phys) {
    /* Maps temporarily or assumes direct access in kernel space */
    vmm_fast_copy((void*)dest_phys, (void*)src_phys);
}
