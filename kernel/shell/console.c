/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SERIAL DRIVER + EARLY CONSOLE (v1.0 - PURE C11)
 * =============================================================================
 * COM1 (0x3F8) 16550A UART â€ used for kernel kprintf before VGA/GPU init.
 * Baud: 115200 | 8N1 | No FIFO threshold for maximum throughput.
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "core/sigma_kernel_types.h"
#include "core/sigma_types.h"   /* compiler built-in â€ no libc */

/* =========================================================================
 * COM1 Port Map
 * ========================================================================= */
#define COM1_BASE      0x3F8
#define COM1_DATA      (COM1_BASE + 0)   /* RX/TX Data register */
#define COM1_IER       (COM1_BASE + 1)   /* Interrupt Enable */
#define COM1_FCR       (COM1_BASE + 2)   /* FIFO Control */
#define COM1_LCR       (COM1_BASE + 3)   /* Line Control */
#define COM1_MCR       (COM1_BASE + 4)   /* Modem Control */
#define COM1_LSR       (COM1_BASE + 5)   /* Line Status */
#define COM1_MSR       (COM1_BASE + 6)   /* Modem Status */
#define COM1_SCRATCH   (COM1_BASE + 7)   /* Scratch */

/* Line Status bits */
#define LSR_TX_EMPTY   BIT(5)            /* Transmit holding register empty */
#define LSR_RX_READY   BIT(0)            /* Data ready */

/* =========================================================================
 * VGA Text Mode (80Ã—25, at physical 0xB8000)
 * ========================================================================= */
#define VGA_BASE    0xB8000
#define VGA_COLS    80u
#define VGA_ROWS    25u

typedef struct VGAConsole {
    sigma_u16* buf;
    sigma_u32  col;
    sigma_u32  row;
    sigma_u8   attr;  /* colour attributes (high=bg, low=fg) */
} VGAConsole;

static VGAConsole g_vga;

static void vga_scroll(void) {
    sigma_u32 i;
    for (i = 0; i < (VGA_ROWS - 1) * VGA_COLS; i++)
        g_vga.buf[i] = g_vga.buf[i + VGA_COLS];
    for (i = (VGA_ROWS - 1) * VGA_COLS; i < VGA_ROWS * VGA_COLS; i++)
        g_vga.buf[i] = (sigma_u16)((sigma_u16)g_vga.attr << 8 | ' ');
    g_vga.row = VGA_ROWS - 1;
}

static void vga_putc(char c) {
    if (c == '\n') {
        g_vga.col = 0;
        if (++g_vga.row >= VGA_ROWS) vga_scroll();
        return;
    }
    if (c == '\r') { g_vga.col = 0; return; }
    if (c == '\t') { vga_putc(' '); vga_putc(' '); vga_putc(' '); vga_putc(' '); return; }
    g_vga.buf[g_vga.row * VGA_COLS + g_vga.col] =
        (sigma_u16)((sigma_u16)g_vga.attr << 8 | (sigma_u8)c);
    if (++g_vga.col >= VGA_COLS) {
        g_vga.col = 0;
        if (++g_vga.row >= VGA_ROWS) vga_scroll();
    }
}

static void vga_init(void) {
    g_vga.buf  = (sigma_u16*)(sigma_usize)VGA_BASE;
    g_vga.col  = 0;
    g_vga.row  = 0;
    g_vga.attr = 0x0F;   /* bright white on black */
    sigma_u32 i;
    for (i = 0; i < VGA_ROWS * VGA_COLS; i++)
        g_vga.buf[i] = (sigma_u16)((sigma_u16)g_vga.attr << 8 | ' ');
}

/* =========================================================================
 * Serial UART Init
 * ========================================================================= */
static sigma_bool g_serial_ok = SIGMA_FALSE;

void serial_init(void) {
    port_outb(COM1_IER, 0x00);   /* Disable all interrupts */
    port_outb(COM1_LCR, 0x80);   /* Enable DLAB (baud divisor) */
    port_outb(COM1_DATA, 0x01);  /* Divisor lo: 115200 baud */
    port_outb(COM1_IER,  0x00);  /* Divisor hi: 0 */
    port_outb(COM1_LCR, 0x03);   /* 8N1: 8 bits, no parity, 1 stop */
    port_outb(COM1_FCR, 0xC7);   /* Enable FIFO, 14-byte threshold */
    port_outb(COM1_MCR, 0x0B);   /* RTS+DTR+OUT2 */

    /* Loopback self-test */
    port_outb(COM1_MCR, 0x1E);
    port_outb(COM1_DATA, 0xAE);
    if (port_inb(COM1_DATA) != 0xAE) {
        g_serial_ok = SIGMA_FALSE;
        return;
    }
    port_outb(COM1_MCR, 0x0F);
    g_serial_ok = SIGMA_TRUE;
}

void serial_putc(char c) {
    if (!g_serial_ok) return;
    /* Wait for TX register empty */
    while (!(port_inb(COM1_LSR) & LSR_TX_EMPTY)) cpu_pause();
    port_outb(COM1_DATA, (sigma_u8)c);
}

static void serial_puts(const char* s) {
    while (*s) {
        if (*s == '\n') serial_putc('\r');
        serial_putc(*s++);
    }
}

/* =========================================================================
 * kprintf â€ kernel variadic kprintf (serial + VGA)
 * ========================================================================= */

static void kprint_u64(sigma_u64 v, int base) {
    const char* digits = "0123456789ABCDEF";
    char buf[20]; int i = 19;
    buf[i] = '\0';
    if (v == 0) { buf[--i] = '0'; }
    else while (v && i > 0) { buf[--i] = digits[v % (sigma_u64)base]; v /= (sigma_u64)base; }
    serial_puts(&buf[i]);
    char c2;
    for (c2 = buf[i]; c2; c2 = buf[++i]) vga_putc(c2);
}

static void kprint_str(const char* s) {
    serial_puts(s);
    while (s && *s) { vga_putc(*s++); }
}

static void kprint_char(char c) {
    serial_putc(c);
    vga_putc(c);
}

void kprintf(const char* fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    for (; *fmt; fmt++) {
        if (*fmt != '%') { kprint_char(*fmt); continue; }
        fmt++;
        switch (*fmt) {
            case 'd': {
                sigma_i64 v = va_arg(ap, sigma_i64);
                if (v < 0) { kprint_char('-'); v = -v; }
                kprint_u64((sigma_u64)v, 10);
                break;
            }
            case 'u': kprint_u64(va_arg(ap, sigma_u64), 10); break;
            case 'x': kprint_u64(va_arg(ap, sigma_u64), 16); break;
            case 'p': kprint_str("0x"); kprint_u64((sigma_u64)(sigma_usize)va_arg(ap, void*), 16); break;
            case 's': kprint_str(va_arg(ap, const char*)); break;
            case 'c': kprint_char((char)va_arg(ap, int)); break;
            case 'l':
                fmt++;
                if (*fmt == 'l') { fmt++; } /* skip 'll' */
                if (*fmt == 'u') kprint_u64(va_arg(ap, sigma_u64), 10);
                else if (*fmt == 'd') { sigma_i64 v = va_arg(ap, sigma_i64); if(v<0){kprint_char('-');v=-v;} kprint_u64((sigma_u64)v,10); }
                break;
            default:  kprint_char('%'); kprint_char(*fmt); break;
        }
    }
    va_end(ap);
}

/* =========================================================================
 * Console Init
 * ========================================================================= */
void console_init(void) {
    serial_init();
    vga_init();
    kprintf("\n");
    kprintf("Î£ ============================================================ Î£\n");
    kprintf("  SigmaOS Sovereign Kernel v1.0 â€ Pure C11 + x86_64 Assembly\n");
    kprintf("  Serial: COM1 @ 115200 baud | VGA: 80x25 Text Mode\n");
    kprintf("  Language: C11(98%%) | ASM(0.7%%) | Rust(0.3%%)\n");
    kprintf("Î£ ============================================================ Î£\n\n");
}
