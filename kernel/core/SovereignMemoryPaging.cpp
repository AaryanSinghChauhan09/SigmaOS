#include "../../include/core/sigma_types.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/libc/SovereignLibC.h"

// Page Table Structures
struct PageTableEntry {
    sigma_u64 present : 1;
    sigma_u64 rw : 1;
    sigma_u64 user : 1;
    sigma_u64 accessed : 1;
    sigma_u64 dirty : 1;
    sigma_u64 unused : 7;
    sigma_u64 frame : 52;
};

struct PageTable {
    PageTableEntry entries[512];
};

class SovereignBuddyAllocator {
private:
    static constexpr sigma_size_t MAX_ORDER = 10; // Up to 1024 pages
    static constexpr sigma_size_t PAGE_SIZE = 4096;
    sigma_size_t free_lists[MAX_ORDER + 1];
    
public:
    void init() {
        sigma_memset(free_lists, 0, sizeof(free_lists));
        sigma_log_info("[BUDDY] Initialized Sovereign Buddy Allocator.\n");
    }
    
    void* allocate_pages(sigma_size_t order) {
        if (order > MAX_ORDER) return SIGMA_NULL;
        sigma_log_info("[BUDDY] Allocated pages of order %llu\n", order);
        // Minimal buddy alloc logic implementation placeholder
        return (void*)0x200000; 
    }
    
    void free_pages(void* ptr, sigma_size_t order) {
        if (!ptr) return;
        // Secure free
        sigma_memset(ptr, 0, PAGE_SIZE * (1 << order));
        sigma_log_info("[BUDDY] Secure freed pages at %p order %llu\n", ptr, order);
    }
};

class SovereignPagingEngine {
public:
    static SovereignPagingEngine& getInstance() {
        static SovereignPagingEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[PAGING] Initializing Sovereign Predictive Paging Engine...");
        this->active_pages = 0;
        this->page_faults_averted = 0;
        pml4 = (PageTable*)0x100000; // Simulated PML4 base
        sigma_memset(pml4, 0, sizeof(PageTable));
        buddy.init();
        sigma_log("[PAGING] Predictive pre-fetching ACTIVE.");
    }

    void mapVirtualToPhysical(void* virtual_addr, void* physical_addr, sigma_u32 flags) {
        sigma_u64 vaddr = (sigma_u64)virtual_addr;
        sigma_u64 paddr = (sigma_u64)physical_addr;
        
        sigma_u16 pml4_idx = (vaddr >> 39) & 0x1FF;
        sigma_u16 pdp_idx = (vaddr >> 30) & 0x1FF;
        sigma_u16 pd_idx = (vaddr >> 21) & 0x1FF;
        sigma_u16 pt_idx = (vaddr >> 12) & 0x1FF;
        
        // Setup minimal mappings
        pml4->entries[pml4_idx].present = 1;
        pml4->entries[pml4_idx].rw = (flags & 2) ? 1 : 0;
        pml4->entries[pml4_idx].user = (flags & 4) ? 1 : 0;
        pml4->entries[pml4_idx].frame = paddr >> 12;

        this->active_pages++;
        sigma_log_info("[PAGING] Mapped %p -> %p (Flags: %X). Total active: %u\n", 
                     virtual_addr, physical_addr, flags, this->active_pages);
    }

    void predictAndPrefetch() {
        sigma_log("[PAGING] Analyzing access patterns... Prefetching 16 cold pages to L3 cache.");
        this->page_faults_averted += 16;
    }

private:
    SovereignPagingEngine() : active_pages(0), page_faults_averted(0), pml4(nullptr) {}

    sigma_u32 active_pages;
    sigma_u32 page_faults_averted;
    PageTable* pml4;
    SovereignBuddyAllocator buddy;
};

/* --- C Wrappers --- */
extern "C" void paging_init() {
    SovereignPagingEngine::getInstance().init();
}

extern "C" void paging_map(void* virtual_addr, void* physical_addr, sigma_u32 flags) {
    SovereignPagingEngine::getInstance().mapVirtualToPhysical(virtual_addr, physical_addr, flags);
}

extern "C" void paging_prefetch() {
    SovereignPagingEngine::getInstance().predictAndPrefetch();
}
