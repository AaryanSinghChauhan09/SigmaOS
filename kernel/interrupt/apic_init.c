// SPDX-License-Identifier: MIT
// SigmaOS APIC Initialization & Interrupt Vector Setup
// x86_64 assembly-level interrupt vector installation

#include <stddef.h>
#include <stdint.h>
#include <string.h>

// ============================================================================
// Global Descriptor Table (GDT) Setup for x86_64
// ============================================================================

typedef struct {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t base_mid;
    uint8_t access;
    uint8_t granularity;
    uint8_t base_high;
} __attribute__((packed)) gdt_entry_t;

typedef struct {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed)) gdt_ptr_t;

#define GDT_ENTRIES 7
static gdt_entry_t gdt[GDT_ENTRIES];
static gdt_ptr_t gdt_ptr;

void gdt_set_gate(uint32_t num, uint32_t base, uint32_t limit, uint8_t access, uint8_t gran) {
    gdt[num].base_low = (base & 0xFFFF);
    gdt[num].base_mid = (base >> 16) & 0xFF;
    gdt[num].base_high = (base >> 24) & 0xFF;
    gdt[num].limit_low = (limit & 0xFFFF);
    gdt[num].granularity = (limit >> 16) & 0x0F;
    gdt[num].granularity |= gran & 0xF0;
    gdt[num].access = access;
}

void gdt_init() {
    gdt_ptr.limit = (sizeof(gdt_entry_t) * GDT_ENTRIES) - 1;
    gdt_ptr.base = (uint64_t)&gdt;

    // Null descriptor
    gdt_set_gate(0, 0, 0, 0, 0);
    
    // Kernel code segment (base=0, limit=4GB)
    gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
    
    // Kernel data segment (base=0, limit=4GB)
    gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
    
    // User code segment
    gdt_set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF);
    
    // User data segment
    gdt_set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);
    
    // TSS segment
    gdt_set_gate(5, 0, sizeof(void *), 0xE9, 0x00);
}

// ============================================================================
// Interrupt Descriptor Table (IDT) Setup
// ============================================================================

typedef struct {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t ist;
    uint8_t type_attr;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t reserved;
} __attribute__((packed)) idt_entry_t;

typedef struct {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed)) idt_ptr_t;

#define IDT_ENTRIES 256
static idt_entry_t idt[IDT_ENTRIES];
static idt_ptr_t idt_ptr;

void idt_set_gate(uint32_t num, uint64_t base, uint16_t selector, uint8_t type_attr) {
    idt[num].offset_low = base & 0xFFFF;
    idt[num].offset_mid = (base >> 16) & 0xFFFF;
    idt[num].offset_high = (base >> 32) & 0xFFFFFFFF;
    idt[num].selector = selector;
    idt[num].type_attr = type_attr;
    idt[num].ist = 0; // IST index (0 = use ESP/RSP)
    idt[num].reserved = 0;
}

void idt_init() {
    idt_ptr.limit = (sizeof(idt_entry_t) * IDT_ENTRIES) - 1;
    idt_ptr.base = (uint64_t)&idt;
    
    // Clear IDT
    memset(&idt, 0, sizeof(idt_entry_t) * IDT_ENTRIES);
}

// ============================================================================
// APIC MSR (Model-Specific Register) Operations
// ============================================================================

#define IA32_APIC_BASE_MSR 0x1B

uint64_t read_msr(uint32_t msr) {
    uint32_t edx, eax;
    __asm__ __volatile__("rdmsr" : "=a"(eax), "=d"(edx) : "c"(msr));
    return ((uint64_t)edx << 32) | eax;
}

void write_msr(uint32_t msr, uint64_t value) {
    uint32_t edx = value >> 32;
    uint32_t eax = value & 0xFFFFFFFF;
    __asm__ __volatile__("wrmsr" : : "a"(eax), "d"(edx), "c"(msr));
}

// ============================================================================
// APIC Hardware Initialization
// ============================================================================

void apic_enable() {
    // Enable APIC by setting bit 11 in IA32_APIC_BASE MSR
    uint64_t apic_base = read_msr(IA32_APIC_BASE_MSR);
    apic_base |= (1ULL << 11); // Enable bit
    apic_base |= (1ULL << 10); // x2APIC bit (optional, for modern systems)
    write_msr(IA32_APIC_BASE_MSR, apic_base);
}

// Memory-mapped APIC register writes (for xAPIC mode)
void apic_write(uint32_t offset, uint32_t value) {
    volatile uint32_t *apic_reg = (volatile uint32_t *)(0xfee00000 + offset);
    *apic_reg = value;
}

uint32_t apic_read(uint32_t offset) {
    volatile uint32_t *apic_reg = (volatile uint32_t *)(0xfee00000 + offset);
    return *apic_reg;
}

// ============================================================================
// Interrupt Vector Setup & Dispatch
// ============================================================================

// Forward declarations for interrupt handlers (defined in assembly/Rust)
extern void sigma_irq_handler_0(void);
extern void sigma_irq_handler_1(void);
extern void sigma_irq_handler_8(void);
extern void sigma_irq_handler_13(void);
extern void sigma_irq_handler_14(void);
extern void sigma_irq_handler_timer(void);
extern void sigma_irq_handler_keyboard(void);
extern void sigma_irq_handler_network(void);
extern void sigma_irq_handler_disk(void);
extern void sigma_irq_handler_default(void);

void sigma_irq_init() {
    // Initialize GDT and IDT
    gdt_init();
    idt_init();

    // Install exception handlers (vectors 0-31)
    idt_set_gate(0, (uint64_t)sigma_irq_handler_0, 0x08, 0x8E);      // Division by zero
    idt_set_gate(1, (uint64_t)sigma_irq_handler_1, 0x08, 0x8E);      // Debug
    idt_set_gate(8, (uint64_t)sigma_irq_handler_8, 0x08, 0x8E);      // Double fault
    idt_set_gate(13, (uint64_t)sigma_irq_handler_13, 0x08, 0x8E);    // General protection
    idt_set_gate(14, (uint64_t)sigma_irq_handler_14, 0x08, 0x8E);    // Page fault

    // Install IRQ handlers (vectors 32+)
    idt_set_gate(32, (uint64_t)sigma_irq_handler_timer, 0x08, 0x8E);     // Timer (IRQ0)
    idt_set_gate(33, (uint64_t)sigma_irq_handler_keyboard, 0x08, 0x8E);  // Keyboard (IRQ1)
    idt_set_gate(37, (uint64_t)sigma_irq_handler_network, 0x08, 0x8E);   // Network (IRQ5)
    idt_set_gate(38, (uint64_t)sigma_irq_handler_disk, 0x08, 0x8E);      // Disk (IRQ6)

    // Fill remaining with default handler
    for (int i = 2; i < 256; i++) {
        if (i != 0 && i != 1 && i != 8 && i != 13 && i != 14 && 
            i != 32 && i != 33 && i != 37 && i != 38) {
            idt_set_gate(i, (uint64_t)sigma_irq_handler_default, 0x08, 0x8E);
        }
    }

    // Enable APIC
    apic_enable();

    // Load GDT and IDT
    __asm__ __volatile__(
        "lgdt gdt_ptr; "
        "lidt idt_ptr; "
        : : "m"(gdt_ptr), "m"(idt_ptr)
    );

    // Clear interrupts until system is ready
    __asm__ __volatile__("cli");
}

void sigma_irq_enable() {
    __asm__ __volatile__("sti");
}

void sigma_irq_disable() {
    __asm__ __volatile__("cli");
}

// ============================================================================
// Generic IRQ Dispatch
// ============================================================================

// This will be called from Rust
typedef void (*irq_handler_t)(uint8_t);

static irq_handler_t irq_handlers[256] = {0};

void sigma_register_irq_handler(uint8_t vector, irq_handler_t handler) {
    if (vector < 256) {
        irq_handlers[vector] = handler;
    }
}

void sigma_dispatch_irq(uint8_t vector) {
    if (vector < 256 && irq_handlers[vector]) {
        irq_handlers[vector](vector);
    }
}

// ============================================================================
// PIC (8259 Programmable Interrupt Controller) Legacy Support
// ============================================================================

#define PIC_MASTER_CMD 0x20
#define PIC_MASTER_DATA 0x21
#define PIC_SLAVE_CMD 0xA0
#define PIC_SLAVE_DATA 0xA1

#define PIC_EOI 0x20

void pic_send_eoi(uint8_t irq) {
    if (irq >= 8) {
        // IRQ on slave PIC
        __asm__ __volatile__("outb %b0, %w1" : : "a"(PIC_EOI), "N"(PIC_SLAVE_CMD));
    }
    // Always send EOI to master
    __asm__ __volatile__("outb %b0, %w1" : : "a"(PIC_EOI), "N"(PIC_MASTER_CMD));
}

void pic_disable() {
    // Mask all interrupts on master
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0xFF), "N"(PIC_MASTER_DATA));
    // Mask all interrupts on slave
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0xFF), "N"(PIC_SLAVE_DATA));
}

void pic_remap(uint8_t offset1, uint8_t offset2) {
    // ICW1: begin initialization sequence (bit 4 set, ICW4 needed)
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x11), "N"(PIC_MASTER_CMD));
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x11), "N"(PIC_SLAVE_CMD));

    // ICW2: set interrupt vector offsets
    __asm__ __volatile__("outb %b0, %w1" : : "a"(offset1), "N"(PIC_MASTER_DATA));
    __asm__ __volatile__("outb %b0, %w1" : : "a"(offset2), "N"(PIC_SLAVE_DATA));

    // ICW3: configure cascading
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x04), "N"(PIC_MASTER_DATA));
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x02), "N"(PIC_SLAVE_DATA));

    // ICW4: environment control
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x01), "N"(PIC_MASTER_DATA));
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x01), "N"(PIC_SLAVE_DATA));

    // OCW1: unmask all interrupts
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x00), "N"(PIC_MASTER_DATA));
    __asm__ __volatile__("outb %b0, %w1" : : "a"(0x00), "N"(PIC_SLAVE_DATA));
}

void sigma_platform_init() {
    // Initialize interrupt infrastructure
    sigma_irq_init();
    
    // Remap PIC to vectors 32-39 and 40-47 (move away from CPU exceptions)
    pic_remap(32, 40);
    
    // Enable interrupts
    sigma_irq_enable();
}
