#include "libc/SovereignLibC.h"
#include "sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: PS/2 KEYBOARD DRIVER (v1.1)
 * =============================================================================
 * Principles: Zero-Abstract Human Input & ASCII Mapping.
 * =============================================================================
 */
#include "sigma_kernel_types.h"

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

/* =========================================================================
 * Public API
 * ========================================================================= */
bool_t kbd_poll(KeyEvent* ev) {
    if (!g_kbd.ev_count) return FALSE;
    *ev = g_kbd.events[g_kbd.ev_head % KBD_EVENT_MAX];
    g_kbd.ev_head++;
    g_kbd.ev_count--;
    return TRUE;
}

char kbd_getchar(void) {
    KeyEvent ev;
    while (!kbd_poll(&ev)) cpu_pause();
    return ev.ascii;
}

void kbd_init(void) {
    u32 i;
    for (i = 0; i < KBD_RINGBUF_SZ; i++) g_kbd.ringbuf[i] = 0;
    g_kbd.rb_head = g_kbd.rb_tail = g_kbd.rb_count = 0;
    g_kbd.ev_head = g_kbd.ev_count = 0;
    g_kbd.shift = g_kbd.ctrl = g_kbd.alt = g_kbd.capslock = FALSE;
    g_kbd.total_keys = 0;

    extern void idt_register_handler(u32, void (*)(SigmaInterruptFrame*));
    extern void pic_unmask_irq(u8);
    idt_register_handler(33, kbd_irq_handler);   /* IRQ1 → vector 33 */
    pic_unmask_irq(1);

    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[KBD]: PS/2 keyboard driver online. IRQ1 unmasked.\n");
}
