#ifndef SOVEREIGN_PMM_H
#define SOVEREIGN_PMM_H

#include "suites/S01_Genesis/shards/sigma_types.h"

#define PAGE_SIZE 4096
#define TOTAL_FRAMES 1048576
#define FRAME_BITMAP_WORDS (TOTAL_FRAMES / 64)

void      pmm_init(void);
sigma_u64 pmm_alloc_frame(void);
void      pmm_free_frame(sigma_u64 phys);

#endif
