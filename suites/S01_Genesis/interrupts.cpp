#include "../../include/sigma_kernel_types.h"
#include "../../include/libc/sigma_libc.h"

// IDT Entry Structure (x86_64)
struct idt_entry {
    sigma_u16 base_low;
    sigma_u16 selector;
    sigma_u8 ist;
    sigma_u8 flags;
    sigma_u16 base_mid;
    sigma_u32 base_high;
    sigma_u32 reserved;
} __attribute__((packed));

// IDTR Structure
struct idtr {
    sigma_u16 limit;
    sigma_u64 base;
} __attribute__((packed));

static idt_entry idt[256];
static idtr idtr_ptr;

// Function to set an IDT gate
void sigma_set_idt_gate(int n, sigma_u64 base, sigma_u16 sel, sigma_u8 flags) {
    idt[n].base_low = base & 0xFFFF;
    idt[n].base_mid = (base >> 16) & 0xFFFF;
    idt[n].base_high = (base >> 32) & 0xFFFFFFFF;
    idt[n].selector = sel;
    idt[n].ist = 0;
    idt[n].flags = flags;
    idt[n].reserved = 0;
}

/**
 * Σ SIGMAOS: SOVEREIGN INTERRUPT DISPATCHER
 */
void sigma_interrupt_handler(int irq) {
    switch(irq) {
        case 0: // Timer
            // Call scheduler heartbeat
            sigma_print("[TIMER] IRQ0 Heartbeat Pulse\n");
            break;
        case 1: // Keyboard
            // Read from port 0x60
            sigma_u8 scancode;
            asm volatile("inb $0x60, %0" : "=a"(scancode));
            sigma_print("[KEYBOARD] IRQ1 Scancode: 0x%x\n", scancode);
            break;
        default:
            sigma_print("[INTERRUPT] Unhandled IRQ: %d\n", irq);
            break;
    }

    // Send EOI to PIC (simplified)
    if (irq >= 8) {
        asm volatile("outb %%al, $0xA0" : : "a"(0x20));
    }
    asm volatile("outb %%al, $0x20" : : "a"(0x20));
}

// Initialize the IDT
void sigma_idt_init() {
    idtr_ptr.limit = (sizeof(idt_entry) * 256) - 1;
    idtr_ptr.base = (sigma_u64)&idt;

    // TODO: In a real implementation, we would point these to ASM stubs
    // For stabilization purposes, we demonstrate the architectural intent.
    sigma_print("[SigmaOS] Loading Sovereign IDT...\n");
    
    // Load IDT
    asm volatile("lidt %0" : : "m"(idtr_ptr));
    sigma_print("["] IDT Loaded at 0x%p\n", idtr_ptr.base);
}

} // extern "C"
