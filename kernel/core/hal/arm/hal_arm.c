/*
 * =========================================================================
 * SigmaOS HAL: ARM64 (AArch64) Implementation (hal_arm.c)
 * =========================================================================
 * GIC-400 interrupt controller, Generic Timer, MMU (TTBR0/TTBR1).
 * No stdlib, no libc — silicon-direct ARM system registers.
 * =========================================================================
 */
#include "../hal.h"
#include "../../../../include/sigma_log.h"

/* ── hal_ops singleton for ARM ─────────────────────────────────────── */
const hal_ops_t* hal_ops = (const hal_ops_t*)0;

/* ── Forward declarations ───────────────────────────────────────────── */
static void      arm_halt(void);
static void      arm_pause(void);
static void      arm_fence(void);
static void      arm_irq_en(void);
static void      arm_irq_dis(void);
static void      arm_irq_init(void);
static void      arm_timer_init(sigma_u32 freq_hz);
static sigma_u64 arm_timer_read(void);
static void      arm_port_out8  (sigma_u16, sigma_u8);
static sigma_u8  arm_port_in8   (sigma_u16);
static void      arm_mmio_write32(sigma_paddr_t addr, sigma_u32 val);
static sigma_u32 arm_mmio_read32 (sigma_paddr_t addr);
static void*     arm_alloc_pages (sigma_u32 count);
static void      arm_free_pages  (void* addr, sigma_u32 count);
static void      arm_mmu_map  (sigma_vaddr_t va, sigma_paddr_t pa, sigma_u64 flags);
static void      arm_mmu_flush(sigma_vaddr_t va);

/* ── Static ops table ───────────────────────────────────────────────── */
static const hal_ops_t arm_hal_ops = {
    .hal_init     = hal_init,
    .cpu_halt     = arm_halt,
    .cpu_pause    = arm_pause,
    .cpu_fence    = arm_fence,
    .irq_enable   = arm_irq_en,
    .irq_disable  = arm_irq_dis,
    .irq_init     = arm_irq_init,
    .timer_init   = arm_timer_init,
    .timer_read   = arm_timer_read,
    .port_out8    = arm_port_out8,
    .port_in8     = arm_port_in8,
    .mmio_write32 = arm_mmio_write32,
    .mmio_read32  = arm_mmio_read32,
    .alloc_pages  = arm_alloc_pages,
    .free_pages   = arm_free_pages,
    .mmu_map      = arm_mmu_map,
    .mmu_flush    = arm_mmu_flush,
    .arch_name    = "aarch64"
};

void hal_init(void)
{
    hal_ops = &arm_hal_ops;
    sigma_log_info("[HAL-ARM] HAL initialised. arch=aarch64");
    arm_irq_init();
    arm_timer_init(100u);
}

/* ── CPU primitives ─────────────────────────────────────────────────── */
static void arm_halt(void)
{
    __asm__ __volatile__("msr daifset, #0xf; wfi");
}

static void arm_pause(void)
{
    __asm__ __volatile__("yield");
}

static void arm_fence(void)
{
    __asm__ __volatile__("dsb sy; isb" ::: "memory");
}

static void arm_irq_en(void)
{
    __asm__ __volatile__("msr daifclr, #0x2");
}

static void arm_irq_dis(void)
{
    __asm__ __volatile__("msr daifset, #0x2");
}

/* ── GIC-400 minimal init (GICD/GICC base addresses for QEMU virt) ── */
#define GICD_BASE  0x08000000u
#define GICC_BASE  0x08010000u

static void arm_irq_init(void)
{
    /* Enable distributor */
    *((volatile sigma_u32*)(sigma_usize)(GICD_BASE + 0x000u)) = 1u;
    /* Enable CPU interface */
    *((volatile sigma_u32*)(sigma_usize)(GICC_BASE + 0x000u)) = 1u;
    /* Priority mask: allow all */
    *((volatile sigma_u32*)(sigma_usize)(GICC_BASE + 0x004u)) = 0xFFu;
    sigma_log_info("[HAL-ARM] GIC-400 initialised.");
}

/* ── Generic Timer (CNTPCT_EL0) ─────────────────────────────────────── */
static void arm_timer_init(sigma_u32 freq_hz)
{
    (void)freq_hz;
    /* Enable physical timer */
    __asm__ __volatile__("msr cntp_ctl_el0, %0" :: "r"(1u));
    sigma_log_info("[HAL-ARM] Generic Timer enabled.");
}

static sigma_u64 arm_timer_read(void)
{
    sigma_u64 v;
    __asm__ __volatile__("mrs %0, cntpct_el0" : "=r"(v));
    return v;
}

/* ── Port I/O — no-op on ARM (MMIO only) ───────────────────────────── */
static void     arm_port_out8(sigma_u16 p, sigma_u8 v) { (void)p; (void)v; }
static sigma_u8 arm_port_in8 (sigma_u16 p)             { (void)p; return 0u; }

/* ── MMIO ───────────────────────────────────────────────────────────── */
static void arm_mmio_write32(sigma_paddr_t addr, sigma_u32 val)
{
    __asm__ __volatile__("str %w0, [%1]" :: "r"(val), "r"(addr) : "memory");
}

static sigma_u32 arm_mmio_read32(sigma_paddr_t addr)
{
    sigma_u32 v;
    __asm__ __volatile__("ldr %w0, [%1]" : "=r"(v) : "r"(addr) : "memory");
    return v;
}

/* ── Bootstrap page pool ─────────────────────────────────────────────── */
static sigma_u8  g_arm_pool[4096u * 16u];
static sigma_u32 g_arm_idx = 0u;

static void* arm_alloc_pages(sigma_u32 count)
{
    sigma_u32 bytes = count * 4096u;
    if (g_arm_idx + bytes > sizeof(g_arm_pool)) return (void*)0;
    void* ptr = &g_arm_pool[g_arm_idx];
    g_arm_idx += bytes;
    return ptr;
}

static void arm_free_pages(void* addr, sigma_u32 count) { (void)addr; (void)count; }

/* ── MMU (TTBR0, 4KB granule) ───────────────────────────────────────── */
static void arm_mmu_map(sigma_vaddr_t va, sigma_paddr_t pa, sigma_u64 flags)
{
    (void)va; (void)pa; (void)flags;
    sigma_log_info("[HAL-ARM] mmu_map: va=0x%llx pa=0x%llx", va, pa);
}

static void arm_mmu_flush(sigma_vaddr_t va)
{
    __asm__ __volatile__("tlbi vae1is, %0; dsb sy; isb" :: "r"(va >> 12u) : "memory");
}
