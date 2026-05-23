/*
 * Σ SigmaOS — sigma_printf: Sovereign Output Utility
 * Zero-Dependency: No libc. Hardware-direct formatting.
 */

typedef unsigned int   u32;
typedef unsigned char  u8;
typedef unsigned short u16;

/* Assume VGA Driver is linked */
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_puts(const char* s);

/* Base-10 Integer to String Helper */
static void print_uint(u32 val) {
    if (val == 0) {
        sigma_vga_putchar('0');
        return;
    }

    char buf[12];
    int idx = 11;
    buf[idx] = '\0';

    while (val > 0) {
        idx--;
        buf[idx] = '0' + (val % 10);
        val /= 10;
    }

    sigma_vga_puts(&buf[idx]);
}

/* Base-16 Hexadecimal to String Helper */
static void print_hex(u32 val) {
    if (val == 0) {
        sigma_vga_puts("0x0");
        return;
    }

    char buf[12];
    int idx = 11;
    buf[idx] = '\0';

    const char hex_chars[] = "0123456789ABCDEF";

    while (val > 0) {
        idx--;
        buf[idx] = hex_chars[val & 0xF];
        val >>= 4;
    }
    
    idx--;
    buf[idx] = 'x';
    idx--;
    buf[idx] = '0';

    sigma_vga_puts(&buf[idx]);
}

/* 
 * Sovereign printf logic 
 * Supported: %s (string), %u (unsigned int), %x (hex int), %c (char)
 */
extern "C" void sigma_vga_printf(const char* fmt, ...) {
    /* 
     * Since we have no standard <stdarg.h>, we manually parse the stack.
     * x86_64 ABI places args in RDI, RSI, RDX, RCX, R8, R9, then stack.
     * For a simple stub, we will assume a basic structure. 
     * Below is a conceptual placeholder for parsing args.
     */
     
    __builtin_va_list args;
    __builtin_va_start(args, fmt);

    while (*fmt) {
        if (*fmt == '%') {
            fmt++;
            if (*fmt == 's') {
                const char* s = __builtin_va_arg(args, const char*);
                sigma_vga_puts(s ? s : "(null)");
            } else if (*fmt == 'u') {
                u32 val = __builtin_va_arg(args, u32);
                print_uint(val);
            } else if (*fmt == 'x' || *fmt == 'X') {
                u32 val = __builtin_va_arg(args, u32);
                print_hex(val);
            } else if (*fmt == 'c') {
                char c = (char)__builtin_va_arg(args, int);
                sigma_vga_putchar(c);
            } else if (*fmt == '%') {
                sigma_vga_putchar('%');
            } else {
                sigma_vga_putchar('%');
                sigma_vga_putchar(*fmt);
            }
        } else {
            sigma_vga_putchar(*fmt);
        }
        fmt++;
    }

    __builtin_va_end(args);
}
