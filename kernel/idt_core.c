/*
 * Cosmos AI-OS: Interrupt Descriptor Table (IDT, C Layer)
 * =======================================================
 * Mission: CPU Exception handling & Hardware IRQ routing.
 * Without this, the CPU Triple-Faults and resets on any error.
 */

#include <stdint.h>

// x86_64 IDT Entry Structure
typedef struct __attribute__((packed)) {
  uint16_t isr_low;   // Lower 16 bits of ISR's address
  uint16_t kernel_cs; // Kernel segment selector
  uint8_t ist;        // Interrupt Stack Table offset
  uint8_t attributes; // Type and attributes (Priority, Ring)
  uint16_t isr_mid;   // Next 16 bits of ISR's address
  uint32_t isr_high;  // Upper 32 bits of ISR's address
  uint32_t reserved;  // Set to zero
} idt_entry_t;

typedef struct __attribute__((packed)) {
  uint16_t limit;
  uint64_t base;
} idtr_t;

// Standard 256 vector IDs
static idt_entry_t idt_table[256];
static idtr_t idtr;

// Set a specific gate
void cosmos_set_idt_gate(int vector, void *isr, uint8_t flags) {
  uint64_t addr = (uint64_t)isr;

  idt_table[vector].isr_low = (uint16_t)(addr & 0xFFFF);
  idt_table[vector].kernel_cs = 0x08; // Assuming 0x08 is Kernel Code Segment
  idt_table[vector].ist = 0;
  idt_table[vector].attributes = flags;
  idt_table[vector].isr_mid = (uint16_t)((addr >> 16) & 0xFFFF);
  idt_table[vector].isr_high = (uint32_t)((addr >> 32) & 0xFFFFFFFF);
  idt_table[vector].reserved = 0;
}

// Global Init callable from Cosmos Init or Python ctypes module
void cosmos_idt_init() {
  idtr.base = (uint64_t)&idt_table[0];
  idtr.limit = (uint16_t)sizeof(idt_entry_t) * 256 - 1;

  // By default, set all to a panic stub.
  // In a full implementation, we set 0-31 as CPU exceptions (Divide by Zero,
  // Page Fault) and 32+ as Hardware IRQs / MSI-X targets.

  // Load the IDT into the CPU processor register
  __asm__ volatile("lidt %0" : : "m"(idtr));
}

// Example Exception Handler
void cosmos_page_fault_handler(uint64_t error_code, uint64_t faulting_addr) {
  // Neural Prefetcher integration here.
  // If accessing a 'Ghost Page', upgrade to read-write.
  // Otherwise, SEGFAULT the offending Ring-3 app.
}
