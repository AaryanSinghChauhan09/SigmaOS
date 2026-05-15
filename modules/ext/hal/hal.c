#include "../../../include/libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Hardware Abstraction Layer (HAL) Prototype
// ---------------------------------------------------------

// Abstract Interrupt Frame
typedef struct {
    uint64_t registers[32];
    uint64_t program_counter;
    uint64_t status;
} interrupt_frame_t;

// Abstract Interrupt Handler Type
typedef void (*interrupt_handler_t)(interrupt_frame_t* frame);

#define MAX_INTERRUPTS 256
static interrupt_handler_t isr_table[MAX_INTERRUPTS];

// HAL Initialization
void hal_init() {
    // Zero out ISR table
    for (int i = 0; i < MAX_INTERRUPTS; i++) {
        isr_table[i] = 0;
    }
    // Architecture specific setup would go here (e.g., IDT for x86, vector table for ARM)
}

// Register an Interrupt Service Routine
void hal_register_interrupt(uint8_t vector, interrupt_handler_t handler) {
    isr_table[vector] = handler;
}

// CPU Halting / Power Management
void hal_cpu_halt() {
    // Architecture specific halt instruction
#if defined(__x86_64__)
    __asm__ volatile ("hlt");
#elif defined(__aarch64__)
    __asm__ volatile ("wfi");
#elif defined(__riscv)
    __asm__ volatile ("wfi");
#else
    // Fallback infinite loop
    while(1);
#endif
}

// Virtual Memory Management Abstraction
void hal_map_page(void* physical_addr, void* virtual_addr, uint32_t flags) {
    // Map a single page
    // Needs architecture-specific page table manipulation
}

void* hal_get_physical_address(void* virtual_addr) {
    // Translate virtual to physical
    return 0; 
}
