/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PS/2 KEYBOARD DRIVER (v1.1)
 * =============================================================================
 * Principles: Zero-Abstract Human Input & ASCII Mapping.
 * =============================================================================
 */
#include "../include/sigma_kernel_types.h"

static const char kbd_us[128] = {
    0,  27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0,  'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',   0,
    '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',   0, '*',
    0,  ' '
};

extern u8 port_inb(u16 port);
extern void vga_putc(char c, u8 color);

void keyboard_handler() {
    u8 scancode = port_inb(0x60);
    
    /* Key release has high bit set */
    if (scancode & 0x80) {
        return;
    }

    char c = kbd_us[scancode];
    if (c) {
        /* Print to VGA Buffer */
        vga_putc(c, 0x0F);
        
        /* Forward to Shell (Future implementation) */
        /* shell_buffer_push(c); */
    }
}
