#include "sigma_keyboard.h"
#include "sigma_kernel_types.h" // For port_inb, etc. if available, or we'll declare them

// We need port I/O functions. If sigma_kernel_types.h doesn't define them properly,
// we define them here.
static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ( "inb %1, %0" : "=a"(ret) : "Nd"(port) );
    return ret;
}

static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ( "outb %0, %1" : : "a"(val), "Nd"(port) );
}

// Scancode Set 1 (US QWERTY)
static const char kbd_us[128] = {
    0,  27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0,  'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',   0,
    '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',   0, '*',
    0,  ' '
};

static const char kbd_us_shift[128] = {
    0,  27, '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\b',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    0,  'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '\"', '~',   0,
    '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?',   0, '*',
    0,  ' '
};

// Circular buffer
static char kbd_buffer[KBD_BUFFER_SIZE];
static volatile int kbd_head = 0;
static volatile int kbd_tail = 0;

// Modifier states
static int shift_pressed = 0;
static int caps_lock = 0;

void sigma_keyboard_init(void) {
    // Usually IRQ1 is unmasked in the PIC initialized elsewhere,
    // but we can flush the current buffer just in case.
    while (inb(0x64) & 1) {
        inb(0x60);
    }
    kbd_head = 0;
    kbd_tail = 0;
}

void sigma_keyboard_handler(void) {
    uint8_t scancode = inb(0x60);

    // Handle shift keys
    if (scancode == 0x2A || scancode == 0x36) {
        shift_pressed = 1;
        return;
    }
    if (scancode == 0xAA || scancode == 0xB6) {
        shift_pressed = 0;
        return;
    }
    
    // Handle caps lock (press only)
    if (scancode == 0x3A) {
        caps_lock = !caps_lock;
        return;
    }

    // Ignore other key releases (high bit set)
    if (scancode & 0x80) {
        return;
    }

    char c = 0;
    if (scancode < 128) {
        if (shift_pressed) {
            c = kbd_us_shift[scancode];
        } else {
            c = kbd_us[scancode];
        }
        
        // Apply caps lock for letters
        if (caps_lock && c >= 'a' && c <= 'z' && !shift_pressed) {
            c -= 32;
        } else if (caps_lock && c >= 'A' && c <= 'Z' && shift_pressed) {
            c += 32;
        }
    }

    if (c) {
        int next_head = (kbd_head + 1) % KBD_BUFFER_SIZE;
        if (next_head != kbd_tail) {
            kbd_buffer[kbd_head] = c;
            kbd_head = next_head;
        }
    }
}

char sigma_keyboard_read(void) {
    if (kbd_head == kbd_tail) {
        return 0; // Buffer empty
    }
    char c = kbd_buffer[kbd_tail];
    kbd_tail = (kbd_tail + 1) % KBD_BUFFER_SIZE;
    return c;
}

// Assume we have some form of yield or we can just busy wait with hlt
char sigma_keyboard_getchar(void) {
    char c;
    while ((c = sigma_keyboard_read()) == 0) {
        __asm__ volatile ("hlt"); // Wait for interrupt
    }
    return c;
}
