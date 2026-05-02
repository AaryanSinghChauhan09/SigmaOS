/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: INTERRUPT DESCRIPTOR TABLE (v1.0)
 * =============================================================================
 * Principles: Event-Driven Sovereignty.
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

typedef struct {
    sigma_u16 offset_low;
    sigma_u16 selector;
    sigma_u8  ist;
    sigma_u8  types_attr;
    sigma_u16 offset_mid;
    sigma_u32 offset_high;
    sigma_u32 zero;
} __attribute__((packed)) idt_entry_t;

typedef struct {
    sigma_u16 limit;
    sigma_u64 base;
} __attribute__((packed)) idt_ptr_t;

static idt_entry_t idt[256];
static idt_ptr_t   idtp;

extern void idt_flush(sigma_u64 ptr);
extern void kprintf(const char* fmt, ...);

void idt_set_gate(sigma_u8 num, sigma_u64 base, sigma_u16 sel, sigma_u8 flags) {
    idt[num].offset_low  = (base & 0xFFFF);
    idt[num].offset_mid  = (base >> 16) & 0xFFFF;
    idt[num].offset_high = (base >> 32) & 0xFFFFFFFF;
    idt[num].selector    = sel;
    idt[num].ist         = 0;
    idt[num].types_attr  = flags;
    idt[num].zero        = 0;
}

void idt_init() {
    idtp.limit = (sizeof(idt_entry_t) * 256) - 1;
    idtp.base  = (sigma_u64)&idt;

    sigma_memset(&idt, 0, sizeof(idt_entry_t) * 256);

    /* Load IDT */
    idt_flush((sigma_u64)&idtp);
    
    kprintf("Î£ [IDT]: Gate array established.\n");
}
