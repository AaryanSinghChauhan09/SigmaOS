#ifndef SIGMA_IDT_H
#define SIGMA_IDT_H

#include <stdint.h>

/* =========================================================================
 * SIGMA OS: INTERRUPT DESCRIPTOR TABLE (IDT) SHARD
 * Defines CPU exception and hardware interrupt vector routing.
 * ========================================================================= */

#define IDT_ENTRIES 256

struct idt_entry_struct {
    uint16_t base_low;
    uint16_t sel;
    uint8_t  always0;
    uint8_t  flags;
    uint16_t base_middle;
    uint32_t base_high;
    uint32_t reserved;
} __attribute__((packed));

typedef struct idt_entry_struct idt_entry_t;

struct idt_ptr_struct {
    uint16_t limit;
    uintptr_t base;
} __attribute__((packed));

typedef struct idt_ptr_struct idt_ptr_t;

void sigma_cpu_init_idt();
void idt_set_gate(uint8_t num, uintptr_t base, uint16_t sel, uint8_t flags);

// Assembly externs logic
extern void idt_flush(uintptr_t);

#endif
