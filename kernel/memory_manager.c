/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Memory Manager (Native Core)
 * =====================================
 * Complete virtual memory management system
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

// Memory page size (4KB standard)
#define PAGE_SIZE 4096
#define PAGE_ALIGN(addr) (((addr) + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1))

// Memory regions
#define KERNEL_BASE 0xFFFFFFFF80000000
#define KERNEL_SIZE (256 * 1024 * 1024)  // 256MB kernel space
#define USER_BASE   0x0000000000400000
#define USER_SIZE   (128 * 1024 * 1024)  // 128MB per process

// Page table entries
typedef struct {
    uint64_t present    : 1;
    uint64_t writable   : 1;
    uint64_t user       : 1;
    uint64_t writethrough : 1;
    uint64_t nocache    : 1;
    uint64_t accessed   : 1;
    uint64_t dirty      : 1;
    uint64_t pat        : 1;
    uint64_t global     : 1;
    uint64_t available  : 3;
    uint64_t frame      : 40;
    uint64_t reserved   : 12;
} __attribute__((packed)) page_entry_t;

// Memory management structures
typedef struct {
    page_entry_t *pml4;
    uint64_t start_addr;
    uint64_t end_addr;
    size_t used_pages;
    size_t total_pages;
} memory_space_t;

// Physical memory manager
typedef struct {
    uint64_t base_addr;
    size_t total_pages;
    size_t used_pages;
    uint8_t *bitmap;
} physical_memory_t;

static physical_memory_t phys_mem;
static memory_space_t kernel_space;

// Initialize physical memory manager
void sigma_mem_init_physical(uint64_t base_addr, size_t total_memory) {
    phys_mem.base_addr = base_addr;
    phys_mem.total_pages = total_memory / PAGE_SIZE;
    phys_mem.used_pages = 0;
    
    // Allocate bitmap for page tracking
    size_t bitmap_size = (phys_mem.total_pages + 7) / 8;
    phys_mem.bitmap = (uint8_t*)base_addr;
    
    // Mark all pages as free initially
    for (size_t i = 0; i < bitmap_size; i++) {
        phys_mem.bitmap[i] = 0;
    }
}

// Allocate a physical page
uint64_t sigma_mem_alloc_page(void) {
    for (size_t i = 0; i < phys_mem.total_pages; i++) {
        size_t byte_index = i / 8;
        size_t bit_index = i % 8;
        
        if (!(phys_mem.bitmap[byte_index] & (1 << bit_index))) {
            phys_mem.bitmap[byte_index] |= (1 << bit_index);
            phys_mem.used_pages++;
            return phys_mem.base_addr + (i * PAGE_SIZE);
        }
    }
    return 0; // Out of memory
}

// Free a physical page
void sigma_mem_free_page(uint64_t page_addr) {
    uint64_t page_index = (page_addr - phys_mem.base_addr) / PAGE_SIZE;
    size_t byte_index = page_index / 8;
    size_t bit_index = page_index % 8;
    
    phys_mem.bitmap[byte_index] &= ~(1 << bit_index);
    phys_mem.used_pages--;
}

// Initialize kernel memory space
void sigma_mem_init_kernel(void) {
    // Allocate page for PML4
    kernel_space.pml4 = (page_entry_t*)sigma_mem_alloc_page();
    kernel_space.start_addr = KERNEL_BASE;
    kernel_space.end_addr = KERNEL_BASE + KERNEL_SIZE;
    kernel_space.used_pages = 0;
    kernel_space.total_pages = KERNEL_SIZE / PAGE_SIZE;
    
    // Initialize PML4 entries
    for (int i = 0; i < 512; i++) {
        kernel_space.pml4[i].present = 0;
        kernel_space.pml4[i].writable = 0;
        kernel_space.pml4[i].user = 0;
    }
}

// Map virtual page to physical page
bool sigma_mem_map_page(memory_space_t *space, uint64_t virt_addr, uint64_t phys_addr, bool user, bool writable) {
    // Extract page table indices
    uint64_t pml4_index = (virt_addr >> 39) & 0x1FF;
    uint64_t pdpt_index = (virt_addr >> 30) & 0x1FF;
    uint64_t pd_index   = (virt_addr >> 21) & 0x1FF;
    uint64_t pt_index   = (virt_addr >> 12) & 0x1FF;
    
    // Ensure PML4 entry exists
    if (!space->pml4[pml4_index].present) {
        uint64_t pdpt_addr = sigma_mem_alloc_page();
        if (!pdpt_addr) return false;
        
        space->pml4[pml4_index].present = 1;
        space->pml4[pml4_index].writable = 1;
        space->pml4[pml4_index].user = user;
        space->pml4[pml4_index].frame = pdpt_addr >> 12;
        
        // Clear new page table
        page_entry_t *pdpt = (page_entry_t*)pdpt_addr;
        for (int i = 0; i < 512; i++) {
            pdpt[i].present = 0;
        }
    }
    
    page_entry_t *pdpt = (page_entry_t*)(space->pml4[pml4_index].frame << 12);
    
    // Ensure PDPT entry exists
    if (!pdpt[pdpt_index].present) {
        uint64_t pd_addr = sigma_mem_alloc_page();
        if (!pd_addr) return false;
        
        pdpt[pdpt_index].present = 1;
        pdpt[pdpt_index].writable = 1;
        pdpt[pdpt_index].user = user;
        pdpt[pdpt_index].frame = pd_addr >> 12;
        
        // Clear new page table
        page_entry_t *pd = (page_entry_t*)pd_addr;
        for (int i = 0; i < 512; i++) {
            pd[i].present = 0;
        }
    }
    
    page_entry_t *pd = (page_entry_t*)(pdpt[pdpt_index].frame << 12);
    
    // Ensure PD entry exists
    if (!pd[pd_index].present) {
        uint64_t pt_addr = sigma_mem_alloc_page();
        if (!pt_addr) return false;
        
        pd[pd_index].present = 1;
        pd[pd_index].writable = 1;
        pd[pd_index].user = user;
        pd[pd_index].frame = pt_addr >> 12;
        
        // Clear new page table
        page_entry_t *pt = (page_entry_t*)pt_addr;
        for (int i = 0; i < 512; i++) {
            pt[i].present = 0;
        }
    }
    
    page_entry_t *pt = (page_entry_t*)(pd[pd_index].frame << 12);
    
    // Map the page
    pt[pt_index].present = 1;
    pt[pt_index].writable = writable;
    pt[pt_index].user = user;
    pt[pt_index].frame = phys_addr >> 12;
    
    space->used_pages++;
    return true;
}

// Unmap virtual page
void sigma_mem_unmap_page(memory_space_t *space, uint64_t virt_addr) {
    uint64_t pml4_index = (virt_addr >> 39) & 0x1FF;
    uint64_t pdpt_index = (virt_addr >> 30) & 0x1FF;
    uint64_t pd_index   = (virt_addr >> 21) & 0x1FF;
    uint64_t pt_index   = (virt_addr >> 12) & 0x1FF;
    
    if (!space->pml4[pml4_index].present) return;
    
    page_entry_t *pdpt = (page_entry_t*)(space->pml4[pml4_index].frame << 12);
    if (!pdpt[pdpt_index].present) return;
    
    page_entry_t *pd = (page_entry_t*)(pdpt[pdpt_index].frame << 12);
    if (!pd[pd_index].present) return;
    
    page_entry_t *pt = (page_entry_t*)(pd[pd_index].frame << 12);
    if (!pt[pt_index].present) return;
    
    // Free physical page
    sigma_mem_free_page(pt[pt_index].frame << 12);
    
    // Clear page table entry
    pt[pt_index].present = 0;
    space->used_pages--;
}

// Create new memory space for process
memory_space_t* sigma_mem_create_space(void) {
    memory_space_t *space = (memory_space_t*)sigma_mem_alloc_page();
    if (!space) return NULL;
    
    space->pml4 = (page_entry_t*)sigma_mem_alloc_page();
    if (!space->pml4) {
        sigma_mem_free_page((uint64_t)space);
        return NULL;
    }
    
    space->start_addr = USER_BASE;
    space->end_addr = USER_BASE + USER_SIZE;
    space->used_pages = 0;
    space->total_pages = USER_SIZE / PAGE_SIZE;
    
    // Copy kernel mappings
    for (int i = 256; i < 512; i++) {
        space->pml4[i] = kernel_space.pml4[i];
    }
    
    return space;
}

// Destroy memory space
void sigma_mem_destroy_space(memory_space_t *space) {
    // Unmap all user pages
    for (uint64_t addr = space->start_addr; addr < space->end_addr; addr += PAGE_SIZE) {
        sigma_mem_unmap_page(space, addr);
    }
    
    // Free page tables
    sigma_mem_free_page((uint64_t)space->pml4);
    sigma_mem_free_page((uint64_t)space);
}

// Memory pressure detection
bool sigma_mem_is_under_pressure(void) {
    return (phys_mem.used_pages * 100 / phys_mem.total_pages) > 90;
}

// OOM killer trigger
void sigma_mem_trigger_oom(void) {
    // Simple OOM killer - kill the process with most memory usage
    // This is a placeholder - real implementation would track process memory
    extern void sigma_scheduler_kill_largest_process(void);
    sigma_scheduler_kill_largest_process();
}

// Initialize complete memory management system
void sigma_mem_init(uint64_t total_memory) {
    sigma_mem_init_physical(0x1000000, total_memory); // Start at 16MB
    sigma_mem_init_kernel();
}

// Get memory statistics
typedef struct {
    size_t total_memory;
    size_t used_memory;
    size_t free_memory;
    size_t kernel_memory;
    size_t user_memory;
} mem_stats_t;

void sigma_mem_get_stats(mem_stats_t *stats) {
    stats->total_memory = phys_mem.total_pages * PAGE_SIZE;
    stats->used_memory = phys_mem.used_pages * PAGE_SIZE;
    stats->free_memory = stats->total_memory - stats->used_memory;
    stats->kernel_memory = kernel_space.used_pages * PAGE_SIZE;
    stats->user_memory = stats->used_memory - stats->kernel_memory;
}

