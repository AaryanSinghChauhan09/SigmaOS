#include <stdint.h>
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Capability-Routed Interrupt Controller
// USP: Interrupts are not raw signals; they are capability
// verified events routed exclusively to isolated drivers.
// ---------------------------------------------------------

#define MAX_IRQS 256
#define MAX_HANDLERS_PER_IRQ 4

typedef struct {
    uint32_t driver_pid;
    void (*handler_func)(void);
    uint32_t capability_token;
} irq_handler_t;

typedef struct {
    irq_handler_t handlers[MAX_HANDLERS_PER_IRQ];
    uint8_t count;
    uint64_t last_fire_tick;
    uint32_t storm_count;
} irq_vector_t;

static irq_vector_t idt_routing[MAX_IRQS];

extern int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights);
extern void watchdog_trigger_fault(uint32_t pid, const char* reason);
extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Register an interrupt handler (Requires CAP_IRQ_BIND)
int interrupt_register(uint8_t irq, uint32_t pid, void (*handler)(void), uint32_t cap_token) {
    if (irq >= MAX_IRQS) return -1;
    
    // USP: seL4-style capability isolation.
    // A random process cannot hijack the keyboard or NIC IRQ.
    if (!cap_registry_verify(cap_token, pid, 0x01 /* CAP_BIND */)) {
        audit_chain_append(pid, 3, "IRQ_BIND_DENIED_CAP_FAILURE");
        return -2;
    }

    irq_vector_t* vec = &idt_routing[irq];
    if (vec->count >= MAX_HANDLERS_PER_IRQ) return -3;

    vec->handlers[vec->count].driver_pid = pid;
    vec->handlers[vec->count].handler_func = handler;
    vec->handlers[vec->count].capability_token = cap_token;
    vec->count++;

    return 0;
}

// Master Interrupt Dispatcher (called by low-level assembly ASM wrappers)
void interrupt_dispatch(uint8_t irq, uint64_t current_tick) {
    if (irq >= MAX_IRQS) return;
    irq_vector_t* vec = &idt_routing[irq];

    // USP: Self-Healing IRQ Storm Protection
    if (current_tick - vec->last_fire_tick < 2) {
        vec->storm_count++;
        if (vec->storm_count > 1000) {
            // Faulty hardware or buggy driver! Disable IRQ to save system lockup.
            audit_chain_append(0, 3, "IRQ_STORM_DETECTED_DISABLING");
            vec->count = 0; // Drop handlers
            return;
        }
    } else {
        vec->storm_count = 0;
    }
    vec->last_fire_tick = current_tick;

    // Dispatch to authorized drivers
    for (uint8_t i = 0; i < vec->count; i++) {
        // Double-check capability is still valid at runtime
        if (cap_registry_verify(vec->handlers[i].capability_token, vec->handlers[i].driver_pid, 0x01)) {
            // Execute the driver's handler
            if (vec->handlers[i].handler_func) {
                vec->handlers[i].handler_func();
            }
        } else {
            // Capability was revoked dynamically! Remove handler.
            watchdog_trigger_fault(vec->handlers[i].driver_pid, "IRQ_CAPABILITY_REVOKED_AT_RUNTIME");
        }
    }
}
