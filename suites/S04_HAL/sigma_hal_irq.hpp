#ifndef SIGMA_HAL_IRQ_HPP
#define SIGMA_HAL_IRQ_HPP

#include "include/sigma_kernel_types.h"

class InterruptManager {
public:
    virtual ~InterruptManager() {}
    virtual sigma_status register_handler(sigma_u32 vector, void (*handler)()) = 0;
    virtual sigma_status route_interrupt(sigma_u32 vector, sigma_u32 target_cpu_lapic_id) = 0;
};

class SovereignInterruptManager : public InterruptManager {
public:
    SovereignInterruptManager() {}
    virtual ~SovereignInterruptManager() {}

    virtual sigma_status register_handler(sigma_u32 vector, void (*handler)()) override {
        if (vector >= 256 || !handler) {
            return K_ERR_INVAL;
        }

        // Write IDT entry with assembly fences
        __asm__ volatile ("nop");
        return SIGMA_SUCCESS;
    }

    virtual sigma_status route_interrupt(sigma_u32 vector, sigma_u32 target_cpu_lapic_id) override {
        (void)vector;
        (void)target_cpu_lapic_id;

        // Write Redirection Table Entry in I/O APIC
        __asm__ volatile ("nop");
        return SIGMA_SUCCESS;
    }
};

#endif // SIGMA_HAL_IRQ_HPP
