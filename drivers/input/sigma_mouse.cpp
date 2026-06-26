/*
 * Σ SigmaOS Zenith — PS/2 Mouse Driver Shard
 * Absorbs: Linux drivers/input/mouse/psmouse-base.c
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef long               i32;
typedef unsigned long long u64;

/* ─────────────── Port I/O ─────────────── */
static inline u8 sigma_inb(u16 port) {
    u8 ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void sigma_outb(u16 port, u8 val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

/* ─────────────── PS/2 Constants ─────────────── */
#define PS2_DATA    0x60
#define PS2_STATUS  0x64
#define PS2_CMD     0x64

/* ─────────────── Mouse Event ─────────────── */
struct SigmaMouseEvent {
    i32 dx;
    i32 dy;
    bool left_button;
    bool right_button;
    bool middle_button;
};

/* ─────────────── SPSC Ring Buffer ─────────────── */
#define MOUSE_RING_SIZE 64
static struct SigmaMouseEvent mouse_ring[MOUSE_RING_SIZE];
static volatile u32 mouse_head = 0;
static volatile u32 mouse_tail = 0;

/* ─────────────── Internal State ─────────────── */
static u8 mouse_cycle = 0;
static u8 mouse_bytes[3];
static i32 mouse_x = 400;  /* Start at screen center (800x600 assumption) */
static i32 mouse_y = 300;

/* ─────────────── PS/2 Controller Helpers ─────────────── */
static void ps2_wait_input() {
    u32 timeout = 100000;
    while ((sigma_inb(PS2_STATUS) & 0x02) && --timeout);
}

static void ps2_wait_output() {
    u32 timeout = 100000;
    while (!(sigma_inb(PS2_STATUS) & 0x01) && --timeout);
}

static void ps2_write_mouse(u8 data) {
    ps2_wait_input();
    sigma_outb(PS2_CMD, 0xD4);  /* Send next byte to port 2 (mouse) */
    ps2_wait_input();
    sigma_outb(PS2_DATA, data);
}

static u8 ps2_read() {
    ps2_wait_output();
    return sigma_inb(PS2_DATA);
}

/* ─────────────── API: Initialize Mouse ─────────────── */
extern "C" void sigma_mouse_init() {
    mouse_cycle = 0;
    mouse_head  = 0;
    mouse_tail  = 0;
    mouse_x     = 400;
    mouse_y     = 300;

    /* Enable auxiliary device (mouse) */
    ps2_wait_input();
    sigma_outb(PS2_CMD, 0xA8);

    /* Enable IRQ12 */
    ps2_wait_input();
    sigma_outb(PS2_CMD, 0x20); /* Read controller config */
    u8 config = ps2_read();
    config |= 0x02;            /* Enable IRQ12 (mouse interrupt) */
    config &= ~0x20;           /* Enable mouse clock */
    ps2_wait_input();
    sigma_outb(PS2_CMD, 0x60); /* Write controller config */
    ps2_wait_input();
    sigma_outb(PS2_DATA, config);

    /* Reset mouse */
    ps2_write_mouse(0xFF);
    ps2_read(); /* ACK */
    ps2_read(); /* Self-test pass (0xAA) */
    ps2_read(); /* Mouse ID */

    /* Set defaults */
    ps2_write_mouse(0xF6);
    ps2_read(); /* ACK */

    /* Enable data streaming */
    ps2_write_mouse(0xF4);
    ps2_read(); /* ACK */
}

/* ─────────────── API: IRQ12 Handler (called from IDT stub) ─────────────── */
extern "C" void sigma_mouse_irq_handler() {
    u8 byte = sigma_inb(PS2_DATA);

    switch (mouse_cycle) {
        case 0:
            /* First byte: flags */
            if (!(byte & 0x08)) break; /* Bit 3 must be set — alignment check */
            mouse_bytes[0] = byte;
            mouse_cycle = 1;
            break;
        case 1:
            mouse_bytes[1] = byte;
            mouse_cycle = 2;
            break;
        case 2:
            mouse_bytes[2] = byte;
            mouse_cycle = 0;

            /* Decode the 3-byte PS/2 packet */
            struct SigmaMouseEvent ev;
            ev.left_button   = (mouse_bytes[0] & 0x01) != 0;
            ev.right_button  = (mouse_bytes[0] & 0x02) != 0;
            ev.middle_button = (mouse_bytes[0] & 0x04) != 0;

            /* Sign-extend X movement */
            ev.dx = (i32)mouse_bytes[1];
            if (mouse_bytes[0] & 0x10) ev.dx |= (i32)0xFFFFFF00; /* X sign bit */

            /* Sign-extend Y movement (invert for screen coords) */
            ev.dy = -(i32)mouse_bytes[2];
            if (mouse_bytes[0] & 0x20) ev.dy |= (i32)0xFFFFFF00; /* Y sign bit */
            ev.dy = -ev.dy; /* PS/2 Y is inverted */

            /* Update absolute cursor */
            mouse_x += ev.dx;
            mouse_y += ev.dy;
            if (mouse_x < 0) mouse_x = 0;
            if (mouse_y < 0) mouse_y = 0;
            if (mouse_x > 799) mouse_x = 799;
            if (mouse_y > 599) mouse_y = 599;

            /* Push to ring buffer */
            u32 next = (mouse_head + 1) % MOUSE_RING_SIZE;
            if (next != mouse_tail) {
                mouse_ring[mouse_head] = ev;
                mouse_head = next;
            }
            break;
    }
}

/* ─────────────── API: Poll Mouse Event ─────────────── */
extern "C" bool sigma_mouse_poll(struct SigmaMouseEvent* out) {
    if (mouse_head == mouse_tail) return false;
    *out = mouse_ring[mouse_tail];
    mouse_tail = (mouse_tail + 1) % MOUSE_RING_SIZE;
    return true;
}

/* ─────────────── API: Get Absolute Cursor Position ─────────────── */
extern "C" void sigma_mouse_get_pos(i32* x, i32* y) {
    *x = mouse_x;
    *y = mouse_y;
}
