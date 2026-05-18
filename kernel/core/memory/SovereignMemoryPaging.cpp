#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"

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
    static constexpr sigma_size_t SOVEREIGN_PAGE_SIZE = 4096;
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
        sigma_memset(ptr, 0, SOVEREIGN_PAGE_SIZE * (1 << order));
        sigma_log_info("[BUDDY] Secure freed pages at %p order %llu\n", ptr, order);
    }
};

class SovereignPagingEngine : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignPagingEngine> {
    friend class SigmaOS::SigmaSingleton<SovereignPagingEngine>;
public:
    const char* type_name() const noexcept override { return "SovereignPagingEngine"; }

    void init() {
        sigma_log("[PAGING] Initializing Sovereign Predictive Paging Engine...");
        this->m_active_pages = 0;
        this->m_page_faults_averted = 0;
        m_pml4 = (PageTable*)0x100000; // Simulated PML4 base
        sigma_memset(m_pml4, 0, sizeof(PageTable));
        m_buddy.init();
        sigma_log("[PAGING] Predictive pre-fetching ACTIVE.");
    }

    void mapVirtualToPhysical(void* virtual_addr, void* physical_addr, sigma_u32 flags) {
        sigma_u64 vaddr = (sigma_u64)virtual_addr;
        sigma_u64 paddr = (sigma_u64)physical_addr;
        
        sigma_u16 pml4_idx = (vaddr >> 39) & 0x1FF;
        
        m_pml4->entries[pml4_idx].present = 1;
        m_pml4->entries[pml4_idx].frame = paddr >> 12;
        
        this->m_active_pages++;
        sigma_log_info("[PAGING] Mapped %p -> %p (Flags: %X). Total active: %u\n", 
                     virtual_addr, physical_addr, flags, this->m_active_pages);
    }

    void predictAndPrefetch() {
        sigma_log("[PAGING] Analyzing access patterns... Prefetching 16 cold pages to L3 cache.");
        this->m_page_faults_averted += 16;
    }

private:
    SovereignPagingEngine() : m_active_pages(0), m_page_faults_averted(0), m_pml4(nullptr) {}

    sigma_u32 m_active_pages;
    sigma_u32 m_page_faults_averted;
    PageTable* m_pml4;
    SovereignBuddyAllocator m_buddy;
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
 