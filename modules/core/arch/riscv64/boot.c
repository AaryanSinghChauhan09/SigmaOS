#include "../../../../include/libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS RISC-V Sv39 Virtual Memory Management (VMM)
// ---------------------------------------------------------

#define SATP_SV39 (8ULL << 60)
#define PAGE_SIZE 4096

typedef uint64_t pte_t;

void arch_riscv64_vmm_init(uint64_t root_page_table) {
    // Setup SATP (Supervisor Address Translation and Protection) register
    uint64_t satp = SATP_SV39 | (root_page_table >> 12);
    asm volatile("csrw satp, %0" : : "r"(satp));
    asm volatile("sfence.vma");
}

pte_t* walk_page_table(pte_t* root, uint64_t va, int alloc) {
    for (int level = 2; level > 0; level--) {
        pte_t* pte = &root[(va >> (12 + level * 9)) & 0x1FF];
        if (*pte & 1) { // Valid bit
            root = (pte_t*)((*pte >> 10) << 12);
        } else {
            if (!alloc) return 0;
            // Allocate new page table (Stub)
            return 0;
        }
    }
    return &root[(va >> 12) & 0x1FF];
}

void arch_riscv64_init() {
    // Supervisor Mode and Sv39 setup
}
