/*
 * Σ SigmaOS — sigma_allocator: Sovereign Buddy Memory Allocator
 * Zero-Dependency: No libc (no malloc/free).
 * Implements a buddy system allocator for physical and virtual memory.
 */

typedef unsigned long long u64;
typedef unsigned int       u32;
typedef unsigned short     u16;
typedef unsigned char      u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

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

static bool list_is_empty(SigmaFreeNode* list) {
    return list->next == list;
}

/* 
 * Initialize the sovereign allocator with a raw memory region 
 * (typically called by the bootloader after mapping physical memory).
 */
extern "C" void sigma_allocator_init(void* memory_base, u64 size) {
    kernel_allocator.base_address = memory_base;
    kernel_allocator.total_pages = size / PAGE_SIZE;
    kernel_allocator.free_pages = kernel_allocator.total_pages;

    // Allocate dummy nodes for list heads (in a real system, these might be static arrays)
    // Here we use the first few bytes of memory_base for list heads
    SigmaFreeNode* heads = (SigmaFreeNode*)memory_base;
    for (int i = 0; i <= MAX_ORDER; i++) {
        kernel_allocator.free_lists[i] = &heads[i];
        list_init(kernel_allocator.free_lists[i]);
    }

    // Usable memory starts after the heads
    u64 header_size = (MAX_ORDER + 1) * sizeof(SigmaFreeNode);
    // Align to page boundary
    header_size = (header_size + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1);
    
    u8* usable_mem = (u8*)memory_base + header_size;
    u64 usable_size = size - header_size;
    
    // Add all usable memory to the largest possible orders
    u64 remaining = usable_size;
    u8* current = usable_mem;
    
    while (remaining >= PAGE_SIZE) {
        int order = MAX_ORDER;
        while ((1ULL << order) * PAGE_SIZE > remaining && order > 0) {
            order--;
        }
        
        SigmaFreeNode* block = (SigmaFreeNode*)current;
        list_add(block, kernel_allocator.free_lists[order]);
        
        u64 block_size = (1ULL << order) * PAGE_SIZE;
        current += block_size;
        remaining -= block_size;
    }
}

/* 
 * Allocate memory (Sovereign malloc)
 * Returns pointer to allocated memory or 0 on failure.
 */
struct AllocHeader {
    u64 magic;
    u64 size;
};

#define ALLOC_MAGIC 0x5163A05163A0ULL
#define FREE_MAGIC  0xFBEEFBEEULL

/* 
 * Allocate memory (Sovereign malloc)
 * Returns pointer to allocated memory or 0 on failure.
 */
extern "C" void* sigma_malloc(u64 size) {
    if (size == 0) return 0;
    
    // Add space for size header to allow proper freeing
    size += sizeof(AllocHeader);
    
    /* Calculate order required */
    u64 pages_needed = (size + PAGE_SIZE - 1) / PAGE_SIZE;
    int order = 0;
    while ((1ULL << order) < pages_needed) {
        order++;
    }

    if (order > MAX_ORDER) return 0; /* Request too large */

    /* Find lowest available order */
    int current_order = order;
    while (current_order <= MAX_ORDER && list_is_empty(kernel_allocator.free_lists[current_order])) {
        current_order++;
    }

    if (current_order > MAX_ORDER) return 0; /* OOM */

    /* We have found a block. Now split it down to the requested order. */
    SigmaFreeNode* block = kernel_allocator.free_lists[current_order]->next;
    list_remove(block);

    while (current_order > order) {
        current_order--;
        u64 block_size = (1ULL << current_order) * PAGE_SIZE;
        SigmaFreeNode* buddy = (SigmaFreeNode*)((u8*)block + block_size);
        list_add(buddy, kernel_allocator.free_lists[current_order]);
    }
    
    kernel_allocator.free_pages -= (1ULL << order);
    
    // Store original requested size (including header) and magic for free()
    AllocHeader* hdr = (AllocHeader*)block;
    hdr->magic = ALLOC_MAGIC;
    hdr->size = size;
    
    // Return pointer after the header
    return (void*)((u8*)block + sizeof(AllocHeader));
}

/* 
 * Free memory (Sovereign free)
 */
extern "C" void sigma_free(void* ptr) {
    if (!ptr) return;

    // Retrieve original size from header
    AllocHeader* hdr = (AllocHeader*)((u8*)ptr - sizeof(AllocHeader));
    
    if (hdr->magic == FREE_MAGIC) {
        sigma_vga_printf("[Memory Warning] Double free detected at %p!\n", ptr);
        return;
    }
    if (hdr->magic != ALLOC_MAGIC) {
        sigma_vga_printf("[Memory Warning] Heap corruption or invalid free detected at %p!\n", ptr);
        return;
    }
    
    u64 size = hdr->size;
    hdr->magic = FREE_MAGIC; // Mark as free to prevent double-free
    
    u64 pages_freed = (size + PAGE_SIZE - 1) / PAGE_SIZE;
    int order = 0;
    while ((1ULL << order) < pages_freed) {
        order++;
    }
    
    kernel_allocator.free_pages += (1ULL << order);

    SigmaFreeNode* block = (SigmaFreeNode*)hdr;
    
    // Buddy merging
    while (order < MAX_ORDER) {
        u64 block_size = (1ULL << order) * PAGE_SIZE;
        u64 offset = (u64)block - (u64)kernel_allocator.base_address;
        
        // Find buddy address
        SigmaFreeNode* buddy = (SigmaFreeNode*)((u64)kernel_allocator.base_address + (offset ^ block_size));
        
        // Check if buddy is free (simplified check for bare metal stub)
        bool buddy_is_free = false;
        SigmaFreeNode* curr = kernel_allocator.free_lists[order]->next;
        while (curr != kernel_allocator.free_lists[order]) {
            if (curr == buddy) {
                buddy_is_free = true;
                break;
            }
            curr = curr->next;
        }
        
        if (!buddy_is_free) break;
        
        // Buddy is free, merge them
        list_remove(buddy);
        if (buddy < block) block = buddy; // Block starts at the lower address
        order++;
    }
    
    list_add(block, kernel_allocator.free_lists[order]);
}

/*
 * Allocate zero-initialized memory
 */
extern "C" void* sigma_calloc(u64 num, u64 size) {
    u64 total_size = num * size;
    void* ptr = sigma_malloc(total_size);
    if (ptr) {
        // Simple memset
        u8* p = (u8*)ptr;
        for (u64 i = 0; i < total_size; i++) {
            p[i] = 0;
        }
    }
    return ptr;
}

/*
 * Reallocate memory
 */
extern "C" void* sigma_realloc(void* ptr, u64 new_size) {
    if (!ptr) return sigma_malloc(new_size);
    if (new_size == 0) {
        sigma_free(ptr);
        return 0;
    }
    
    AllocHeader* hdr = (AllocHeader*)((u8*)ptr - sizeof(AllocHeader));
    if (hdr->magic != ALLOC_MAGIC) return 0;
    u64 old_size = hdr->size - sizeof(AllocHeader);
    
    if (new_size <= old_size) return ptr;
    
    void* new_ptr = sigma_malloc(new_size);
    if (new_ptr) {
        // Simple memcpy
        const u8* src = (const u8*)ptr;
        u8* dst = (u8*)new_ptr;
        for (u64 i = 0; i < old_size; i++) {
            dst[i] = src[i];
        }
        sigma_free(ptr);
    }
    return new_ptr;
}

/*
 * Print memory statistics
 */
extern "C" void sigma_mem_stats() {
    u64 used_pages = kernel_allocator.total_pages - kernel_allocator.free_pages;
    sigma_vga_printf("[Memory] Total: %llu MB, Free: %llu MB, Used: %llu MB\n", 
                     (kernel_allocator.total_pages * PAGE_SIZE) / (1024 * 1024),
                     (kernel_allocator.free_pages * PAGE_SIZE) / (1024 * 1024),
                     (used_pages * PAGE_SIZE) / (1024 * 1024));
    
    sigma_vga_printf("[Memory] Free Lists:\n");
    for (int i = 0; i <= MAX_ORDER; i++) {
        int count = 0;
        SigmaFreeNode* curr = kernel_allocator.free_lists[i]->next;
        while (curr != kernel_allocator.free_lists[i]) {
            count++;
            curr = curr->next;
        }
        if (count > 0) {
            sigma_vga_printf("  Order %d (%llu KB): %d blocks\n", 
                             i, ((1ULL << i) * PAGE_SIZE) / 1024, count);
        }
    }
}
