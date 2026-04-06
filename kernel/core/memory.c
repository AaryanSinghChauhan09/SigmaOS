#include "../../libc/SovereignLibC.h"

#define PAGE_SIZE 4096
#define MAX_PAGES 1024

static sigma_u8  g_memory_pool[MAX_PAGES * PAGE_SIZE];
static sigma_u8  g_page_bitmap[MAX_PAGES / 8];

void SovereignMemory_Init() {
    sigma_printf("Σ [INIT]: Sovereign Slab-Based Physical Page Allocator Online.\n");
    sigma_memset(g_page_bitmap, 0, sizeof(g_page_bitmap));
}

void* sigma_alloc_page() {
    for(int i=0; i<MAX_PAGES; i++) {
        int byte = i / 8;
        int bit = i % 8;
        if(!(g_page_bitmap[byte] & (1 << bit))) {
            g_page_bitmap[byte] |= (1 << bit);
            return (void*)&g_memory_pool[i * PAGE_SIZE];
        }
    }
    return SIGMA_NULL;
}


