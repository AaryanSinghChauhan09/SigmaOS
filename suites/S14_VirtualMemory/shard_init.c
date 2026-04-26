#include "sigma_libc.h"

// SigmaOS Sovereign VMM (Virtual Memory Manager)
// Architecture: AArch64 (ARMv8-A) 4-Level Paging (L0 -> L1 -> L2 -> L3)
// 4KB Page Granularity, 48-bit Virtual Address Space.

#define PAGE_SIZE      4096
#define TABLE_ENTRIES  512
#define ATTR_AF        (1ULL << 10)
#define ATTR_SH_INNER  (3ULL << 8)
#define ATTR_MEM_ATTR  (0ULL << 2) // Normal memory
#define ATTR_VALID     (1ULL << 0)
#define ATTR_TABLE     (1ULL << 1)

typedef uint64_t pte_t;

// L0 Translation Table (Translation Control Base)
static pte_t* l0_table;

void vmm_init() {
    sigma_printf("[VMM] Initializing AArch64 4-Level Paging Shard...\n");
    
    // Allocate 4KB for L0 table (Must be 4KB aligned)
    l0_table = (pte_t*)sigma_slab_alloc(PAGE_SIZE);
    sigma_memset(l0_table, 0, PAGE_SIZE);
    
    sigma_printf("[VMM] L0 Table created at %p\n", l0_table);
}

// Map a 4KB virtual page to a physical address
void vmm_map_page(uint64_t vaddr, uint64_t paddr, uint64_t flags) {
    uint32_t l0_idx = (vaddr >> 39) & 0x1FF;
    uint32_t l1_idx = (vaddr >> 30) & 0x1FF;
    uint32_t l2_idx = (vaddr >> 21) & 0x1FF;
    uint32_t l3_idx = (vaddr >> 12) & 0x1FF;

    // L0 -> L1
    if (!(l0_table[l0_idx] & ATTR_VALID)) {
        pte_t* l1 = (pte_t*)sigma_slab_alloc(PAGE_SIZE);
        sigma_memset(l1, 0, PAGE_SIZE);
        l0_table[l0_idx] = (uint64_t)l1 | ATTR_TABLE | ATTR_VALID;
    }
    
    pte_t* l1_table = (pte_t*)(l0_table[l0_idx] & ~0xFFF);
    
    // L1 -> L2
    if (!(l1_table[l1_idx] & ATTR_VALID)) {
        pte_t* l2 = (pte_t*)sigma_slab_alloc(PAGE_SIZE);
        sigma_memset(l2, 0, PAGE_SIZE);
        l1_table[l1_idx] = (uint64_t)l2 | ATTR_TABLE | ATTR_VALID;
    }

    pte_t* l2_table = (pte_t*)(l1_table[l1_idx] & ~0xFFF);

    // L2 -> L3 (Leaf Page)
    if (!(l2_table[l2_idx] & ATTR_VALID)) {
        pte_t* l3 = (pte_t*)sigma_slab_alloc(PAGE_SIZE);
        sigma_memset(l3, 0, PAGE_SIZE);
        l2_table[l2_idx] = (uint64_t)l3 | ATTR_TABLE | ATTR_VALID;
    }

    pte_t* l3_table = (pte_t*)(l2_table[l2_idx] & ~0xFFF);
    
    // Final L3 mapping to physical page
    l3_table[l3_idx] = paddr | ATTR_AF | ATTR_SH_INNER | ATTR_MEM_ATTR | ATTR_VALID | (1ULL << 1); // 1<<1 for L3 leaf
    
    sigma_printf("[VMM] Mapped Virtual %p -> Physical %p\n", vaddr, paddr);
}

void shard_init() {
    sigma_shard_init();
    vmm_init();
    
    // Test mapping: Map virtual 0xFFFF0000 to physical 0x80000000
    vmm_map_page(0xFFFF0000, 0x80000000, 0);
}
