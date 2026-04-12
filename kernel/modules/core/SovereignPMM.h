#ifndef SOVEREIGN_PMM_H
#define SOVEREIGN_PMM_H

#include "../../../include/sigma_kernel.h"

#define PAGE_SIZE           4096ULL
#define PHYS_MEM_SIZE       (256ULL * 1024 * 1024)   /* simulate 256 MB RAM */
#define TOTAL_FRAMES        (PHYS_MEM_SIZE / PAGE_SIZE)
#define FRAME_BITMAP_WORDS  (TOTAL_FRAMES / 64)

void pmm_init(void);
sigma_u64 pmm_alloc_frame(void);
void pmm_free_frame(sigma_u64 phys);
sigma_u32 pmm_get_free_count(void);

#endif /* SOVEREIGN_PMM_H */
