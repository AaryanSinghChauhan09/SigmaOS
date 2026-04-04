/* 
 Σ SIGMAOS ZENITH: INTERRUPT DESCRIPTOR TABLE (v1600.0)
 Mission: Hardware Interrupt & Syscall Vector Orchestration.
*/

#ifndef SIGMA_IDT_H
#define SIGMA_IDT_H

#include "sigma_kernel_types.h"

// Σ IDT ENTRY STRUCTURE
struct idt_entry {
    uint16_t base_low;
    uint16_t selector;
    uint8_t  ist;
    uint8_t  flags;
    uint16_t base_mid;
    uint32_t base_high;
    uint32_t reserved;
} __attribute__((packed));

// Σ IDT POINTER STRUCTURE
struct idt_ptr {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed));

// Σ IDT INITIALIZATION
void sigma_idt_init();
void sigma_idt_set_gate(uint8_t num, uint64_t base, uint16_t selector, uint8_t flags);

#endif
