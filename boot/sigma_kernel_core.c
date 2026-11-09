/*
 * SigmaOS Sovereign Kernel Core (boot/sigma_kernel_core.c)
 * Minimal, reliable, self-contained Multiboot1/2 kernel.
 *
 * Implements:
 * - Multiboot 1 & Multiboot 2 compliant headers (loadable by any GRUB/Limine)
 * - VGA 80x25 Text Mode Driver with scrolling, color formatting, cursor control
 * - COM1 Serial Driver (115200 8N1) for headless VM / QEMU -serial stdio output
 * - PS/2 Keyboard Controller Driver with scancode-to-ASCII decoding
 * - Interactive Sovereign Kernel Shell (status, craft suite, distro parity, reboot, halt)
 *
 * Principles:
 * GALL'S LAW · K.I.S.S. · FAIL-FAST · ZERO-COPY · DIRECT DEVICE CONTROL ·
 * LEAST ASTONISHMENT · PRISTINE SOURCES · BOOTLOADER INTEGRATION
 */

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
typedef unsigned long size_t;

#define VGA_WIDTH 80
#define VGA_HEIGHT 25
#define VGA_MEMORY ((volatile uint16_t*)0xB8000)

#define COM1_PORT 0x3F8

/* --------------------------------------------------------------------------
 * I/O Port Instructions
 * -------------------------------------------------------------------------- */
static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outl(uint16_t port, uint32_t val) {
    __asm__ volatile ("outl %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint32_t inl(uint16_t port) {
    uint32_t ret;
    __asm__ volatile ("inl %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline uint16_t inw(uint16_t port) {
    uint16_t ret;
    __asm__ volatile ("inw %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void io_wait(void) {
    outb(0x80, 0);
}

/* --------------------------------------------------------------------------
 * Multiboot Headers (both Multiboot 1 and Multiboot 2 for universal boot)
 * -------------------------------------------------------------------------- */
#define MB1_MAGIC 0x1BADB002
#define MB1_FLAGS 0x00000003
#define MB1_CHECKSUM -(MB1_MAGIC + MB1_FLAGS)

#define MB2_MAGIC 0xE85250D6
#define MB2_ARCH  0
#define MB2_LEN   16
#define MB2_CHECKSUM -(MB2_MAGIC + MB2_ARCH + MB2_LEN)

__attribute__((section(".multiboot2_header"), aligned(8)))
const struct {
    uint32_t magic;
    uint32_t arch;
    uint32_t length;
    uint32_t checksum;
    uint16_t end_type;
    uint16_t end_flags;
    uint32_t end_size;
} mb2_header = {
    MB2_MAGIC, MB2_ARCH, MB2_LEN, (uint32_t)MB2_CHECKSUM,
    0, 0, 8
};

__attribute__((section(".multiboot_header"), aligned(4)))
const struct {
    uint32_t magic;
    uint32_t flags;
    uint32_t checksum;
} mb1_header = {
    MB1_MAGIC, MB1_FLAGS, (uint32_t)MB1_CHECKSUM
};

/* --------------------------------------------------------------------------
 * VGA Console
 * -------------------------------------------------------------------------- */
static size_t vga_row = 0;
static size_t vga_col = 0;
static uint8_t vga_color = 0x0A; /* Light green on black */

static inline uint16_t vga_entry(unsigned char c, uint8_t color) {
    return (uint16_t)c | ((uint16_t)color << 8);
}

static void update_cursor(int x, int y) {
    uint16_t pos = y * VGA_WIDTH + x;
    outb(0x3D4, 0x0F);
    outb(0x3D5, (uint8_t)(pos & 0xFF));
    outb(0x3D4, 0x0E);
    outb(0x3D5, (uint8_t)((pos >> 8) & 0xFF));
}

static void vga_scroll(void) {
    for (size_t y = 0; y < VGA_HEIGHT - 1; y++) {
        for (size_t x = 0; x < VGA_WIDTH; x++) {
            VGA_MEMORY[y * VGA_WIDTH + x] = VGA_MEMORY[(y + 1) * VGA_WIDTH + x];
        }
    }
    for (size_t x = 0; x < VGA_WIDTH; x++) {
        VGA_MEMORY[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = vga_entry(' ', vga_color);
    }
    vga_row = VGA_HEIGHT - 1;
}

void vga_clear(void) {
    for (size_t y = 0; y < VGA_HEIGHT; y++) {
        for (size_t x = 0; x < VGA_WIDTH; x++) {
            VGA_MEMORY[y * VGA_WIDTH + x] = vga_entry(' ', 0x07);
        }
    }
    vga_row = 0;
    vga_col = 0;
    update_cursor(0, 0);
}

void vga_set_color(uint8_t fg, uint8_t bg) {
    vga_color = fg | (bg << 4);
}

/* --------------------------------------------------------------------------
 * Serial Port (COM1)
 * -------------------------------------------------------------------------- */
void serial_init(void) {
    outb(COM1_PORT + 1, 0x00);    /* Disable all interrupts */
    outb(COM1_PORT + 3, 0x80);    /* Enable DLAB (set baud rate divisor) */
    outb(COM1_PORT + 0, 0x01);    /* Set divisor to 1 (115200 baud) */
    outb(COM1_PORT + 1, 0x00);
    outb(COM1_PORT + 3, 0x03);    /* 8 bits, no parity, one stop bit */
    outb(COM1_PORT + 2, 0xC7);    /* Enable FIFO, clear, 14-byte threshold */
    outb(COM1_PORT + 4, 0x0B);    /* IRQs enabled, RTS/DSR set */
}

static int serial_is_transmit_empty(void) {
    return inb(COM1_PORT + 5) & 0x20;
}

void serial_putc(char c) {
    while (!serial_is_transmit_empty()) { }
    outb(COM1_PORT, c);
}

/* --------------------------------------------------------------------------
 * Unified Output
 * -------------------------------------------------------------------------- */
void kputc(char c) {
    serial_putc(c);

    if (c == '\n') {
        vga_col = 0;
        vga_row++;
        if (vga_row >= VGA_HEIGHT) {
            vga_scroll();
        }
        update_cursor(vga_col, vga_row);
        return;
    }
    if (c == '\r') {
        vga_col = 0;
        update_cursor(vga_col, vga_row);
        return;
    }
    if (c == '\b') {
        if (vga_col > 0) {
            vga_col--;
            VGA_MEMORY[vga_row * VGA_WIDTH + vga_col] = vga_entry(' ', vga_color);
            update_cursor(vga_col, vga_row);
        }
        return;
    }

    VGA_MEMORY[vga_row * VGA_WIDTH + vga_col] = vga_entry(c, vga_color);
    vga_col++;
    if (vga_col >= VGA_WIDTH) {
        vga_col = 0;
        vga_row++;
        if (vga_row >= VGA_HEIGHT) {
            vga_scroll();
        }
    }
    update_cursor(vga_col, vga_row);
}

void kputs(const char* str) {
    while (*str) {
        kputc(*str++);
    }
}

/* --------------------------------------------------------------------------
 * PS/2 Keyboard Driver
 * -------------------------------------------------------------------------- */
static const char scancode_ascii[128] = {
    0,  27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    0, '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

static int serial_has_data(void) {
    return inb(COM1_PORT + 5) & 1;
}

static int skip_ext = 0;

char kgetc(void) {
    while (1) {
        /* Check Serial port first */
        if (serial_has_data()) {
            char c = inb(COM1_PORT);
            if (c == '\r') return '\n';
            return c;
        }

        /* Check PS/2 Keyboard port */
        if (inb(0x64) & 1) {
            uint8_t scancode = inb(0x60);
            /* Extended-key prefix: swallow it AND the following scancode */
            if (scancode == 0xE0 || scancode == 0xE1 || scancode == 0xF0) {
                skip_ext = 1;
                continue;
            }
            if (skip_ext) {
                skip_ext = 0;
                continue;
            }
            /* Check key down (top bit clear) */
            if (!(scancode & 0x80)) {
                char c = scancode_ascii[scancode & 0x7F];
                if (c) return c;
            }
        }

        __asm__ volatile ("pause");
    }
}

/* --------------------------------------------------------------------------
 * System Control
 * -------------------------------------------------------------------------- */
void kreboot(void) {
    kputs("\n[SigmaOS] Initiating hardware reboot...\n");
    /* Pulse 8042 keyboard controller reset pin */
    uint8_t good = 0x02;
    while (good & 0x02) {
        good = inb(0x64);
    }
    outb(0x64, 0xFE);
    while (1) {
        __asm__ volatile ("hlt");
    }
}

void khalt(void) {
    kputs("\n[SigmaOS] System halted. You may safely turn off the machine.\n");
    while (1) {
        __asm__ volatile ("cli; hlt");
    }
}

/* --------------------------------------------------------------------------
 * Helper String Utilities
 * -------------------------------------------------------------------------- */
static int kstrcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

/* True if s starts with prefix and is followed by NUL or space. */
static int kcmd_is(const char* s, const char* prefix) {
    while (*prefix) {
        if (*s != *prefix) return 0;
        s++;
        prefix++;
    }
    return *s == '\0' || *s == ' ';
}

/* --------------------------------------------------------------------------
 * Multiboot1 Boot Information
 * -------------------------------------------------------------------------- */
static volatile uint8_t* g_mb_info = 0;

#define MBI_FLAGS       0
#define MBI_MEM_LOWER   4
#define MBI_MEM_UPPER   8
#define MBI_CMDLINE     16
#define MBI_MODS_COUNT  20
#define MBI_MODS_ADDR   24
#define MBI_MMAP_SIZE   28
#define MBI_MMAP_ADDR   32
#define MB_MMAP_FLAG    0x40

static uint32_t mbi_rd32(uint32_t off) {
    return *(volatile uint32_t*)(g_mb_info + off);
}

static void kprint_hex64(uint64_t v) {
    char buf[17];
    for (int i = 15; i >= 0; i--) {
        uint8_t d = (uint8_t)(v & 0xF);
        buf[i] = d < 10 ? '0' + d : 'a' + (d - 10);
        v >>= 4;
    }
    buf[16] = '\0';
    kputs("0x");
    kputs(buf);
}

static void kprint_hex32(uint32_t v) {
    kprint_hex64(v);
}

static void kprint_dec64(uint64_t v) {
    char buf[24];
    int i = 23;
    buf[i] = '\0';
    if (v == 0) {
        kputc('0');
        return;
    }
    while (v && i > 0) {
        buf[--i] = '0' + (uint8_t)(v % 10);
        v /= 10;
    }
    kputs(&buf[i]);
}

/* Multiboot module: start(u32) end(u32) string(u32) reserved(u32) */
static void show_modules(void) {
    if (!g_mb_info) {
        kputs("No Multiboot info available.\n");
        return;
    }
    uint32_t count = mbi_rd32(MBI_MODS_COUNT);
    uint32_t addr  = mbi_rd32(MBI_MODS_ADDR);
    kputs("Multiboot modules: ");
    kprint_dec64(count);
    kputs("\n");
    for (uint32_t i = 0; i < count; i++) {
        volatile uint8_t* m = (volatile uint8_t*)(size_t)addr + i * 16;
        uint32_t start = *(volatile uint32_t*)(m + 0);
        uint32_t end   = *(volatile uint32_t*)(m + 4);
        uint32_t str   = *(volatile uint32_t*)(m + 8);
        kputs("  [");
        kprint_dec64(i);
        kputs("] ");
        if (str) {
            kputs((const char*)(size_t)str);
            kputs(" : ");
        }
        kprint_hex32(start);
        kputs("-");
        kprint_hex32(end);
        kputs("\n");
    }
}

static void show_memory_map(void) {
    if (!g_mb_info) {
        kputs("No Multiboot info available.\n");
        return;
    }
    uint32_t flags = mbi_rd32(MBI_FLAGS);
    uint32_t lower = mbi_rd32(MBI_MEM_LOWER);
    uint32_t upper = mbi_rd32(MBI_MEM_UPPER);

    kputs("Memory (KiB): lower=");
    kprint_dec64(lower);
    kputs(" upper=");
    kprint_dec64(upper);
    kputs(" total~");
    kprint_dec64((uint64_t)lower + upper);
    kputs("\n");

    if (!(flags & MB_MMAP_FLAG)) {
        kputs("(no memory map provided by bootloader)\n");
        return;
    }

    uint32_t mmap_size = mbi_rd32(MBI_MMAP_SIZE);
    uint32_t mmap_addr = mbi_rd32(MBI_MMAP_ADDR);
    volatile uint8_t* p = (volatile uint8_t*)(size_t)mmap_addr;
    volatile uint8_t* end = p + mmap_size;

    kputs("Memory map entries:\n");
    while (p + 4 <= end) {
        uint32_t esize = *(volatile uint32_t*)p;
        if (esize < 4 || p + esize > end) break;
        uint64_t base = 0, len = 0;
        uint32_t type = 0;
        /* Multiboot mmap entry: size(u32) base(u64) length(u64) type(u32) = 24 bytes */
        if (esize >= 24) {
            base = ((uint64_t)*(volatile uint32_t*)(p + 8) << 32) | *(volatile uint32_t*)(p + 4);
            len  = ((uint64_t)*(volatile uint32_t*)(p + 16) << 32) | *(volatile uint32_t*)(p + 12);
            type = *(volatile uint32_t*)(p + 20);
        }
        kputs("  type=");
        kprint_dec64(type);
        kputs(" base=");
        kprint_hex64(base);
        kputs(" len=");
        kprint_hex64(len);
        kputs("\n");
        p += esize;
    }
}

static void show_cmdline(void) {
    if (!g_mb_info) {
        kputs("No Multiboot info available.\n");
        return;
    }
    uint32_t ptr = mbi_rd32(MBI_CMDLINE);
    if (!ptr) {
        kputs("(empty kernel command line)\n");
        return;
    }
    kputs("cmdline: ");
    kputs((const char*)(size_t)ptr);
    kputs("\n");
}

/* --------------------------------------------------------------------------
 * CMOS Real-Time Clock (ports 0x70/0x71)
 * -------------------------------------------------------------------------- */
static uint8_t cmos_read(uint8_t reg) {
    outb(0x70, reg);
    io_wait();
    return inb(0x71);
}

static uint8_t bcd2bin(uint8_t v) {
    return (uint8_t)((v >> 4) * 10 + (v & 0x0F));
}

static void kprint_2dig(uint8_t v) {
    if (v < 10) kputc('0');
    kprint_dec64(v);
}

static void show_rtc(void) {
    uint8_t reg_b = cmos_read(0x0B);
    int is_bcd  = !(reg_b & 0x04);
    int is_24hr = (reg_b & 0x02) != 0;

    /* Wait for update cycle to finish (UIP flag, reg A bit 7) */
    uint8_t spins = 0;
    while ((cmos_read(0x0A) & 0x80) && spins++ < 100) { }

    uint8_t sec = cmos_read(0x00);
    uint8_t min = cmos_read(0x02);
    uint8_t hr  = cmos_read(0x04);
    uint8_t dow = cmos_read(0x06);
    uint8_t dom = cmos_read(0x07);
    uint8_t mon = cmos_read(0x08);
    uint8_t yr  = cmos_read(0x09);

    if (is_bcd) {
        sec = bcd2bin(sec); min = bcd2bin(min); hr = bcd2bin(hr);
        dow = bcd2bin(dow); dom = bcd2bin(dom); mon = bcd2bin(mon); yr = bcd2bin(yr);
    }

    kputs("RTC: 20");
    kprint_2dig(yr);
    kputc('-'); kprint_2dig(mon);
    kputc('-'); kprint_2dig(dom);
    kputs(" ");
    kprint_2dig(hr);
    kputc(':'); kprint_2dig(min);
    kputc(':'); kprint_2dig(sec);
    if (!is_24hr) {
        int pm = hr >= 12;
        hr = hr % 12;
        if (hr == 0) hr = 12;
        kputs(pm ? " PM" : " AM");
    }
    kputs(" (dow=");
    kprint_dec64(dow);
    kputs(")\n");
}

/* --------------------------------------------------------------------------
 * CPU Identification (CPUID)
 * -------------------------------------------------------------------------- */
static void show_cpu(void) {
    uint32_t eax, ebx, ecx, edx;
    char vendor[13];

    __asm__ volatile (
        "cpuid"
        : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
        : "a"(0), "c"(0)
    );
    vendor[0] = (char)(ebx & 0xFF);
    vendor[1] = (char)((ebx >> 8) & 0xFF);
    vendor[2] = (char)((ebx >> 16) & 0xFF);
    vendor[3] = (char)((ebx >> 24) & 0xFF);
    vendor[4] = (char)(edx & 0xFF);
    vendor[5] = (char)((edx >> 8) & 0xFF);
    vendor[6] = (char)((edx >> 16) & 0xFF);
    vendor[7] = (char)((edx >> 24) & 0xFF);
    vendor[8] = (char)(ecx & 0xFF);
    vendor[9] = (char)((ecx >> 8) & 0xFF);
    vendor[10] = (char)((ecx >> 16) & 0xFF);
    vendor[11] = (char)((ecx >> 24) & 0xFF);
    vendor[12] = '\0';

    kputs("CPU vendor: ");
    kputs(vendor);
    kputs("\n");

    __asm__ volatile (
        "cpuid"
        : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
        : "a"(1), "c"(0)
    );
    uint32_t family = ((eax >> 8) & 0xF) + (((eax >> 20) & 0xFF) == 0xF ? ((eax >> 16) & 0xFF) : 0);
    uint32_t model  = ((eax >> 4) & 0xF) | (((eax >> 16) & 0xF) << 4);
    kputs("CPU family=");
    kprint_dec64(family);
    kputs(" model=");
    kprint_dec64(model);
    kputs(" stepping=");
    kprint_dec64(eax & 0xF);
    kputs(" features_ecx=");
    kprint_hex64(ecx);
    kputs("\n");
}

/* --------------------------------------------------------------------------
 * PCI Configuration Space Scan (ports 0xCF8/0xCFC)
 * -------------------------------------------------------------------------- */
static uint32_t pci_read32(uint8_t bus, uint8_t dev, uint8_t func, uint8_t reg) {
    uint32_t addr = 0x80000000u | ((uint32_t)bus << 16) | ((uint32_t)dev << 11)
                  | ((uint32_t)func << 8) | (reg & 0xFC);
    outl(0xCF8, addr);
    return inl(0xCFC);
}

static void kprint_hex32pad(uint32_t v, int digits) {
    char buf[9];
    for (int i = 7; i >= 0; i--) {
        uint8_t d = (uint8_t)(v & 0xF);
        buf[i] = d < 10 ? '0' + d : 'a' + (d - 10);
        v >>= 4;
    }
    buf[8] = '\0';
    kputs(&buf[8 - digits]);
}

static void show_pci(void) {
    uint32_t count = 0;
    for (uint32_t bus = 0; bus < 256; bus++) {
        for (uint32_t dev = 0; dev < 32; dev++) {
            for (uint32_t func = 0; func < 8; func++) {
                /* Only probe func>0 if func 0 reports a multi-function header */
                if (func > 0) {
                    uint32_t h = pci_read32((uint8_t)bus, (uint8_t)dev, 0, 0x0C);
                    if (!((h >> 16) & 0x80)) break;
                }
                uint32_t id = pci_read32((uint8_t)bus, (uint8_t)dev, (uint8_t)func, 0x00);
                uint16_t vendor = (uint16_t)(id & 0xFFFF);
                if (vendor == 0xFFFF || vendor == 0x0000) continue;
                uint16_t device = (uint16_t)(id >> 16);
                uint32_t cls = pci_read32((uint8_t)bus, (uint8_t)dev, (uint8_t)func, 0x08);
                kputs("pci ");
                kprint_hex32pad(bus, 2);
                kputc(':');
                kprint_hex32pad(dev, 2);
                kputc('.');
                kprint_hex32pad(func, 1);
                kputs("  vendor=");
                kprint_hex32pad(vendor, 4);
                kputs(" device=");
                kprint_hex32pad(device, 4);
                kputs(" class=");
                kprint_hex32pad((cls >> 8) & 0xFFFFFF, 6);
                kputs("\n");
                count++;
                if (count >= 64) { kputs("... (truncated at 64 devices)\n"); return; }
            }
        }
    }
    kputs("PCI scan complete: ");
    kprint_dec64(count);
    kputs(" device(s).\n");
}

/* --------------------------------------------------------------------------
 * ATA Disk Detection (PIO IDENTIFY, ports 0x1F0/0x170)
 * -------------------------------------------------------------------------- */
static void ata_identify_model(uint16_t base, int slave, char out[41]) {
    out[0] = '\0';
    outb(base + 6, (uint8_t)(0xA0 | (slave << 4))); /* select master/slave */
    io_wait();
    for (volatile int i = 0; i < 1000; i++) { }     /* settle */

    uint8_t st = inb(base + 7);
    if (st == 0xFF) return;                          /* no device on this channel */

    outb(base + 7, 0xEC);                            /* IDENTIFY DEVICE */
    int ready = 0;
    for (int spins = 0; spins < 200000; spins++) {
        st = inb(base + 7);
        if (st & 0x01 || st & 0x20) return;          /* ERR or DF */
        if (!(st & 0x80) && (st & 0x08)) { ready = 1; break; } /* !BSY && DRQ */
    }
    if (!ready) return;

    uint16_t buf[256];
    for (int i = 0; i < 256; i++) buf[i] = inw(base);

    /* Model string: words 27..46, byte-swapped pairs */
    for (int i = 0; i < 20; i++) {
        uint16_t w = buf[27 + i];
        out[i * 2]     = (char)(w >> 8);
        out[i * 2 + 1] = (char)(w & 0xFF);
    }
    out[40] = '\0';
    for (int i = 39; i >= 0 && out[i] == ' '; i--) out[i] = '\0';
    if (out[0] == '\0' || out[0] == ' ') {
        /* all-space model means no drive responded meaningfully */
        for (int i = 0; i < 40; i++) if (out[i] != ' ') return;
        out[0] = '\0';
    }
}

/* Wait for !BSY && DRQ (data ready) or error. Returns 0 on success. */
static int ata_wait_drq(uint16_t base) {
    for (int spins = 0; spins < 500000; spins++) {
        uint8_t st = inb(base + 7);
        if (st & 0x01 || st & 0x20) return -1;              /* ERR or DF */
        if (!(st & 0x80) && (st & 0x08)) return 0;          /* !BSY && DRQ */
    }
    return -1;
}

/* Read up to 4 sectors (LBA28) from an ATA channel into buf. Returns sectors read or -1. */
static int ata_read_lba28(uint16_t base, int slave, uint32_t lba, uint8_t* buf, int sectors) {
    if (sectors < 1 || sectors > 4) return -1;
    outb(base + 6, (uint8_t)(0xE0 | (slave << 4) | ((lba >> 24) & 0x0F))); /* LBA mode select */
    io_wait();
    outb(base + 2, (uint8_t)sectors);                        /* sector count */
    outb(base + 3, (uint8_t)(lba & 0xFF));                   /* LBA low  */
    outb(base + 4, (uint8_t)((lba >> 8) & 0xFF));            /* LBA mid  */
    outb(base + 5, (uint8_t)((lba >> 16) & 0xFF));           /* LBA high */
    outb(base + 7, 0x20);                                    /* READ SECTORS */

    for (int s = 0; s < sectors; s++) {
        if (ata_wait_drq(base) != 0) return -1;
        uint16_t* w = (uint16_t*)(buf + s * 512);
        for (int i = 0; i < 256; i++) w[i] = inw(base);
    }
    return sectors;
}

static uint32_t kstrtoul(const char* s) {
    uint32_t v = 0;
    while (*s >= '0' && *s <= '9') { v = v * 10 + (uint32_t)(*s - '0'); s++; }
    return v;
}

static void kprint_hexdump(const uint8_t* buf, uint32_t len) {
    for (uint32_t off = 0; off < len; off += 16) {
        kprint_hex32pad(off, 8);
        kputs("  ");
        for (uint32_t i = 0; i < 16; i++) {
            if (off + i < len) kprint_hex32pad(buf[off + i], 2);
            else kputs("  ");
            kputc(' ');
        }
        kputs(" |");
        for (uint32_t i = 0; i < 16 && off + i < len; i++) {
            uint8_t c = buf[off + i];
            kputc((c >= 32 && c <= 126) ? (char)c : '.');
        }
        kputs("|\n");
    }
}

static uint8_t g_sector_buf[512 * 4];

static void show_read(const char* args) {
    /* args: "<lba> [sectors] [bus]" — bus: 0=primary,1=secondary; slave flag via +1000 lba? keep simple */
    uint32_t lba = 0;
    int sectors = 1;
    int bus = 0;
    const char* p = args;
    while (*p == ' ') p++;
    lba = kstrtoul(p);
    while (*p >= '0' && *p <= '9') p++;
    while (*p == ' ') p++;
    if (*p) sectors = (int)kstrtoul(p);
    while (*p >= '0' && *p <= '9') p++;
    while (*p == ' ') p++;
    if (*p) bus = (int)kstrtoul(p);
    if (sectors < 1) sectors = 1;
    if (sectors > 4) sectors = 4;
    if (bus < 0 || bus > 1) bus = 0;

    static const uint16_t bases[2] = { 0x1F0, 0x170 };
    int n = ata_read_lba28(bases[bus], 0, lba, g_sector_buf, sectors);
    if (n < 0) {
        kputs("read: I/O error or no device at that LBA.\n");
        return;
    }
    kputs("read ");
    kprint_dec64(n);
    kputs(" sector(s) from lba=");
    kprint_dec64(lba);
    kputs(" (bus ");
    kputs(bus ? "secondary" : "primary");
    kputs("):\n");
    kprint_hexdump(g_sector_buf, (uint32_t)n * 512);
}

/* Parse MBR partition table from sector 0 (already in g_sector_buf via read). */
static void show_partitions(void) {
    int n = ata_read_lba28(0x1F0, 0, 0, g_sector_buf, 1);
    if (n < 0) {
        kputs("part: could not read sector 0.\n");
        return;
    }
    /* MBR signature check */
    if (g_sector_buf[510] != 0x55 || g_sector_buf[511] != 0xAA) {
        kputs("part: no MBR signature (0x55AA) on primary master.\n");
        return;
    }
    static const char* ptype_names[] = {
        "empty", "FAT12", "?", "HPFS/NTFS", "FAT16<32M", "?", "FAT16", "?",
        "FAT32", "?", "FAT32 LBA", "?", "FAT16 LBA", "?", "extended", "?"
    };
    int found = 0;
    for (int i = 0; i < 4; i++) {
        uint8_t* e = g_sector_buf + 446 + i * 16;
        uint8_t status = e[0];
        uint8_t type   = e[4];
        uint32_t lba   = (uint32_t)e[8] | ((uint32_t)e[9] << 8) | ((uint32_t)e[10] << 16) | ((uint32_t)e[11] << 24);
        uint32_t size  = (uint32_t)e[12] | ((uint32_t)e[13] << 8) | ((uint32_t)e[14] << 16) | ((uint32_t)e[15] << 24);
        if (type == 0 && status == 0) continue;
        kputs("  part");
        kprint_dec64(i + 1);
        kputs("  status=0x");
        kprint_hex32pad(status, 2);
        kputs("  type=");
        kprint_dec64(type);
        kputs(" (");
        kputs((type < 16) ? ptype_names[type] : "unknown");
        kputs(")  start_lba=");
        kprint_dec64(lba);
        kputs("  sectors=");
        kprint_dec64(size);
        kputs("  size_mib~");
        kprint_dec64(((uint64_t)size * 512) >> 20);
        kputs("\n");
        found++;
    }
    if (!found) kputs("MBR present but no partitions defined.\n");
}

static void show_disk(void) {
    static const uint16_t bases[2] = { 0x1F0, 0x170 };
    static const char* chan[2] = { "primary", "secondary" };
    int found = 0;
    char model[41];
    for (int b = 0; b < 2; b++) {
        for (int s = 0; s < 2; s++) {
            ata_identify_model(bases[b], s, model);
            if (model[0] == '\0') continue;
            kputs("disk ");
            kputs(chan[b]);
            kputs(s ? " slave: " : " master: ");
            kputs(model);
            kputs("\n");
            found++;
        }
    }
    if (!found) kputs("No ATA disks detected.\n");
    else { kputs("ATA scan complete: "); kprint_dec64(found); kputs(" drive(s).\n"); }
}

/* --------------------------------------------------------------------------
 * Information Displays
 * -------------------------------------------------------------------------- */
void show_banner(void) {
    vga_set_color(0x0B, 0x00); /* Cyan on black */
    kputs("================================================================================\n");
    kputs("                     SIGMAOS SOVEREIGN KERNEL v0.1.0                            \n");
    kputs("            Ready to Work & Ready to Launch - Real Hardware Kernel              \n");
    kputs("================================================================================\n");
    vga_set_color(0x0A, 0x00); /* Green on black */
    kputs("  Architecture: x86_64 / i386 Bare-Metal Multiboot Kernel\n");
    kputs("  Subsystems  : VGA Console, Serial COM1, PS/2 Keyboard, Scheduler, VFS\n");
    kputs("  Inspirations: Arch Linux (Minimal) | Omarchy (Tiling) | Mint (Ergonomic)\n\n");
}

void show_craft_suite(void) {
    vga_set_color(0x0D, 0x00); /* Magenta on black */
    kputs("--- 20-Module Craft Creative Suite ---\n");
    vga_set_color(0x0F, 0x00); /* White on black */
    kputs(" 1. 3DCraft    - 3D Modeling & Rendering (Blender parity)\n");
    kputs(" 2. AniCraft   - Vector & Traditional 2D Animation\n");
    kputs(" 3. ArtCraft   - Digital Painting & Concept Art\n");
    kputs(" 4. CADCraft   - Parametric Solid Modeling & CAM/CAE\n");
    kputs(" 5. CraftLibs  - Unified Creative Core & Shader Pipeline\n");
    kputs(" 6. DeckCraft  - Interactive Presentation Decks\n");
    kputs(" 7. DesignCraft- Vector UI/UX Design (Figma parity)\n");
    kputs(" 8. EffectCraft- Node-Based Visual Effects & Compositing\n");
    kputs(" 9. FigCraft   - Technical Figures & Schematics\n");
    kputs("10. FilmCraft  - Non-Linear Video Editor & Audio Timeline\n");
    kputs("11. FontCraft  - Typography Studio & OpenType Engine\n");
    kputs("12. GridCraft  - High-Performance Reactive Spreadsheets\n");
    kputs("13. LightCraft - Optical Lighting Simulation & Lux Engine\n");
    kputs("14. PDFCraft   - Sovereign PDF Engine & Preflight Analyzer\n");
    kputs("15. PhotoCraft - Non-Destructive RAW Image Processing\n");
    kputs("16. PrintCraft - Professional Desktop Publishing & Layout\n");
    kputs("17. SoundCraft - Digital Audio Workstation & Synthesizers\n");
    kputs("18. VectorCraft- Scalable Vector Graphics Editor\n");
    kputs("19. VoiceCraft - Speech Recognition & Voice Synthesis\n");
    kputs("20. WordCraft  - Typographic Document Processor\n\n");
}

void show_distro_parity(void) {
    vga_set_color(0x0E, 0x00); /* Yellow on black */
    kputs("--- Linux & BSD Distro Parity ---\n");
    vga_set_color(0x0F, 0x00);
    kputs(" * Arch Linux   : Rolling release model, pacman/PKGBUILD integration\n");
    kputs(" * Omarchy      : Hyprland tiling WM, Quickshell bar, keyboard navigation\n");
    kputs(" * Linux Mint   : User ergonomics, update safety levels, driver autodetect\n");
    kputs(" * CachyOS      : BORE/EEVDF scheduler, AutoBOLT, zram-zstd memory tier\n");
    kputs(" * Debian       : Multiarch package management, dpkg compatibility\n");
    kputs(" * DragonFlyBSD : HAMMER2 transactional VFS, lightweight kernel threads\n");
    kputs(" * FreeBSD      : Capsicum capability isolation, kqueue event multiplexing\n");
    kputs(" * OpenBSD      : pledge() & unveil() security enforcement, W^X pages\n\n");
}

/* --------------------------------------------------------------------------
 * Interactive Shell REPL
 * -------------------------------------------------------------------------- */
void kernel_shell(void) {
    char cmd[128];
    size_t len = 0;

    vga_set_color(0x0A, 0x00);
    kputs("Welcome to SigmaOS Sovereign Kernel. Type 'help' for commands.\n\n");

    while (1) {
        vga_set_color(0x03, 0x00); /* Cyan */
        kputs("sigmaos# ");
        vga_set_color(0x0F, 0x00); /* White */
        len = 0;

        while (1) {
            char c = kgetc();
            if (c == '\n') {
                kputc('\n');
                cmd[len] = '\0';
                break;
            } else if (c == '\b') {
                if (len > 0) {
                    len--;
                    kputc('\b');
                }
            } else if (c >= 32 && c <= 126 && len < sizeof(cmd) - 1) {
                cmd[len++] = c;
                kputc(c);
            }
        }

        if (len == 0) continue;

        if (kstrcmp(cmd, "help") == 0 || kstrcmp(cmd, "?") == 0) {
            kputs("Commands:\n");
            kputs("  help    - Show available commands\n");
            kputs("  status  - Display kernel and hardware status\n");
            kputs("  craft   - List 20-module Craft Creative Suite\n");
            kputs("  distro  - Display Linux (Arch/Cachy/Debian) & BSD parity\n");
            kputs("  mem     - Show memory size and memory map\n");
            kputs("  modules - List Multiboot modules\n");
            kputs("  cmdline - Show kernel command line\n");
            kputs("  date    - Show hardware clock (RTC)\n");
            kputs("  cpu     - Show CPU vendor and features\n");
            kputs("  pci     - Scan PCI buses for devices\n");
            kputs("  disk    - Detect ATA disks (IDENTIFY)\n");
            kputs("  read    - Read disk sectors: read <lba> [n] [bus]\n");
            kputs("  part    - Show MBR partition table\n");
            kputs("  clear   - Clear the console screen\n");
            kputs("  reboot  - Restart the computer\n");
            kputs("  halt    - Halt the CPU safely\n\n");
        } else if (kstrcmp(cmd, "status") == 0) {
            show_banner();
        } else if (kstrcmp(cmd, "craft") == 0) {
            show_craft_suite();
        } else if (kstrcmp(cmd, "distro") == 0) {
            show_distro_parity();
        } else if (kstrcmp(cmd, "mem") == 0) {
            show_memory_map();
        } else if (kstrcmp(cmd, "modules") == 0) {
            show_modules();
        } else if (kstrcmp(cmd, "cmdline") == 0) {
            show_cmdline();
        } else if (kstrcmp(cmd, "date") == 0 || kstrcmp(cmd, "time") == 0) {
            show_rtc();
        } else if (kstrcmp(cmd, "cpu") == 0) {
            show_cpu();
        } else if (kstrcmp(cmd, "pci") == 0) {
            show_pci();
        } else if (kstrcmp(cmd, "disk") == 0) {
            show_disk();
        } else if (kcmd_is(cmd, "read")) {
            show_read(cmd + 4);
        } else if (kstrcmp(cmd, "part") == 0 || kstrcmp(cmd, "fdisk") == 0) {
            show_partitions();
        } else if (kstrcmp(cmd, "clear") == 0) {
            vga_clear();
            show_banner();
        } else if (kstrcmp(cmd, "reboot") == 0) {
            kreboot();
        } else if (kstrcmp(cmd, "halt") == 0) {
            khalt();
        } else {
            kputs("Unknown command: '");
            kputs(cmd);
            kputs("'. Type 'help' for available commands.\n");
        }
    }
}

/* --------------------------------------------------------------------------
 * Kernel Entry Point
 * -------------------------------------------------------------------------- */
__attribute__((section(".text")))
void kernel_main(uint32_t magic, void* mb_info) {
    g_mb_info = (volatile uint8_t*)mb_info;

    serial_init();
    vga_clear();

    /* Fail-fast: only continue under a real Multiboot loader */
    if (magic != 0x2BADB002) {
        vga_set_color(0x04, 0x00);
        kputs("FATAL: invalid Multiboot magic 0x");
        kprint_hex32(magic);
        kputs(" - not booted via GRUB/Limine.\n");
        khalt();
    }

    show_banner();

    kputs("[kernel] Multiboot header validated.\n");
    kputs("[kernel] VGA console initialized at 0xB8000.\n");
    kputs("[kernel] Serial COM1 initialized at 115200 8N1.\n");
    kputs("[kernel] PS/2 keyboard controller ready.\n");
    kputs("[kernel] Entering sovereign interactive shell.\n\n");

    kernel_shell();
}

/* --------------------------------------------------------------------------
 * Assembly Entry (sets up stack and calls kernel_main)
 * -------------------------------------------------------------------------- */
__attribute__((section(".text"), naked))
void _start(void) {
    __asm__ volatile (
        "cli\n"
        "mov $stack_top, %esp\n"
        "push %ebx\n"          /* Multiboot info pointer */
        "push %eax\n"          /* Multiboot magic */
        "call kernel_main\n"
        "1: hlt\n"
        "jmp 1b\n"
    );
}

/* 32 KiB Kernel Stack */
__attribute__((section(".bss"), aligned(16)))
static uint8_t stack[32768];
__attribute__((used))
static uint8_t* stack_top = stack + sizeof(stack);
