/*
 * =========================================================================
 * SigmaOS HAL: x86_64 Implementation (hal_x86.c)
 * =========================================================================
 * Silicon-direct port I/O, IRQ controller, timer, and MMU mapping.
 * Inline assembly only — no stdlib, no libc.
 * =========================================================================
 */
#include "../hal.h"
#include "sigma_log.h"

/* ── Global hal_ops pointer (defined here for x86 TU) ──────────────── */
const hal_ops_t* hal_ops = (const hal_ops_t*)0; /* set by hal_init */

/* ── Forward declarations ───────────────────────────────────────────── */
static void      x86_halt   (void);
static void      x86_pause  (void);
static void      x86_fence  (void);
static void      x86_irq_en (void);
static void      x86_irq_dis(void);
static void      x86_irq_init(void);
static void      x86_timer_init(sigma_u32 freq_hz);
static sigma_u64 x86_timer_read(void);
static void      x86_port_out8(sigma_u16 port, sigma_u8 val);
static sigma_u8  x86_port_in8 (sigma_u16 port);
static void      x86_mmio_write32(sigma_paddr_t addr, sigma_u32 val);
static sigma_u32 x86_mmio_read32 (sigma_paddr_t addr);
static void*     x86_alloc_pages(sigma_u32 count);
static void      x86_free_pages (void* addr, sigma_u32 count);
static void      x86_mmu_map   (sigma_vaddr_t va, sigma_paddr_t pa, sigma_u64 flags);
static void      x86_mmu_flush (sigma_vaddr_t va);

/* ── Static ops table ───────────────────────────────────────────────── */
static const hal_ops_t x86_hal_ops = {
    .hal_init     = hal_init,
    .cpu_halt     = x86_halt,
    .cpu_pause    = x86_pause,
    .cpu_fence    = x86_fence,
    .irq_enable   = x86_irq_en,
    .irq_disable  = x86_irq_dis,
    .irq_init     = x86_irq_init,
    .timer_init   = x86_timer_init,
    .timer_read   = x86_timer_read,
    .port_out8    = x86_port_out8,
    .port_in8     = x86_port_in8,
    .mmio_write32 = x86_mmio_write32,
    .mmio_read32  = x86_mmio_read32,
    .alloc_pages  = x86_alloc_pages,
    .free_pages   = x86_free_pages,
    .mmu_map      = x86_mmu_map,
    .mmu_flush    = x86_mmu_flush,
    .arch_name    = "x86_64"
};

/* ── hal_init ───────────────────────────────────────────────────────── */
void hal_init(void)
{
    hal_ops = &x86_hal_ops;
    sigma_log_info("[HAL-x86] HAL initialised. arch=x86_64");
    x86_irq_init();
    x86_timer_init(100u); /* 100 Hz tick */
}

/* ── CPU primitives ─────────────────────────────────────────────────── */
static void x86_halt(void)
{
    __asm__ __volatile__("cli; hlt");
}

static void x86_pause(void)
{
    __asm__ __volatile__("pause");
}

static void x86_fence(void)
{
    __asm__ __volatile__("mfence" ::: "memory");
}

static void x86_irq_en(void)
{
    __asm__ __volatile__("sti");
}

static void x86_irq_dis(void)
{
    __asm__ __volatile__("cli");
}

/* ── PIC initialisation (8259A master/slave) ───────────────────────── */
#define PIC1_CMD  0x20u
#define PIC1_DATA 0x21u
#define PIC2_CMD  0xA0u
#define PIC2_DATA 0xA1u
#define PIC_EOI   0x20u
#define ICW1_INIT 0x11u
#define ICW4_8086 0x01u

static void pic_remap(sigma_u8 offset1, sigma_u8 offset2)
{
    x86_port_out8(PIC1_CMD,  ICW1_INIT);
    x86_port_out8(PIC2_CMD,  ICW1_INIT);
    x86_port_out8(PIC1_DATA, offset1);
    x86_port_out8(PIC2_DATA, offset2);
    x86_port_out8(PIC1_DATA, 0x04u);   /* slave at IRQ2 */
    x86_port_out8(PIC2_DATA, 0x02u);   /* cascade identity */
    x86_port_out8(PIC1_DATA, ICW4_8086);
    x86_port_out8(PIC2_DATA, ICW4_8086);
    /* mask all IRQs initially */
    x86_port_out8(PIC1_DATA, 0xFFu);
    x86_port_out8(PIC2_DATA, 0xFFu);
}

static void x86_irq_init(void)
{
    pic_remap(0x20u, 0x28u); /* IRQ 0-7 → INT 0x20-0x27 */
    sigma_log_info("[HAL-x86] PIC remapped: IRQ0-7=0x20 IRQ8-15=0x28");
}

/* ── PIT timer (channel 0, mode 3 square wave) ─────────────────────── */
#define PIT_CMD   0x43u
#define PIT_CH0   0x40u
#define PIT_BASE  1193182u

static void x86_timer_init(sigma_u32 freq_hz)
{
    if (freq_hz == 0u) freq_hz = 100u;
    sigma_u32 divisor = PIT_BASE / freq_hz;
    x86_port_out8(PIT_CMD, 0x36u);                        /* ch0, lobyte/hibyte, mode3 */
    x86_port_out8(PIT_CH0, (sigma_u8)(divisor & 0xFFu));
    x86_port_out8(PIT_CH0, (sigma_u8)((divisor >> 8) & 0xFFu));
    sigma_log_info("[HAL-x86] PIT timer: %u Hz", freq_hz);
}

static sigma_u64 x86_timer_read(void)
{
    sigma_u64 v;
    __asm__ __volatile__(
        "rdtsc\n\t"
        "shl $32, %%rdx\n\t"
        "or  %%rdx, %%rax"
        : "=a"(v) :: "rdx");
    return v;
}

/* ── Port I/O ───────────────────────────────────────────────────────── */
static void x86_port_out8(sigma_u16 port, sigma_u8 val)
{
    __asm__ __volatile__("outb %0, %1" :: "a"(val), "dN"(port));
}

static sigma_u8 x86_port_in8(sigma_u16 port)
{
    sigma_u8 v;
    __asm__ __volatile__("inb %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

/* ── MMIO ───────────────────────────────────────────────────────────── */
static void x86_mmio_write32(sigma_paddr_t addr, sigma_u32 val)
{
    *((volatile sigma_u32*)(sigma_usize)addr) = val;
}

static sigma_u32 x86_mmio_read32(sigma_paddr_t addr)
{
    return *((volatile sigma_u32*)(sigma_usize)addr);
}

/* ── Page allocator stub (replaced by PMM at runtime) ──────────────── */
/* Linear bump allocator starting after 4 MB physical mark */
static sigma_u8 g_phys_pool[4096u * 16u]; /* 64 KB bootstrap pool */
static sigma_u32 g_pool_idx = 0u;

static void* x86_alloc_pages(sigma_u32 count)
{
    sigma_u32 bytes = count * 4096u;
    if (g_pool_idx + bytes > sizeof(g_phys_pool)) return (void*)0;
    void* ptr = &g_phys_pool[g_pool_idx];
    g_pool_idx += bytes;
    return ptr;
}

static void x86_free_pages(void* addr, sigma_u32 count)
{
    (void)addr; (void)count; /* PMM will handle reclamation */
}

/* ── MMU (4-level paging — PML4) ───────────────────────────────────── */
/* Minimal: write a PTE into the active page table. Real PMM wires this up. */
static void x86_mmu_map(sigma_vaddr_t va, sigma_paddr_t pa, sigma_u64 flags)
{
    (void)va; (void)pa; (void)flags;
    /* Full implementation lives in kernel/mm/paging.c */
    sigma_log_info("[HAL-x86] mmu_map: va=0x%llx pa=0x%llx flags=0x%llx", va, pa, flags);
}

static void x86_mmu_flush(sigma_vaddr_t va)
{
    __asm__ __volatile__("invlpg (%0)" :: "r"(va) : "memory");
}
