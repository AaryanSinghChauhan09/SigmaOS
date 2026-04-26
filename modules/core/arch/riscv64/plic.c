#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS RISC-V PLIC (Platform-Level Interrupt Controller)
// ---------------------------------------------------------

#define PLIC_PRIORITY_BASE 0x0c000000
#define PLIC_PENDING_BASE  0x0c001000
#define PLIC_ENABLE_BASE   0x0c002000
#define PLIC_CONTEXT_BASE  0x0c200000

void plic_set_priority(uint32_t irq, uint32_t priority) {
    uint32_t* addr = (uint32_t*)(PLIC_PRIORITY_BASE + irq * 4);
    *addr = priority;
}

void plic_enable(uint32_t context, uint32_t irq) {
    uint32_t* addr = (uint32_t*)(PLIC_ENABLE_BASE + context * 0x80 + (irq / 32) * 4);
    *addr |= (1 << (irq % 32));
}

uint32_t plic_claim(uint32_t context) {
    uint32_t* addr = (uint32_t*)(PLIC_CONTEXT_BASE + context * 0x1000 + 4);
    return *addr;
}

void plic_complete(uint32_t context, uint32_t irq) {
    uint32_t* addr = (uint32_t*)(PLIC_CONTEXT_BASE + context * 0x1000 + 4);
    *addr = irq;
}

void arch_riscv64_plic_init() {
    // Initial PLIC setup for sovereign context
    for(uint32_t i = 1; i < 32; i++) {
        plic_set_priority(i, 1);
        plic_enable(0, i); // Enable for Context 0 (Machine Mode / Supervisor)
    }
}
