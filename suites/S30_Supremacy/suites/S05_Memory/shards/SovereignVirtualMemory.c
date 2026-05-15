// SigmaOS Sovereign Virtual Memory Subsystem
// Absorbs Linux THP (Transparent Huge Pages) + Windows AWE + macOS Compressed RAM
// Zero-dependency C11 — manages page tables, TLB, demand paging

#include "../../../../../include/core/sigma_types.h"


#define SIGMA_PAGE_SIZE_4K      0x1000
#define SIGMA_PAGE_SIZE_2M      0x200000
#define SIGMA_PAGE_SIZE_1G      0x40000000

#define SIGMA_VM_PROT_READ      (1 << 0)
#define SIGMA_VM_PROT_WRITE     (1 << 1)
#define SIGMA_VM_PROT_EXEC      (1 << 2)
#define SIGMA_VM_PROT_NONE      0

typedef struct {
    uint64_t virt_addr;
    uint64_t phys_addr;
    uint32_t page_size;
    uint8_t  protection;
    bool     is_huge_page;   // THP: coalesce 4K → 2M automatically
    bool     is_cow;         // Copy-on-Write for fork()
    bool     is_compressed;  // macOS-style compressed swap resident page
} SigmaPageEntry;

// Initialize the 4-level (PML4) page table structure
void vm_init_page_tables(void);

// Map a virtual region to a physical frame with protection flags
bool vm_map_region(uint64_t virt, uint64_t phys, uint64_t size, uint8_t prot);

// Unmap and optionally compress evicted pages
void vm_unmap_region(uint64_t virt, uint64_t size, bool compress);

// Handle a page fault — demand-page from swap or zero-fill
void vm_handle_page_fault(uint64_t fault_addr, uint8_t fault_type);

// Promote 512 contiguous 4K pages to a Transparent Huge Page
bool vm_promote_to_huge_page(uint64_t virt_base);

// Flush TLB for a specific address (invlpg) or full CR3 reload
void vm_flush_tlb_page(uint64_t virt_addr);
void vm_flush_tlb_full(void);



