/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: UART SERIAL DRIVER (v1.0)
 * =============================================================================
 * Principles: Bare-Metal Debugging & Agentic Heartbeats.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

#define COM1 0x3F8

void serial_init() {
    port_outb(COM1 + 1, 0x00);    // Disable all interrupts
    port_outb(COM1 + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    port_outb(COM1 + 0, 0x03);    // Set divisor to 3 (38400 baud)
    port_outb(COM1 + 1, 0x00);
    port_outb(COM1 + 3, 0x03);    // 8 bits, no parity, one stop bit
    port_outb(COM1 + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
    port_outb(COM1 + 4, 0x0B);    // IRQs enabled, RTS/DSR set
}

int is_transmit_empty() {
    return port_inb(COM1 + 5) & 0x20;
}

void serial_putc(char c) {
    while (is_transmit_empty() == 0);
    port_outb(COM1, c);
}

void serial_puts(const char* s) {
    while (*s) serial_putc(*s++);
}
