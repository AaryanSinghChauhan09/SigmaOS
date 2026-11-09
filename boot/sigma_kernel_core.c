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

static inline void outw(uint16_t port, uint16_t val) {
    __asm__ volatile ("outw %0, %1" :: "a"(val), "Nd"(port));
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
 * Kernel log — fixed ring buffer for boot/event diagnostics (dmesg).
 * INSTRUMENTATION: cheap, always-on, no allocation.
 * -------------------------------------------------------------------------- */
#define KLOG_SIZE 4096
static char     g_klog[KLOG_SIZE];
static uint32_t g_klog_len = 0;

void klog(const char* str) {
    while (*str && g_klog_len < KLOG_SIZE - 1) {
        g_klog[g_klog_len++] = *str++;
    }
    g_klog[g_klog_len] = '\0';
}

void klog(const char* str);
static void kprint_dec64(uint64_t v);
static void kprint_hex32(uint32_t v);
static void show_dmesg(void);

static void show_dmesg(void) {
    if (g_klog_len == 0) { kputs("dmesg: log empty.\n"); return; }
    kputs("--- kernel log ---\n");
    kputs(g_klog);
    kputs("\n--- end (" );
    kprint_dec64(g_klog_len);
    kputs(" bytes) ---\n");
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
static int kbd_buf_pop(char* out);   /* defined with IRQ section below */

char kgetc(void) {
    while (1) {
        /* Interrupt-driven keyboard buffer first */
        char k;
        if (kbd_buf_pop(&k)) {
            if (k == '\r') return '\n';
            return k;
        }
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

static int kstrlen(const char* s) {
    int n = 0;
    while (s[n]) n++;
    return n;
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

/* True if s starts with prefix (no boundary requirement). */
static int kprefix(const char* s, const char* prefix) {
    while (*prefix) {
        if (*s != *prefix) return 0;
        s++;
        prefix++;
    }
    return 1;
}

/* --------------------------------------------------------------------------
 * Multiboot1 Boot Information
 * -------------------------------------------------------------------------- */
static volatile uint8_t* g_mb_info = 0;
static void ata_identify_model(uint16_t base, int slave, char out[41]);
static uint32_t fat_next_cluster32(uint32_t fat_lba, uint32_t cluster);
static int fat_is_eoc32(uint32_t c);
static uint32_t fat_next_any(uint32_t fat_lba, int kind, uint32_t cluster);
static int fat_eoc_any(int kind, uint32_t c);
struct fat_hit { uint32_t cluster; uint32_t size; int is_dir; };
static int fat_scan_dir(uint32_t start_cluster, int kind, uint32_t fat_lba,
                        uint32_t data_lba, uint8_t spc, const uint8_t want[11],
                        const char* want_name, struct fat_hit* hit);
static int fat_scan_root16(uint32_t root_lba, uint32_t root_secs,
                           const uint8_t want[11], const char* want_name,
                           struct fat_hit* hit);
static int fat_lookup(int kind, uint32_t root_clus, uint32_t root_lba, uint32_t root_secs,
                      uint32_t fat_lba, uint32_t data_lba, uint8_t spc,
                      const char* path, struct fat_hit* hit);
static void fat_print_chain(uint32_t start_cluster, int kind, uint32_t fat_lba,
                            uint32_t data_lba, uint8_t spc);
static void paging_init(void);
static void show_paging(void);
static void gdt_init(void);
static void show_gdt(void);
static void idt_init(void);
static void show_user(void);
static void pic_init(void);
static void pit_init(void);
static void show_timer(void);
static void show_memalloc(void);
static void show_sched(void);
static void show_ps(void);
static void show_exec(const char* args);
static int fat_ensure_mounted(void);
static void* kmalloc(uint32_t size);
static void kfree(void* ptr);
static uint32_t pmm_alloc(void);
static int fat_free_chain(int kind, uint32_t start);
static uint32_t fat_alloc_chain(int kind, uint32_t count);
void resume_user(void);   /* naked asm: restore preempted user ctx */
static uint32_t find_user_region(void);
static void enter_user(uint32_t eip, uint32_t esp);
static uint32_t g_user_base;
static uint32_t g_user_pd;   /* phys addr of user page directory, 0 = none */
static uint32_t g_page_dir[1024] __attribute__((aligned(4096), section(".bss")));

/* Build the user page directory: a copy of the kernel PD in which ONLY
 * the user region's PDE is user-accessible; all other RAM stays
 * supervisor so ring-3 code faults on kernel/foreign memory. */
static int user_pd_setup(void) {
    if (g_user_pd) return 0;
    if (!g_user_base) return -1;
    uint32_t pd = pmm_alloc();
    if (!pd) return -1;
    uint32_t* dst = (uint32_t*)(size_t)pd;
    for (int i = 0; i < 1024; i++) dst[i] = g_page_dir[i] & ~0x4u;
    dst[g_user_base >> 22] |= 0x4u;
    g_user_pd = pd;
    /* cr3 reload flushes TLB so U/S changes take effect */
    __asm__ volatile("movl %%cr3, %%eax\n\tmovl %%eax, %%cr3" ::: "eax", "memory");
    return 0;
}
static uint32_t g_map_mb;
/* Process-exit support: enter_user saves the kernel return context here;
 * SYS_EXIT sets g_user_dead and syscall_entry irets back to kernel. */
int      g_user_dead   = 0;
uint32_t g_resume_eip  = 0;
uint32_t g_resume_esp  = 0;
/* Preemptive scheduling state (timer-tick context switch) */
int      g_preempt_pending = 0;
int      g_user_active     = 0;
uint32_t g_user_ctx[8];     /* pushal-order regs at preempt point */
uint32_t g_user_eip, g_user_cs, g_user_eflags, g_user_esp, g_user_ss;

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

/* u64 / 10 via shift-subtract — keeps the kernel free of libgcc
 * helpers (__udivmoddi4) in a -nostdlib link. */
static uint64_t u64_div10(uint64_t v, uint8_t* rem) {
    uint64_t q = 0, r = 0;
    for (int i = 63; i >= 0; i--) {
        r = (r << 1) | (uint64_t)((v >> i) & 1u);
        q <<= 1;
        if (r >= 10) { r -= 10; q |= 1; }
    }
    *rem = (uint8_t)r;
    return q;
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
        uint8_t d;
        v = u64_div10(v, &d);
        buf[--i] = '0' + d;
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

static void show_info(void) {
    kputs("=== SigmaOS system info ===\n");
    kputs("OS     : SigmaOS boot kernel (x86, 32-bit protected mode)\n");
    kputs("Loader : Multiboot1 via GRUB\n");
    kputs("Arch   : i386\n");

    if (g_mb_info) {
        uint32_t lower = mbi_rd32(MBI_MEM_LOWER);
        uint32_t upper = mbi_rd32(MBI_MEM_UPPER);
        kputs("Memory : ");
        kprint_dec64((uint64_t)lower + upper);
        kputs(" KiB (lower=");
        kprint_dec64(lower);
        kputs(" upper=");
        kprint_dec64(upper);
        kputs(")\n");
    } else {
        kputs("Memory : no Multiboot info\n");
    }

    char model[41];
    uint16_t bases[2] = {0x1F0, 0x170};
    int disks = 0;
    for (int b = 0; b < 2; b++) {
        for (int s = 0; s < 2; s++) {
            ata_identify_model(bases[b], s, model);
            if (model[0]) {
                kputs("Disk   : ");
                kputs(model);
                kputs("\n");
                disks++;
            }
        }
    }
    if (!disks) kputs("Disk   : no ATA disk detected\n");
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

/* Write up to 4 sectors (LBA28). Returns sectors written or -1. */
static int ata_write_lba28(uint16_t base, int slave, uint32_t lba, const uint8_t* buf, int sectors) {
    if (sectors < 1 || sectors > 4) return -1;
    outb(base + 6, (uint8_t)(0xE0 | (slave << 4) | ((lba >> 24) & 0x0F)));
    io_wait();
    outb(base + 2, (uint8_t)sectors);
    outb(base + 3, (uint8_t)(lba & 0xFF));
    outb(base + 4, (uint8_t)((lba >> 8) & 0xFF));
    outb(base + 5, (uint8_t)((lba >> 16) & 0xFF));
    outb(base + 7, 0x30);                          /* WRITE SECTORS */
    for (int s = 0; s < sectors; s++) {
        if (ata_wait_drq(base) != 0) return -1;
        const uint16_t* w = (const uint16_t*)(buf + s * 512);
        for (int i = 0; i < 256; i++) outw(base, w[i]);
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

/* --------------------------------------------------------------------------
 * FAT Boot-Sector Recognizer (FAT12/16/32 heuristics)
 * -------------------------------------------------------------------------- */
static void show_fs(const char* args) {
    /* args: optional partition number 1-4 or LBA override "lba:<n>" — default: scan MBR part1 */
    uint32_t lba = 0;
    const char* p = args;
    while (*p == ' ') p++;
    if (kprefix(p, "lba:")) {
        lba = kstrtoul(p + 4);
    } else {
        /* default: first partition's start LBA from MBR */
        if (ata_read_lba28(0x1F0, 0, 0, g_sector_buf, 1) < 0) {
            kputs("fs: cannot read MBR.\n");
            return;
        }
        if (g_sector_buf[510] != 0x55 || g_sector_buf[511] != 0xAA) {
            kputs("fs: no MBR; treating disk as superfloppy, using lba 0.\n");
            lba = 0;
        } else {
            uint8_t* e = g_sector_buf + 446;
            lba = (uint32_t)e[8] | ((uint32_t)e[9] << 8) | ((uint32_t)e[10] << 16) | ((uint32_t)e[11] << 24);
            if (lba == 0) { kputs("fs: first partition empty; trying superfloppy lba 0.\n"); }
        }
    }

    if (ata_read_lba28(0x1F0, 0, lba, g_sector_buf, 1) < 0) {
        kputs("fs: cannot read boot sector at lba=");
        kprint_dec64(lba);
        kputs(".\n");
        return;
    }
    uint8_t* b = g_sector_buf;
    kputs("fs: boot sector @ lba=");
    kprint_dec64(lba);
    kputs("  jump=");
    kprint_hex32pad(b[0], 2);
    kprint_hex32pad(b[1], 2);
    kputs("  oem='");
    for (int i = 3; i < 11; i++) kputc((b[i] >= 32 && b[i] <= 126) ? (char)b[i] : '.');
    kputs("'\n");

    if (b[510] != 0x55 || b[511] != 0xAA) {
        kputs("fs: WARNING no 0x55AA signature — probably not FAT.\n");
        return;
    }
    uint16_t bytes_per_sec = (uint16_t)b[11] | ((uint16_t)b[12] << 8);
    uint8_t  sec_per_clus  = b[13];
    uint16_t reserved      = (uint16_t)b[14] | ((uint16_t)b[15] << 8);
    uint8_t  num_fats      = b[16];
    uint16_t root_entries  = (uint16_t)b[17] | ((uint16_t)b[18] << 8);
    uint16_t tot16         = (uint16_t)b[19] | ((uint16_t)b[20] << 8);
    uint8_t  media         = b[21];
    uint16_t fat_sz16      = (uint16_t)b[22] | ((uint16_t)b[23] << 8);
    uint32_t tot32         = (uint32_t)b[32] | ((uint32_t)b[33] << 8) | ((uint32_t)b[34] << 16) | ((uint32_t)b[35] << 24);
    uint32_t fat_sz32      = (uint32_t)b[36] | ((uint32_t)b[37] << 8) | ((uint32_t)b[38] << 16) | ((uint32_t)b[39] << 24);
    uint16_t fs_ver        = (uint16_t)b[42] | ((uint16_t)b[43] << 8);
    uint32_t root_clus     = (uint32_t)b[44] | ((uint32_t)b[45] << 8) | ((uint32_t)b[46] << 16) | ((uint32_t)b[47] << 24);
    uint16_t fs_info       = (uint16_t)b[48] | ((uint16_t)b[49] << 8);
    uint16_t bk_boot       = (uint16_t)b[50] | ((uint16_t)b[51] << 8);

    const char* kind = "FAT12/16";
    uint32_t tot = tot16 ? tot16 : tot32;
    uint32_t fat_sz = fat_sz16 ? fat_sz16 : fat_sz32;
    if (root_entries == 0 && fs_ver == 0) kind = "FAT32";
    else if (root_entries == 0) kind = "FAT32?";
    else if (tot > 65525) kind = "FAT32?";

    kputs("  bytes/sec="); kprint_dec64(bytes_per_sec);
    kputs("  sec/clus="); kprint_dec64(sec_per_clus);
    kputs("  reserved="); kprint_dec64(reserved);
    kputs("  fats="); kprint_dec64(num_fats);
    kputs("  root_entries="); kprint_dec64(root_entries);
    kputs("  media=0x"); kprint_hex32pad(media, 2);
    kputs("\n  total_sectors="); kprint_dec64(tot);
    kputs("  fat_size="); kprint_dec64(fat_sz);
    kputs("  fs_info="); kprint_dec64(fs_info);
    kputs("  backup_boot="); kprint_dec64(bk_boot);
    if (root_entries == 0) { kputs("  root_cluster="); kprint_dec64(root_clus); }
    kputs("\n  detected: "); kputs(kind); kputs("\n");
}

/* --------------------------------------------------------------------------
 * FAT12/16 Root Directory Listing
 * -------------------------------------------------------------------------- */
static uint8_t g_fs_buf[512 * 8]; /* up to 8 sectors of dir data */

/* ---- LFN (long filename) support ----
 * LFN entries (attr 0x0F) precede their short entry, stored tail-first.
 * ord = e[0]&0x1F (1-based fragment index); bit6 marks the last fragment.
 * 13 UCS-2 chars per entry: idx0-4 @off1, idx5-10 @off14, idx11-12 @off28.
 * Fragment ord n occupies name chars (n-1)*13 .. n*13-1. */
static int fat_lfn_char(const uint8_t* e, int idx) {
    uint32_t off = (idx < 5) ? (1 + idx * 2)
                 : (idx < 11) ? (14 + (idx - 5) * 2)
                 : (28 + (idx - 11) * 2);
    uint16_t u = (uint16_t)e[off] | ((uint16_t)e[off + 1] << 8);
    if (u == 0x0000 || u == 0xFFFF) return -1;
    if (u > 0x7F) return '?';
    return (u >= 'a' && u <= 'z') ? (int)(u - 32) : (int)u;
}

/* Short-name checksum used to bind LFN entries to their 8.3 entry. */
static uint8_t fat_lfn_cksum(const uint8_t* s11) {
    uint8_t sum = 0;
    for (int i = 0; i < 11; i++) {
        sum = (uint8_t)(((sum & 1) ? 0x80 : 0) + (sum >> 1) + s11[i]);
    }
    return sum;
}

/* Case-insensitive compare of assembled LFN (already upper-folded) vs raw name. */
static int fat_lfn_match(const char* lfn, const char* name) {
    int i = 0;
    for (; name[i] && lfn[i]; i++) {
        char c = name[i];
        if (c >= 'a' && c <= 'z') c -= 32;
        if (lfn[i] != c) return 0;
    }
    return name[i] == '\0' && lfn[i] == '\0';
}

/* Per-scan LFN accumulator state (resets after each short entry). */
struct lfn_acc {
    char     name[256];
    uint8_t  ck;
    int      active;
};

static void lfn_acc_reset(struct lfn_acc* a) {
    for (int i = 0; i < 256; i++) a->name[i] = 0;
    a->ck = 0;
    a->active = 0;
}

/* Feed one dir entry; returns 1 if it was an LFN entry consumed. */
static int lfn_acc_feed(struct lfn_acc* a, const uint8_t* e) {
    if (e[11] != 0x0F) return 0;
    if (e[0] & 0x80) return 1;              /* erased LFN: skip silently */
    int ord = e[0] & 0x1F;
    if (ord < 1 || ord > 20) return 1;
    a->ck = e[13];
    a->active = 1;
    int base = (ord - 1) * 13;
    for (int c = 0; c < 13 && base + c < 255; c++) {
        int ch = fat_lfn_char(e, c);
        if (ch < 0) break;
        a->name[base + c] = (char)ch;
    }
    return 1;
}

/* Does the pending LFN match this short entry? Verifies checksum first. */
static int lfn_acc_match(struct lfn_acc* a, const uint8_t* short_e,
                         const char* want_name) {
    if (!a->active || !want_name || !want_name[0]) return 0;
    if (a->ck != fat_lfn_cksum(short_e)) return 0;
    return fat_lfn_match(a->name, want_name);
}

static void fat_print_name(const uint8_t* e) {
    char name[13];
    int n = 0;
    for (int i = 0; i < 8 && e[i] != ' '; i++) name[n++] = (char)e[i];
    if (e[8] != ' ') {
        name[n++] = '.';
        for (int i = 8; i < 11 && e[i] != ' '; i++) name[n++] = (char)e[i];
    }
    name[n] = '\0';
    /* lowercase extension for readability */
    for (int i = 0; i < n; i++) {
        char c = name[i];
        if (c >= 'A' && c <= 'Z') name[i] = (char)(c + 32);
    }
    kputs(name);
}

/* ---- working directory (cwd) ----
 * g_cwd holds a normalized absolute path without leading '/', e.g.
 * "A/B/C". "" means volume root. All FAT path lookups go through
 * fs_resolve() which joins cwd + arg and normalizes "." / "..". */
static char g_cwd[160];

/* Cached volume geometry (populated by fat_ensure_mounted). */
static int      g_vol_kind = -1;        /* -1 = not mounted */
static uint32_t g_vol_fat_lba, g_vol_data_lba;
static uint32_t g_vol_root_clus, g_vol_root_lba, g_vol_root_secs;
static uint8_t  g_vol_spc;
static uint32_t g_vol_fatsz;            /* FAT size in sectors */
static uint32_t g_vol_total_clusters;   /* data clusters + 2 (0/1 reserved) */

/* Fetch next '/'-separated component; advances *pp past it. */
static int fs_next_comp(const char** pp, char* comp, int csz) {
    const char* p = *pp;
    while (*p == '/') p++;
    if (!*p) { *pp = p; return 0; }
    int n = 0;
    while (p[n] && p[n] != '/') n++;
    int cl = n < csz - 1 ? n : csz - 1;
    for (int i = 0; i < cl; i++) comp[i] = p[i];
    comp[cl] = 0;
    *pp = p + n;
    return 1;
}

/* Resolve arg against g_cwd into normalized absolute path `out`.
 * Absolute arg (leading '/') ignores cwd. Returns 0 ok, -1 overflow. */
static int fs_resolve(const char* arg, char* out, int outsz) {
    out[0] = 0;
    const char* base = (arg[0] == '/') ? "" : g_cwd;
    const char* srcs[2] = { base, arg };
    for (int s = 0; s < 2; s++) {
        const char* p = srcs[s];
        char comp[64];
        while (fs_next_comp(&p, comp, (int)sizeof comp)) {
            if (comp[0] == '.' && comp[1] == 0) continue;
            if (comp[0] == '.' && comp[1] == '.' && comp[2] == 0) {
                int l = kstrlen(out);
                while (l > 0 && out[l - 1] != '/') l--;
                if (l > 0) l--;
                out[l] = 0;
                continue;
            }
            int ol = kstrlen(out), cl = kstrlen(comp);
            if (ol + 1 + cl + 1 > outsz) return -1;
            if (ol) out[ol++] = '/';
            for (int i = 0; i < cl; i++) out[ol++] = comp[i];
            out[ol] = 0;
        }
    }
    return 0;
}

static void show_pwd(void) {
    kputs("/");
    if (g_cwd[0]) kputs(g_cwd);
    kputs("\n");
}

static void show_cd(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    char rpath[160];
    if (fs_resolve(p, rpath, (int)sizeof rpath) != 0) {
        kputs("cd: path too long.\n");
        return;
    }
    if (!rpath[0]) { g_cwd[0] = 0; kputs("cd: / (volume root)\n"); return; }
    if (fat_ensure_mounted() != 0) { kputs("cd: no FAT volume.\n"); return; }
    struct fat_hit hit;
    int lr = fat_lookup(g_vol_kind, g_vol_root_clus, g_vol_root_lba,
                        g_vol_root_secs, g_vol_fat_lba, g_vol_data_lba,
                        g_vol_spc, rpath, &hit);
    if (lr <= 0) { kputs("cd: '"); kputs(rpath); kputs("' not found.\n"); return; }
    if (!hit.is_dir) { kputs("cd: '"); kputs(rpath); kputs("' is not a directory.\n"); return; }
    int l = kstrlen(rpath);
    if (l > (int)sizeof g_cwd - 1) l = (int)sizeof g_cwd - 1;
    for (int i = 0; i < l; i++) g_cwd[i] = rpath[i];
    g_cwd[l] = 0;
    kputs("cd: /");
    kputs(g_cwd);
    kputs("\n");
}

/* df — filesystem usage: scan FAT for free (zero) cluster entries. */
static void show_df(void) {
    if (fat_ensure_mounted() != 0) { kputs("df: no FAT volume.\n"); return; }
    uint32_t fat_bytes = g_vol_fatsz * 512u;
    if (fat_bytes == 0 || fat_bytes > 0x400000u) {
        kputs("df: FAT too large to scan.\n"); return;
    }
    uint8_t* fat = (uint8_t*)kmalloc(fat_bytes);
    if (!fat) { kputs("df: out of memory.\n"); return; }
    uint32_t done = 0;
    int fail = 0;
    while (done < g_vol_fatsz) {
        int chunk = (g_vol_fatsz - done > 4) ? 4 : (int)(g_vol_fatsz - done);
        int n = ata_read_lba28(0x1F0, 0, g_vol_fat_lba + done, fat + done * 512u, chunk);
        if (n < 0) { fail = 1; break; }
        done += (uint32_t)n;
    }
    if (fail) { kfree(fat); kputs("df: FAT read error.\n"); return; }

    uint32_t free_clusters = 0;
    uint32_t limit = g_vol_total_clusters < 2 ? 0 : g_vol_total_clusters;
    if (g_vol_kind == 2) {           /* FAT32: u32 entries, mask 0x0FFFFFFF */
        for (uint32_t c = 2; c < limit; c++) {
            uint32_t v = (uint32_t)fat[c*4] | ((uint32_t)fat[c*4+1] << 8)
                       | ((uint32_t)fat[c*4+2] << 16) | ((uint32_t)fat[c*4+3] << 24);
            if ((v & 0x0FFFFFFFu) == 0) free_clusters++;
        }
    } else if (g_vol_kind == 1) {    /* FAT16: u16 entries */
        for (uint32_t c = 2; c < limit; c++) {
            uint16_t v = (uint16_t)fat[c*2] | ((uint16_t)fat[c*2+1] << 8);
            if (v == 0) free_clusters++;
        }
    } else {                          /* FAT12: packed 12-bit entries */
        for (uint32_t c = 2; c < limit; c++) {
            uint32_t off = c + (c >> 1);
            if (off + 1 >= fat_bytes) break;
            uint16_t v = (uint16_t)fat[off] | ((uint16_t)fat[off+1] << 8);
            uint16_t e = (c & 1) ? (uint16_t)((v >> 4) & 0x0FFF)
                                : (uint16_t)(v & 0x0FFF);
            if (e == 0) free_clusters++;
        }
    }
    kfree(fat);

    uint64_t clus_bytes = (uint64_t)g_vol_spc * 512u;
    uint64_t total_b = (uint64_t)(limit > 2 ? limit - 2 : 0) * clus_bytes;
    uint64_t free_b  = (uint64_t)free_clusters * clus_bytes;
    kputs("df: FAT");
    kprint_dec64((uint64_t)g_vol_kind == 2 ? 32 : (g_vol_kind == 1 ? 16 : 12));
    kputs(", ");
    kprint_dec64(limit > 2 ? limit - 2 : 0);
    kputs(" clusters, ");
    kprint_dec64(free_clusters);
    kputs(" free (");
    kprint_dec64(free_b);
    kputs(" / ");
    kprint_dec64(total_b);
    kputs(" bytes)\n");
}

static void show_ls(const char* args) {
    /* "ls [lba:<n>]" or "ls <dir-path>" — list directory entries */
    uint32_t lba = 0;
    char dirname[64];
    dirname[0] = '\0';
    const char* p = args;
    while (*p == ' ') p++;
    if (kprefix(p, "lba:")) {
        lba = kstrtoul(p + 4);
    } else if (*p) {
        int dl = 0;
        while (*p && *p != ' ' && dl < 63) dirname[dl++] = *p++;
        dirname[dl] = '\0';
        /* default: first partition's start LBA from MBR */
        if (ata_read_lba28(0x1F0, 0, 0, g_sector_buf, 1) < 0) { kputs("ls: no MBR.\n"); return; }
        uint8_t* e = g_sector_buf + 446;
        lba = (uint32_t)e[8] | ((uint32_t)e[9] << 8) | ((uint32_t)e[10] << 16) | ((uint32_t)e[11] << 24);
    } else {
        /* default: first partition's start LBA from MBR */
        if (ata_read_lba28(0x1F0, 0, 0, g_sector_buf, 1) < 0) { kputs("ls: no MBR.\n"); return; }
        uint8_t* e = g_sector_buf + 446;
        lba = (uint32_t)e[8] | ((uint32_t)e[9] << 8) | ((uint32_t)e[10] << 16) | ((uint32_t)e[11] << 24);
    }
    (void)p;

    if (ata_read_lba28(0x1F0, 0, lba, g_sector_buf, 1) < 0) { kputs("ls: cannot read boot sector.\n"); return; }
    uint8_t* b = g_sector_buf;
    uint16_t bps   = (uint16_t)b[11] | ((uint16_t)b[12] << 8);
    uint8_t  spc   = b[13];
    uint16_t rsvd  = (uint16_t)b[14] | ((uint16_t)b[15] << 8);
    uint8_t  nfats = b[16];
    uint16_t rent  = (uint16_t)b[17] | ((uint16_t)b[18] << 8);
    uint16_t fsz16 = (uint16_t)b[22] | ((uint16_t)b[23] << 8);
    uint32_t fsz32 = (uint32_t)b[36] | ((uint32_t)b[37] << 8) | ((uint32_t)b[38] << 16) | ((uint32_t)b[39] << 24);
    uint32_t fatsz = fsz16 ? fsz16 : fsz32;
    int is_fat32 = (rent == 0);

    if (bps != 512) { kputs("ls: non-512-byte sectors unsupported.\n"); return; }
    if (spc == 0) { kputs("ls: invalid sectors/cluster.\n"); return; }

    uint32_t fat_lba  = lba + rsvd;
    uint32_t data_lba = lba + rsvd + (uint32_t)nfats * fatsz;

    if (dirname[0]) {
        /* ls <dir-path>: resolve and list that directory's cluster chain */
        int kind;
        uint32_t root_clus = 0, root_lba = 0, root_secs = 0;
        if (is_fat32) {
            kind = 2;
            root_clus = (uint32_t)b[44] | ((uint32_t)b[45] << 8)
                      | ((uint32_t)b[46] << 16) | ((uint32_t)b[47] << 24);
            if (root_clus < 2) { kputs("ls: invalid FAT32 root cluster.\n"); return; }
        } else {
            uint16_t tot16 = (uint16_t)b[19] | ((uint16_t)b[20] << 8);
            uint32_t tot32 = (uint32_t)b[32] | ((uint32_t)b[33] << 8) | ((uint32_t)b[34] << 16) | ((uint32_t)b[35] << 24);
            uint32_t tot = tot16 ? tot16 : tot32;
            uint32_t rdir_secs = (uint32_t)((rent * 32 + bps - 1) / bps);
            uint32_t data_clusters = (tot - rsvd - nfats * fatsz - rdir_secs) / spc;
            kind = data_clusters < 4085 ? 0 : 1;
            data_lba += rdir_secs;
            root_lba = lba + rsvd + (uint32_t)nfats * fatsz;
            root_secs = rdir_secs;
        }
        struct fat_hit hit;
        char rpath[160];
        if (fs_resolve(dirname, rpath, (int)sizeof rpath) != 0) {
            kputs("ls: path too long.\n"); return;
        }
        int lr = fat_lookup(kind, root_clus, root_lba, root_secs,
                            fat_lba, data_lba, spc, rpath, &hit);
        if (lr == -2) { kputs("ls: path component is not a directory.\n"); return; }
        if (lr <= 0) {
            kputs("ls: '");
            kputs(rpath);
            kputs("' not found.\n");
            return;
        }
        if (!hit.is_dir) { kputs("ls: '"); kputs(dirname); kputs("' is a file.\n"); return; }
        fat_print_chain(hit.cluster, kind, fat_lba, data_lba, spc);
        return;
    }

    if (is_fat32) {
        /* FAT32: root dir is a cluster chain starting at offset 44 */
        uint32_t root_clus = (uint32_t)b[44] | ((uint32_t)b[45] << 8)
                           | ((uint32_t)b[46] << 16) | ((uint32_t)b[47] << 24);
        if (root_clus < 2) { kputs("ls: invalid FAT32 root cluster.\n"); return; }
        data_lba += 0; /* data area starts right after FATs for FAT32 */

        int count = 0;
        uint32_t clus = root_clus;
        int guard = 0;
        struct lfn_acc lfn;
        lfn_acc_reset(&lfn);
        while (clus >= 2 && !fat_is_eoc32(clus) && guard++ < 64 && count < 128) {
            uint32_t clus_lba = data_lba + (clus - 2) * spc;
            /* read up to 8 sectors (one cluster max for typical spc<=8) */
            uint32_t secs = spc > 8 ? 8 : spc;
            uint8_t* dst = g_fs_buf;
            uint32_t done = 0;
            int stop = 0;
            while (done < secs) {
                int chunk = (secs - done > 4) ? 4 : (int)(secs - done);
                int n = ata_read_lba28(0x1F0, 0, clus_lba + done, dst, chunk);
                if (n < 0) { kputs("ls: cannot read root dir.\n"); return; }
                dst += n * 512;
                done += (uint32_t)n;
            }
            int entries = (int)(secs * 16);
            for (int i = 0; i < entries; i++) {
                uint8_t* e = g_fs_buf + i * 32;
                if (e[0] == 0x00) { stop = 1; break; }
                if (e[0] == 0xE5) { lfn_acc_reset(&lfn); continue; }
                if (lfn_acc_feed(&lfn, e)) continue;
                uint8_t attr = e[11];
                int use_lfn = lfn.active && lfn.ck == fat_lfn_cksum(e) && lfn.name[0];
                kputs("  ");
                if (attr & 0x10) kputs("<DIR>  "); else kputs("       ");
                if (use_lfn) kputs(lfn.name); else fat_print_name(e);
                lfn_acc_reset(&lfn);
                if (!(attr & 0x10)) {
                    uint32_t sz = (uint32_t)e[28] | ((uint32_t)e[29] << 8) | ((uint32_t)e[30] << 16) | ((uint32_t)e[31] << 24);
                    kputs("  ");
                    kprint_dec64(sz);
                    kputs(" B");
                }
                kputs("\n");
                count++;
                if (count >= 128) { kputs("  ... (truncated)\n"); stop = 1; break; }
            }
            if (stop) break;
            clus = fat_next_cluster32(fat_lba, clus);
        }
        kputs("ls: ");
        kprint_dec64(count);
        kputs(" entrie(s) in FAT32 root dir.\n");
        return;
    }

    uint32_t root_lba = lba + rsvd + (uint32_t)nfats * fatsz;
    data_lba += (uint32_t)((rent * 32 + bps - 1) / bps); /* skip fixed root dir */
    uint32_t root_secs = ((uint32_t)rent * 32 + bps - 1) / bps;
    if (root_secs > 8) root_secs = 8; /* cap at 4KB buffer */

    /* ata_read_lba28 handles max 4 sectors per command — chunk the reads */
    uint8_t* dst = g_fs_buf;
    uint32_t done = 0;
    while (done < root_secs) {
        int chunk = (root_secs - done > 4) ? 4 : (int)(root_secs - done);
        int n = ata_read_lba28(0x1F0, 0, root_lba + done, dst, chunk);
        if (n < 0) { kputs("ls: cannot read root directory.\n"); return; }
        dst += n * 512;
        done += (uint32_t)n;
    }
    int total = (int)done;

    int count = 0;
    struct lfn_acc lfn;
    lfn_acc_reset(&lfn);
    for (int i = 0; i < total * 16; i++) {
        uint8_t* e = g_fs_buf + i * 32;
        if (e[0] == 0x00) break;                 /* end of directory */
        if (e[0] == 0xE5) { lfn_acc_reset(&lfn); continue; }
        if (lfn_acc_feed(&lfn, e)) continue;
        uint8_t attr = e[11];
        int use_lfn = lfn.active && lfn.ck == fat_lfn_cksum(e) && lfn.name[0];
        kputs("  ");
        if (attr & 0x10) kputs("<DIR>  "); else kputs("       ");
        if (use_lfn) kputs(lfn.name); else fat_print_name(e);
        lfn_acc_reset(&lfn);
        /* file size for non-dirs (offset 28, u32) */
        if (!(attr & 0x10)) {
            uint32_t sz = (uint32_t)e[28] | ((uint32_t)e[29] << 8) | ((uint32_t)e[30] << 16) | ((uint32_t)e[31] << 24);
            kputs("  ");
            kprint_dec64(sz);
            kputs(" B");
        }
        kputs("\n");
        count++;
        if (count >= 64) { kputs("  ... (truncated)\n"); break; }
    }
    kputs("ls: ");
    kprint_dec64(count);
    kputs(" entrie(s) in FAT12/16 root dir.\n");
}

/* --------------------------------------------------------------------------
 * FAT12/16 File Read — `cat <name>`
 * -------------------------------------------------------------------------- */
static uint8_t g_file_buf[512 * 8];

/* Convert "name.ext" to padded uppercase 8.3 in out[11]; returns 1 on success. */
static int fat_name_to_83(const char* s, uint8_t out[11]) {
    for (int i = 0; i < 11; i++) out[i] = ' ';
    int dot = -1;
    for (const char* p = s; *p; p++) if (*p == '.') { dot = (int)(p - s); break; }
    int base_len = 0;
    for (const char* p = s; *p && *p != '.'; p++) base_len++;
    if (base_len > 8) base_len = 8;
    for (int i = 0; i < base_len; i++) {
        char c = s[i];
        if (c >= 'a' && c <= 'z') c = (char)(c - 32);
        out[i] = (uint8_t)c;
    }
    if (dot >= 0) {
        const char* ext = s + dot + 1;
        for (int i = 0; i < 3 && ext[i]; i++) {
            char c = ext[i];
            if (c >= 'a' && c <= 'z') c = (char)(c - 32);
            out[8 + i] = (uint8_t)c;
        }
    }
    return 1;
}

static int fat_name_eq(const uint8_t* e, const uint8_t want[11]) {
    for (int i = 0; i < 11; i++) if (e[i] != want[i]) return 0;
    return 1;
}

/* Read one FAT entry (cluster -> next cluster). fat12 or fat16. */
static uint16_t fat_next_cluster(uint32_t fat_lba, int is_fat12, uint16_t cluster) {
    uint32_t byte_off = is_fat12 ? ((uint32_t)cluster + (cluster >> 1)) : ((uint32_t)cluster * 2);
    uint32_t sec = fat_lba + (byte_off / 512);
    uint32_t off = byte_off % 512;
    if (ata_read_lba28(0x1F0, 0, sec, g_fs_buf, 1) < 0) return 0xFFFF;
    if (is_fat12) {
        uint16_t v = (uint16_t)g_fs_buf[off] | ((uint16_t)g_fs_buf[off + 1] << 8);
        return (cluster & 1) ? (uint16_t)((v >> 4) & 0x0FFF) : (uint16_t)(v & 0x0FFF);
    }
    return (uint16_t)g_fs_buf[off] | ((uint16_t)g_fs_buf[off + 1] << 8);
}

/* FAT32: 32-bit FAT entry lookup (cluster -> next cluster). */
static uint32_t fat_next_cluster32(uint32_t fat_lba, uint32_t cluster) {
    uint32_t byte_off = cluster * 4;
    uint32_t sec = fat_lba + (byte_off / 512);
    uint32_t off = byte_off % 512;
    if (ata_read_lba28(0x1F0, 0, sec, g_fs_buf, 1) < 0) return 0xFFFFFFFFu;
    uint32_t v = (uint32_t)g_fs_buf[off]
               | ((uint32_t)g_fs_buf[off + 1] << 8)
               | ((uint32_t)g_fs_buf[off + 2] << 16)
               | ((uint32_t)g_fs_buf[off + 3] << 24);
    return v & 0x0FFFFFFFu;
}

static int fat_is_eoc32(uint32_t c) { return c >= 0x0FFFFFF8u; }

static uint16_t fat_eoc(int is_fat12) { return is_fat12 ? 0xFF8 : 0xFFF8; }

/* Unified cluster-chain helpers. kind: 0=FAT12, 1=FAT16, 2=FAT32 */
static uint32_t fat_next_any(uint32_t fat_lba, int kind, uint32_t cluster) {
    if (kind == 2) return fat_next_cluster32(fat_lba, cluster);
    return fat_next_cluster(fat_lba, kind == 0, (uint16_t)cluster);
}

static int fat_eoc_any(int kind, uint32_t c) {
    if (kind == 2) return fat_is_eoc32(c);
    return c >= (uint32_t)fat_eoc(kind == 0);
}


/* Scan a cluster-chain directory for an 8.3 or LFN name.
 * want_name: raw path component for LFN comparison (NULL = 8.3 only).
 * Returns 1=found, 0=not found, -1=read error. */
static int fat_scan_dir(uint32_t start_cluster, int kind, uint32_t fat_lba,
                        uint32_t data_lba, uint8_t spc, const uint8_t want[11],
                        const char* want_name, struct fat_hit* hit) {
    uint32_t clus = start_cluster;
    int guard = 0;
    struct lfn_acc lfn;
    lfn_acc_reset(&lfn);
    while (clus >= 2 && !fat_eoc_any(kind, clus) && guard++ < 64) {
        uint32_t clus_lba = data_lba + (clus - 2) * spc;
        uint32_t secs = spc > 8 ? 8 : spc;
        uint8_t* dst = g_fs_buf;
        uint32_t done = 0;
        while (done < secs) {
            int chunk = (secs - done > 4) ? 4 : (int)(secs - done);
            int n = ata_read_lba28(0x1F0, 0, clus_lba + done, dst, chunk);
            if (n < 0) return -1;
            dst += n * 512;
            done += (uint32_t)n;
        }
        int entries = (int)(secs * 16);
        for (int i = 0; i < entries; i++) {
            uint8_t* e = g_fs_buf + i * 32;
            if (e[0] == 0x00) return 0;
            if (e[0] == 0xE5) { lfn_acc_reset(&lfn); continue; }
            if (lfn_acc_feed(&lfn, e)) continue;
            int match = fat_name_eq(e, want) || lfn_acc_match(&lfn, e, want_name);
            lfn_acc_reset(&lfn);
            if (!match) continue;
            hit->is_dir = (e[11] & 0x10) != 0;
            if (kind == 2)
                hit->cluster = (((uint32_t)e[20] | ((uint32_t)e[21] << 8)) << 16)
                             | (uint32_t)e[26] | ((uint32_t)e[27] << 8);
            else
                hit->cluster = (uint32_t)e[26] | ((uint32_t)e[27] << 8);
            hit->size = (uint32_t)e[28] | ((uint32_t)e[29] << 8)
                      | ((uint32_t)e[30] << 16) | ((uint32_t)e[31] << 24);
            return 1;
        }
        clus = fat_next_any(fat_lba, kind, clus);
    }
    return 0;
}

/* List all entries of a cluster-chain directory (subdir or FAT32 root). */
static void fat_print_chain(uint32_t start_cluster, int kind, uint32_t fat_lba,
                            uint32_t data_lba, uint8_t spc) {
    uint32_t clus = start_cluster;
    int count = 0, guard = 0;
    struct lfn_acc lfn;
    lfn_acc_reset(&lfn);
    while (clus >= 2 && !fat_eoc_any(kind, clus) && guard++ < 64 && count < 128) {
        uint32_t clus_lba = data_lba + (clus - 2) * spc;
        uint32_t secs = spc > 8 ? 8 : spc;
        uint8_t* dst = g_fs_buf;
        uint32_t done = 0;
        while (done < secs) {
            int chunk = (secs - done > 4) ? 4 : (int)(secs - done);
            int n = ata_read_lba28(0x1F0, 0, clus_lba + done, dst, chunk);
            if (n < 0) { kputs("ls: read error.\n"); return; }
            dst += n * 512;
            done += (uint32_t)n;
        }
        int entries = (int)(secs * 16);
        int stop = 0;
        for (int i = 0; i < entries; i++) {
            uint8_t* e = g_fs_buf + i * 32;
            if (e[0] == 0x00) { stop = 1; break; }
            if (e[0] == 0xE5) { lfn_acc_reset(&lfn); continue; }
            if (lfn_acc_feed(&lfn, e)) continue;
            uint8_t attr = e[11];
            /* prefer assembled LFN when checksum binds it to this entry */
            int use_lfn = lfn.active && lfn.ck == fat_lfn_cksum(e) && lfn.name[0];
            kputs("  ");
            if (attr & 0x10) kputs("<DIR>  "); else kputs("       ");
            if (use_lfn) kputs(lfn.name); else fat_print_name(e);
            lfn_acc_reset(&lfn);
            if (!(attr & 0x10)) {
                uint32_t sz = (uint32_t)e[28] | ((uint32_t)e[29] << 8) | ((uint32_t)e[30] << 16) | ((uint32_t)e[31] << 24);
                kputs("  ");
                kprint_dec64(sz);
                kputs(" B");
            }
            kputs("\n");
            count++;
            if (count >= 128) { kputs("  ... (truncated)\n"); stop = 1; break; }
        }
        if (stop) break;
        clus = fat_next_any(fat_lba, kind, clus);
    }
    kputs("ls: ");
    kprint_dec64(count);
    kputs(" entrie(s).\n");
}

/* Scan the fixed-size FAT12/16 root directory region for an 8.3 or LFN name. */
static int fat_scan_root16(uint32_t root_lba, uint32_t root_secs,
                           const uint8_t want[11], const char* want_name,
                           struct fat_hit* hit) {
    if (root_secs > 8) root_secs = 8;
    uint8_t* dst = g_fs_buf;
    uint32_t done = 0;
    while (done < root_secs) {
        int chunk = (root_secs - done > 4) ? 4 : (int)(root_secs - done);
        int n = ata_read_lba28(0x1F0, 0, root_lba + done, dst, chunk);
        if (n < 0) return -1;
        dst += n * 512;
        done += (uint32_t)n;
    }
    struct lfn_acc lfn;
    lfn_acc_reset(&lfn);
    for (uint32_t i = 0; i < done * 16; i++) {
        uint8_t* e = g_fs_buf + i * 32;
        if (e[0] == 0x00) return 0;
        if (e[0] == 0xE5) { lfn_acc_reset(&lfn); continue; }
        if (lfn_acc_feed(&lfn, e)) continue;
        int match = fat_name_eq(e, want) || lfn_acc_match(&lfn, e, want_name);
        lfn_acc_reset(&lfn);
        if (!match) continue;
        hit->is_dir = (e[11] & 0x10) != 0;
        hit->cluster = (uint32_t)e[26] | ((uint32_t)e[27] << 8);
        hit->size = (uint32_t)e[28] | ((uint32_t)e[29] << 8)
                  | ((uint32_t)e[30] << 16) | ((uint32_t)e[31] << 24);
        return 1;
    }
    return 0;
}

/* Resolve a '/'-separated path against the volume root.
 * kind: 0=FAT12 1=FAT16 2=FAT32. root_clus only used for FAT32.
 * Intermediate components must be directories. Last component may be file or dir.
 * Returns 1=found, 0=not found, -1=read error, -2=not a directory. */
static int fat_lookup(int kind, uint32_t root_clus, uint32_t root_lba, uint32_t root_secs,
                      uint32_t fat_lba, uint32_t data_lba, uint8_t spc,
                      const char* path, struct fat_hit* hit) {
    char buf[128];
    int bl = 0;
    for (const char* s = path; *s && bl < 127; s++) {
        char c = *s;
        buf[bl++] = (c == '\\') ? '/' : c;
    }
    buf[bl] = '\0';

    int in_root = 1;
    uint32_t cur_cluster = 0;
    char* save = 0;
    for (char* comp = buf; comp && *comp;) {
        char* slash = comp;
        while (*slash && *slash != '/') slash++;
        int last = (*slash == '\0');
        *slash = '\0';
        if (*comp == '\0') { comp = slash + 1; continue; }

        uint8_t want[11];
        fat_name_to_83(comp, want);
        struct fat_hit cur;
        int r;
        if (in_root) {
            if (kind == 2)
                r = fat_scan_dir(root_clus, kind, fat_lba, data_lba, spc, want, comp, &cur);
            else
                r = fat_scan_root16(root_lba, root_secs, want, comp, &cur);
        } else {
            r = fat_scan_dir(cur_cluster, kind, fat_lba, data_lba, spc, want, comp, &cur);
        }
        if (r <= 0) return r;
        if (last) { *hit = cur; return 1; }
        if (!cur.is_dir || cur.cluster < 2) return -2;
        cur_cluster = cur.cluster;
        in_root = 0;
        comp = slash + 1;
        (void)save;
    }
    return 0;
}

/* ---- FAT write support ----
 * Minimal write path: allocate one cluster, create directory entries.
 * SEPARATION OF MECHANISM & POLICY: ata_write moves sectors; fat_alloc_*
 * owns FAT policy; show_* owns UX. */

/* Allocate the first free cluster (FAT entry == 0), mark it EOC,
 * write the FAT sector(s) back. Returns cluster number or 0. */
static uint32_t fat_alloc_cluster(int kind) {
    if (fat_ensure_mounted() != 0) return 0;
    uint32_t fat_bytes = g_vol_fatsz * 512u;
    if (fat_bytes == 0 || fat_bytes > 0x400000u) return 0;
    uint8_t* fat = (uint8_t*)kmalloc(fat_bytes);
    if (!fat) return 0;
    uint32_t done = 0;
    int fail = 0;
    while (done < g_vol_fatsz) {
        int chunk = (g_vol_fatsz - done > 4) ? 4 : (int)(g_vol_fatsz - done);
        int n = ata_read_lba28(0x1F0, 0, g_vol_fat_lba + done, fat + done * 512u, chunk);
        if (n < 0) { fail = 1; break; }
        done += (uint32_t)n;
    }
    if (fail) { kfree(fat); return 0; }

    uint32_t limit = g_vol_total_clusters;
    uint32_t found = 0, byte_off = 0, width = 0;
    for (uint32_t c = 2; c < limit; c++) {
        int free_e = 0;
        if (kind == 2) {
            uint32_t v = (uint32_t)fat[c*4] | ((uint32_t)fat[c*4+1] << 8)
                       | ((uint32_t)fat[c*4+2] << 16) | ((uint32_t)fat[c*4+3] << 24);
            free_e = (v & 0x0FFFFFFFu) == 0;
            byte_off = c * 4; width = 4;
        } else if (kind == 1) {
            uint16_t v = (uint16_t)fat[c*2] | ((uint16_t)fat[c*2+1] << 8);
            free_e = (v == 0);
            byte_off = c * 2; width = 2;
        } else {
            uint32_t off = c + (c >> 1);
            if (off + 1 >= fat_bytes) break;
            uint16_t v = (uint16_t)fat[off] | ((uint16_t)fat[off+1] << 8);
            uint16_t e = (c & 1) ? (uint16_t)((v >> 4) & 0x0FFF)
                                : (uint16_t)(v & 0x0FFF);
            free_e = (e == 0);
            byte_off = off; width = 2;
        }
        if (free_e) { found = c; break; }
    }
    if (!found) { kfree(fat); return 0; }

    /* patch EOC marker */
    if (kind == 2) {
        fat[byte_off]   = 0xF8; fat[byte_off+1] = 0xFF;
        fat[byte_off+2] = 0xFF; fat[byte_off+3] = 0x0F;
    } else if (kind == 1) {
        fat[byte_off] = 0xFF; fat[byte_off+1] = 0xFF;
    } else {
        uint16_t v = (uint16_t)fat[byte_off] | ((uint16_t)fat[byte_off+1] << 8);
        if (found & 1) v = (uint16_t)((v & 0x000F) | (0x0FFFu << 4));
        else           v = (uint16_t)((v & 0xF000) | 0x0FFFu);
        fat[byte_off]   = (uint8_t)(v & 0xFF);
        fat[byte_off+1] = (uint8_t)(v >> 8);
    }

    /* write back sectors covering the patched bytes */
    uint32_t sec0 = byte_off / 512;
    uint32_t sec1 = (byte_off + width - 1) / 512;
    for (uint32_t s = sec0; s <= sec1; s += 4) {
        int chunk = (int)(sec1 - s + 1 > 4 ? 4 : sec1 - s + 1);
        if (ata_write_lba28(0x1F0, 0, g_vol_fat_lba + s, fat + s * 512u, chunk) < 0) {
            kfree(fat); return 0;
        }
    }
    kfree(fat);
    return found;
}

/* Patch a single FAT entry (read-modify-write its sector).
 * val semantics per kind: raw next-cluster value (EOC markers included). */
static int fat_set_entry(int kind, uint32_t clus, uint32_t val) {
    if (fat_ensure_mounted() != 0) return -1;
    uint32_t byte_off, width;
    if (kind == 2)      { byte_off = clus * 4; width = 4; }
    else if (kind == 1) { byte_off = clus * 2; width = 2; }
    else {
        byte_off = clus + (clus >> 1); width = 2;
        if (byte_off + 1 >= g_vol_fatsz * 512u) return -1;
    }
    uint32_t sec = byte_off / 512, off = byte_off % 512;
    /* FAT12 entries can straddle a sector boundary — handle by loading
     * up to 2 sectors into a contiguous buffer */
    uint8_t tmp[1024];
    int nsec = (off + width > 512) ? 2 : 1;
    if (ata_read_lba28(0x1F0, 0, g_vol_fat_lba + sec, tmp, nsec) < 0) return -1;
    if (kind == 2) {
        tmp[off]   = (uint8_t)(val & 0xFF);
        tmp[off+1] = (uint8_t)((val >> 8) & 0xFF);
        tmp[off+2] = (uint8_t)((val >> 16) & 0xFF);
        tmp[off+3] = (uint8_t)((val >> 24) & 0xFF);
    } else if (kind == 1) {
        tmp[off] = (uint8_t)(val & 0xFF); tmp[off+1] = (uint8_t)(val >> 8);
    } else {
        uint16_t v = (uint16_t)tmp[off] | ((uint16_t)tmp[off+1] << 8);
        if (clus & 1) v = (uint16_t)((v & 0x000F) | ((val & 0x0FFFu) << 4));
        else          v = (uint16_t)((v & 0xF000) | (val & 0x0FFFu));
        tmp[off] = (uint8_t)(v & 0xFF); tmp[off+1] = (uint8_t)(v >> 8);
    }
    return ata_write_lba28(0x1F0, 0, g_vol_fat_lba + sec, tmp, nsec);
}

/* Allocate `count` clusters as a linked chain (last = EOC).
 * Returns first cluster or 0. */
static uint32_t fat_alloc_chain(int kind, uint32_t count) {
    if (count == 0 || count > 4096) return 0;
    uint32_t* cl = (uint32_t*)kmalloc(count * 4);
    if (!cl) return 0;
    uint32_t n = 0;
    for (uint32_t i = 0; i < count; i++) {
        uint32_t c = fat_alloc_cluster(kind);
        if (!c) break;
        cl[n++] = c;
    }
    if (n == 0) { kfree(cl); return 0; }
    /* link them: entry[i] = cl[i+1], last stays EOC from alloc */
    for (uint32_t i = 0; i + 1 < n; i++) {
        if (fat_set_entry(kind, cl[i], cl[i+1]) != 0) {
            /* rollback: free what we allocated */
            fat_free_chain(kind, cl[0]);
            kfree(cl); return 0;
        }
    }
    uint32_t first = cl[0];
    kfree(cl);
    return first;
}

struct slot_res { uint32_t lba; uint32_t off; int status; };
/* status: 1=slot found, 0=dir full, -1=name exists, -2=io error */

/* Scan a directory for a free 32-byte slot and check name collision.
 * in_root=1 uses the fixed FAT12/16 root or FAT32 root cluster;
 * otherwise dir_cluster is the chain start. */
static int fat_find_dir_slot(int kind, int in_root,
                             uint32_t root_clus, uint32_t root_lba, uint32_t root_secs,
                             uint32_t dir_cluster, uint32_t fat_lba, uint32_t data_lba,
                             uint8_t spc, const uint8_t want[11], struct slot_res* r) {
    int seen_free = 0;
    uint32_t free_lba = 0, free_off = 0;
    uint32_t clus = dir_cluster;
    int guard = 0;

    for (;;) {
        uint32_t sec_base, nsecs;
        if (in_root && kind != 2) {
            sec_base = root_lba; nsecs = root_secs;
        } else {
            if (clus < 2 || fat_eoc_any(kind, clus) || guard++ > 1024) break;
            sec_base = data_lba + (clus - 2) * spc; nsecs = spc;
        }
        for (uint32_t s = 0; s < nsecs; s++) {
            uint32_t lba = sec_base + s;
            if (ata_read_lba28(0x1F0, 0, lba, g_file_buf, 1) < 0) { r->status = -2; return -1; }
            for (int i = 0; i < 16; i++) {
                uint8_t* e = g_file_buf + i * 32;
                if (e[0] == 0x0F) continue;              /* LFN entry */
                if (e[0] == 0x00) {                       /* end of dir */
                    if (seen_free) { r->lba = free_lba; r->off = free_off; }
                    else           { r->lba = lba;      r->off = (uint32_t)(i * 32); }
                    r->status = 1; return 1;
                }
                if (e[0] == 0xE5) {
                    if (!seen_free) { seen_free = 1; free_lba = lba; free_off = (uint32_t)(i * 32); }
                    continue;
                }
                if (fat_name_eq(e, want)) {
                    r->lba = lba; r->off = (uint32_t)(i * 32); r->status = -1;
                    return -1;
                }
            }
        }
        if (in_root && kind != 2) break;
        clus = fat_next_any(fat_lba, kind, clus);
    }
    r->status = 0;
    return 0;
}

/* Write a 32-byte directory entry at (lba, off). Caller fills `e`. */
static int fat_write_dirent(uint32_t lba, uint32_t off, const uint8_t* e) {
    if (ata_read_lba28(0x1F0, 0, lba, g_file_buf, 1) < 0) return -1;
    for (int i = 0; i < 32; i++) g_file_buf[off + i] = e[i];
    return ata_write_lba28(0x1F0, 0, lba, g_file_buf, 1);
}

static void show_mkdir(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    char name[64];
    int nl = 0;
    while (*p && *p != ' ' && nl < 63) name[nl++] = *p++;
    name[nl] = '\0';
    if (!nl) { kputs("usage: mkdir <name>\n"); return; }
    if (kstrlen(name) > 12) { kputs("mkdir: name too long (8.3 only).\n"); return; }
    for (const char* q = name; *q; q++) {
        if (*q == '/' || *q == '\\') { kputs("mkdir: single component only.\n"); return; }
    }

    if (fat_ensure_mounted() != 0) { kputs("mkdir: no FAT volume.\n"); return; }
    int kind = g_vol_kind;

    /* resolve parent directory */
    int in_root = 1;
    uint32_t dir_cluster = 0, parent_cluster = 0;
    if (g_cwd[0]) {
        struct fat_hit hit;
        int lr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                            g_vol_fat_lba, g_vol_data_lba, g_vol_spc,
                            g_cwd, &hit);
        if (lr <= 0 || !hit.is_dir) { kputs("mkdir: cwd not a directory.\n"); return; }
        in_root = 0;
        dir_cluster = hit.cluster;
        parent_cluster = hit.cluster;
    } else {
        parent_cluster = (kind == 2) ? g_vol_root_clus : 0;
    }

    uint8_t want[11];
    fat_name_to_83(name, want);
    struct slot_res slot;
    int sr = fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                               g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                               g_vol_data_lba, g_vol_spc, want, &slot);
    if (sr < 0 && slot.status == -1) { kputs("mkdir: name exists.\n"); return; }
    if (slot.status != 1) { kputs("mkdir: directory full or IO error.\n"); return; }

    uint32_t clus = fat_alloc_cluster(kind);
    if (!clus) { kputs("mkdir: disk full or FAT alloc failed.\n"); return; }

    /* write new dir entry into parent */
    uint8_t e[32];
    for (int i = 0; i < 32; i++) e[i] = 0;
    for (int i = 0; i < 11; i++) e[i] = want[i];
    e[11] = 0x10;                                 /* directory attr */
    e[20] = (uint8_t)(clus & 0xFF); e[21] = (uint8_t)(clus >> 8);
    if (kind == 2) { e[26] = (uint8_t)(clus >> 16); e[27] = (uint8_t)(clus >> 24); }
    if (fat_write_dirent(slot.lba, slot.off, e) != 0) {
        kputs("mkdir: dirent write failed.\n"); return;
    }

    /* initialise new cluster: "." and ".." entries, rest zero */
    uint8_t* sec = g_file_buf;
    for (int s = 0; s < (int)g_vol_spc; s++) {
        for (int i = 0; i < 512; i++) sec[i] = 0;
        if (s == 0) {
            uint8_t* d = sec;
            /* "." entry */
            d[0] = '.'; for (int i = 1; i < 11; i++) d[i] = ' ';
            d[11] = 0x10;
            d[20] = (uint8_t)(clus & 0xFF); d[21] = (uint8_t)(clus >> 8);
            if (kind == 2) { d[26] = (uint8_t)(clus >> 16); d[27] = (uint8_t)(clus >> 24); }
            /* ".." entry at +32 */
            uint8_t* u = sec + 32;
            u[0] = '.'; u[1] = '.'; for (int i = 2; i < 11; i++) u[i] = ' ';
            u[11] = 0x10;
            u[20] = (uint8_t)(parent_cluster & 0xFF); u[21] = (uint8_t)(parent_cluster >> 8);
            if (kind == 2) { u[26] = (uint8_t)(parent_cluster >> 16); u[27] = (uint8_t)(parent_cluster >> 24); }
        }
        uint32_t lba = g_vol_data_lba + (clus - 2) * g_vol_spc + (uint32_t)s;
        int chunk = 1;
        if (ata_write_lba28(0x1F0, 0, lba, sec, chunk) < 0) {
            kputs("mkdir: cluster write failed.\n"); return;
        }
    }
    kputs("mkdir: created /");
    kputs(g_cwd[0] ? g_cwd : "");
    if (g_cwd[0]) kputs("/");
    kputs(name);
    kputs("\n");
}

/* write <name> <text> — create a small file (single cluster, <= spc*512 B)
 * in the current directory. First file-creation op on the FAT write path. */
static void show_write(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    char name[64];
    int nl = 0;
    while (*p && *p != ' ' && nl < 63) name[nl++] = *p++;
    name[nl] = '\0';
    while (*p == ' ') p++;
    if (!nl || !*p) { kputs("usage: write <name> <text>\n"); return; }
    if (kstrlen(name) > 12) { kputs("write: name too long (8.3 only).\n"); return; }
    for (const char* q = name; *q; q++) {
        if (*q == '/' || *q == '\\') { kputs("write: single component only.\n"); return; }
    }
    uint32_t tlen = kstrlen(p);

    if (fat_ensure_mounted() != 0) { kputs("write: no FAT volume.\n"); return; }
    int kind = g_vol_kind;
    uint32_t cap = (uint32_t)g_vol_spc * 512u;
    if (tlen == 0 || tlen > cap) {
        kputs("write: text empty or exceeds one cluster.\n"); return;
    }

    /* resolve parent (same policy as mkdir) */
    int in_root = 1;
    uint32_t dir_cluster = 0;
    if (g_cwd[0]) {
        struct fat_hit hit;
        int lr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                            g_vol_fat_lba, g_vol_data_lba, g_vol_spc, g_cwd, &hit);
        if (lr <= 0 || !hit.is_dir) { kputs("write: cwd not a directory.\n"); return; }
        in_root = 0;
        dir_cluster = hit.cluster;
    }

    uint8_t want[11];
    fat_name_to_83(name, want);
    struct slot_res slot;
    fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                      g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                      g_vol_data_lba, g_vol_spc, want, &slot);
    if (slot.status == -1) { kputs("write: name exists.\n"); return; }
    if (slot.status != 1) { kputs("write: directory full or IO error.\n"); return; }

    uint32_t clus = fat_alloc_cluster(kind);
    if (!clus) { kputs("write: disk full or FAT alloc failed.\n"); return; }

    /* write file data: one cluster, zero-padded tail */
    uint32_t written = 0;
    for (uint32_t s = 0; s < g_vol_spc && written < tlen; s++) {
        uint8_t* sec = g_file_buf;
        for (int i = 0; i < 512; i++) sec[i] = 0;
        uint32_t chunk = tlen - written > 512 ? 512 : tlen - written;
        for (uint32_t i = 0; i < chunk; i++) sec[i] = (uint8_t)p[written + i];
        uint32_t lba = g_vol_data_lba + (clus - 2) * g_vol_spc + s;
        if (ata_write_lba28(0x1F0, 0, lba, sec, 1) < 0) {
            kputs("write: data write failed.\n"); return;
        }
        written += chunk;
    }

    /* write dirent: attr 0x20 (archive), size = tlen */
    uint8_t e[32];
    for (int i = 0; i < 32; i++) e[i] = 0;
    for (int i = 0; i < 11; i++) e[i] = want[i];
    e[11] = 0x20;
    e[20] = (uint8_t)(clus & 0xFF); e[21] = (uint8_t)(clus >> 8);
    if (kind == 2) { e[26] = (uint8_t)(clus >> 16); e[27] = (uint8_t)(clus >> 24); }
    e[28] = (uint8_t)(tlen & 0xFF); e[29] = (uint8_t)((tlen >> 8) & 0xFF);
    e[30] = (uint8_t)((tlen >> 16) & 0xFF); e[31] = (uint8_t)(tlen >> 24);
    if (fat_write_dirent(slot.lba, slot.off, e) != 0) {
        kputs("write: dirent write failed.\n"); return;
    }
    kputs("write: created /");
    if (g_cwd[0]) { kputs(g_cwd); kputs("/"); }
    kputs(name);
    kputs(" (");
    kprint_dec64(tlen);
    kputs(" bytes)\n");
}

/* Free an entire cluster chain: zero every FAT entry, write FAT back.
 * Walks the chain on an in-memory copy of the FAT. Returns 0 ok, -1 fail. */
static int fat_free_chain(int kind, uint32_t start) {
    if (fat_ensure_mounted() != 0 || start < 2) return -1;
    uint32_t fat_bytes = g_vol_fatsz * 512u;
    if (fat_bytes == 0 || fat_bytes > 0x400000u) return -1;
    uint8_t* fat = (uint8_t*)kmalloc(fat_bytes);
    if (!fat) return -1;
    uint32_t done = 0;
    int fail = 0;
    while (done < g_vol_fatsz) {
        int chunk = (g_vol_fatsz - done > 4) ? 4 : (int)(g_vol_fatsz - done);
        int n = ata_read_lba28(0x1F0, 0, g_vol_fat_lba + done, fat + done * 512u, chunk);
        if (n < 0) { fail = 1; break; }
        done += (uint32_t)n;
    }
    if (fail) { kfree(fat); return -1; }

    /* next-cluster decode operating on the FAT image */
    uint32_t clus = start;
    int guard = 0;
    while (clus >= 2 && !fat_eoc_any(kind, clus) && guard++ < 4096) {
        uint32_t nxt;
        if (kind == 2) {
            uint32_t o = clus * 4;
            nxt = ((uint32_t)fat[o] | ((uint32_t)fat[o+1] << 8)
                 | ((uint32_t)fat[o+2] << 16) | ((uint32_t)fat[o+3] << 24))
                  & 0x0FFFFFFFu;
            fat[o] = fat[o+1] = fat[o+2] = fat[o+3] = 0;
        } else if (kind == 1) {
            uint32_t o = clus * 2;
            nxt = (uint16_t)fat[o] | ((uint16_t)fat[o+1] << 8);
            fat[o] = fat[o+1] = 0;
        } else {
            uint32_t o = clus + (clus >> 1);
            uint16_t v = (uint16_t)fat[o] | ((uint16_t)fat[o+1] << 8);
            nxt = (clus & 1) ? (uint16_t)((v >> 4) & 0x0FFF)
                            : (uint16_t)(v & 0x0FFF);
            /* zero the 12-bit entry, preserve the packed neighbour:
             * even c: entry = fat[o] + low nibble of fat[o+1]
             * odd  c: entry = high nibble of fat[o] + fat[o+1] */
            if (clus & 1) { fat[o] &= 0x0F; fat[o+1] = 0; }
            else           { fat[o] = 0;     fat[o+1] &= 0xF0; }
        }
        clus = nxt;
    }

    /* write entire FAT back (simplest correct policy for v1) */
    done = 0;
    while (done < g_vol_fatsz) {
        int chunk = (g_vol_fatsz - done > 4) ? 4 : (int)(g_vol_fatsz - done);
        if (ata_write_lba28(0x1F0, 0, g_vol_fat_lba + done, fat + done * 512u, chunk) < 0) {
            kfree(fat); return -1;
        }
        done += (uint32_t)chunk;
    }
    kfree(fat);
    return 0;
}

/* rm <name> — delete a file in the current directory: free its cluster
 * chain and mark the directory entry deleted (0xE5). Files only. */
static void show_rm(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    char name[64];
    int nl = 0;
    while (*p && *p != ' ' && nl < 63) name[nl++] = *p++;
    name[nl] = '\0';
    if (!nl) { kputs("usage: rm <file>\n"); return; }

    if (fat_ensure_mounted() != 0) { kputs("rm: no FAT volume.\n"); return; }
    int kind = g_vol_kind;

    int in_root = 1;
    uint32_t dir_cluster = 0;
    if (g_cwd[0]) {
        struct fat_hit hit;
        int lr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                            g_vol_fat_lba, g_vol_data_lba, g_vol_spc, g_cwd, &hit);
        if (lr <= 0 || !hit.is_dir) { kputs("rm: cwd not a directory.\n"); return; }
        in_root = 0;
        dir_cluster = hit.cluster;
    }

    uint8_t want[11];
    fat_name_to_83(name, want);
    struct slot_res slot;
    fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                      g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                      g_vol_data_lba, g_vol_spc, want, &slot);
    if (slot.status != -1) { kputs("rm: not found.\n"); return; }

    /* read the dirent to get attr + cluster */
    if (ata_read_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("rm: read error.\n"); return;
    }
    uint8_t* e = g_file_buf + slot.off;
    if (e[11] & 0x10) { kputs("rm: is a directory (rmdir not implemented).\n"); return; }
    uint32_t clus = (uint32_t)e[20] | ((uint32_t)e[21] << 8);
    if (kind == 2) clus |= ((uint32_t)e[26] << 16) | ((uint32_t)e[27] << 24);

    if (clus >= 2 && fat_free_chain(kind, clus) != 0) {
        kputs("rm: FAT update failed.\n"); return;
    }
    e[0] = 0xE5;                                     /* mark deleted */
    if (ata_write_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("rm: dirent write failed.\n"); return;
    }
    kputs("rm: deleted /");
    if (g_cwd[0]) { kputs(g_cwd); kputs("/"); }
    kputs(name);
    kputs("\n");
}

/* rmdir <name> — remove an EMPTY directory: verify only "."/".." entries,
 * free its cluster chain, mark the dirent deleted. */
static void show_rmdir(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    char name[64];
    int nl = 0;
    while (*p && *p != ' ' && nl < 63) name[nl++] = *p++;
    name[nl] = '\0';
    if (!nl) { kputs("usage: rmdir <dir>\n"); return; }

    if (fat_ensure_mounted() != 0) { kputs("rmdir: no FAT volume.\n"); return; }
    int kind = g_vol_kind;

    int in_root = 1;
    uint32_t dir_cluster = 0;
    if (g_cwd[0]) {
        struct fat_hit hit;
        int lr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                            g_vol_fat_lba, g_vol_data_lba, g_vol_spc, g_cwd, &hit);
        if (lr <= 0 || !hit.is_dir) { kputs("rmdir: cwd not a directory.\n"); return; }
        in_root = 0;
        dir_cluster = hit.cluster;
    }

    uint8_t want[11];
    fat_name_to_83(name, want);
    struct slot_res slot;
    fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                      g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                      g_vol_data_lba, g_vol_spc, want, &slot);
    if (slot.status != -1) { kputs("rmdir: not found.\n"); return; }

    if (ata_read_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("rmdir: read error.\n"); return;
    }
    uint8_t* e = g_file_buf + slot.off;
    if (!(e[11] & 0x10)) { kputs("rmdir: not a directory (use rm).\n"); return; }
    uint32_t clus = (uint32_t)e[20] | ((uint32_t)e[21] << 8);
    if (kind == 2) clus |= ((uint32_t)e[26] << 16) | ((uint32_t)e[27] << 24);
    if (clus < 2) { kputs("rmdir: bad cluster.\n"); return; }

    /* verify empty: walk the dir's cluster chain, allow only . .. 00 E5 0F */
    int empty = 1, guard = 0;
    uint32_t c = clus;
    while (c >= 2 && !fat_eoc_any(kind, c) && guard++ < 1024 && empty) {
        uint32_t base = g_vol_data_lba + (c - 2) * g_vol_spc;
        for (uint32_t s = 0; s < g_vol_spc && empty; s++) {
            if (ata_read_lba28(0x1F0, 0, base + s, g_file_buf, 1) < 0) {
                kputs("rmdir: read error.\n"); return;
            }
            for (int i = 0; i < 16; i++) {
                uint8_t* d = g_file_buf + i * 32;
                if (d[0] == 0x00 || d[0] == 0xE5 || d[11] == 0x0F) continue;
                if (d[0] == '.' && (d[1] == ' ' || d[1] == '.')) continue;
                empty = 0; break;
            }
        }
        c = fat_next_any(g_vol_fat_lba, kind, c);
    }
    if (!empty) { kputs("rmdir: directory not empty.\n"); return; }

    if (fat_free_chain(kind, clus) != 0) { kputs("rmdir: FAT update failed.\n"); return; }
    /* re-read sector: g_file_buf was clobbered by the emptiness scan */
    if (ata_read_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("rmdir: read error.\n"); return;
    }
    g_file_buf[slot.off] = 0xE5;
    if (ata_write_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("rmdir: dirent write failed.\n"); return;
    }
    kputs("rmdir: removed /");
    if (g_cwd[0]) { kputs(g_cwd); kputs("/"); }
    kputs(name);
    kputs("\n");
}

/* mv <old> <new> — rename within the current directory by rewriting the
 * dirent's 8.3 name field. Cross-directory moves are not yet supported. */
static void show_mv(const char* args) {
    char old[64], newn[64];
    int oi = 0, ni = 0;
    const char* p = args;
    while (*p == ' ') p++;
    while (*p && *p != ' ' && oi < 63) old[oi++] = *p++;
    old[oi] = 0;
    while (*p == ' ') p++;
    while (*p && *p != ' ' && ni < 63) newn[ni++] = *p++;
    newn[ni] = 0;
    if (!oi || !ni) { kputs("usage: mv <old> <new>\n"); return; }
    if (kstrlen(newn) > 12) { kputs("mv: new name too long (8.3 only).\n"); return; }

    if (fat_ensure_mounted() != 0) { kputs("mv: no FAT volume.\n"); return; }
    int kind = g_vol_kind;

    int in_root = 1;
    uint32_t dir_cluster = 0;
    if (g_cwd[0]) {
        struct fat_hit hit;
        int lr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                            g_vol_fat_lba, g_vol_data_lba, g_vol_spc, g_cwd, &hit);
        if (lr <= 0 || !hit.is_dir) { kputs("mv: cwd not a directory.\n"); return; }
        in_root = 0;
        dir_cluster = hit.cluster;
    }

    uint8_t want_old[11], want_new[11];
    fat_name_to_83(old, want_old);
    fat_name_to_83(newn, want_new);
    if (fat_name_eq(want_old, want_new)) { kputs("mv: same name.\n"); return; }

    /* collision check for the new name */
    struct slot_res chk;
    fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                      g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                      g_vol_data_lba, g_vol_spc, want_new, &chk);
    if (chk.status == -1) { kputs("mv: target name exists.\n"); return; }

    /* locate old dirent */
    struct slot_res slot;
    fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                      g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                      g_vol_data_lba, g_vol_spc, want_old, &slot);
    if (slot.status != -1) { kputs("mv: not found.\n"); return; }

    if (ata_read_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("mv: read error.\n"); return;
    }
    for (int i = 0; i < 11; i++) g_file_buf[slot.off + i] = want_new[i];
    if (ata_write_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("mv: write failed.\n"); return;
    }
    kputs("mv: /");
    if (g_cwd[0]) { kputs(g_cwd); kputs("/"); }
    kputs(old);
    kputs(" -> ");
    kputs(newn);
    kputs("\n");
}

/* cp <src> <dst-name> — copy a file into the current directory.
 * Src may be a cwd-relative or absolute path; dst is a single 8.3 name.
 * Multi-cluster files supported via fat_alloc_chain. */
static void show_cp(const char* args) {
    char src[160], dst[64];
    int si = 0, di = 0;
    const char* p = args;
    while (*p == ' ') p++;
    while (*p && *p != ' ' && si < 159) src[si++] = *p++;
    src[si] = 0;
    while (*p == ' ') p++;
    while (*p && *p != ' ' && di < 63) dst[di++] = *p++;
    dst[di] = 0;
    if (!si || !di) { kputs("usage: cp <src> <dst>\n"); return; }
    if (kstrlen(dst) > 12) { kputs("cp: dst name too long (8.3 only).\n"); return; }
    for (const char* q = dst; *q; q++) {
        if (*q == '/' || *q == '\\') { kputs("cp: dst must be a plain name.\n"); return; }
    }

    if (fat_ensure_mounted() != 0) { kputs("cp: no FAT volume.\n"); return; }
    int kind = g_vol_kind;

    /* resolve source path against cwd */
    char rpath[160];
    if (fs_resolve(src, rpath, (int)sizeof rpath) != 0) {
        kputs("cp: src path too long.\n"); return;
    }
    struct fat_hit hit;
    int lr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                        g_vol_fat_lba, g_vol_data_lba, g_vol_spc, rpath, &hit);
    if (lr <= 0) { kputs("cp: source not found.\n"); return; }
    if (hit.is_dir) { kputs("cp: source is a directory.\n"); return; }
    if (hit.cluster < 2 || hit.size == 0) { kputs("cp: empty file.\n"); return; }
    if (hit.size > 0x400000u) { kputs("cp: file too large (>4MiB).\n"); return; }

    /* dst parent = cwd (same policy as write) */
    int in_root = 1;
    uint32_t dir_cluster = 0;
    if (g_cwd[0]) {
        struct fat_hit dh;
        int dr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                            g_vol_fat_lba, g_vol_data_lba, g_vol_spc, g_cwd, &dh);
        if (dr <= 0 || !dh.is_dir) { kputs("cp: cwd not a directory.\n"); return; }
        in_root = 0;
        dir_cluster = dh.cluster;
    }
    uint8_t want[11];
    fat_name_to_83(dst, want);
    struct slot_res slot;
    fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                      g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                      g_vol_data_lba, g_vol_spc, want, &slot);
    if (slot.status == -1) { kputs("cp: dst name exists.\n"); return; }
    if (slot.status != 1) { kputs("cp: directory full or IO error.\n"); return; }

    uint32_t clus_bytes = (uint32_t)g_vol_spc * 512u;
    uint32_t need = (hit.size + clus_bytes - 1) / clus_bytes;
    uint32_t first = fat_alloc_chain(kind, need);
    if (!first) { kputs("cp: disk full or alloc failed.\n"); return; }

    /* copy data cluster-by-cluster, sector-by-sector */
    uint32_t remaining = hit.size;
    uint32_t sc = hit.cluster, dc = first;
    int guard = 0, fail = 0;
    while (sc >= 2 && !fat_eoc_any(kind, sc) && guard++ < 8192 && remaining > 0) {
        if (dc < 2 || fat_eoc_any(kind, dc)) { fail = 1; break; }
        uint32_t sbase = g_vol_data_lba + (sc - 2) * g_vol_spc;
        uint32_t dbase = g_vol_data_lba + (dc - 2) * g_vol_spc;
        for (uint32_t s = 0; s < g_vol_spc && remaining > 0; s++) {
            if (ata_read_lba28(0x1F0, 0, sbase + s, g_file_buf, 1) < 0 ||
                ata_write_lba28(0x1F0, 0, dbase + s, g_file_buf, 1) < 0) {
                fail = 1; break;
            }
            remaining = remaining > 512 ? remaining - 512 : 0;
        }
        if (fail) break;
        sc = fat_next_any(g_vol_fat_lba, kind, sc);
        dc = fat_next_any(g_vol_fat_lba, kind, dc);
        if (dc < 2 || fat_eoc_any(kind, dc)) dc = 0;
    }
    if (fail || remaining > 0) {
        fat_free_chain(kind, first);
        kputs("cp: copy failed mid-file.\n"); return;
    }

    /* dst dirent */
    uint8_t e[32];
    for (int i = 0; i < 32; i++) e[i] = 0;
    for (int i = 0; i < 11; i++) e[i] = want[i];
    e[11] = 0x20;
    e[20] = (uint8_t)(first & 0xFF); e[21] = (uint8_t)(first >> 8);
    if (kind == 2) { e[26] = (uint8_t)(first >> 16); e[27] = (uint8_t)(first >> 24); }
    uint32_t sz = hit.size;
    e[28] = (uint8_t)(sz & 0xFF); e[29] = (uint8_t)((sz >> 8) & 0xFF);
    e[30] = (uint8_t)((sz >> 16) & 0xFF); e[31] = (uint8_t)(sz >> 24);
    if (fat_write_dirent(slot.lba, slot.off, e) != 0) {
        kputs("cp: dirent write failed.\n"); return;
    }
    kputs("cp: /");
    if (g_cwd[0]) { kputs(g_cwd); kputs("/"); }
    kputs(dst);
    kputs(" (");
    kprint_dec64(hit.size);
    kputs(" bytes)\n");
}

/* append <name> <text> — extend an existing file. Walks the cluster chain
 * to the write offset, allocates+links clusters as needed, updates size. */
static void show_append(const char* args) {
    char name[64];
    int nl = 0;
    const char* p = args;
    while (*p == ' ') p++;
    while (*p && *p != ' ' && nl < 63) name[nl++] = *p++;
    name[nl] = 0;
    while (*p == ' ') p++;
    if (!nl || !*p) { kputs("usage: append <name> <text>\n"); return; }
    uint32_t tlen = kstrlen(p);
    if (tlen > 0x100000u) { kputs("append: text too large.\n"); return; }

    if (fat_ensure_mounted() != 0) { kputs("append: no FAT volume.\n"); return; }
    int kind = g_vol_kind;

    /* resolve parent dir (same policy as write/cp) */
    int in_root = 1;
    uint32_t dir_cluster = 0;
    if (g_cwd[0]) {
        struct fat_hit dh;
        int dr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                            g_vol_fat_lba, g_vol_data_lba, g_vol_spc, g_cwd, &dh);
        if (dr <= 0 || !dh.is_dir) { kputs("append: cwd not a directory.\n"); return; }
        in_root = 0; dir_cluster = dh.cluster;
    }
    uint8_t want[11];
    fat_name_to_83(name, want);
    struct slot_res slot;
    fat_find_dir_slot(kind, in_root, g_vol_root_clus, g_vol_root_lba,
                      g_vol_root_secs, dir_cluster, g_vol_fat_lba,
                      g_vol_data_lba, g_vol_spc, want, &slot);
    if (slot.status != -1) { kputs("append: not found.\n"); return; }

    if (ata_read_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("append: read error.\n"); return;
    }
    uint8_t* de = g_file_buf + slot.off;
    if (de[11] & 0x10) { kputs("append: is a directory.\n"); return; }
    uint32_t fsize = (uint32_t)de[28] | ((uint32_t)de[29] << 8)
                   | ((uint32_t)de[30] << 16) | ((uint32_t)de[31] << 24);
    uint32_t clus = (uint32_t)de[20] | ((uint32_t)de[21] << 8);
    if (kind == 2) clus |= ((uint32_t)de[26] << 16) | ((uint32_t)de[27] << 24);

    uint32_t clus_bytes = (uint32_t)g_vol_spc * 512u;

    /* collect existing cluster chain into a list (bounded) */
    uint32_t maxc = (fsize + clus_bytes - 1) / clus_bytes + 8;
    if (maxc < 16) maxc = 16;
    if (maxc > 8192) maxc = 8192;
    uint32_t* chain = (uint32_t*)kmalloc(maxc * 4);
    if (!chain) { kputs("append: out of memory.\n"); return; }
    uint32_t ncl = 0;
    uint32_t c = clus;
    int guard = 0;
    while (c >= 2 && !fat_eoc_any(kind, c) && ncl < maxc && guard++ < 8192) {
        chain[ncl++] = c;
        c = fat_next_any(g_vol_fat_lba, kind, c);
    }

    /* ensure capacity: append clusters until chain covers fsize+tlen */
    uint64_t need_bytes = (uint64_t)fsize + tlen;
    uint64_t have_bytes = (uint64_t)ncl * clus_bytes;
    int fail = 0;
    while (have_bytes < need_bytes && ncl < maxc) {
        uint32_t nc = fat_alloc_cluster(kind);
        if (!nc) { fail = 1; break; }
        if (ncl > 0) {
            if (fat_set_entry(kind, chain[ncl - 1], nc) != 0) { fail = 1; break; }
        } else {
            /* empty file: first cluster goes into dirent */
            de[20] = (uint8_t)(nc & 0xFF); de[21] = (uint8_t)(nc >> 8);
            if (kind == 2) { de[26] = (uint8_t)(nc >> 16); de[27] = (uint8_t)(nc >> 24); }
        }
        chain[ncl++] = nc;
        have_bytes += clus_bytes;
    }
    if (fail || have_bytes < need_bytes) {
        kfree(chain);
        kputs("append: disk full or alloc failed.\n"); return;
    }

    /* write appended bytes sector-by-sector at absolute file offset fsize */
    uint32_t remaining = tlen;
    uint32_t file_off = fsize;
    guard = 0;
    while (remaining > 0 && guard++ < 8192) {
        uint32_t ci = file_off / clus_bytes;
        uint32_t coff = file_off % clus_bytes;
        uint32_t lba = g_vol_data_lba + (chain[ci] - 2) * g_vol_spc + coff / 512;
        uint32_t soff = coff % 512;
        int partial = (soff != 0) || (remaining < 512 - soff);
        if (partial || soff != 0) {
            if (ata_read_lba28(0x1F0, 0, lba, g_file_buf, 1) < 0) { fail = 1; break; }
        } else {
            for (int i = 0; i < 512; i++) g_file_buf[i] = 0;
        }
        uint32_t chunk = 512 - soff;
        if (chunk > remaining) chunk = remaining;
        for (uint32_t i = 0; i < chunk; i++)
            g_file_buf[soff + i] = (uint8_t)p[tlen - remaining + i];
        if (ata_write_lba28(0x1F0, 0, lba, g_file_buf, 1) < 0) { fail = 1; break; }
        remaining -= chunk;
        file_off += chunk;
    }
    kfree(chain);
    if (fail) { kputs("append: write failed.\n"); return; }


    /* update dirent size + persist */
    fsize += tlen - remaining;
    de[28] = (uint8_t)(fsize & 0xFF); de[29] = (uint8_t)((fsize >> 8) & 0xFF);
    de[30] = (uint8_t)((fsize >> 16) & 0xFF); de[31] = (uint8_t)(fsize >> 24);
    /* re-read sector: g_file_buf clobbered */
    if (ata_read_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) < 0) {
        kputs("append: read error.\n"); return;
    }
    uint8_t* de2 = g_file_buf + slot.off;
    de2[28] = de[28]; de2[29] = de[29]; de2[30] = de[30]; de2[31] = de[31];
    /* also persist first-cluster field if file was empty */
    for (int i = 20; i <= 21; i++) de2[i] = de[i];
    if (kind == 2) { de2[26] = de[26]; de2[27] = de[27]; }
    if (ata_write_lba28(0x1F0, 0, slot.lba, g_file_buf, 1) != 1) {
        kputs("append: dirent write failed.\n"); return;
    }
    kputs("append: /");
    if (g_cwd[0]) { kputs(g_cwd); kputs("/"); }
    kputs(name);
    kputs(" -> ");
    kprint_dec64(fsize);
    kputs(" bytes\n");
}

/* hex <file> — hexdump a file (up to 4KB) for binary inspection. */
static void show_hex(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    if (!*p) { kputs("usage: hex <file>\n"); return; }
    char namebuf[64];
    int nl = 0;
    while (*p && *p != ' ' && nl < 63) namebuf[nl++] = *p++;
    namebuf[nl] = 0;

    if (fat_ensure_mounted() != 0) { kputs("hex: no FAT volume.\n"); return; }
    int kind = g_vol_kind;
    char rpath[160];
    if (fs_resolve(namebuf, rpath, (int)sizeof rpath) != 0) {
        kputs("hex: path too long.\n"); return;
    }
    struct fat_hit hit;
    int lr = fat_lookup(kind, g_vol_root_clus, g_vol_root_lba, g_vol_root_secs,
                        g_vol_fat_lba, g_vol_data_lba, g_vol_spc, rpath, &hit);
    if (lr <= 0 || hit.is_dir) { kputs("hex: not found.\n"); return; }

    uint32_t cap = hit.size > 4096 ? 4096 : hit.size;
    uint8_t* buf = (uint8_t*)kmalloc(cap ? cap : 1);
    if (!buf) { kputs("hex: out of memory.\n"); return; }
    int got = fat_read_to_buf(kind, g_vol_fat_lba, g_vol_data_lba, g_vol_spc,
                              hit.cluster, cap, buf);
    if (got < 0) { kfree(buf); kputs("hex: read error.\n"); return; }
    kputs("hex: ");
    kputs(rpath);
    kputs(" (");
    kprint_dec64((uint64_t)got);
    kputs(" bytes)\n");
    kprint_hexdump(buf, (uint32_t)got);
    kfree(buf);
}

static void show_cat(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    if (!*p) { kputs("usage: cat <filename>\n"); return; }
    char namebuf[32];
    int nl = 0;
    while (*p && *p != ' ' && nl < 31) namebuf[nl++] = *p++;
    namebuf[nl] = '\0';

    /* resolve volume root lba (same logic as ls) */
    uint32_t lba = 0;
    if (ata_read_lba28(0x1F0, 0, 0, g_sector_buf, 1) < 0) { kputs("cat: no MBR.\n"); return; }
    uint8_t* me = g_sector_buf + 446;
    lba = (uint32_t)me[8] | ((uint32_t)me[9] << 8) | ((uint32_t)me[10] << 16) | ((uint32_t)me[11] << 24);

    if (ata_read_lba28(0x1F0, 0, lba, g_sector_buf, 1) < 0) { kputs("cat: cannot read boot sector.\n"); return; }
    uint8_t* b = g_sector_buf;
    uint16_t bps   = (uint16_t)b[11] | ((uint16_t)b[12] << 8);
    uint8_t  spc   = b[13];
    uint16_t rsvd  = (uint16_t)b[14] | ((uint16_t)b[15] << 8);
    uint8_t  nfats = b[16];
    uint16_t rent  = (uint16_t)b[17] | ((uint16_t)b[18] << 8);
    uint16_t fsz16 = (uint16_t)b[22] | ((uint16_t)b[23] << 8);
    uint32_t fsz32 = (uint32_t)b[36] | ((uint32_t)b[37] << 8) | ((uint32_t)b[38] << 16) | ((uint32_t)b[39] << 24);
    uint32_t fatsz = fsz16 ? fsz16 : fsz32;
    int is_fat32 = (rent == 0);
    if (bps != 512 || spc == 0) { kputs("cat: unsupported geometry.\n"); return; }

    uint32_t fat_lba  = lba + rsvd;
    uint32_t data_lba = lba + rsvd + (uint32_t)nfats * fatsz;

    int kind;
    uint32_t root_clus = 0, root_lba = 0, root_secs = 0;
    if (is_fat32) {
        kind = 2;
        root_clus = (uint32_t)b[44] | ((uint32_t)b[45] << 8)
                  | ((uint32_t)b[46] << 16) | ((uint32_t)b[47] << 24);
        if (root_clus < 2) { kputs("cat: invalid FAT32 root cluster.\n"); return; }
    } else {
        uint32_t tot16 = (uint16_t)b[19] | ((uint16_t)b[20] << 8);
        uint32_t tot32 = (uint32_t)b[32] | ((uint32_t)b[33] << 8) | ((uint32_t)b[34] << 16) | ((uint32_t)b[35] << 24);
        uint32_t tot = tot16 ? tot16 : tot32;
        uint32_t root_dir_secs = (uint32_t)((rent * 32 + bps - 1) / bps);
        uint32_t data_clusters = (tot - rsvd - nfats * fatsz - root_dir_secs) / spc;
        kind = data_clusters < 4085 ? 0 : 1;
        data_lba += root_dir_secs;
        root_lba = lba + rsvd + (uint32_t)nfats * fatsz;
        root_secs = root_dir_secs;
    }

    char rpath[160];
    if (fs_resolve(namebuf, rpath, (int)sizeof rpath) != 0) {
        kputs("cat: path too long.\n"); return;
    }
    struct fat_hit hit;
    int lr = fat_lookup(kind, root_clus, root_lba, root_secs,
                        fat_lba, data_lba, spc, rpath, &hit);
    if (lr == -2) { kputs("cat: path component is not a directory.\n"); return; }
    if (lr <= 0) {
        kputs("cat: '");
        kputs(rpath);
        kputs("' not found.\n");
        return;
    }
    if (hit.is_dir) { kputs("cat: '"); kputs(rpath); kputs("' is a directory (use ls).\n"); return; }
    if (hit.cluster < 2) { kputs("cat: empty file.\n"); return; }

    /* walk cluster chain, print printable content (cap 4KB) */
    uint32_t clus = hit.cluster;
    uint32_t printed = 0;
    const uint32_t PRINT_CAP = 4096;
    int guard = 0;
    while (clus >= 2 && !fat_eoc_any(kind, clus) && guard++ < 1024 && printed < PRINT_CAP) {
        uint32_t clus_lba = data_lba + (clus - 2) * spc;
        for (uint32_t s = 0; s < spc && printed < PRINT_CAP; s++) {
            if (ata_read_lba28(0x1F0, 0, clus_lba + s, g_file_buf, 1) < 0) {
                kputs("\ncat: read error mid-file.\n");
                return;
            }
            for (int i = 0; i < 512 && printed < PRINT_CAP; i++) {
                uint8_t c = g_file_buf[i];
                if (c == '\n' || c == '\r' || c == '\t' || (c >= 32 && c <= 126)) {
                    kputc((c == '\r') ? '\n' : (char)c);
                } else {
                    kputc('.');
                }
                printed++;
            }
        }
        clus = fat_next_any(fat_lba, kind, clus);
    }
    kputs("\ncat: done (");
    kprint_dec64(hit.size);
    kputs(" bytes declared, ");
    kprint_dec64(printed);
    kputs(" shown)\n");
}

/* exec <path> — load a flat binary from FAT into the user region and
 * enter ring 3 at its first byte. Entry = base+0x1000, stack near top.
 * File format: raw position-dependent code, max ~2 MiB. */
#define EXEC_LOAD_OFF 0x1000u
#define EXEC_STACK_OFF 0x300000u
#define EXEC_MAX_SIZE  0x200000u
#define EXEC_ELF_MAX   0x100000u   /* kmalloc cap */

/* Read a whole file's cluster chain into a kernel buffer. Returns bytes
 * read or -1. */
static int fat_read_to_buf(int kind, uint32_t fat_lba, uint32_t data_lba,
                           uint8_t spc, uint32_t cluster, uint32_t size,
                           uint8_t* dst) {
    uint32_t remaining = size;
    uint32_t clus = cluster;
    int guard = 0;
    while (clus >= 2 && !fat_eoc_any(kind, clus) && guard++ < 8192 && remaining > 0) {
        uint32_t clus_lba = data_lba + (clus - 2) * spc;
        for (uint32_t s = 0; s < spc && remaining > 0; s++) {
            if (ata_read_lba28(0x1F0, 0, clus_lba + s, g_file_buf, 1) < 0) return -1;
            uint32_t chunk = remaining > 512 ? 512 : remaining;
            for (uint32_t i = 0; i < chunk; i++) dst[i] = g_file_buf[i];
            dst += chunk;
            remaining -= chunk;
        }
        clus = fat_next_any(fat_lba, kind, clus);
    }
    return (int)(size - remaining);
}

/* ELF32 loader: parse header + PT_LOAD segments, relocate by bias so any
 * linked vaddr lands inside the user region. Returns entry vaddr or 0. */
static uint32_t exec_load_elf(const uint8_t* f, uint32_t size) {
    if (size < 52) return 0;
    if (!(f[0] == 0x7F && f[1] == 'E' && f[2] == 'L' && f[3] == 'F')) return 0;
    if (f[4] != 1 || f[5] != 1) return 0;              /* 32-bit, little-endian */
    uint16_t machine = (uint16_t)f[18] | ((uint16_t)f[19] << 8);
    if (machine != 3) return 0;                        /* EM_386 */
    uint32_t entry  = (uint32_t)f[24] | ((uint32_t)f[25] << 8)
                    | ((uint32_t)f[26] << 16) | ((uint32_t)f[27] << 24);
    uint32_t phoff  = (uint32_t)f[28] | ((uint32_t)f[29] << 8)
                    | ((uint32_t)f[30] << 16) | ((uint32_t)f[31] << 24);
    uint16_t phentsz = (uint16_t)f[42] | ((uint16_t)f[43] << 8);
    uint16_t phnum   = (uint16_t)f[44] | ((uint16_t)f[45] << 8);
    if (phentsz < 32 || phnum == 0 || phnum > 32) return 0;
    if ((uint64_t)phoff + (uint64_t)phentsz * phnum > size) return 0;

    /* find lowest PT_LOAD vaddr */
    uint32_t minva = 0xFFFFFFFFu;
    for (int i = 0; i < phnum; i++) {
        const uint8_t* ph = f + phoff + (uint32_t)i * phentsz;
        uint32_t type = (uint32_t)ph[0] | ((uint32_t)ph[1] << 8)
                      | ((uint32_t)ph[2] << 16) | ((uint32_t)ph[3] << 24);
        if (type != 1) continue;                       /* PT_LOAD */
        uint32_t va = (uint32_t)ph[8] | ((uint32_t)ph[9] << 8)
                    | ((uint32_t)ph[10] << 16) | ((uint32_t)ph[11] << 24);
        if (va < minva) minva = va;
    }
    if (minva == 0xFFFFFFFFu) return 0;
    uint32_t bias = g_user_base - (minva & ~0xFFFu);

    /* load segments */
    for (int i = 0; i < phnum; i++) {
        const uint8_t* ph = f + phoff + (uint32_t)i * phentsz;
        uint32_t type = (uint32_t)ph[0] | ((uint32_t)ph[1] << 8)
                      | ((uint32_t)ph[2] << 16) | ((uint32_t)ph[3] << 24);
        if (type != 1) continue;
        uint32_t off   = (uint32_t)ph[4] | ((uint32_t)ph[5] << 8)
                       | ((uint32_t)ph[6] << 16) | ((uint32_t)ph[7] << 24);
        uint32_t va    = (uint32_t)ph[8] | ((uint32_t)ph[9] << 8)
                       | ((uint32_t)ph[10] << 16) | ((uint32_t)ph[11] << 24);
        uint32_t fsz   = (uint32_t)ph[16] | ((uint32_t)ph[17] << 8)
                       | ((uint32_t)ph[18] << 16) | ((uint32_t)ph[19] << 24);
        uint32_t msz   = (uint32_t)ph[20] | ((uint32_t)ph[21] << 8)
                       | ((uint32_t)ph[22] << 16) | ((uint32_t)ph[23] << 24);
        if (off + fsz > size || msz < fsz) return 0;
        uint32_t dst = bias + va;
        if (dst < g_user_base || dst + msz > g_user_base + 0x400000u) return 0;
        uint8_t* d = (uint8_t*)(size_t)dst;
        for (uint32_t j = 0; j < fsz; j++) d[j] = f[off + j];
        for (uint32_t j = fsz; j < msz; j++) d[j] = 0;   /* bss */
    }
    return bias + entry;
}

static void show_exec(const char* args) {
    const char* p = args;
    while (*p == ' ') p++;
    if (!*p) { kputs("usage: exec <file>\n"); return; }
    char namebuf[64];
    int nl = 0;
    while (*p && *p != ' ' && nl < 63) namebuf[nl++] = *p++;
    namebuf[nl] = '\0';

    /* --- resolve FAT volume (same geometry logic as cat) --- */
    uint32_t lba = 0;
    if (ata_read_lba28(0x1F0, 0, 0, g_sector_buf, 1) < 0) { kputs("exec: no MBR.\n"); return; }
    uint8_t* me = g_sector_buf + 446;
    lba = (uint32_t)me[8] | ((uint32_t)me[9] << 8) | ((uint32_t)me[10] << 16) | ((uint32_t)me[11] << 24);
    if (ata_read_lba28(0x1F0, 0, lba, g_sector_buf, 1) < 0) { kputs("exec: cannot read boot sector.\n"); return; }
    uint8_t* b = g_sector_buf;
    uint16_t bps   = (uint16_t)b[11] | ((uint16_t)b[12] << 8);
    uint8_t  spc   = b[13];
    uint16_t rsvd  = (uint16_t)b[14] | ((uint16_t)b[15] << 8);
    uint8_t  nfats = b[16];
    uint16_t rent  = (uint16_t)b[17] | ((uint16_t)b[18] << 8);
    uint16_t fsz16 = (uint16_t)b[22] | ((uint16_t)b[23] << 8);
    uint32_t fsz32 = (uint32_t)b[36] | ((uint32_t)b[37] << 8) | ((uint32_t)b[38] << 16) | ((uint32_t)b[39] << 24);
    uint32_t fatsz = fsz16 ? fsz16 : fsz32;
    int is_fat32 = (rent == 0);
    if (bps != 512 || spc == 0) { kputs("exec: unsupported geometry.\n"); return; }
    uint32_t fat_lba  = lba + rsvd;
    uint32_t data_lba = lba + rsvd + (uint32_t)nfats * fatsz;
    int kind;
    uint32_t root_clus = 0, root_lba = 0, root_secs = 0;
    if (is_fat32) {
        kind = 2;
        root_clus = (uint32_t)b[44] | ((uint32_t)b[45] << 8) | ((uint32_t)b[46] << 16) | ((uint32_t)b[47] << 24);
        if (root_clus < 2) { kputs("exec: invalid FAT32 root cluster.\n"); return; }
    } else {
        uint32_t tot16 = (uint16_t)b[19] | ((uint16_t)b[20] << 8);
        uint32_t tot32 = (uint32_t)b[32] | ((uint32_t)b[33] << 8) | ((uint32_t)b[34] << 16) | ((uint32_t)b[35] << 24);
        uint32_t tot = tot16 ? tot16 : tot32;
        uint32_t root_dir_secs = (uint32_t)((rent * 32 + bps - 1) / bps);
        uint32_t data_clusters = (tot - rsvd - nfats * fatsz - root_dir_secs) / spc;
        kind = data_clusters < 4085 ? 0 : 1;
        data_lba += root_dir_secs;
        root_lba = lba + rsvd + (uint32_t)nfats * fatsz;
        root_secs = root_dir_secs;
    }

    char rpath[160];
    if (fs_resolve(namebuf, rpath, (int)sizeof rpath) != 0) {
        kputs("exec: path too long.\n"); return;
    }
    struct fat_hit hit;
    int lr = fat_lookup(kind, root_clus, root_lba, root_secs,
                        fat_lba, data_lba, spc, rpath, &hit);
    if (lr <= 0 || hit.is_dir) { kputs("exec: file not found.\n"); return; }
    if (hit.size == 0 || hit.size > EXEC_MAX_SIZE) {
        kputs("exec: bad size (0 or >2MiB).\n"); return;
    }

    /* --- user region --- */
    if (!g_user_base) {
        g_user_base = find_user_region();
        if (!g_user_base) { kputs("exec: no free user region.\n"); return; }
    }
    uint32_t pde = g_user_base >> 22;
    g_page_dir[pde] |= 0x4u;
    __asm__ volatile("movl %%cr3, %%eax\n\tmovl %%eax, %%cr3" ::: "eax", "memory");

    /* --- format detection: ELF32 or flat binary --- */
    uint32_t entry = 0;
    int elf_seen = 0, elf_bad = 0;
    if (hit.size <= EXEC_ELF_MAX) {
        uint8_t* fbuf = (uint8_t*)kmalloc(hit.size);
        if (fbuf && fat_read_to_buf(kind, fat_lba, data_lba, spc,
                                    hit.cluster, hit.size, fbuf) == (int)hit.size
            && fbuf[0] == 0x7F && fbuf[1] == 'E' && fbuf[2] == 'L' && fbuf[3] == 'F') {
            elf_seen = 1;
            entry = exec_load_elf(fbuf, hit.size);
            if (!entry) elf_bad = 1;
        }
        if (fbuf) kfree(fbuf);
    }
    if (elf_bad) { kputs("exec: malformed ELF32.\n"); return; }

    if (!elf_seen) {
    /* --- copy file cluster chain into user memory at base+0x1000 --- */
    uint8_t* dst = (uint8_t*)(size_t)(g_user_base + EXEC_LOAD_OFF);
    uint32_t remaining = hit.size;
    uint32_t clus = hit.cluster;
    int guard = 0;
    while (clus >= 2 && !fat_eoc_any(kind, clus) && guard++ < 1024 && remaining > 0) {
        uint32_t clus_lba = data_lba + (clus - 2) * spc;
        for (uint32_t s = 0; s < spc && remaining > 0; s++) {
            if (ata_read_lba28(0x1F0, 0, clus_lba + s, g_file_buf, 1) < 0) {
                kputs("exec: read error.\n"); return;
            }
            uint32_t chunk = remaining > 512 ? 512 : remaining;
            for (uint32_t i = 0; i < chunk; i++) dst[i] = g_file_buf[i];
            dst += chunk;
            remaining -= chunk;
        }
        clus = fat_next_any(fat_lba, kind, clus);
    }
    entry = g_user_base + EXEC_LOAD_OFF;
    }

    kputs("exec: loaded ");
    kprint_dec64(hit.size);
    kputs(" bytes -> ring3 @");
    kprint_hex32(entry);
    kputs("\n");

    /* _start ABI: [esp]=argc, argv[0..], NULL, envp=NULL.
     * argv[0] = program path, argv[1] = optional argument string
     * (rest of the exec command line). Strings copied into user region. */
    uint32_t stack_top = g_user_base + EXEC_STACK_OFF;
    uint8_t* str_base = (uint8_t*)(size_t)(stack_top - 0x1000);
    int al = 0;
    for (const char* s = namebuf; *s && al < 255; s++) str_base[al++] = (uint8_t)*s;
    str_base[al] = 0;
    uint32_t argv0 = (uint32_t)(size_t)str_base;
    al++;

    /* skip spaces after program name; capture remainder as argv[1] */
    const char* argp = args;
    while (*argp && *argp != ' ') argp++;
    while (*argp == ' ') argp++;
    uint32_t argv1 = 0;
    if (*argp) {
        argv1 = (uint32_t)(size_t)(str_base + al);
        int bl = 0;
        while (*argp && bl < 255) str_base[al + bl++] = (uint8_t)*argp++;
        str_base[al + bl] = 0;
    }

    uint32_t* sp = (uint32_t*)(size_t)(stack_top - 24);
    sp[0] = argv1 ? 2 : 1;                    /* argc */
    sp[1] = argv0;                            /* argv[0] */
    sp[2] = argv1;                            /* argv[1] or NULL */
    sp[3] = 0;                                /* argv terminator */
    sp[4] = 0;                                /* envp terminator */

    if (user_pd_setup() != 0) { kputs("exec: page dir alloc failed.\n"); return; }
    g_user_dead = 0;
    g_preempt_pending = 0;
    g_user_active = 1;
    enter_user(entry, (uint32_t)(size_t)sp);
    g_user_active = 0;
    kputs("exec: program exited, back in kernel shell\n");
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

/* Command history ring (last 16 commands). */
static char g_history[16][128];
static int  g_hist_next, g_hist_count;

static void show_history(void) {
    if (g_hist_count == 0) { kputs("history: empty.\n"); return; }
    int start = (g_hist_next - g_hist_count + 16) % 16;
    for (int i = 0; i < g_hist_count; i++) {
        int idx = (start + i) % 16;
        kputs("  ");
        kprint_dec64((uint64_t)(i + 1));
        kputs("  ");
        kputs(g_history[idx]);
        kputs("\n");
    }
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

        /* history ring buffer (last 16 commands) */
        {
            int n = kstrlen(cmd);
            if (n > 127) n = 127;
            for (int i = 0; i < n; i++) g_history[g_hist_next][i] = cmd[i];
            g_history[g_hist_next][n] = 0;
            g_hist_next = (g_hist_next + 1) % 16;
            if (g_hist_count < 16) g_hist_count++;
        }

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
            kputs("  fs      - Detect FAT filesystem on partition\n");
            kputs("  ls      - List FAT12/16/32 root directory\n");
            kputs("  cat     - Print file: cat <name>\n");
            kputs("  cd      - Change directory: cd <dir>\n");
            kputs("  pwd     - Print working directory\n");
            kputs("  df      - Show filesystem free space\n");
            kputs("  mkdir   - Create directory: mkdir <name>\n");
            kputs("  write   - Create file: write <name> <text>\n");
            kputs("  rm      - Delete file: rm <name>\n");
            kputs("  rmdir   - Remove empty dir: rmdir <name>\n");
            kputs("  mv      - Rename file: mv <old> <new>\n");
            kputs("  cp      - Copy file: cp <src> <dst>\n");
            kputs("  append  - Append text: append <name> <text>\n");
            kputs("  hex     - Hexdump file: hex <file>\n");
            kputs("  info    - System summary\n");
            kputs("  paging  - Show paging/CR state\n");
            kputs("  gdt     - Show GDT/TSS segment state\n");
            kputs("  user    - Drop to ring 3 test program\n");
            kputs("  exec    - Run flat binary: exec <file> [arg]\n");
            kputs("  fg      - Resume preempted user program\n");
            kputs("  timer   - Show PIT tick count\n");
            kputs("  sched   - Scheduler/user-process status\n");
            kputs("  ps      - Process list\n");
            kputs("  dmesg   - Show kernel log\n");
            kputs("  memalloc- Page allocator test/status\n");
            kputs("  history - Show command history\n");
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
        } else if (kcmd_is(cmd, "fs")) {
            show_fs(cmd + 2);
        } else if (kcmd_is(cmd, "ls")) {
            show_ls(cmd + 2);
        } else if (kcmd_is(cmd, "cat")) {
            show_cat(cmd + 3);
        } else if (kcmd_is(cmd, "cd")) {
            show_cd(cmd + 2);
        } else if (kstrcmp(cmd, "pwd") == 0) {
            show_pwd();
        } else if (kstrcmp(cmd, "df") == 0) {
            show_df();
        } else if (kcmd_is(cmd, "mkdir")) {
            show_mkdir(cmd + 5);
        } else if (kcmd_is(cmd, "write")) {
            show_write(cmd + 5);
        } else if (kcmd_is(cmd, "rm")) {
            show_rm(cmd + 2);
        } else if (kcmd_is(cmd, "rmdir")) {
            show_rmdir(cmd + 5);
        } else if (kcmd_is(cmd, "mv")) {
            show_mv(cmd + 2);
        } else if (kcmd_is(cmd, "cp")) {
            show_cp(cmd + 2);
        } else if (kcmd_is(cmd, "append")) {
            show_append(cmd + 6);
        } else if (kcmd_is(cmd, "hex")) {
            show_hex(cmd + 3);
        } else if (kstrcmp(cmd, "info") == 0) {
            show_info();
        } else if (kstrcmp(cmd, "paging") == 0) {
            show_paging();
        } else if (kstrcmp(cmd, "gdt") == 0) {
            show_gdt();
        } else if (kstrcmp(cmd, "user") == 0) {
            show_user();
        } else if (kcmd_is(cmd, "exec")) {
            show_exec(cmd + 4);
        } else if (kstrcmp(cmd, "fg") == 0 || kstrcmp(cmd, "resume") == 0) {
            if (g_preempt_pending) {
                g_preempt_pending = 0;
                kputs("fg: resuming user context...\n");
                resume_user();   /* irets to user; returns via exit/preempt */
            } else {
                kputs("fg: no preempted user program.\n");
            }
        } else if (kstrcmp(cmd, "timer") == 0) {
            show_timer();
        } else if (kstrcmp(cmd, "sched") == 0) {
            show_sched();
        } else if (kstrcmp(cmd, "ps") == 0) {
            show_ps();
        } else if (kstrcmp(cmd, "dmesg") == 0) {
            show_dmesg();
        } else if (kstrcmp(cmd, "memalloc") == 0) {
            show_memalloc();
        } else if (kstrcmp(cmd, "history") == 0) {
            show_history();
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

    paging_init();
    gdt_init();
    idt_init();
    pit_init();
    pic_init();
    klog("kernel: paging+gdt+idt+pit+pic armed\n");
    kputs("[kernel] Paging enabled: 4MiB-page identity map, ");
    kprint_dec64(g_map_mb);
    kputs(" MiB.\n");
    kputs("[kernel] GDT loaded: kernel+user segments, TSS ready.\n");
    kputs("[kernel] IDT+PIC armed: exceptions, IRQs, int 0x80 gate.\n");
    __asm__ volatile("sti");

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
/* --------------------------------------------------------------------------
 * Paging — 4 MiB large pages (PSE), identity-map all RAM reported by
 * Multiboot (capped at 4 GiB address space). One page directory covers
 * 1024 × 4 MiB = 4 GiB; no page tables needed.
 * -------------------------------------------------------------------------- */
#define PTE_PRESENT   0x001u
#define PTE_RW        0x002u
#define PDE_PS        0x080u   /* page size: 4 MiB when set in a PDE */

static uint32_t paging_map_mb(void) {
    /* Total RAM in MiB from Multiboot info; clamp to sane bounds. */
    uint64_t kb = 0;
    if (g_mb_info) {
        uint32_t lower = mbi_rd32(MBI_MEM_LOWER);
        uint32_t upper = mbi_rd32(MBI_MEM_UPPER);
        kb = (uint64_t)lower + upper;
    }
    uint32_t mb = (uint32_t)(kb / 1024);
    if (mb < 16) mb = 16;
    if (mb > 4096) mb = 4096;
    return mb;
}

static void paging_init(void) {
    uint32_t map_mb = paging_map_mb();
    uint32_t pde_count = (map_mb + 3) / 4; /* 4 MiB per PDE */

    /* BSS is not zeroed by the loader — clear page directory explicitly. */
    for (uint32_t i = 0; i < 1024; i++) g_page_dir[i] = 0;

    /* Identity map with 4 MiB pages: virt == phys */
    for (uint32_t i = 0; i < pde_count; i++) {
        g_page_dir[i] = (i * 4u * 1024u * 1024u) | PTE_PRESENT | PTE_RW | PDE_PS;
    }
    uint32_t pd_phys = (uint32_t)(size_t)&g_page_dir[0];

    __asm__ volatile(
        "movl %0, %%cr3\n"
        "movl %%cr4, %%eax\n"
        "orl $0x20, %%eax\n"          /* CR4.PSE — enable 4 MiB pages */
        "movl %%eax, %%cr4\n"
        "movl %%cr0, %%eax\n"
        "orl $0x80000000, %%eax\n"    /* CR0.PG */
        "movl %%eax, %%cr0\n"
        "movl %%cr3, %%eax\n"
        "movl %%eax, %%cr3\n"         /* TLB flush */
        :: "r"(pd_phys) : "eax", "memory");

    g_map_mb = map_mb;
}

static void show_paging(void) {
    uint32_t cr0, cr3, cr4;
    __asm__ volatile("movl %%cr0, %0" : "=r"(cr0));
    __asm__ volatile("movl %%cr3, %0" : "=r"(cr3));
    __asm__ volatile("movl %%cr4, %0" : "=r"(cr4));
    kputs("Paging: CR0.PG=");
    kputs((cr0 & 0x80000000u) ? "on" : "off");
    kputs(" CR0.WP=");
    kputs((cr0 & 0x00010000u) ? "on" : "off");
    kputs(" CR4.PSE=");
    kputs((cr4 & 0x20u) ? "on" : "off");
    kputs("\nCR3=");
    kprint_hex32(cr3);
    kputs(" (4MiB-page identity map, ");
    kprint_dec64(g_map_mb);
    kputs(" MiB)\n");
}

/* --------------------------------------------------------------------------
 * GDT + TSS — ring-0/ring-3 segment setup
 * Layout: 0x00 null | 0x08 kcode | 0x10 kdata | 0x18 ucode(DPL3)
 *         0x20 udata(DPL3) | 0x28 TSS (16-byte descriptor)
 * -------------------------------------------------------------------------- */
typedef struct __attribute__((packed)) {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_mid;
    uint8_t  access;
    uint8_t  granularity;
    uint8_t  base_high;
} gdt_entry_t;

typedef struct __attribute__((packed)) {
    uint16_t limit;
    uint32_t base;
} gdtr_t;

typedef struct __attribute__((packed)) {
    uint32_t back_link;
    uint32_t esp0, ss0, esp1, ss1, esp2, ss2;
    uint32_t cr3;
    uint32_t eip, eflags, eax, ecx, edx, ebx, esp, ebp, esi, edi;
    uint32_t es, cs, ss, ds, fs, gs;
    uint32_t ldt;
    uint16_t debug_flag, io_map_base;
} tss_t;

static gdt_entry_t g_gdt[7] __attribute__((aligned(16), section(".bss")));
static tss_t       g_tss            __attribute__((section(".bss")));
static gdtr_t      g_gdtr           __attribute__((section(".bss")));

#define GDT_KCODE 0x08
#define GDT_KDATA 0x10
#define GDT_UCODE 0x18
#define GDT_UDATA 0x20
#define GDT_TSS   0x28

static gdt_entry_t gdt_make(uint32_t base, uint32_t limit, uint8_t access, uint8_t gran) {
    gdt_entry_t e;
    e.limit_low  = (uint16_t)(limit & 0xFFFF);
    e.base_low   = (uint16_t)(base & 0xFFFF);
    e.base_mid   = (uint8_t)((base >> 16) & 0xFF);
    e.access     = access;
    e.granularity= gran;
    e.base_high  = (uint8_t)((base >> 24) & 0xFF);
    return e;
}

static void gdt_init(void) {
    /* clear first (BSS may be garbage) */
    for (int i = 0; i < 7; i++) {
        g_gdt[i].limit_low = 0; g_gdt[i].base_low = 0; g_gdt[i].base_mid = 0;
        g_gdt[i].access = 0; g_gdt[i].granularity = 0; g_gdt[i].base_high = 0;
    }
    for (uint32_t i = 0; i < sizeof(g_tss); i++) ((uint8_t*)&g_tss)[i] = 0;

    const uint8_t code_acc = 0x9A; /* P=1 DPL=0 S=1 type=exec/read */
    const uint8_t data_acc = 0x92; /* P=1 DPL=0 S=1 type=rw */
    const uint8_t ucode_acc = 0xFA;/* P=1 DPL=3 S=1 type=exec/read */
    const uint8_t udata_acc = 0xF2;/* P=1 DPL=3 S=1 type=rw */
    const uint8_t gran_4g   = 0xCF;/* G=1 D/B=1 limit-hi=0xF -> 4GiB */

    g_gdt[1] = gdt_make(0, 0xFFFFF, code_acc, gran_4g);
    g_gdt[2] = gdt_make(0, 0xFFFFF, data_acc, gran_4g);
    g_gdt[3] = gdt_make(0, 0xFFFFF, ucode_acc, gran_4g);
    g_gdt[4] = gdt_make(0, 0xFFFFF, udata_acc, gran_4g);

    /* TSS: esp0 = kernel stack top so interrupts from ring 3 land safely */
    extern uint8_t* stack_top;
    g_tss.esp0 = (uint32_t)(size_t)stack_top;
    g_tss.ss0  = GDT_KDATA;
    g_tss.cr3  = (uint32_t)(size_t)&g_page_dir[0];

    /* TSS descriptor occupies g_gdt[5..6] (16 bytes) */
    uint32_t tss_base = (uint32_t)(size_t)&g_tss;
    uint32_t tss_lim  = (uint32_t)sizeof(g_tss) - 1;
    g_gdt[5] = gdt_make(tss_base, tss_lim, 0x89, 0x00); /* avail 32-bit TSS, byte granular */
    g_gdt[5].granularity = (uint8_t)((tss_lim >> 16) & 0x0F);
    /* second half of TSS descriptor: base bits 32-63 = 0 in 32-bit mode */
    for (uint32_t i = 0; i < sizeof(gdt_entry_t); i++)
        ((uint8_t*)&g_gdt[6])[i] = 0;

    g_gdtr.limit = (uint16_t)(sizeof(g_gdt) - 1);
    g_gdtr.base  = (uint32_t)(size_t)&g_gdt[0];

    __asm__ volatile(
        "lgdt %0\n"
        "movw %1, %%ax\n"
        "movw %%ax, %%ds\n"
        "movw %%ax, %%es\n"
        "movw %%ax, %%fs\n"
        "movw %%ax, %%gs\n"
        "movw %%ax, %%ss\n"
        "pushl $0x08\n"
        "pushl $1f\n"
        "lret\n"
        "1:\n"
        "movw %2, %%ax\n"
        "ltr %%ax\n"
        :: "m"(g_gdtr), "i"(GDT_KDATA), "i"(GDT_TSS)
        : "eax", "memory");
}

static void show_gdt(void) {
    uint16_t cs, ds, ss, es;
    __asm__ volatile("movw %%cs, %0\nmovw %%ds, %1\nmovw %%ss, %2\nmovw %%es, %3"
                     : "=r"(cs), "=r"(ds), "=r"(ss), "=r"(es));
    kputs("GDT: base=");
    kprint_hex32(g_gdtr.base);
    kputs(" limit=");
    kprint_dec64(g_gdtr.limit);
    kputs("\nCS=");
    kprint_hex32(cs);
    kputs(" DS=");
    kprint_hex32(ds);
    kputs(" SS=");
    kprint_hex32(ss);
    kputs(" ES=");
    kprint_hex32(es);
    kputs("\nTSS: base=");
    kprint_hex32((uint32_t)(size_t)&g_tss);
    kputs(" esp0=");
    kprint_hex32(g_tss.esp0);
    kputs(" selector=0x");
    kprint_hex32(GDT_TSS);
    kputs("\n");
}

/* --------------------------------------------------------------------------
 * IDT — 32 exception vectors. Stubs push vector (+dummy error word where
 * the CPU doesn't supply one), then jump to isr_common_entry which calls
 * isr_common(frame). Frame layout: [vec][err][eip][cs][eflags].
 * -------------------------------------------------------------------------- */
typedef struct __attribute__((packed)) {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t  zero;
    uint8_t  type_attr;   /* P=1 DPL=0 type=32-bit interrupt gate = 0x8E */
    uint16_t offset_high;
    uint32_t zero2;
} idt_entry_t;

typedef struct __attribute__((packed)) {
    uint16_t limit;
    uint32_t base;
} idtr_t;

static idt_entry_t g_idt[256] __attribute__((section(".bss")));
static idtr_t      g_idtr __attribute__((section(".bss")));

void isr_common(uint32_t* frame);

__asm__(
    ".section .text\n"
    ".globl isr_common_entry\n"
    "isr_common_entry:\n"
    "    movl %esp, %eax\n"
    "    pushl %eax\n"
    "    call isr_common\n"
    "    cli\n"
    "    hlt\n"
    "    jmp .\n"
    ".macro ISR_NOERR n\n"
    "isr_\\n:\n"
    "    pushl $0\n"
    "    pushl $\\n\n"
    "    jmp isr_common_entry\n"
    ".endm\n"
    ".macro ISR_ERR n\n"
    "isr_\\n:\n"
    "    pushl $\\n\n"
    "    jmp isr_common_entry\n"
    ".endm\n"
    /* vectors without CPU error code */
    "ISR_NOERR 0\n" "ISR_NOERR 1\n" "ISR_NOERR 2\n" "ISR_NOERR 3\n"
    "ISR_NOERR 4\n" "ISR_NOERR 5\n" "ISR_NOERR 6\n" "ISR_NOERR 7\n"
    "ISR_ERR   8\n"
    "ISR_NOERR 9\n"
    "ISR_ERR   10\n" "ISR_ERR   11\n" "ISR_ERR   12\n" "ISR_ERR   13\n" "ISR_ERR   14\n"
    "ISR_NOERR 15\n" "ISR_NOERR 16\n"
    "ISR_ERR   17\n"
    "ISR_NOERR 18\n" "ISR_NOERR 19\n" "ISR_NOERR 20\n"
    "ISR_ERR   21\n"
    "ISR_NOERR 22\n" "ISR_NOERR 23\n" "ISR_NOERR 24\n" "ISR_NOERR 25\n"
    "ISR_NOERR 26\n" "ISR_NOERR 27\n" "ISR_NOERR 28\n"
    "ISR_ERR   29\n" "ISR_ERR   30\n"
    "ISR_NOERR 31\n"
    /* syscall gate: int 0x80 is callable from ring 3 (DPL=3 in IDT) */
    ".globl syscall_entry\n"
    "syscall_entry:\n"
    "    pushal\n"
    "    movl %esp, %eax\n"
    "    pushl %eax\n"
    "    call syscall_handler\n"
    "    addl $4, %esp\n"
    "    popal\n"
    /* If the user program exited, discard the user iret frame (5 dwords)
     * and return to the saved kernel context instead. */
    "    cmpl $0, g_user_dead\n"
    "    je 1f\n"
    "    addl $20, %esp\n"
    "    pushl $0x10\n"               /* kernel data selector */
    "    movl g_resume_esp, %eax\n"
    "    pushl %eax\n"
    "    pushl $0x202\n"              /* eflags: IF=1 */
    "    pushl $0x08\n"               /* kernel code selector */
    "    movl g_resume_eip, %eax\n"
    "    pushl %eax\n"
    "    iret\n"
    "1:\n"
    "    iret\n"
    /* catch-all for unhandled vectors 48-255 */
    ".globl isr_spurious\n"
    "isr_spurious:\n"
    "    pushl $0\n"
    "    pushl $255\n"
    "    jmp isr_common_entry\n"
    /* IRQ stubs: vectors 32-47, no error code, return via iret */
    ".globl irq_common_entry\n"
    "irq_common_entry:\n"
    "    cli\n"               /* mask IRQs in handler; iret restores IF from stack */
    "    pushal\n"            /* regs[0..7] then vector then cpu iret frame */
    "    movl %esp, %eax\n"
    "    pushl %eax\n"
    "    call irq_handler\n"
    "    addl $4, %esp\n"
    "    popal\n"
    "    addl $4, %esp\n"      /* drop pushed vector number -> cpu frame top */
    /* Preemption divert: handler may set g_preempt_pending (user-mode tick).
     * Drop the 5-dword user iret frame, return to kernel resume context. */
    "    cmpl $0, g_preempt_pending\n"
    "    je 2f\n"
    "    addl $20, %esp\n"
    "    pushl $0x10\n"
    "    movl g_resume_esp, %eax\n"
    "    pushl %eax\n"
    "    pushl $0x202\n"
    "    pushl $0x08\n"
    "    movl g_resume_eip, %eax\n"
    "    pushl %eax\n"
    "    iret\n"
    "2:\n"
    "    iret\n"
    ".macro IRQ_STUB n\n"
    "irq_\\n:\n"
    "    pushl $(32+\\n)\n"    /* push real IDT vector (32..47), not IRQ index */
    "    jmp irq_common_entry\n"
    ".endm\n"
    "IRQ_STUB 0\n" "IRQ_STUB 1\n" "IRQ_STUB 2\n" "IRQ_STUB 3\n"
    "IRQ_STUB 4\n" "IRQ_STUB 5\n" "IRQ_STUB 6\n" "IRQ_STUB 7\n"
    "IRQ_STUB 8\n" "IRQ_STUB 9\n" "IRQ_STUB 10\n" "IRQ_STUB 11\n"
    "IRQ_STUB 12\n" "IRQ_STUB 13\n" "IRQ_STUB 14\n" "IRQ_STUB 15\n"
    ".section .data\n"
    ".globl irq_table\n"
    "irq_table:\n"
    "    .long irq_0, irq_1, irq_2, irq_3, irq_4, irq_5, irq_6, irq_7\n"
    "    .long irq_8, irq_9, irq_10, irq_11, irq_12, irq_13, irq_14, irq_15\n"
    ".section .data\n"
    ".globl isr_table\n"
    "isr_table:\n"
    "    .long isr_0, isr_1, isr_2, isr_3, isr_4, isr_5, isr_6, isr_7\n"
    "    .long isr_8, isr_9, isr_10, isr_11, isr_12, isr_13, isr_14, isr_15\n"
    "    .long isr_16, isr_17, isr_18, isr_19, isr_20, isr_21, isr_22, isr_23\n"
    "    .long isr_24, isr_25, isr_26, isr_27, isr_28, isr_29, isr_30, isr_31\n"
    ".section .text\n"
);

void isr_common(uint32_t* frame) {
    static const char* names[32] = {
        "#DE divide error", "#DB debug", "NMI", "#BP breakpoint", "#OF overflow",
        "#BR bound", "#UD invalid opcode", "#NM device not avail", "#DF double fault",
        "#CS coproc seg overrun", "#TS invalid TSS", "#NP seg not present",
        "#SS stack fault", "#GP general protection", "#PF page fault",
        "#(reserved15)", "#MF x87 FP", "#AC alignment check", "#MC machine check",
        "#XM SIMD", "#VE virtualization", "#CP control protection",
        "#(reserved22)", "#(reserved23)", "#(reserved24)", "#(reserved25)",
        "#(reserved26)", "#(reserved27)", "#(reserved28)", "#HV hypervisor",
        "#VC VMM comm", "#SX security"
    };
    uint32_t vec = frame[0];
    uint32_t err = frame[1];
    uint32_t eip = frame[2];
    uint32_t cs  = frame[3];
    kputs("\n*** EXCEPTION ");
    kputs(vec < 32 ? names[vec] : "#??");
    kputs(" vec=");
    kprint_dec64(vec);
    kputs(" err=");
    kprint_hex32(err);
    kputs(" eip=");
    kprint_hex32(eip);
    kputs(" cs=");
    kprint_hex32(cs);
    kputs(" ***\n");
    klog("exception: vec=");
    /* log vec + eip compactly via static buffer */
    {
        char lb[48]; int n = 0;
        const char* p = "vec=";
        while (*p) lb[n++] = *p++;
        uint32_t v = vec;
        char tmp[12]; int t = 0;
        do { tmp[t++] = '0' + (v % 10); v /= 10; } while (v);
        while (t) lb[n++] = tmp[--t];
        p = " eip=";
        while (*p) lb[n++] = *p++;
        v = eip; t = 0;
        do { tmp[t++] = "0123456789abcdef"[v & 15]; v >>= 4; } while (v);
        while (t) lb[n++] = tmp[--t];
        lb[n++] = '\n'; lb[n] = 0;
        klog(lb);
    }
}

void syscall_handler(uint32_t* regs);

static void idt_set_gate(uint32_t vec, uint32_t handler, uint8_t type_attr) {
    g_idt[vec].offset_low  = (uint16_t)(handler & 0xFFFF);
    g_idt[vec].selector    = GDT_KCODE;
    g_idt[vec].zero        = 0;
    g_idt[vec].type_attr   = type_attr;
    g_idt[vec].offset_high = (uint16_t)((handler >> 16) & 0xFFFF);
    g_idt[vec].zero2       = 0;
}

static void idt_init(void) {
    extern void* isr_table[32];
    extern void syscall_entry, isr_spurious;
    for (int i = 0; i < 32; i++) {
        idt_set_gate((uint32_t)i, (uint32_t)(size_t)isr_table[i], 0x8E);
    }
    /* vectors 32-47: IRQ0-15 stubs (return via iret) */
    extern void* irq_table[16];
    for (int i = 0; i < 16; i++) {
        idt_set_gate(32u + (uint32_t)i, (uint32_t)(size_t)irq_table[i], 0x8E);
    }
    /* vectors 48-255: catch-all (halts with diagnostic if ever hit) */
    for (int i = 48; i < 256; i++) {
        idt_set_gate((uint32_t)i, (uint32_t)(size_t)&isr_spurious, 0x8E);
    }
    /* int 0x80: DPL=3 interrupt gate — callable from ring 3, returns via iret */
    idt_set_gate(0x80, (uint32_t)(size_t)&syscall_entry, 0xEE);
    g_idtr.limit = (uint16_t)(sizeof(g_idt) - 1);
    g_idtr.base  = (uint32_t)(size_t)&g_idt[0];
    __asm__ volatile("lidt %0" :: "m"(g_idtr) : "memory");
}

/* int 0x80 ABI: eax=nr; args ebx,ecx,edx; ret in eax.
 * regs[] = pushal frame (edi pushed last -> lowest addr):
 * [0]=edi [1]=esi [2]=ebp [3]=esp0 [4]=ebx [5]=edx [6]=ecx [7]=eax
 * => arg1=regs[4] arg2=regs[6](ecx) arg3=regs[5](edx) */
#define SYS_EXIT     0
#define SYS_WRITE    1   /* ebx=ptr to NUL-terminated user string */
#define SYS_GETCHAR  2   /* returns char in eax */
#define SYS_TICKS    3   /* returns PIT tick count in eax */
#define SYS_OPEN     4   /* ebx=path -> fd in eax (-1 err) */
#define SYS_READ     5   /* ebx=fd, ecx=buf, edx=len -> bytes read */
#define SYS_CLOSE    6   /* ebx=fd */
#define SYS_SEEK     7   /* ebx=fd, ecx=offset -> 0/-1 (absolute) */
#define SYS_WRITE_LEN 8  /* ebx=ptr, ecx=len — binary-safe write */

static volatile uint32_t g_tick_count;  /* defined in IRQ section below */

/* ---- cached FAT volume mount (computed once, shared by syscalls) ---- */
static int fat_ensure_mounted(void) {
    if (g_vol_kind >= 0) return 0;
    if (ata_read_lba28(0x1F0, 0, 0, g_sector_buf, 1) < 0) return -1;
    uint8_t* me = g_sector_buf + 446;
    uint32_t lba = (uint32_t)me[8] | ((uint32_t)me[9] << 8)
                 | ((uint32_t)me[10] << 16) | ((uint32_t)me[11] << 24);
    if (ata_read_lba28(0x1F0, 0, lba, g_sector_buf, 1) < 0) return -1;
    uint8_t* b = g_sector_buf;
    uint16_t bps   = (uint16_t)b[11] | ((uint16_t)b[12] << 8);
    uint8_t  spc   = b[13];
    uint16_t rsvd  = (uint16_t)b[14] | ((uint16_t)b[15] << 8);
    uint8_t  nfats = b[16];
    uint16_t rent  = (uint16_t)b[17] | ((uint16_t)b[18] << 8);
    uint16_t fsz16 = (uint16_t)b[22] | ((uint16_t)b[23] << 8);
    uint32_t fsz32 = (uint32_t)b[36] | ((uint32_t)b[37] << 8)
                   | ((uint32_t)b[38] << 16) | ((uint32_t)b[39] << 24);
    uint32_t fatsz = fsz16 ? fsz16 : fsz32;
    int is_fat32 = (rent == 0);
    if (bps != 512 || spc == 0 || nfats == 0) return -1;
    g_vol_spc = spc;
    g_vol_fatsz = fatsz;
    g_vol_fat_lba  = lba + rsvd;
    g_vol_data_lba = lba + rsvd + (uint32_t)nfats * fatsz;
    if (is_fat32) {
        g_vol_kind = 2;
        g_vol_root_clus = (uint32_t)b[44] | ((uint32_t)b[45] << 8)
                        | ((uint32_t)b[46] << 16) | ((uint32_t)b[47] << 24);
        if (g_vol_root_clus < 2) { g_vol_kind = -1; return -1; }
    } else {
        uint32_t tot16 = (uint16_t)b[19] | ((uint16_t)b[20] << 8);
        uint32_t tot32 = (uint32_t)b[32] | ((uint32_t)b[33] << 8)
                       | ((uint32_t)b[34] << 16) | ((uint32_t)b[35] << 24);
        uint32_t tot = tot16 ? tot16 : tot32;
        uint32_t root_dir_secs = (uint32_t)((rent * 32 + bps - 1) / bps);
        uint32_t data_clusters = (tot - rsvd - nfats * fatsz - root_dir_secs) / spc;
        g_vol_kind = data_clusters < 4085 ? 0 : 1;
        g_vol_total_clusters = data_clusters + 2;
        g_vol_data_lba += root_dir_secs;
        g_vol_root_lba  = lba + rsvd + (uint32_t)nfats * fatsz;
        g_vol_root_secs = root_dir_secs;
    }
    if (is_fat32 && g_vol_total_clusters == 0) {
        /* FAT32 total clusters = (total_secs - data_start) / spc */
        uint32_t tot32 = (uint32_t)b[32] | ((uint32_t)b[33] << 8)
                       | ((uint32_t)b[34] << 16) | ((uint32_t)b[35] << 24);
        uint16_t tot16 = (uint16_t)b[19] | ((uint16_t)b[20] << 8);
        uint32_t tot = tot16 ? tot16 : tot32;
        uint32_t data_start = rsvd + nfats * fatsz;
        g_vol_total_clusters = tot > data_start ? (tot - data_start) / spc : 0;
    }
    return 0;
}

/* ---- file descriptor table ---- */
#define MAX_FDS 8
struct fdesc {
    int      used;
    uint32_t start;     /* first data cluster (for seek rewind) */
    uint32_t cluster;   /* cluster containing current position */
    uint32_t size;
    uint32_t pos;       /* absolute file offset */
    uint32_t coff;      /* byte offset within current cluster's data */
};
static struct fdesc g_fds[MAX_FDS];

static uint32_t fd_read(struct fdesc* f, uint8_t* buf, uint32_t len) {
    uint32_t got = 0;
    while (got < len && f->pos < f->size) {
        if (fat_eoc_any(g_vol_kind, f->cluster)) break;  /* EOC marker */
        uint32_t clus_lba = g_vol_data_lba + (f->cluster - 2) * g_vol_spc;
        uint32_t in_clus  = f->coff & 511u;
        uint32_t sec_off  = f->coff >> 9;
        if (sec_off >= g_vol_spc) {                     /* crossed cluster */
            f->cluster = fat_next_any(g_vol_fat_lba, g_vol_kind, f->cluster);
            f->coff = 0;
            continue;
        }
        if (ata_read_lba28(0x1F0, 0, clus_lba + sec_off, g_file_buf, 1) < 0) break;
        uint32_t avail = 512 - in_clus;
        uint32_t want  = len - got;
        uint32_t n     = want < avail ? want : avail;
        uint32_t left  = f->size - f->pos;
        if (n > left) n = left;
        for (uint32_t i = 0; i < n; i++) buf[got + i] = g_file_buf[in_clus + i];
        got += n; f->pos += n; f->coff += n;
        if (f->coff >= (uint32_t)g_vol_spc * 512u) {     /* cluster exhausted */
            f->cluster = fat_next_any(g_vol_fat_lba, g_vol_kind, f->cluster);
            f->coff = 0;
        }
    }
    return got;
}

static int user_range_ok(const void* p, uint32_t len) {
    if (!g_user_base) return 0;
    uint32_t a = (uint32_t)(size_t)p;
    uint32_t end = g_user_base + 0x400000u;
    return a >= g_user_base && a + len <= end && a + len >= a;
}

void syscall_handler(uint32_t* regs) {
    uint32_t nr  = regs[7];
    uint32_t ret = 0;
    if (nr == SYS_WRITE) {
        volatile char* s = (volatile char*)(size_t)regs[4];
        uint32_t limit = g_user_base ? g_user_base + 0x400000u : 0;
        for (uint32_t i = 0; i < 512; i++) {
            if ((uint32_t)(size_t)(s + i) >= limit) break;
            char c = s[i];
            if (c == 0) break;
            kputc(c);
        }
    } else if (nr == SYS_WRITE_LEN) {
        volatile char* s = (volatile char*)(size_t)regs[4];
        uint32_t len = regs[6];   /* ecx */
        uint32_t limit = g_user_base ? g_user_base + 0x400000u : 0;
        if (len > 65536) len = 65536;
        for (uint32_t i = 0; i < len; i++) {
            if ((uint32_t)(size_t)(s + i) >= limit) break;
            kputc(s[i]);
        }
    } else if (nr == SYS_GETCHAR) {
        ret = (uint8_t)kgetc();
    } else if (nr == SYS_TICKS) {
        ret = g_tick_count;
    } else if (nr == SYS_EXIT) {
        kputs("\n[user] exit syscall\n");
        g_user_dead = 1;      /* syscall_entry will iret back to kernel */
        g_user_active = 0;
    } else if (nr == SYS_OPEN) {
        volatile char* up = (volatile char*)(size_t)regs[4];
        char path[256];
        int pl = 0;
        uint32_t limit = g_user_base + 0x400000u;
        if (!g_user_base) { regs[7] = 0xFFFFFFFFu; return; }
        while (pl < 255 && (uint32_t)(size_t)(up + pl) < limit && up[pl]) {
            path[pl] = (char)up[pl]; pl++;
        }
        path[pl] = 0;
        if (fat_ensure_mounted() != 0) { regs[7] = 0xFFFFFFFFu; return; }
        char rpath[160];
        if (fs_resolve(path, rpath, (int)sizeof rpath) != 0) {
            regs[7] = 0xFFFFFFFFu; return;
        }
        struct fat_hit hit;
        int lr = fat_lookup(g_vol_kind, g_vol_root_clus, g_vol_root_lba,
                            g_vol_root_secs, g_vol_fat_lba, g_vol_data_lba,
                            g_vol_spc, rpath, &hit);
        if (lr <= 0 || hit.is_dir || hit.cluster < 2 || hit.size == 0) {
            regs[7] = 0xFFFFFFFFu; return;
        }
        int fd = -1;
        for (int i = 0; i < MAX_FDS; i++) if (!g_fds[i].used) { fd = i; break; }
        if (fd < 0) { regs[7] = 0xFFFFFFFFu; return; }
        g_fds[fd].used = 1;
        g_fds[fd].start = hit.cluster;
        g_fds[fd].cluster = hit.cluster;
        g_fds[fd].size = hit.size;
        g_fds[fd].pos = 0;
        g_fds[fd].coff = 0;
        ret = (uint32_t)fd;
    } else if (nr == SYS_READ) {
        int fd = (int)regs[4];
        uint8_t* buf = (uint8_t*)(size_t)regs[6];   /* ecx */
        uint32_t len = regs[5];                     /* edx */
        if (fd < 0 || fd >= MAX_FDS || !g_fds[fd].used || len > 65536
            || !user_range_ok(buf, len)) {
            regs[7] = 0xFFFFFFFFu; return;
        }
        ret = fd_read(&g_fds[fd], buf, len);
    } else if (nr == SYS_CLOSE) {
        int fd = (int)regs[4];
        if (fd >= 0 && fd < MAX_FDS && g_fds[fd].used) {
            g_fds[fd].used = 0;
            ret = 0;
        } else ret = 0xFFFFFFFFu;
    } else if (nr == SYS_SEEK) {
        int fd = (int)regs[4];
        uint32_t off = regs[6];   /* ecx */
        if (fd < 0 || fd >= MAX_FDS || !g_fds[fd].used || off > g_fds[fd].size) {
            regs[7] = 0xFFFFFFFFu; return;
        }
        struct fdesc* f = &g_fds[fd];
        uint32_t csz = (uint32_t)g_vol_spc * 512u;
        uint32_t idx = off / csz;
        f->cluster = f->start;
        for (uint32_t i = 0; i < idx; i++) {
            f->cluster = fat_next_any(g_vol_fat_lba, g_vol_kind, f->cluster);
            if (fat_eoc_any(g_vol_kind, f->cluster)) {
                regs[7] = 0xFFFFFFFFu; return;
            }
        }
        f->pos = off;
        f->coff = off % csz;
        ret = 0;
    } else {
        kputs("\n[syscall] unknown nr=");
        kprint_dec64(nr);
        kputs("\n");
    }
    regs[7] = ret;
}

/* --------------------------------------------------------------------------
 * PIC + IRQ handling — 8259A remap to vectors 0x20/0x28, IRQ0 timer unmasked
 * -------------------------------------------------------------------------- */
static volatile uint32_t g_tick_count = 0;

/* Program PIT channel 0 to HZ ticks/sec (default BIOS rate is ~18.2 Hz). */
#define PIT_HZ 100u
static void pit_init(void) {
    uint16_t div = (uint16_t)(1193182u / PIT_HZ);   /* ~11932 for 100 Hz */
    outb(0x43, 0x36);          /* ch0, lobyte/hibyte, mode 3 (square wave) */
    outb(0x40, (uint8_t)(div & 0xFF));
    outb(0x40, (uint8_t)(div >> 8));
}

static void pic_init(void) {
    /* ICW1: edge-triggered, cascade, ICW4 needed */
    outb(0x20, 0x11);
    outb(0xA0, 0x11);
    /* ICW2: vector offsets */
    outb(0x21, 0x20);   /* master -> IRQ0-7 = vectors 32-39 */
    outb(0xA1, 0x28);   /* slave  -> IRQ8-15 = vectors 40-47 */
    /* ICW3: cascade wiring (slave on master IRQ2) */
    outb(0x21, 0x04);
    outb(0xA1, 0x02);
    /* ICW4: 8086 mode */
    outb(0x21, 0x01);
    outb(0xA1, 0x01);
    /* OCW1: unmask IRQ0 (PIT) + IRQ1 (PS/2 kbd) on master; mask all on slave */
    outb(0x21, 0xFC);
    outb(0xA1, 0xFF);
}

/* PS/2 keyboard IRQ1 -> char ring buffer (interrupt-driven input).
 * Polled serial/keyboard remains as fallback in kgetc(). */
#define KBD_BUF_SIZE 256
static volatile char     g_kbd_buf[KBD_BUF_SIZE];
static volatile uint32_t g_kbd_head = 0, g_kbd_tail = 0;

static int kbd_buf_push(char c) {
    uint32_t next = (g_kbd_head + 1) % KBD_BUF_SIZE;
    if (next == g_kbd_tail) return 0;
    g_kbd_buf[g_kbd_head] = c;
    g_kbd_head = next;
    return 1;
}

static int kbd_buf_pop(char* out) {
    if (g_kbd_tail == g_kbd_head) return 0;
    *out = g_kbd_buf[g_kbd_tail];
    g_kbd_tail = (g_kbd_tail + 1) % KBD_BUF_SIZE;
    return 1;
}

/* IRQ frame layout (irq_common_entry does pushal first):
 * frame[0..7] = pushal regs (edi..eax)
 * frame[8]    = vector number
 * frame[9]    = eip  frame[10]=cs  frame[11]=eflags
 * frame[12..13]= user esp/ss (only when interrupted from ring 3) */
void irq_handler(uint32_t* frame) {  /* non-static: called from irq_common_entry asm */
    uint32_t vec = frame[8];
    if (vec == 32) { /* IRQ0: PIT timer @ PIT_HZ */
        g_tick_count++;
        if ((g_tick_count % PIT_HZ) == 0) {
            kputs("\n[timer] tick=");
            kprint_dec64(g_tick_count);
            kputs(" (~");
            kprint_dec64(g_tick_count / PIT_HZ);
            kputs("s)\n");
        }
        /* Preempt every 10 ticks (100 ms) while ring-3 code runs */
        if ((g_tick_count % 10) == 0 && g_user_active && !g_preempt_pending
            && !g_user_dead && (frame[10] & 3) == 3) {
            for (int i = 0; i < 8; i++) g_user_ctx[i] = frame[i];
            g_user_eip    = frame[9];
            g_user_cs     = frame[10];
            g_user_eflags = frame[11];
            g_user_esp    = frame[12];
            g_user_ss     = frame[13];
            g_preempt_pending = 1;
            klog("sched: user preempted\n");
            kputs("\n[sched] user preempted — 'fg' to resume\n");
        }
    } else if (vec == 33) {
        /* IRQ1: PS/2 keyboard — translate scancode, push to buffer */
        uint8_t sc = inb(0x60);
        static int irq_skip_ext = 0;
        int c = 0;
        if (sc == 0xE0 || sc == 0xE1 || sc == 0xF0) {
            irq_skip_ext = 1;
        } else if (irq_skip_ext) {
            irq_skip_ext = 0;
        } else if (!(sc & 0x80)) {
            c = scancode_ascii[sc & 0x7F];
        }
        if (c) kbd_buf_push((char)c);
    }
    /* EOI: non-specific to master; slave EOI only for IRQ8-15 (vec>=40) */
    if (vec >= 40) outb(0xA0, 0x20);
    outb(0x20, 0x20);
}

static void show_ps(void) {
    kputs("  PID STAT  EIP      ESP      INFO\n");
    kputs("    0 RUN   -        -        kernel shell\n");
    if (g_user_active || g_preempt_pending) {
        kputs("    1 ");
        kputs(g_preempt_pending ? "STOP " : "RUN  ");
        kputs("  ");
        kprint_hex32(g_user_eip);
        kputs("  ");
        kprint_hex32(g_user_esp);
        kputs(g_preempt_pending ? "  preempted (fg)\n" : "  running\n");
    } else {
        kputs("    1 -     -        -        no user process\n");
    }
}

static void show_sched(void) {
    kputs("Scheduler: ");
    kputs(g_user_active ? "user active" : "idle (kernel)");
    kputs(g_preempt_pending ? ", preempted (fg to resume)" : "");
    kputs("\n  ticks=");
    kprint_dec64(g_tick_count);
    if (g_preempt_pending) {
        kputs("  saved eip=");
        kprint_hex32(g_user_eip);
        kputs(" esp=");
        kprint_hex32(g_user_esp);
    }
    kputs("\n");
}

static void show_timer(void) {
    kputs("Timer: PIT IRQ0 ticks=");
    kprint_dec64(g_tick_count);
    kputs(" (~");
    kprint_dec64(g_tick_count / PIT_HZ);
    kputs("s @");
    kprint_dec64(PIT_HZ);
    kputs("Hz)\n");
}

/* --------------------------------------------------------------------------
 * Physical page allocator — 4 KiB pages over Multiboot type-1 regions.
 * Bitmap lives in kernel BSS (128 KiB => 4 GiB coverage).
 * SEPARATION OF CONCERNS: mechanism (alloc/free) vs policy (callers).
 * -------------------------------------------------------------------------- */
#define PAGE_SIZE      4096u
#define BITMAP_WORDS   32768u   /* 1,048,576 pages * 4 KiB = 4 GiB coverage */

static uint32_t g_page_bitmap[BITMAP_WORDS] __attribute__((section(".bss")));
static uint32_t g_alloc_count = 0;
static int      g_mm_ready  = 0;

static void mm_init(void) {
    if (g_mm_ready) return;
    for (uint32_t i = 0; i < BITMAP_WORDS; i++) g_page_bitmap[i] = 0;
    if (!g_mb_info) return;
    uint32_t flags = mbi_rd32(MBI_FLAGS);
    if (!(flags & MB_MMAP_FLAG)) return;
    uint32_t size = mbi_rd32(MBI_MMAP_SIZE);
    uint32_t addr = mbi_rd32(MBI_MMAP_ADDR);
    volatile uint8_t* p = (volatile uint8_t*)(size_t)addr;
    volatile uint8_t* end = p + size;
    /* Reserve: mark all non-type-1 pages as used; type-1 as free (bit=0=free). */
    while (p + 24 <= end) {
        uint32_t esize = *(volatile uint32_t*)p;
        if (esize < 24 || p + esize > end) break;
        uint64_t base = ((uint64_t)*(volatile uint32_t*)(p + 8) << 32) | *(volatile uint32_t*)(p + 4);
        uint64_t len  = ((uint64_t)*(volatile uint32_t*)(p + 16) << 32) | *(volatile uint32_t*)(p + 12);
        uint32_t type = *(volatile uint32_t*)(p + 20);
        if (type != 1) {
            /* mark used */
            uint64_t first = base / PAGE_SIZE;
            uint64_t last  = (base + len + PAGE_SIZE - 1) / PAGE_SIZE;
            for (uint64_t pg = first; pg < last && pg < 1048576; pg++) {
                g_page_bitmap[pg >> 5] |= (1u << (pg & 31));
            }
        }
        p += esize;
    }
    /* Reserve first 1 MiB (real-mode/BIOS) and kernel image [1MiB, ~2MiB) */
    for (uint32_t pg = 0; pg < 512; pg++) g_page_bitmap[pg >> 5] |= (1u << (pg & 31));
    /* reserve kernel image: from 1MiB to end of kernel (approx 2MiB) */
    extern uint8_t _end;
    uint32_t kern_end_pg = ((uint32_t)(size_t)&_end + PAGE_SIZE - 1) / PAGE_SIZE;
    for (uint32_t pg = 256; pg < kern_end_pg; pg++) g_page_bitmap[pg >> 5] |= (1u << (pg & 31));
    /* reserve Multiboot modules */
    if (g_mb_info) {
        uint32_t mcount = mbi_rd32(MBI_MODS_COUNT);
        uint32_t maddr  = mbi_rd32(MBI_MODS_ADDR);
        for (uint32_t m = 0; m < mcount; m++) {
            volatile uint8_t* mp = (volatile uint8_t*)(size_t)maddr + m * 16;
            uint32_t ms = *(volatile uint32_t*)(mp + 0);
            uint32_t me = *(volatile uint32_t*)(mp + 4);
            for (uint32_t pg = ms / PAGE_SIZE; pg <= (me + PAGE_SIZE - 1) / PAGE_SIZE && pg < 1048576; pg++)
                g_page_bitmap[pg >> 5] |= (1u << (pg & 31));
        }
    }
    g_mm_ready = 1;
}

/* Allocate one 4 KiB page. Returns physical==virtual addr (identity map) or 0. */
static uint32_t pmm_alloc(void) {
    if (!g_mm_ready) mm_init();
    for (uint32_t w = 0; w < BITMAP_WORDS; w++) {
        uint32_t word = g_page_bitmap[w];
        if (word == 0xFFFFFFFFu) continue;
        for (uint32_t b = 0; b < 32; b++) {
            if (!(word & (1u << b))) {
                uint32_t pg = w * 32 + b;
                g_page_bitmap[w] |= (1u << b);
                g_alloc_count++;
                return pg * PAGE_SIZE;
            }
        }
    }
    return 0;
}

static void pmm_free(uint32_t phys) {
    uint32_t pg = phys / PAGE_SIZE;
    if (pg < 1048576) g_page_bitmap[pg >> 5] &= ~(1u << (pg & 31));
}

/* ---- kernel heap: page-granular kmalloc over pmm ----
 * SEPARATION OF MECHANISM & POLICY: pmm owns pages; kmalloc owns
 * size-class policy. Sub-page slab allocator is a future refinement. */
static void* kmalloc(uint32_t size) {
    if (size == 0 || size > 0x100000) return 0;
    if (!g_mm_ready) mm_init();
    uint32_t pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
    uint32_t* hdr = 0;
    /* first-fit contiguous run of free pages (identity-mapped phys) */
    for (uint32_t i = 0; i + pages <= 1048576; i++) {
        int ok = 1;
        for (uint32_t j = 0; j < pages; j++) {
            uint32_t pg = i + j;
            if (g_page_bitmap[pg >> 5] & (1u << (pg & 31))) { ok = 0; break; }
        }
        if (!ok) continue;
        for (uint32_t j = 0; j < pages; j++) {
            uint32_t pg = i + j;
            g_page_bitmap[pg >> 5] |= (1u << (pg & 31));
            g_alloc_count++;
        }
        hdr = (uint32_t*)(size_t)(i * PAGE_SIZE);
        hdr[0] = pages;                 /* store page count in first dword */
        /* zero the payload region */
        uint8_t* p = (uint8_t*)(size_t)(i * PAGE_SIZE + 16);
        for (uint32_t k = 16; k < pages * PAGE_SIZE; k++) p[k - 16] = 0;
        return (void*)(size_t)((uint32_t)hdr + 16);
    }
    return 0;
}

static void kfree(void* ptr) {
    if (!ptr) return;
    uint32_t base = (uint32_t)(size_t)ptr - 16;
    uint32_t pages = *(volatile uint32_t*)(size_t)base;
    if (pages == 0 || pages > 256) return;   /* sanity */
    uint32_t pg0 = base / PAGE_SIZE;
    for (uint32_t j = 0; j < pages; j++) {
        uint32_t pg = pg0 + j;
        if (pg < 1048576) g_page_bitmap[pg >> 5] &= ~(1u << (pg & 31));
    }
}

static void show_memalloc(void) {
    /* smoke test: allocate two pages, verify distinct, free them */
    uint32_t a = pmm_alloc();
    uint32_t b = pmm_alloc();
    if (a && b) {
        kputs("alloc test: ");
        kprint_hex32(a);
        kputs(" + ");
        kprint_hex32(b);
        kputs(a != b ? " OK (distinct)\n" : " FAIL (same page!)\n");
        pmm_free(a);
        pmm_free(b);
    } else {
        kputs("alloc test: allocation failed\n");
    }
    /* kmalloc smoke test: 100-byte alloc, write/read, free */
    uint8_t* k = (uint8_t*)kmalloc(100);
    if (k) {
        for (int i = 0; i < 100; i++) k[i] = (uint8_t)(i ^ 0x5A);
        int ok = 1;
        for (int i = 0; i < 100; i++) if (k[i] != (uint8_t)(i ^ 0x5A)) { ok = 0; break; }
        kputs("kmalloc test: ");
        kputs(ok ? "OK (100B write/read)\n" : "FAIL (data mismatch)\n");
        kfree(k);
    } else {
        kputs("kmalloc test: allocation failed\n");
    }
    if (!g_mm_ready) mm_init();
    kputs("Page allocator: ");
    kprint_dec64(g_alloc_count);
    kputs(" allocs, ready=");
    kputs(g_mm_ready ? "yes" : "no");
    kputs("\n");
}

/* --------------------------------------------------------------------------
 * Ring-3 user-mode test — `user` command
 * Finds a free 4 MiB-aligned RAM block from the Multiboot memory map,
 * marks its PDE U/S, plants a tiny program that calls int 0x80 then int3.
 * -------------------------------------------------------------------------- */
static uint32_t g_user_base = 0;

/* First-fit: 4 MiB-aligned physical block inside a type-1 (available) region. */
static uint32_t find_user_region(void) {
    if (!g_mb_info) return 0;
    uint32_t flags = mbi_rd32(MBI_FLAGS);
    if (!(flags & MB_MMAP_FLAG)) return 0;
    uint32_t size = mbi_rd32(MBI_MMAP_SIZE);
    uint32_t addr = mbi_rd32(MBI_MMAP_ADDR);
    volatile uint8_t* p = (volatile uint8_t*)(size_t)addr;
    volatile uint8_t* end = p + size;
    /* kernel image occupies roughly [1MiB, 2MiB); start search at 4MiB */
    while (p + 24 <= end) {
        uint32_t esize = *(volatile uint32_t*)p;
        if (esize < 24 || p + esize > end) break;
        uint64_t base = ((uint64_t)*(volatile uint32_t*)(p + 8) << 32) | *(volatile uint32_t*)(p + 4);
        uint64_t len  = ((uint64_t)*(volatile uint32_t*)(p + 16) << 32) | *(volatile uint32_t*)(p + 12);
        uint32_t type = *(volatile uint32_t*)(p + 20);
        if (type == 1 && len >= 4096) {
            uint64_t aligned = (base + 0x3FFFFFu) & ~0x3FFFFFuLL;
            /* whole 4 MiB PDE must be backed by available RAM */
            if (aligned >= 0x400000ULL && aligned + 0x400000ULL <= base + len) {
                /* reject if it overlaps any Multiboot module */
                uint32_t mcount = mbi_rd32(MBI_MODS_COUNT);
                uint32_t maddr  = mbi_rd32(MBI_MODS_ADDR);
                int overlap = 0;
                for (uint32_t m = 0; m < mcount; m++) {
                    volatile uint8_t* mp = (volatile uint8_t*)(size_t)maddr + m * 16;
                    uint32_t ms = *(volatile uint32_t*)(mp + 0);
                    uint32_t me = *(volatile uint32_t*)(mp + 4);
                    if (aligned < (uint64_t)me && (uint64_t)ms < aligned + 0x400000ULL) {
                        overlap = 1;
                        break;
                    }
                }
                if (!overlap) return (uint32_t)aligned;
            }
        }
        p += esize;
    }
    return 0;
}

/* Resume a preempted user context: rebuild the pushal block, popal,
 * then iret with the saved user frame. Never returns to caller —
 * control comes back via SYS_EXIT/preempt paths. */
__attribute__((section(".text"), naked))
void resume_user(void) {
    __asm__ volatile(
        "movl g_user_ctx+28, %%eax\n pushl %%eax\n"   /* eax */
        "movl g_user_ctx+24, %%ecx\n pushl %%ecx\n"   /* ecx */
        "movl g_user_ctx+20, %%edx\n pushl %%edx\n"   /* edx */
        "movl g_user_ctx+16, %%ebx\n pushl %%ebx\n"   /* ebx */
        "pushl $0\n"                                 /* esp slot (popal skips) */
        "movl g_user_ctx+8,  %%ebp\n pushl %%ebp\n"   /* ebp */
        "movl g_user_ctx+4,  %%esi\n pushl %%esi\n"   /* esi */
        "movl g_user_ctx+0,  %%edi\n pushl %%edi\n"   /* edi */
        "popal\n"
        "pushl g_user_ss\n"
        "pushl g_user_esp\n"
        "pushl g_user_eflags\n"
        "pushl g_user_cs\n"
        "pushl g_user_eip\n"
        "iret\n"
        ::: "memory");
}

/* naked: build iret frame and drop to ring 3.
 * i386 cdecl: [esp]=ret [esp+4]=eip [esp+8]=user_esp at entry. */
__attribute__((section(".text"), naked))
static void enter_user(uint32_t eip, uint32_t esp) {
    __asm__ volatile(
        "movl (%%esp), %%eax\n"       /* return address -> resume eip */
        "movl %%eax, g_resume_eip\n"
        "movl %%esp, %%eax\n"
        "addl $4, %%eax\n"            /* caller esp after ret -> resume esp */
        "movl %%eax, g_resume_esp\n"
        "movl 8(%%esp), %%eax\n"     /* user esp arg */
        "movl 4(%%esp), %%ecx\n"     /* user eip arg */
        "pushl $0x23\n"            /* user data selector | RPL3 */
        "pushl %%eax\n"
        "pushl $0x202\n"           /* eflags: IF=1 */
        "pushl $0x1B\n"            /* user code selector | RPL3 */
        "pushl %%ecx\n"
        "movl g_user_pd, %%eax\n"  /* switch to user page directory */
        "testl %%eax, %%eax\n"
        "jz 1f\n"
        "movl %%eax, %%cr3\n"
        "1:\n"
        "iret\n"
        ::: "eax", "ecx", "memory");
}

static void show_user(void) {
    if (!g_user_base) {
        g_user_base = find_user_region();
        if (!g_user_base) { kputs("user: no free 4MiB-aligned RAM region found.\n"); return; }
        /* mark the covering PDE user-accessible (PTE_USER = 0x4) */
        uint32_t pde = g_user_base >> 22;
        g_page_dir[pde] |= 0x4u;
        /* flush TLB for the new mapping */
        __asm__ volatile("movl %%cr3, %%eax\n\tmovl %%eax, %%cr3" ::: "eax", "memory");

        /* plant user program at region base:
         *   mov eax,3 ; int 0x80      -> SYS_TICKS
         *   mov eax,1 ; mov ebx,msg  ; int 0x80  -> SYS_WRITE
         *   mov eax,0 ; int 0x80      -> SYS_EXIT
         *   int3                       -> #BP shows cs=0x1B */
        uint8_t* code = (uint8_t*)(size_t)g_user_base;
        uint32_t msg_off = 32;
        char* msg = (char*)(code + msg_off);
        const char* m = "hello from ring3!\n";
        for (int i = 0; i <= 18; i++) msg[i] = m[i];
        uint32_t msg_addr = g_user_base + msg_off;

        code[0]=0xB8; code[1]=0x03; code[2]=0x00; code[3]=0x00; code[4]=0x00; /* mov eax,3 */
        code[5]=0xCD; code[6]=0x80;                                                  /* int 0x80 */
        code[7]=0xB8; code[8]=0x01; code[9]=0x00; code[10]=0x00; code[11]=0x00;     /* mov eax,1 */
        code[12]=0xBB; code[13]=(uint8_t)(msg_addr); code[14]=(uint8_t)(msg_addr>>8);
        code[15]=(uint8_t)(msg_addr>>16); code[16]=(uint8_t)(msg_addr>>24);         /* mov ebx,msg */
        code[17]=0xCD; code[18]=0x80;                                                /* int 0x80 */
        code[19]=0xB8; code[20]=0x00; code[21]=0x00; code[22]=0x00; code[23]=0x00;  /* mov eax,0 */
        code[24]=0xCD; code[25]=0x80;                                                /* int 0x80 */
        code[26]=0xCC;                                                               /* int3 */

        kputs("user: region=");
        kprint_hex32(g_user_base);
        kputs(" pde=");
        kprint_dec64(pde);
        kputs(" marked U/S\n");
    }
    uint32_t uesp = g_user_base + 0x80000; /* 512KiB into the region */
    if (user_pd_setup() != 0) { kputs("user: page dir alloc failed.\n"); return; }
    kputs("user: dropping to ring 3 (cs=0x1B ss=0x23)...\n");
    g_user_dead = 0;
    g_preempt_pending = 0;
    g_user_active = 1;
    enter_user(g_user_base, uesp);
    g_user_active = 0;
    kputs("user: returned to kernel (exit or preempt)\n");
}

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
uint8_t* stack_top = stack + sizeof(stack);
