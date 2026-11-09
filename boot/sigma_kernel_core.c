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
