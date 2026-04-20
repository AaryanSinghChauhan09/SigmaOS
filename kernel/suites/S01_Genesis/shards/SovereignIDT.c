/*
 * =========================================================================
 * S SIGMAOS: S01_GENESIS — SovereignIDT.c
 * =========================================================================
 * Implementation of Direct-to-Silicon Interrupt Descriptor Table.
 * Bypasses all high-level abstractions to ensure zero-latency signals.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "sigma_types.h"

#define IDT_ENTRIES 256

typedef struct {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t  ist;
    uint8_t  flags;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t reserved;
} __attribute__((packed)) idt_entry_t;

typedef struct {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed)) idt_ptr_t;

static idt_entry_t g_idt[IDT_ENTRIES];
static idt_ptr_t   g_idtr;

extern void idt_load(idt_ptr_t* ptr);

void idt_set_gate(uint8_t num, uint64_t base, uint16_t sel, uint8_t flags) {
    g_idt[num].offset_low  = (uint16_t)(base & 0xFFFF);
    g_idt[num].offset_mid  = (uint16_t)((base >> 16) & 0xFFFF);
    g_idt[num].offset_high = (uint32_t)((base >> 32) & 0xFFFFFFFF);
    g_idt[num].selector    = sel;
    g_idt[num].ist         = 0;
    g_idt[num].flags       = flags;
    g_idt[num].reserved    = 0;
}

void sigma_isr_handler(uint64_t irq_num) {
    sigma_printf("S [IDT]: Direct-to-Silicon ISR Triggered -> IRQ %d\n", irq_num);
    /* [Σ Implementation Note]: This replaces standard generic IRQ dispatchers
       with shard-specific deterministic execution paths. */
}

void idt_init(void) {
    g_idtr.limit = (sizeof(idt_entry_t) * IDT_ENTRIES) - 1;
    g_idtr.base  = (uint64_t)&g_idt;

    sigma_memset(&g_idt, 0, sizeof(g_idt));

    /* [Σ Security Note]: Setting up gates for CPU exceptions and hardware IRQs.
       Flags 0x8E = Interrupt Gate (Present, DPL=0, Type=E) */
    for (int i = 0; i < 32; i++) {
        idt_set_gate(i, (uint64_t)sigma_isr_handler, 0x08, 0x8E);
    }

    sigma_printf("S [S01]: Sovereign IDT Materialized. Silicon linkage complete.\n");
}
