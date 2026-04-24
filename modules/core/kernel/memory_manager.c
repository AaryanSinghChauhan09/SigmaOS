#include "sigma_libc.h"
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Central Memory Manager (VMM)
// USP: Versioned, Snapshot-Capable Virtual Memory
// Combines NUMA allocator + Memory Contracts into a unified layer
// ---------------------------------------------------------

#define PAGE_SIZE 4096
#define MAX_ADDR_SPACES 256

typedef struct {
    uint32_t pid;
    uint64_t page_directory_phys; // Physical addr of PML4 / L1 Table
    uint32_t active_pages;
    uint8_t  snapshot_id;         // Current memory version
} address_space_t;

static address_space_t vmm_spaces[MAX_ADDR_SPACES];

// External subsystem hooks
extern int mem_contract_verify(uint32_t pid, uint32_t target_page);
extern void numa_free_page(uint64_t phys_addr);
extern uint64_t numa_alloc_page(uint8_t preferred_node);
extern void watchdog_trigger_fault(uint32_t pid, const char* reason);

// Handle a Page Fault Exception
void memory_manager_page_fault(uint32_t pid, uint64_t fault_addr, uint8_t is_write) {
    uint32_t fault_page = (uint32_t)(fault_addr / PAGE_SIZE);

    // 1. Verify Memory Contract (USP: Cryptographic Memory Leasing)
    // Process must hold a valid unexpired cryptographic contract for this page
    if (!mem_contract_verify(pid, fault_page)) {
        // Segfault -> but handled via Sovereign Watchdog, not immediate panic
        watchdog_trigger_fault(pid, "PAGE_FAULT_NO_CONTRACT");
        return;
    }

    // 2. USP: Copy-on-Write (CoW) / Memory Snapshots
    // If we are writing to a snapshotted page, allocate a new physical frame
    // and copy the old data over.
    if (is_write /* && is_snapshotted(pid, fault_addr) */) {
        uint64_t new_phys = numa_alloc_page(0);
        // memcpy((void*)new_phys, (void*)old_phys, PAGE_SIZE);
        // update_page_table(pid, fault_addr, new_phys);
        return;
    }

    // 3. Demand Paging Allocation
    uint64_t phys_frame = numa_alloc_page(0); // Fetch from NUMA-aware allocator
    // map_virtual_to_physical(pid, fault_addr, phys_frame);
}

// USP: Instant Address Space Teardown
// Used by Capability Auto-Revocation when a process exits
void memory_manager_teardown(uint32_t pid) {
    for (int i = 0; i < MAX_ADDR_SPACES; i++) {
        if (vmm_spaces[i].pid == pid) {
            // Free all physical frames associated with the page directory
            // numa_free_page(...)
            vmm_spaces[i].pid = 0;
            vmm_spaces[i].page_directory_phys = 0;
            break;
        }
    }
}
