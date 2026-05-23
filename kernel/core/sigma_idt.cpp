/*
 * Σ SigmaOS — sigma_idt: Sovereign Interrupt Descriptor Table
 * Zero-Dependency: No POSIX signals, no predefined interrupt headers.
 * Absorbs: x86_64 Intel SDM Vol 3 Chapter 6, Linux arch/x86/kernel/idt.c.
 * Implements: IDT setup, exception handlers, IRQ routing.
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* IDT Gate Descriptor (16 bytes for x86_64) */
struct __attribute__((packed)) IDTGate {
    u16 offset_low;
    u16 segment_selector;
    u8  ist;           /* Interrupt Stack Table offset */
    u8  type_attr;     /* Present, DPL, Gate Type */
    u16 offset_mid;
    u32 offset_high;
    u32 reserved;
};

/* IDT Pointer for LIDT instruction */
struct __attribute__((packed)) IDTPointer {
    u16 limit;
    u64 base;
};

#define IDT_ENTRIES 256
static IDTGate idt[IDT_ENTRIES];
static IDTPointer idt_ptr;

/* Install a gate into the IDT */
static void idt_set_gate(u8 num, u64 handler, u16 selector, u8 flags) {
    idt[num].offset_low  = handler & 0xFFFF;
    idt[num].offset_mid  = (handler >> 16) & 0xFFFF;
    idt[num].offset_high = (handler >> 32) & 0xFFFFFFFF;
    idt[num].segment_selector = selector;
    idt[num].ist = 0;
    idt[num].type_attr = flags;
    idt[num].reserved = 0;
}

/* Exception handler stubs */
static const char* exception_names[] = {
    "Division Error", "Debug", "NMI", "Breakpoint",
    "Overflow", "Bound Range Exceeded", "Invalid Opcode", "Device Not Available",
    "Double Fault", "Coprocessor Segment Overrun", "Invalid TSS", "Segment Not Present",
    "Stack-Segment Fault", "General Protection Fault", "Page Fault", "Reserved",
    "x87 FPU Error", "Alignment Check", "Machine Check", "SIMD FP Exception"
};

/* Generic exception handler — called from assembly stubs */
extern "C" void sigma_exception_handler(u32 vector, u64 error_code, u64 rip) {
    const char* name = (vector < 20) ? exception_names[vector] : "Unknown";
    sigma_vga_printf("[EXCEPTION] #%u (%s) error_code=0x%X RIP=0x%X\n",
        vector, name, (u32)error_code, (u32)rip);

    if (vector == 14) { /* Page Fault */
        u64 cr2;
        __asm__ volatile ("mov %%cr2, %0" : "=r"(cr2));
        sigma_vga_printf("[PAGE FAULT] Faulting address: 0x%X\n", (u32)cr2);
    }

    /* Halt on unrecoverable exceptions */
    if (vector == 8 || vector == 13 || vector == 14) {
        sigma_vga_printf("[KERNEL PANIC] Halting.\n");
        __asm__ volatile ("cli; hlt");
    }
}

/* IRQ handler routing — called from PIC/APIC */
extern "C" void sigma_irq_handler(u32 irq) {
    switch (irq) {
        case 0:  /* Timer (PIT) */
            break;
        case 1:  /* Keyboard (PS/2) */
            break;
        case 12: /* Mouse (PS/2) */
            break;
        default:
            sigma_vga_printf("[IRQ] Unhandled IRQ %u\n", irq);
            break;
    }
    /* Send EOI to PIC */
    __asm__ volatile ("outb %%al, $0x20" : : "a"((u8)0x20));
    if (irq >= 8) __asm__ volatile ("outb %%al, $0xA0" : : "a"((u8)0x20));
}

/* Load the IDT */
extern "C" void sigma_idt_init() {
    idt_ptr.limit = sizeof(idt) - 1;
    idt_ptr.base  = (u64)&idt;

    /* Clear all gates */
    for (int i = 0; i < IDT_ENTRIES; i++)
        idt_set_gate(i, 0, 0x08, 0x8E);

    /* Load via LIDT */
    __asm__ volatile ("lidt %0" : : "m"(idt_ptr));
    sigma_vga_printf("[IDT] Interrupt Descriptor Table loaded (%u entries)\n", IDT_ENTRIES);
}
