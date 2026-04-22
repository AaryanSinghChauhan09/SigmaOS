#ifndef SIGMA_PMM_H
#define SIGMA_PMM_H

#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * SIGMA OS: PHYSICAL MEMORY MANAGER (SYSTEM-LEVEL HEADER)
 * ========================================================================= */

#define SIGMA_PAGE_SIZE 4096
#define SIGMA_BLOCKS_PER_BYTE 8

void sigma_pmm_init(size_t mem_size, void* bitmap_addr);
void sigma_pmm_mark_used(size_t frame);
void sigma_pmm_mark_sigma_free(size_t frame);
void* sigma_pmm_allocate_block();
void sigma_pmm_free_block(void* ptr);
size_t sigma_pmm_get_free_memory();

#endif
