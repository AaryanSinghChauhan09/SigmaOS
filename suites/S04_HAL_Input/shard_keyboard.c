/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: KEYBOARD DRIVER (v1.0 - PURE C11)
 * =============================================================================
 * Interface: PS/2 keyboard via x86 I/O ports (8042 controller)
 * Features:
 *   - US QWERTY scancode-to-ASCII translation (Set 1)
 *   - Shift / CapsLock / Ctrl / Alt modifier tracking
 *   - 256-byte ring buffer (IRQ1 producer, reader consumer)
 *   - Key event queue (keycode + modifiers)
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../sigma_kernel_types.h"

/* =========================================================================
 * PS/2 I/O Ports (8042 controller)
 * ========================================================================= */
#define KBD_DATA_PORT   0x60   /* read scancode / write command */
#define KBD_STAT_PORT   0x64   /* status register */
#define KBD_STAT_OBF    BIT(0) /* output buffer full — data ready */

/* =========================================================================
 * Scancode Set 1 → ASCII (US QWERTY, unshifted)
 * ========================================================================= */
static const char g_sc_normal[128] = {
    0,    0x1B, '1',  '2',  '3',  '4',  '5',  '6',
    '7',  '8',  '9',  '0',  '-',  '=',  '\b', '\t',
    'q',  'w',  'e',  'r',  't',  'y',  'u',  'i',
    'o',  'p',  '[',  ']',  '\n', 0,    'a',  's',
    'd',  'f',  'g',  'h',  'j',  'k',  'l',  ';',
    '\'', '`',  0,    '\\', 'z',  'x',  'c',  'v',
    'b',  'n',  'm',  ',',  '.',  '/',  0,    '*',
    0,    ' ',  0,    0,    0,    0,    0,    0,
    /* F1-F12, arrows, etc. mapped to 0 (handle separately) */
};

static const char g_sc_shift[128] = {
    0,    0x1B, '!',  '@',  '#',  '$',  '%',  '^',
    '&',  '*',  '(',  ')',  '_',  '+',  '\b', '\t',
    'Q',  'W',  'E',  'R',  'T',  'Y',  'U',  'I',
    'O',  'P',  '{',  '}',  '\n', 0,    'A',  'S',
    'D',  'F',  'G',  'H',  'J',  'K',  'L',  ':',
    '"',  '~',  0,    '|',  'Z',  'X',  'C',  'V',
    'B',  'N',  'M',  '<',  '>',  '?',  0,    '*',
    0,    ' ',  0,    0,    0,    0,    0,    0,
};

/* Special scancodes */
#define SC_LSHIFT   0x2A
#define SC_RSHIFT   0x36
#define SC_LCTRL    0x1D
#define SC_LALT     0x38
#define SC_CAPSLOCK 0x3A
#define SC_BREAK    0x80   /* key-release = scancode | 0x80 */

/* =========================================================================
 * Key Event
 * ========================================================================= */
typedef struct KeyEvent {
    char  ascii;
    u8    scancode;
    bool_t ctrl;
    bool_t alt;
    bool_t shift;
} KeyEvent;

/* =========================================================================
 * Keyboard State
 * ========================================================================= */
#define KBD_RINGBUF_SZ  256u
#define KBD_EVENT_MAX   64u

typedef struct KbdState {
    u8        ringbuf[KBD_RINGBUF_SZ];
    u32       rb_head;
    u32       rb_tail;
    u32       rb_count;

    KeyEvent  events[KBD_EVENT_MAX];
    u32       ev_head;
    u32       ev_count;

    bool_t    shift;
    bool_t    ctrl;
    bool_t    alt;
    bool_t    capslock;
    u64       total_keys;
} KbdState;

static KbdState g_kbd;

/* =========================================================================
 * Ring buffer helpers
 * ========================================================================= */
static void rb_push(u8 byte) {
    if (g_kbd.rb_count >= KBD_RINGBUF_SZ) return;  /* overflow: drop */
    g_kbd.ringbuf[g_kbd.rb_tail % KBD_RINGBUF_SZ] = byte;
    g_kbd.rb_tail++;
    g_kbd.rb_count++;
}

static bool_t rb_pop(u8* out) {
    if (!g_kbd.rb_count) return FALSE;
    *out = g_kbd.ringbuf[g_kbd.rb_head % KBD_RINGBUF_SZ];
    g_kbd.rb_head++;
    g_kbd.rb_count--;
    return TRUE;
}

/* =========================================================================
 * IRQ1 Handler — called from IDT vector 33
 * ========================================================================= */
typedef struct SigmaInterruptFrame SigmaInterruptFrame;

void kbd_irq_handler(SigmaInterruptFrame* frame) {
    (void)frame;
    if (!(port_inb(KBD_STAT_PORT) & KBD_STAT_OBF)) return;
    u8 sc = port_inb(KBD_DATA_PORT);
    rb_push(sc);

    bool_t key_up = !!(sc & SC_BREAK);
    u8 code = sc & 0x7F;

    /* Update modifiers */
    if (code == SC_LSHIFT || code == SC_RSHIFT) {
        g_kbd.shift = !key_up;
    } else if (code == SC_LCTRL) {
        g_kbd.ctrl = !key_up;
    } else if (code == SC_LALT) {
        g_kbd.alt = !key_up;
    } else if (code == SC_CAPSLOCK && !key_up) {
        g_kbd.capslock = !g_kbd.capslock;
    } else if (!key_up && code < 128) {
        /* Build key event */
        bool_t shifted = g_kbd.shift ^ g_kbd.capslock;
        char ascii = shifted ? g_sc_shift[code] : g_sc_normal[code];
        if (g_kbd.ev_count < KBD_EVENT_MAX) {
            KeyEvent* ev = &g_kbd.events[(g_kbd.ev_head + g_kbd.ev_count) % KBD_EVENT_MAX];
            ev->ascii    = ascii;
            ev->scancode = code;
            ev->ctrl     = g_kbd.ctrl;
            ev->alt      = g_kbd.alt;
            ev->shift    = g_kbd.shift;
            g_kbd.ev_count++;
        }
        g_kbd.total_keys++;
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
