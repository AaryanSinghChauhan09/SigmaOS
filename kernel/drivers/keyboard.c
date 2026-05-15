/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: PS/2 KEYBOARD DRIVER (v1.1)
 * =============================================================================
 * Principles: Zero-Abstract Human Input & ASCII Mapping.
 * =============================================================================
 */
#include "../../include/core/sigma_kernel_types.h"

static const char kbd_us[128] = {
    0,  27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0,  'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',   0,
    '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',   0, '*',
    0,  ' '
};

extern void kbd_queue_push(char c);

void keyboard_handler() {
    sigma_u8 scancode = port_inb(0x60);
    
    /* Key release has high bit set */
    if (scancode & 0x80) {
        return;
    }

    char c = kbd_us[scancode];
    if (c) {
        /* Atomic push to Sovereign Input Queue */
        kbd_queue_push(c);
    }
}
