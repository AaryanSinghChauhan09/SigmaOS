/*
 * =========================================================================
 * SigmaOS HAL: RISC-V 64 Implementation (hal_riscv.c)
 * =========================================================================
 */
#include "../hal.h"
#include "../../../../include/sigma_log.h"

const hal_ops_t* hal_ops = (const hal_ops_t*)0;

static void      riscv_halt(void)  { __asm__ __volatile__("csrci mstatus, 8; wfi"); }
static void      riscv_pause(void) { __asm__ __volatile__(".word 0x0100000F"); }
static void      riscv_fence(void) { __asm__ __volatile__("fence iorw, iorw" ::: "memory"); }
static void      riscv_irq_en(void)  { __asm__ __volatile__("csrsi mstatus, 8"); }
static void      riscv_irq_dis(void) { __asm__ __volatile__("csrci mstatus, 8"); }

static void riscv_irq_init(void) {
    *((volatile sigma_u32*)0x0C200000u) = 0u;
    *((volatile sigma_u32*)0x0C002000u) = 0xFFFFFFFEu;
    sigma_log_info("[HAL-RISCV] PLIC initialised.");
}

static void riscv_timer_init(sigma_u32 freq_hz) {
    if (!freq_hz) freq_hz = 100u;
    sigma_u64 now = *((volatile sigma_u64*)0x0200BFF8u);
    *((volatile sigma_u64*)0x02004000u) = now + (sigma_u64)(10000000u / freq_hz);
    __asm__ __volatile__("csrsi mie, 0x80");
    sigma_log_info("[HAL-RISCV] CLINT timer: %u Hz", freq_hz);
}

static sigma_u64 riscv_timer_read(void) {
    sigma_u64 v; __asm__ __volatile__("rdtime %0" : "=r"(v)); return v;
}

static void     riscv_port_out8(sigma_u16 p, sigma_u8 v) { (void)p; (void)v; }
static sigma_u8 riscv_port_in8 (sigma_u16 p) { (void)p; return 0u; }

static void riscv_mmio_write32(sigma_paddr_t addr, sigma_u32 val) {
    *((volatile sigma_u32*)(sigma_usize)addr) = val;
    __asm__ __volatile__("fence ow, ow" ::: "memory");
}
static sigma_u32 riscv_mmio_read32(sigma_paddr_t addr) {
    sigma_u32 v = *((volatile sigma_u32*)(sigma_usize)addr);
    __asm__ __volatile__("fence ir, ir" ::: "memory");
    return v;
}

static sigma_u8  g_rv_pool[4096u * 16u];
static sigma_u32 g_rv_idx = 0u;
static void* riscv_alloc_pages(sigma_u32 n) {
    sigma_u32 b = n * 4096u;
    if (g_rv_idx + b > sizeof(g_rv_pool)) return (void*)0;
    void* p = &g_rv_pool[g_rv_idx]; g_rv_idx += b; return p;
}
static void riscv_free_pages(void* a, sigma_u32 n) { (void)a; (void)n; }

static void riscv_mmu_map(sigma_vaddr_t va, sigma_paddr_t pa, sigma_u64 fl) {
    (void)va; (void)pa; (void)fl;
    sigma_log_info("[HAL-RISCV] mmu_map stub va=0x%llx pa=0x%llx", va, pa);
}
static void riscv_mmu_flush(sigma_vaddr_t va) {
    __asm__ __volatile__("sfence.vma %0, zero" :: "r"(va) : "memory");
}

static const hal_ops_t riscv_hal_ops = {
    hal_init, riscv_halt, riscv_pause, riscv_fence,
    riscv_irq_en, riscv_irq_dis, riscv_irq_init,
    riscv_timer_init, riscv_timer_read,
    riscv_port_out8, riscv_port_in8,
    riscv_mmio_write32, riscv_mmio_read32,
    riscv_alloc_pages, riscv_free_pages,
    riscv_mmu_map, riscv_mmu_flush,
    "riscv64"
};

void hal_init(void) {
    hal_ops = &riscv_hal_ops;
    sigma_log_info("[HAL-RISCV] HAL initialised. arch=riscv64");
    riscv_irq_init();
    riscv_timer_init(100u);
}
