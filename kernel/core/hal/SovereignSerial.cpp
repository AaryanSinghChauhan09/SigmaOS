#include "core/sigma_types.h"
#include "sigma_log.h"

#define COM1 0x3F8

extern "C" {

static inline void outb(sigma_u16 port, sigma_u8 val) {
    __asm__ __volatile__ ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline sigma_u8 inb(sigma_u16 port) {
    sigma_u8 ret;
    __asm__ __volatile__ ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

void serial_init() {
    outb(COM1 + 1, 0x00);    // Disable all interrupts
    outb(COM1 + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    outb(COM1 + 0, 0x03);    // Set divisor to 3 (38400 baud)
    outb(COM1 + 1, 0x00);
    outb(COM1 + 3, 0x03);    // 8 bits, no parity, one stop bit
    outb(COM1 + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
    outb(COM1 + 4, 0x0B);    // IRQs enabled, RTS/DSR set
}

int is_transmit_empty() {
    return inb(COM1 + 5) & 0x20;
}

void serial_putc(char c) {
    while (is_transmit_empty() == 0);
    outb(COM1, (sigma_u8)c);
}

} // extern "C"
