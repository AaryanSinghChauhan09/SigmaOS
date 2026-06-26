/*
 * Σ SigmaOS Zenith — PS/2 Keyboard Driver Shard
 * Absorbs: Linux drivers/input/serio/i8042.c, Arch Linux minimal input philosophy
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* ─────────────── PS/2 Controller Ports ─────────────── */
#define PS2_DATA_PORT    0x60
#define PS2_STATUS_PORT  0x64
#define PS2_COMMAND_PORT 0x64

/* Status register bits */
#define PS2_STATUS_OUTPUT_FULL  0x01
#define PS2_STATUS_INPUT_FULL   0x02

/* ─────────────── Port I/O (No headers) ─────────────── */
static inline u8 sigma_inb(u16 port) {
    u8 ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void sigma_outb(u16 port, u8 val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

/* ─────────────── US QWERTY Scan Code Map (Set 1) ─────────────── */
/* Inspired by Linux drivers/input/keyboard/atkbd.c scancode tables */
static const char scancode_normal[128] = {
    0,   27, '1','2','3','4','5','6','7','8','9','0','-','=','\b',
    '\t','q','w','e','r','t','y','u','i','o','p','[',']','\n',
    0,   'a','s','d','f','g','h','j','k','l',';','\'','`',
    0,   '\\','z','x','c','v','b','n','m',',','.','/',0,
    '*', 0,  ' ', 0,
    0,0,0,0,0,0,0,0,0,0, /* F1-F10 */
    0, 0, /* Num Lock, Scroll Lock */
    '7','8','9','-','4','5','6','+','1','2','3','0','.',
    0,0,0,
    0,0, /* F11, F12 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
};

static const char scancode_shift[128] = {
    0,   27, '!','@','#','$','%','^','&','*','(',')','_','+','\b',
    '\t','Q','W','E','R','T','Y','U','I','O','P','{','}','\n',
    0,   'A','S','D','F','G','H','J','K','L',':','"','~',
    0,   '|','Z','X','C','V','B','N','M','<','>','?',0,
    '*', 0,  ' ', 0,
    0,0,0,0,0,0,0,0,0,0,
    0, 0,
    '7','8','9','-','4','5','6','+','1','2','3','0','.',
    0,0,0,
    0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
};

/* ─────────────── Keyboard State ─────────────── */
struct SigmaKeyboardState {
    bool shift_held;
    bool ctrl_held;
    bool alt_held;
    bool caps_lock;
    bool num_lock;
    bool initialized;
};

static struct SigmaKeyboardState kbd_state;

/* ─────────────── Key Event Ring Buffer (SPSC) ─────────────── */
struct SigmaKeyEvent {
    char ascii;
    u8   scancode;
    bool pressed;     /* true = key down, false = key up */
    bool shift;
    bool ctrl;
    bool alt;
};

#define KBD_RING_SIZE 128
static struct SigmaKeyEvent kbd_ring[KBD_RING_SIZE];
static volatile u32 kbd_ring_head = 0;
static volatile u32 kbd_ring_tail = 0;

static void kbd_ring_push(struct SigmaKeyEvent* ev) {
    u32 next = (kbd_ring_head + 1) % KBD_RING_SIZE;
    if (next == kbd_ring_tail) return; /* Buffer full, drop */
    kbd_ring[kbd_ring_head] = *ev;
    kbd_ring_head = next;
}

/* ─────────────── API: Initialize Keyboard ─────────────── */
extern "C" void sigma_kbd_init() {
    kbd_state.shift_held  = false;
    kbd_state.ctrl_held   = false;
    kbd_state.alt_held    = false;
    kbd_state.caps_lock   = false;
    kbd_state.num_lock    = false;
    kbd_state.initialized = true;
    kbd_ring_head = 0;
    kbd_ring_tail = 0;

    /* Flush the PS/2 output buffer */
    while (sigma_inb(PS2_STATUS_PORT) & PS2_STATUS_OUTPUT_FULL)
        sigma_inb(PS2_DATA_PORT);

    /* Enable keyboard (send 0xAE to controller) */
    sigma_outb(PS2_COMMAND_PORT, 0xAE);
}

/* ─────────────── API: IRQ1 Handler (called from IDT stub) ─────────────── */
extern "C" void sigma_kbd_irq_handler() {
    u8 scancode = sigma_inb(PS2_DATA_PORT);
    bool released = (scancode & 0x80) != 0;
    u8 code = scancode & 0x7F;

    /* Modifier tracking */
    if (code == 0x2A || code == 0x36) { kbd_state.shift_held = !released; return; }
    if (code == 0x1D)                 { kbd_state.ctrl_held  = !released; return; }
    if (code == 0x38)                 { kbd_state.alt_held   = !released; return; }
    if (code == 0x3A && !released)    { kbd_state.caps_lock  = !kbd_state.caps_lock; return; }
    if (code == 0x45 && !released)    { kbd_state.num_lock   = !kbd_state.num_lock;  return; }

    /* Translate scancode → ASCII */
    char ch;
    if (kbd_state.shift_held)
        ch = scancode_shift[code];
    else
        ch = scancode_normal[code];

    /* Caps Lock: flip case for alphabetic */
    if (kbd_state.caps_lock && ch >= 'a' && ch <= 'z') ch -= 32;
    else if (kbd_state.caps_lock && ch >= 'A' && ch <= 'Z') ch += 32;

    struct SigmaKeyEvent ev;
    ev.ascii    = ch;
    ev.scancode = code;
    ev.pressed  = !released;
    ev.shift    = kbd_state.shift_held;
    ev.ctrl     = kbd_state.ctrl_held;
    ev.alt      = kbd_state.alt_held;

    kbd_ring_push(&ev);
}

/* ─────────────── API: Poll for Key Event ─────────────── */
extern "C" bool sigma_kbd_poll(struct SigmaKeyEvent* out) {
    if (kbd_ring_head == kbd_ring_tail) return false;
    *out = kbd_ring[kbd_ring_tail];
    kbd_ring_tail = (kbd_ring_tail + 1) % KBD_RING_SIZE;
    return true;
}

/* ─────────────── API: Blocking Read (single character) ─────────────── */
extern "C" char sigma_kbd_getchar() {
    struct SigmaKeyEvent ev;
    while (true) {
        if (sigma_kbd_poll(&ev)) {
            if (ev.pressed && ev.ascii != 0) return ev.ascii;
        }
        __asm__ volatile ("hlt"); /* Wait for next interrupt */
    }
}
