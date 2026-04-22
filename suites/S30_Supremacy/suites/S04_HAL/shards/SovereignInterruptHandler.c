// SigmaOS Sovereign Interrupt Handler
// Absorbs Linux IDT/APIC + Windows Interrupt Dispatch Table + Mach interrupt stacks
// Hardened C11 — all interrupt handlers run at ring-0 with safe stack switching.

#include "sigma_types.h"

#define SIGMA_IRQ_VECTORS    256
#define SIGMA_STACK_SIZE     8192 // Dedicated kernel interrupt stack per vector

typedef void (*IRQHandler)(uint32_t irq_vector, void* saved_context);

typedef struct {
    IRQHandler  handler;
    uint8_t     priority;     // APIC IRQ priority (0 = highest)
    bool        is_shared;    // Multiple drivers sharing one IRQ line
} SigmaIRQDescriptor;

static SigmaIRQDescriptor irq_table[SIGMA_IRQ_VECTORS];

// Register a named handler for a specific interrupt vector
void irq_register(uint32_t vector, IRQHandler handler, uint8_t priority, bool shared);

// Central IRQ dispatch — called by the CPU trap gate after saving context
void irq_dispatch(uint32_t vector, void* saved_context);

// Mask/unmask interrupts on the APIC controller
void irq_mask(uint32_t vector);
void irq_unmask(uint32_t vector);

// EOI — signal End of Interrupt to the APIC
void irq_send_eoi(uint32_t vector);

// Enable/disable non-maskable interrupts (NMI) for watchdog
void irq_enable_nmi(void);
void irq_disable_nmi(void);



