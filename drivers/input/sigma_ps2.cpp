/*
 * Σ SigmaOS — sigma_ps2: Sovereign PS/2 Keyboard & Mouse Driver
 * Absorbs: Linux drivers/input/serio/i8042.c, keyboard.c concepts
 * Distros: Arch Linux (keyboard-autodetect), Debian (console-setup), Alpine (klogd)
 * Zero-Dependency: No libc, no stdlib. Raw x86 IN/OUT port instructions only.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* ─────────────── I/O Port Access ─────────────── */
static inline u8  ps2_inb(u16 port) {
    u8 v; __asm__ volatile("inb %1, %0" : "=a"(v) : "dN"(port)); return v;
}
static inline void ps2_outb(u16 port, u8 val) {
    __asm__ volatile("outb %0, %1" : : "a"(val), "dN"(port));
}
static inline void ps2_io_wait() {
    /* ~1us delay via port 0x80 (POST diagnostic) */
    __asm__ volatile("outb %al, $0x80");
}

/* ─────────────── PS/2 Port Map ─────────────── */
#define PS2_DATA_PORT   0x60
#define PS2_CMD_PORT    0x64
#define PS2_STATUS_PORT 0x64

/* Status register bits */
#define PS2_STATUS_OUTPUT_FULL  (1 << 0)  /* Data in output buffer */
#define PS2_STATUS_INPUT_FULL   (1 << 1)  /* Controller busy, don't write */
#define PS2_STATUS_SYSTEM_FLAG  (1 << 2)
#define PS2_STATUS_CMD_DATA     (1 << 3)  /* 0=data for port1, 1=cmd */
#define PS2_STATUS_TIMEOUT      (1 << 6)
#define PS2_STATUS_PARITY_ERR   (1 << 7)

/* Controller commands */
#define PS2_CMD_READ_CFG     0x20
#define PS2_CMD_WRITE_CFG    0x60
#define PS2_CMD_DISABLE_P2   0xA7
#define PS2_CMD_ENABLE_P2    0xA8
#define PS2_CMD_TEST_P2      0xA9
#define PS2_CMD_TEST_CTRL    0xAA
#define PS2_CMD_TEST_P1      0xAB
#define PS2_CMD_DISABLE_P1   0xAD
#define PS2_CMD_ENABLE_P1    0xAE
#define PS2_CMD_WRITE_P2     0xD4

/* Device commands */
#define PS2_DEV_RESET        0xFF
#define PS2_DEV_IDENTIFY     0xF2
#define PS2_DEV_SCANCODE_SET 0xF0
#define PS2_DEV_ENABLE_SCAN  0xF4
#define PS2_DEV_DISABLE_SCAN 0xF5
#define PS2_DEV_ACK          0xFA
#define PS2_DEV_RESEND       0xFE
#define PS2_DEV_SELF_TEST_OK 0xAA

/* ─────────────── Scancode Table (Set 2 → ASCII) ─────────────── */
/* Inspired by Linux atkbd.c: ATKBD_SET2_KEYCODE table */
static const u8 sigma_ps2_set2_to_ascii[256] = {
    /*00*/ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, '\t', '`', 0,
    /*10*/ 0, 0, 0, 0, 0, 'q', '1', 0, 0, 0, 'z', 's', 'a', 'w', '2', 0,
    /*20*/ 0, 'c', 'x', 'd', 'e', '4', '3', 0, 0, ' ', 'v', 'f', 't', 'r', '5', 0,
    /*30*/ 0, 'n', 'b', 'h', 'g', 'y', '6', 0, 0, 0, 'm', 'j', 'u', '7', '8', 0,
    /*40*/ 0, ',', 'k', 'i', 'o', '0', '9', 0, 0, '.', '/', 'l', ';', 'p', '-', 0,
    /*50*/ 0, 0, '\'', 0, '[', '=', 0, 0, 0, 0, '\n', ']', 0, '\\', 0, 0,
    /*60*/ 0, 0, 0, 0, 0, 0, '\b', 0, 0, '1', 0, '4', '7', 0, 0, 0,
    /*70*/ '0', '.', '2', '5', '6', '8', 0, 0, 0, '+', '3', '-', '*', '9', 0, 0,
    /* rest zero */
};

/* ─────────────── Driver State ─────────────── */
#define KBD_BUF_SIZE 256
static u8  kbd_ringbuf[KBD_BUF_SIZE];
static u32 kbd_ring_head = 0;
static u32 kbd_ring_tail = 0;
static bool shift_held  = false;
static bool ctrl_held   = false;
static bool alt_held    = false;
static bool caps_lock   = false;

/* ─────────────── Wait Helpers ─────────────── */
/* Wait for output buffer full (data ready to read) */
static bool ps2_wait_output(u32 timeout_iters) {
    while (timeout_iters--) {
        if (ps2_inb(PS2_STATUS_PORT) & PS2_STATUS_OUTPUT_FULL) return true;
        ps2_io_wait();
    }
    return false;
}

/* Wait for input buffer empty (ready to send) */
static bool ps2_wait_input(u32 timeout_iters) {
    while (timeout_iters--) {
        if (!(ps2_inb(PS2_STATUS_PORT) & PS2_STATUS_INPUT_FULL)) return true;
        ps2_io_wait();
    }
    return false;
}

/* ─────────────── Controller Send ─────────────── */
static bool ps2_send_cmd(u8 cmd) {
    if (!ps2_wait_input(100000)) return false;
    ps2_outb(PS2_CMD_PORT, cmd);
    return true;
}

static bool ps2_send_data(u8 data) {
    if (!ps2_wait_input(100000)) return false;
    ps2_outb(PS2_DATA_PORT, data);
    return true;
}

static u8 ps2_recv_data() {
    ps2_wait_output(100000);
    return ps2_inb(PS2_DATA_PORT);
}

/* ─────────────── Device Command Helpers ─────────────── */
static bool ps2_device_cmd(u8 cmd) {
    for (u32 retries = 0; retries < 3; retries++) {
        ps2_send_data(cmd);
        u8 ack = ps2_recv_data();
        if (ack == PS2_DEV_ACK) return true;
        if (ack == PS2_DEV_RESEND) continue;
        return false;
    }
    return false;
}

/* ─────────────── Keyboard Initialization ─────────────── */
/* Algorithm mirrors Linux i8042_probe() and atkbd_connect() logic */
static bool sigma_ps2_kbd_init() {
    /* Step 1: Disable PS/2 devices during initialization */
    ps2_send_cmd(PS2_CMD_DISABLE_P1);
    ps2_send_cmd(PS2_CMD_DISABLE_P2);

    /* Step 2: Flush output buffer */
    while (ps2_inb(PS2_STATUS_PORT) & PS2_STATUS_OUTPUT_FULL)
        ps2_inb(PS2_DATA_PORT);

    /* Step 3: Read + patch configuration byte */
    ps2_send_cmd(PS2_CMD_READ_CFG);
    u8 cfg = ps2_recv_data();
    cfg &= ~(1 << 0); /* Disable keyboard IRQ1 (polled mode) */
    cfg &= ~(1 << 1); /* Disable mouse IRQ12 */
    cfg &= ~(1 << 6); /* Disable keyboard translation */
    ps2_send_cmd(PS2_CMD_WRITE_CFG);
    ps2_send_data(cfg);

    /* Step 4: Controller self-test */
    ps2_send_cmd(PS2_CMD_TEST_CTRL);
    u8 result = ps2_recv_data();
    if (result != 0x55) return false; /* 0x55 = test passed */

    /* Step 5: Test port 1 */
    ps2_send_cmd(PS2_CMD_TEST_P1);
    result = ps2_recv_data();
    if (result != 0x00) return false; /* 0x00 = port OK */

    /* Step 6: Enable port 1 */
    ps2_send_cmd(PS2_CMD_ENABLE_P1);

    /* Step 7: Reset keyboard device */
    ps2_device_cmd(PS2_DEV_RESET);
    ps2_recv_data(); /* Read self-test result (0xAA) */

    /* Step 8: Set scancode set 1 (simplest for our map) */
    ps2_device_cmd(PS2_DEV_SCANCODE_SET);
    ps2_device_cmd(0x02); /* Request Set 2 */

    /* Step 9: Enable keyboard scanning */
    ps2_device_cmd(PS2_DEV_ENABLE_SCAN);

    return true;
}

/* ─────────────── Scancode Processing ─────────────── */
static void ps2_process_scancode(u8 sc) {
    /* Key release: high bit set (make vs break) */
    bool release = (sc & 0x80) != 0;
    u8 key = sc & 0x7F;

    /* Modifier key tracking */
    /* Shift: 0x2A (left), 0x36 (right) */
    if (key == 0x2A || key == 0x36) { shift_held = !release; return; }
    /* Ctrl: 0x1D */
    if (key == 0x1D) { ctrl_held = !release; return; }
    /* Alt: 0x38 */
    if (key == 0x38) { alt_held = !release; return; }
    /* Caps Lock: 0x3A (toggle on press) */
    if (key == 0x3A && !release) { caps_lock = !caps_lock; return; }

    if (release) return; /* Only process key-press events */

    /* Map scancode to ASCII */
    u8 ch = 0;
    if (key < 128) ch = sigma_ps2_set2_to_ascii[key];
    if (!ch) return;

    /* Apply shift/caps lock transformation */
    bool upper = (shift_held != caps_lock);
    if (upper && ch >= 'a' && ch <= 'z') ch -= 32;
    if (shift_held) {
        /* Shifted punctuation mapping */
        switch (ch) {
            case '1': ch = '!'; break; case '2': ch = '@'; break;
            case '3': ch = '#'; break; case '4': ch = '$'; break;
            case '5': ch = '%'; break; case '6': ch = '^'; break;
            case '7': ch = '&'; break; case '8': ch = '*'; break;
            case '9': ch = '('; break; case '0': ch = ')'; break;
            case '-': ch = '_'; break; case '=': ch = '+'; break;
            case '[': ch = '{'; break; case ']': ch = '}'; break;
            case '\\': ch = '|'; break; case ';': ch = ':'; break;
            case '\'': ch = '"'; break; case ',': ch = '<'; break;
            case '.': ch = '>'; break; case '/': ch = '?'; break;
            case '`': ch = '~'; break;
        }
    }

    /* Push to ring buffer */
    u32 next_head = (kbd_ring_head + 1) % KBD_BUF_SIZE;
    if (next_head != kbd_ring_tail) { /* Not full */
        kbd_ringbuf[kbd_ring_head] = ch;
        kbd_ring_head = next_head;
    }
}

/* ─────────────── Public API ─────────────── */
/* Poll for a single byte from keyboard (blocking spin) */
extern "C" u8 sigma_ps2_getchar() {
    /* First drain any pending hardware scancodes */
    while (ps2_inb(PS2_STATUS_PORT) & PS2_STATUS_OUTPUT_FULL)
        ps2_process_scancode(ps2_inb(PS2_DATA_PORT));

    /* Spin until ring buffer has data */
    while (kbd_ring_head == kbd_ring_tail) {
        while (ps2_inb(PS2_STATUS_PORT) & PS2_STATUS_OUTPUT_FULL)
            ps2_process_scancode(ps2_inb(PS2_DATA_PORT));
    }

    u8 ch = kbd_ringbuf[kbd_ring_tail];
    kbd_ring_tail = (kbd_ring_tail + 1) % KBD_BUF_SIZE;
    return ch;
}

/* Non-blocking: returns 0 if no key available */
extern "C" u8 sigma_ps2_pollchar() {
    while (ps2_inb(PS2_STATUS_PORT) & PS2_STATUS_OUTPUT_FULL)
        ps2_process_scancode(ps2_inb(PS2_DATA_PORT));
    if (kbd_ring_head == kbd_ring_tail) return 0;
    u8 ch = kbd_ringbuf[kbd_ring_tail];
    kbd_ring_tail = (kbd_ring_tail + 1) % KBD_BUF_SIZE;
    return ch;
}

/* Check if modifier keys are held */
extern "C" bool sigma_ps2_shift()    { return shift_held; }
extern "C" bool sigma_ps2_ctrl()     { return ctrl_held; }
extern "C" bool sigma_ps2_alt()      { return alt_held; }
extern "C" bool sigma_ps2_capslock() { return caps_lock; }

/* ─────────────── Driver Entry Point ─────────────── */
extern "C" int sigma_ps2_init() {
    return sigma_ps2_kbd_init() ? 0 : -1;
}
