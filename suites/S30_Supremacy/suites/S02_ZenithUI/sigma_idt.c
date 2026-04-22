#include "../../../include/sigma_idt.h"
#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* =========================================================================
 * SIGMA OS: HARDWARE INTERRUPT MATRIX (IDT) C11 IMPLEMENTATION
 * Routes CPU faults and PIC hardware IRQs to kernel handlers safely.
 * ========================================================================= */

idt_entry_t idt_entries[IDT_ENTRIES];
idt_ptr_t   idt_ptr;

// Forward declaration of an empty stub catch-all ISR for unhandled hardware faults
extern void isr_stub();

void idt_set_gate(uint8_t num, uintptr_t base, uint16_t sel, uint8_t flags) {
    idt_entries[num].base_low    = (base & 0xFFFF);
    idt_entries[num].base_middle = (base >> 16) & 0xFFFF;
    idt_entries[num].base_high   = (base >> 32) & 0xFFFFFFFF;

    idt_entries[num].sel         = sel;
    idt_entries[num].always0     = 0;
    
    /* Flags specify privilege levels. 0x8E is common for Kernel Present Interrupt Gate */
    idt_entries[num].flags       = flags;
    idt_entries[num].reserved    = 0;
}

void sigma_cpu_init_idt() {
    idt_ptr.limit = sizeof(idt_entry_t) * IDT_ENTRIES - 1;
    idt_ptr.base  = (uintptr_t)&idt_entries;

    /* Initialize all 256 vector slots securely to point to a safe stub handler */
    for (int i = 0; i < IDT_ENTRIES; i++) {
        idt_set_gate(i, (uintptr_t)isr_stub, 0x08, 0x8E); // 0x08 is Kernel Code Segment from our GDT
    }

    // Call raw assembly module to execute `lidt` CPU instruction
    idt_flush((uintptr_t)&idt_ptr);
}
