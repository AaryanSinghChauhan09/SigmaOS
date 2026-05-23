/*
 * Σ SigmaOS — sigma_allocator: Sovereign Buddy Memory Allocator
 * Zero-Dependency: No libc (no malloc/free).
 * Implements a buddy system allocator for physical and virtual memory.
 */

typedef unsigned long long u64;
typedef unsigned int       u32;
typedef unsigned short     u16;
typedef unsigned char      u8;

#define PAGE_SIZE 4096
#define MAX_ORDER 11  /* 2^11 pages = 8MB max contiguous allocation */

/* Sovereign Free List Node */
struct SigmaFreeNode {
    SigmaFreeNode* next;
    SigmaFreeNode* prev;
};

/* Allocator State */
struct SigmaAllocator {
    SigmaFreeNode* free_lists[MAX_ORDER + 1];
    u64            total_pages;
    u64            free_pages;
    void*          base_address;
};

static SigmaAllocator kernel_allocator;

/* Internal helpers */
static void list_init(SigmaFreeNode* list) {
    list->next = list;
    list->prev = list;
}

static void list_add(SigmaFreeNode* node, SigmaFreeNode* list) {
    node->next = list->next;
    node->prev = list;
    list->next->prev = node;
    list->next = node;
}

static void list_remove(SigmaFreeNode* node) {
    node->prev->next = node->next;
    node->next->prev = node->prev;
}

/* 
 * Initialize the sovereign allocator with a raw memory region 
 * (typically called by the bootloader after mapping physical memory).
 */
extern "C" void sigma_allocator_init(void* memory_base, u64 size) {
    kernel_allocator.base_address = memory_base;
    kernel_allocator.total_pages = size / PAGE_SIZE;
    kernel_allocator.free_pages = kernel_allocator.total_pages;

    for (int i = 0; i <= MAX_ORDER; i++) {
        kernel_allocator.free_lists[i] = 0; /* Will point to embedded dummy nodes */
    }

    /* 
     * In a full implementation, we would seed the free lists with the 
     * available memory blocks divided into max-order chunks. 
     */
}

/* 
 * Allocate memory (Sovereign malloc)
 * Returns pointer to allocated memory or 0 on failure.
 */
extern "C" void* sigma_malloc(u64 size) {
    if (size == 0) return 0;
    
    /* Calculate order required */
    u64 pages_needed = (size + PAGE_SIZE - 1) / PAGE_SIZE;
    int order = 0;
    while ((1ULL << order) < pages_needed) {
        order++;
    }

    if (order > MAX_ORDER) return 0; /* Request too large */

    /* Find lowest available order */
    int current_order = order;
    while (current_order <= MAX_ORDER && kernel_allocator.free_lists[current_order] == 0) {
        current_order++;
    }

    if (current_order > MAX_ORDER) return 0; /* OOM */

    /* We have found a block. Now split it down to the requested order. */
    /* Stubbed logic: actual implementation pops node and splits buddies */
    
    /* Dummy return for stub */
    void* allocated_ptr = (void*)((u64)kernel_allocator.base_address + 0x1000); 
    return allocated_ptr;
}

/* 
 * Free memory (Sovereign free)
 */
extern "C" void sigma_free(void* ptr, u64 size) {
    if (!ptr) return;

    /* Stubbed logic: compute order, add to free list, merge with buddies */
}
