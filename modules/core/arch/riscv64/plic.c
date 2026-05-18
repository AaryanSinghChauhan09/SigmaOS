#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS RISC-V PLIC (Platform-Level Interrupt Controller)
// ---------------------------------------------------------

#define PLIC_PRIORITY_BASE 0x0c000000
#define PLIC_PENDING_BASE  0x0c001000
#define PLIC_ENABLE_BASE   0x0c002000
#define PLIC_CONTEXT_BASE  0x0c200000

void plic_set_priority(uint32_t irq, uint32_t priority) {
    uint32_t* addr = (uint32_t*)((uintptr_t)PLIC_PRIORITY_BASE + irq * 4);
    *addr = priority;
}

void plic_enable(uint32_t context, uint32_t irq) {
    uint32_t* addr = (uint32_t*)((uintptr_t)PLIC_ENABLE_BASE + context * 0x80 + (irq / 32) * 4);
    *addr |= (1 << (irq % 32));
}

uint32_t plic_claim(uint32_t context) {
    uint32_t* addr = (uint32_t*)((uintptr_t)PLIC_CONTEXT_BASE + context * 0x1000 + 4);
    return *addr;
}

void plic_complete(uint32_t context, uint32_t irq) {
    uint32_t* addr = (uint32_t*)((uintptr_t)PLIC_CONTEXT_BASE + context * 0x1000 + 4);
    *addr = irq;
}

typedef void (*plic_handler_t)(void);
plic_handler_t plic_handlers[1024];

void plic_register_handler(uint32_t irq, plic_handler_t handler, uint32_t priority) {
    if (irq < 1024) {
        plic_handlers[irq] = handler;
        plic_set_priority(irq, priority);
        plic_enable(0, irq);
    }
}

void plic_handle_irq(uint32_t context) {
    uint32_t irq = plic_claim(context);
    if (irq > 0 && irq < 1024 && plic_handlers[irq]) {
        plic_handlers[irq]();
    }
    plic_complete(context, irq);
}

void arch_riscv64_plic_init() {
    // Initial PLIC setup for sovereign context
    for(uint32_t i = 1; i < 32; i++) {
        plic_set_priority(i, 0); // Default zero priority (disabled)
    }
}
