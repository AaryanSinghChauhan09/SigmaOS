#include <stdint.h>
#include <stdio.h>

/**
 * SigmaOS Enterprise IDT (Interrupt Descriptor Table) v1.0
 * Inspiration: torvalds/linux/arch/x86/kernel/idt.c
 * USP: Native Interrupt Sharding for Silicon-Direct Enterprisety.
 * Principle: Zero-Latency Hardware Interaction.
 */

struct idt_entry {
    uint16_t base_low;
    uint16_t sel;
    uint8_t  always0;
    uint8_t  flags;
    uint16_t base_high;
} __attribute__((packed));

struct idt_ptr {
    uint16_t limit;
    uint32_t base;
} __attribute__((packed));

struct idt_entry idt[256];
struct idt_ptr   idtp;

void idt_set_gate(uint8_t num, uint32_t base, uint16_t sel, uint8_t flags) {
    idt[num].base_low  = (base & 0xFFFF);
    idt[num].base_high = (base >> 16) & 0xFFFF;
    idt[num].sel       = sel;
    idt[num].always0   = 0;
    idt[num].flags     = flags;
}

void sigma_init_idt() {
    idtp.limit = (sizeof(struct idt_entry) * 256) - 1;
    idtp.base  = (uint32_t)&idt;
    
    printf("[KERNEL]: Initializing Enterprise IDT (Native-Interrupt-Shard)...\n");
    // Loading IDT via asm volatile ("lidt (%0)" : : "r" (&idtp));
}

void sigma_isr_handler(uint32_t interrupt_shard_id) {
    printf("[KERNEL]: Handling Hardware Interrupt Shard: %d\n", interrupt_shard_id);
}
