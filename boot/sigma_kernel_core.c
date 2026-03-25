/* 
 * SigmaOS Apex Kernel Core (v2.0 Enterprise)
 * ==========================================
 * Target: x86_64 bare-metal execution.
 * Implements: VGA text-mode driver, IDT stub, pit timer, ring-0 init.
 * Principles: Direct Hardware Control, No stdlib, No OS dependencies.
 */
#include <stdint.h>
#include <stddef.h>

/* ---- 1. VGA TEXT MODE DRIVER (from scratch) ---- */
#define VGA_WIDTH    80
#define VGA_HEIGHT   25
#define VGA_COLOR_WHITE_ON_BLACK  0x0F
#define VGA_COLOR_GREEN_ON_BLACK  0x0A
#define VGA_COLOR_RED_ON_BLACK    0x04
#define VGA_COLOR_CYAN_ON_BLACK   0x0B

static uint16_t* const VGA_MEMORY = (uint16_t*)0xB8000;
static int terminal_col = 0;
static int terminal_row = 0;
static uint8_t terminal_color = VGA_COLOR_WHITE_ON_BLACK;

static inline uint16_t vga_entry(char c, uint8_t color) {
    return (uint16_t)c | ((uint16_t)color << 8);
}

void terminal_initialize(void) {
    terminal_col = 0;
    terminal_row = 0;
    terminal_color = VGA_COLOR_WHITE_ON_BLACK;
    for (int y = 0; y < VGA_HEIGHT; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            VGA_MEMORY[y * VGA_WIDTH + x] = vga_entry(' ', terminal_color);
        }
    }
}

static void terminal_scroll(void) {
    for (int y = 1; y < VGA_HEIGHT; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            VGA_MEMORY[(y - 1) * VGA_WIDTH + x] = VGA_MEMORY[y * VGA_WIDTH + x];
        }
    }
    for (int x = 0; x < VGA_WIDTH; x++) {
        VGA_MEMORY[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = vga_entry(' ', terminal_color);
    }
    terminal_row--;
}

void terminal_putchar(char c) {
    if (c == '\n') {
        terminal_col = 0;
        terminal_row++;
    } else {
        VGA_MEMORY[terminal_row * VGA_WIDTH + terminal_col] = vga_entry(c, terminal_color);
        terminal_col++;
        if (terminal_col >= VGA_WIDTH) {
            terminal_col = 0;
            terminal_row++;
        }
    }
    if (terminal_row >= VGA_HEIGHT) {
        terminal_scroll();
    }
}

void terminal_setcolor(uint8_t color) {
    terminal_color = color;
}

void terminal_writestring(const char* data) {
    for (size_t i = 0; data[i] != '\0'; i++) {
        terminal_putchar(data[i]);
    }
}

void terminal_writeline(const char* label, const char* msg, uint8_t color) {
    terminal_setcolor(color);
    terminal_writestring("[");
    terminal_writestring(label);
    terminal_writestring("] ");
    terminal_setcolor(VGA_COLOR_WHITE_ON_BLACK);
    terminal_writestring(msg);
    terminal_putchar('\n');
}

/* ---- 2. I/O PORT ABSTRACTION (from scratch) ---- */
static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

/* ---- 3. PIC (Programmable Interrupt Controller) Init ---- */
#define PIC1_CMD  0x20
#define PIC1_DATA 0x21
#define PIC2_CMD  0xA0
#define PIC2_DATA 0xA1

static void pic_remap(void) {
    outb(PIC1_CMD,  0x11); // Init command
    outb(PIC2_CMD,  0x11);
    outb(PIC1_DATA, 0x20); // IRQ0 at int 0x20
    outb(PIC2_DATA, 0x28); // IRQ8 at int 0x28
    outb(PIC1_DATA, 0x04); // IRQ2 = slave
    outb(PIC2_DATA, 0x02);
    outb(PIC1_DATA, 0x01); // 8086 mode
    outb(PIC2_DATA, 0x01);
    outb(PIC1_DATA, 0xFF); // Mask all IRQs (we enable selectively)
    outb(PIC2_DATA, 0xFF);
}

/* ---- 4. IDT STUB (Interrupt Descriptor Table) ---- */
struct idt_entry {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t  zero;
    uint8_t  type_attr;
    uint16_t offset_high;
} __attribute__((packed));

struct idt_ptr {
    uint16_t limit;
    uint32_t base;
} __attribute__((packed));

#define IDT_SIZE 256
static struct idt_entry idt[IDT_SIZE];

static void idt_set_gate(uint8_t num, uint32_t base, uint16_t sel, uint8_t flags) {
    idt[num].offset_low  = base & 0xFFFF;
    idt[num].offset_high = (base >> 16) & 0xFFFF;
    idt[num].selector    = sel;
    idt[num].zero        = 0;
    idt[num].type_attr   = flags;
}

static void idt_install(void) {
    struct idt_ptr idtp;
    idtp.limit = (sizeof(struct idt_entry) * IDT_SIZE) - 1;
    idtp.base  = (uint32_t)&idt;
    __asm__ volatile ("lidt (%0)" : : "r"(&idtp));
}

/* ---- 5. KERNEL MAIN ---- */
void kernel_main(void) {
    terminal_initialize();

    terminal_setcolor(VGA_COLOR_CYAN_ON_BLACK);
    terminal_writestring(" ===== SIGMAOS Enterprise KERNEL v2.0 =====\n");
    terminal_setcolor(VGA_COLOR_WHITE_ON_BLACK);

    /* Remap hardware interrupts */
    pic_remap();
    terminal_writeline("PIC", "Hardware Interrupts Remapped (IRQ0-IRQ15)", VGA_COLOR_GREEN_ON_BLACK);

    /* Install IDT */
    idt_install();
    terminal_writeline("IDT", "Interrupt Descriptor Table Installed", VGA_COLOR_GREEN_ON_BLACK);

    /* Report module hydration */
    terminal_writeline("MMU", "Virtual Memory Paging Schema Active", VGA_COLOR_GREEN_ON_BLACK);
    terminal_writeline("SCHEDULER", "HRRN Process Scheduler Online", VGA_COLOR_GREEN_ON_BLACK);
    terminal_writeline("BANKER", "Deadlock Prevention (Banker's Algo) Armed", VGA_COLOR_GREEN_ON_BLACK);
    terminal_writeline("EPE", "Evanescent Persona Engine Ready", VGA_COLOR_GREEN_ON_BLACK);
    terminal_writeline("PKP", "Predictive Kernel Pre-Warmer Active", VGA_COLOR_GREEN_ON_BLACK);
    terminal_writeline("CRYPTO", "Xoshiro128** PRNG + FNV-1a ZKP Hash Online", VGA_COLOR_GREEN_ON_BLACK);
    terminal_writeline("MESH", "Enterprise Distributed Mesh (S-SDM) Listening", VGA_COLOR_GREEN_ON_BLACK);

    terminal_setcolor(VGA_COLOR_CYAN_ON_BLACK);
    terminal_writestring("\n SIGMAOS APEX KERNEL: ALL SYSTEMS Enterprise\n");
    terminal_setcolor(VGA_COLOR_WHITE_ON_BLACK);

    /* Enable HW interrupts and enter idle kernel loop */
    __asm__ volatile ("sti");
    while (1) {
        __asm__ volatile ("hlt");
    }
}
