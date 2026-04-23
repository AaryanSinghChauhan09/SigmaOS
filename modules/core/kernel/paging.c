#include <stdint.h>

// ---------------------------------------------------------
// SigmaOS Memory Paging System Prototype (x86_64 inspired)
// ---------------------------------------------------------

#define PAGE_SIZE 4096
#define PAGE_ENTRIES 512

// Page Table Entry Flags
#define PTE_PRESENT  0x01
#define PTE_WRITABLE 0x02
#define PTE_USER     0x04

typedef uint64_t pte_t;

// A standard 4-level paging structure (PML4, PDPT, PD, PT)
typedef struct {
    pte_t entries[PAGE_ENTRIES];
} page_table_t;

// Example pointer to the current root page table (CR3 in x86)
static page_table_t* current_pml4 = 0;

// Internal function to allocate a physical page for page tables
static void* alloc_physical_page() {
    // In reality, this would call the physical memory manager (bitmap/buddy allocator)
    return (void*)0x100000; // Mock address
}

// Maps a virtual address to a physical address in the current PML4
int map_virtual_page(void* virtual_addr, void* physical_addr, uint64_t flags) {
    if (!current_pml4) return -1;

    uint64_t vaddr = (uint64_t)virtual_addr;
    
    // Extract indices (x86_64 format: 9 bits each)
    uint64_t pml4_index = (vaddr >> 39) & 0x1FF;
    uint64_t pdpt_index = (vaddr >> 30) & 0x1FF;
    uint64_t pd_index   = (vaddr >> 21) & 0x1FF;
    uint64_t pt_index   = (vaddr >> 12) & 0x1FF;

    // Traverse or allocate PML4
    if (!(current_pml4->entries[pml4_index] & PTE_PRESENT)) {
        current_pml4->entries[pml4_index] = (uint64_t)alloc_physical_page() | PTE_PRESENT | PTE_WRITABLE | PTE_USER;
    }
    page_table_t* pdpt = (page_table_t*)(current_pml4->entries[pml4_index] & ~0xFFF);

    // Traverse or allocate PDPT
    if (!(pdpt->entries[pdpt_index] & PTE_PRESENT)) {
        pdpt->entries[pdpt_index] = (uint64_t)alloc_physical_page() | PTE_PRESENT | PTE_WRITABLE | PTE_USER;
    }
    page_table_t* pd = (page_table_t*)(pdpt->entries[pdpt_index] & ~0xFFF);

    // Traverse or allocate PD
    if (!(pd->entries[pd_index] & PTE_PRESENT)) {
        pd->entries[pd_index] = (uint64_t)alloc_physical_page() | PTE_PRESENT | PTE_WRITABLE | PTE_USER;
    }
    page_table_t* pt = (page_table_t*)(pd->entries[pd_index] & ~0xFFF);

    // Map the actual Physical Address into the PT
    pt->entries[pt_index] = ((uint64_t)physical_addr & ~0xFFF) | flags | PTE_PRESENT;

    return 0; // Success
}
