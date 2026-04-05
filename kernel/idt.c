/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: INTERRUPT CONTROLLER + IDT C SETUP (v1.0 - PURE C11)
 * =============================================================================
 * Features:
 *   - IDT: 256-entry 64-bit gate descriptors
 *   - PIC 8259A remapping (IRQ0→32, IRQ8→40)
 *   - Per-vector handler registration
 *   - Fault decoder (GPF, PF, UD, DF, etc.)
 *   - Syscall dispatch (vector 128)
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =========================================================================
 * IDT Gate Descriptor (64-bit interrupt gate)
 * ========================================================================= */
typedef struct __attribute__((packed)) IDTEntry {
    u16 offset_lo;   /* bits 0-15 of handler address */
    u16 selector;    /* code segment selector (GDT) */
    u8  ist;         /* interrupt stack table index (0 = disabled) */
    u8  type_attr;   /* gate type | DPL | P */
    u16 offset_mid;  /* bits 16-31 */
    u32 offset_hi;   /* bits 32-63 */
    u32 zero;        /* reserved */
} IDTEntry;

typedef struct __attribute__((packed)) IDTR {
    u16 limit;
    u64 base;
} IDTR;

/* =========================================================================
 * Interrupt Frame (layout matches isr_common stack layout in idt.asm)
 * ========================================================================= */
typedef struct SigmaInterruptFrame {
    u64 es, ds;
    u64 r15, r14, r13, r12, r11, r10, r9, r8;
    u64 rbp, rdi, rsi, rdx, rcx, rbx, rax;
    u64 vector;
    u64 error_code;
    u64 rip, cs, rflags, rsp, ss;  /* pushed by CPU on exception */
} SigmaInterruptFrame;

/* =========================================================================
 * Handler function pointer type
 * ========================================================================= */
typedef void (*sigma_irq_handler_t)(SigmaInterruptFrame* frame);

/* =========================================================================
 * IDT State
 * ========================================================================= */
#define IDT_ENTRIES   256u
#define IDT_GATE_INT  0x8E   /* P=1 DPL=0 Type=0xE (64-bit interrupt gate) */
#define IDT_GATE_TRAP 0x8F   /* P=1 DPL=0 Type=0xF (64-bit trap gate) */
#define IDT_GATE_USER 0xEE   /* P=1 DPL=3 Type=0xE (user-callable gate) */
#define GDT_KERN_CODE 0x08   /* kernel code segment selector */

static IDTEntry          g_idt[IDT_ENTRIES];
static IDTR              g_idtr;
static sigma_irq_handler_t g_handlers[IDT_ENTRIES];

/* Exported from idt.asm */
extern void*  isr_stub_table[];
extern void   idt_load(IDTR* idtr);

/* =========================================================================
 * PIC 8259A constants
 * ========================================================================= */
#define PIC1_CMD   0x20
#define PIC1_DATA  0x21
#define PIC2_CMD   0xA0
#define PIC2_DATA  0xA1
#define PIC_EOI    0x20    /* End-of-interrupt */
#define PIC1_OFFSET 32
#define PIC2_OFFSET 40

/* =========================================================================
 * PIC: remap IRQ0-15 to vectors 32-47 (avoid collision with CPU exceptions)
 * ========================================================================= */
static void pic_remap(void) {
    /* ICW1: initialize */
    port_outb(PIC1_CMD, 0x11); port_outb(PIC2_CMD, 0x11);
    /* ICW2: vector offsets */
    port_outb(PIC1_DATA, PIC1_OFFSET);
    port_outb(PIC2_DATA, PIC2_OFFSET);
    /* ICW3: cascade */
    port_outb(PIC1_DATA, 4);   /* slave on IRQ2 */
    port_outb(PIC2_DATA, 2);   /* cascade identity */
    /* ICW4: 8086 mode */
    port_outb(PIC1_DATA, 0x01); port_outb(PIC2_DATA, 0x01);
    /* Mask all IRQs initially — unmask individually */
    port_outb(PIC1_DATA, 0xFD);  /* unmask IRQ1 (keyboard) only */
    port_outb(PIC2_DATA, 0xFF);
}

static void pic_eoi(u8 irq) {
    if (irq >= 8) port_outb(PIC2_CMD, PIC_EOI);
    port_outb(PIC1_CMD, PIC_EOI);
}

void pic_unmask_irq(u8 irq) {
    u16 port = (irq < 8) ? PIC1_DATA : PIC2_DATA;
    u8  mask = port_inb(port) & (u8)~(1u << (irq & 7));
    port_outb(port, mask);
}

/* =========================================================================
 * IDT entry setter
 * ========================================================================= */
static void idt_set_gate(u32 vec, void* handler, u8 type_attr, u8 ist) {
    u64 addr = (u64)(usize)handler;
    IDTEntry* e = &g_idt[vec];
    e->offset_lo  = (u16)(addr & 0xFFFF);
    e->selector   = GDT_KERN_CODE;
    e->ist        = ist;
    e->type_attr  = type_attr;
    e->offset_mid = (u16)((addr >> 16) & 0xFFFF);
    e->offset_hi  = (u32)(addr >> 32);
    e->zero       = 0;
}

/* =========================================================================
 * IDT Init
 * ========================================================================= */
void idt_init(void) {
    u32 i;
    /* Install ISR stubs for all 256 vectors */
    for (i = 0; i < IDT_ENTRIES; i++) {
        u8 attr = (i == 128) ? IDT_GATE_USER : IDT_GATE_INT;
        idt_set_gate(i, isr_stub_table[i], attr, 0);
        g_handlers[i] = NULL;
    }

    /* IDTR */
    g_idtr.limit = (u16)(sizeof(g_idt) - 1);
    g_idtr.base  = (u64)(usize)g_idt;

    /* Remap PIC */
    pic_remap();

    /* Load IDT */
    idt_load(&g_idtr);
    cpu_sti();
}

/* =========================================================================
 * Handler registration
 * ========================================================================= */
void idt_register_handler(u32 vec, sigma_irq_handler_t fn) {
    if (vec < IDT_ENTRIES) g_handlers[vec] = fn;
}

/* =========================================================================
 * Common C interrupt handler (called from isr_common in idt.asm)
 * ========================================================================= */
static const char* g_exception_names[] = {
    "Divide-by-Zero",   "Debug",              "NMI",
    "Breakpoint",       "Overflow",           "Bound Range",
    "Invalid Opcode",   "Device Not Avail",   "Double Fault",
    "Coprocessor Seg",  "Invalid TSS",        "Segment Not Present",
    "Stack-Seg Fault",  "General Protection", "Page Fault",
    "Reserved",         "x87 FP Exception",   "Alignment Check",
    "Machine Check",    "SIMD FP Exception",  "Virt Exception",
    "Control Prot",     "Reserved",           "Reserved",
    "Reserved",         "Reserved",           "Reserved",
    "Reserved",         "Hypervisor Inject",  "VMM Comm",
    "Security Except",  "Reserved"
};

/* Already provided by sigma_kernel_types.h */

void sigma_interrupt_handler(SigmaInterruptFrame* frame) {
    u64 vec = frame->vector;

    /* Dispatch to registered handler first */
    if (vec < IDT_ENTRIES && g_handlers[vec]) {
        g_handlers[vec](frame);
        /* Send EOI for hardware IRQs */
        if (vec >= 32 && vec < 48) pic_eoi((u8)(vec - 32));
        return;
    }

    /* Hardware IRQs — send EOI even if no handler */
    if (vec >= 32 && vec < 48) {
        pic_eoi((u8)(vec - 32));
        return;
    }

    if (vec < 32) {
        sigma_kprintf("\n[KERNEL PANIC] Exception #%llu: %s\n",
                vec, g_exception_names[vec]);
        sigma_kprintf("  Error Code : %016llx\n", frame->error_code);
        sigma_kprintf("  RIP        : %016llx\n", frame->rip);
        sigma_kprintf("  CS         : %04llx\n",  frame->cs);
        sigma_kprintf("  RFLAGS     : %016llx\n", frame->rflags);
        sigma_kprintf("  RSP        : %016llx\n", frame->rsp);
        sigma_kprintf("  RAX        : %016llx\n", frame->rax);
        sigma_kprintf("  RBX        : %016llx\n", frame->rbx);
        sigma_kprintf("  RCX        : %016llx\n", frame->rcx);
        sigma_kprintf("  RDX        : %016llx\n", frame->rdx);
        /* Halt and catch fire */
        cpu_cli();
        while (1) cpu_halt();
    }
}
