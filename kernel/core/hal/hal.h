/*
 * =========================================================================
 * SigmaOS: Generic Hardware Abstraction Layer Interface (hal.h)
 * =========================================================================
 * Each architecture implements a static hal_ops_t and assigns hal_ops
 * from its hal_init() function.  The kernel uses only this interface.
 * =========================================================================
 */
#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ── HAL Operations Vtable ──────────────────────────────────────────── */
typedef struct {
    /* Lifecycle */
    void (*hal_init)(void);
    void (*cpu_halt)(void);
    void (*cpu_pause)(void);
    void (*cpu_fence)(void);

    /* Interrupts */
    void (*irq_enable)(void);
    void (*irq_disable)(void);
    void (*irq_init)(void);

    /* Timer */
    void     (*timer_init)(sigma_u32 freq_hz);
    sigma_u64 (*timer_read)(void);

    /* Port I/O (x86 only; no-op on ARM/RISC-V) */
    void     (*port_out8)(sigma_u16 port, sigma_u8  val);
    sigma_u8 (*port_in8) (sigma_u16 port);

    /* MMIO */
    void       (*mmio_write32)(sigma_paddr_t addr, sigma_u32 val);
    sigma_u32  (*mmio_read32) (sigma_paddr_t addr);

    /* Memory management hooks */
    void* (*alloc_pages)(sigma_u32 count);
    void  (*free_pages) (void* addr, sigma_u32 count);

    /* MMU */
    void (*mmu_map)  (sigma_vaddr_t va, sigma_paddr_t pa, sigma_u64 flags);
    void (*mmu_flush)(sigma_vaddr_t va);

    /* Architecture string (e.g. "x86_64", "aarch64", "riscv64") */
    const char* arch_name;
} hal_ops_t;

/* ── Singleton pointer — set by arch hal_init() ─────────────────────── */
extern const hal_ops_t* hal_ops;

/* ── Bootstrap function — implemented per architecture ──────────────── */
void hal_init(void);

/* ── Convenience macros ─────────────────────────────────────────────── */
#define HAL_HALT()          hal_ops->cpu_halt()
#define HAL_IRQ_EN()        hal_ops->irq_enable()
#define HAL_IRQ_DIS()       hal_ops->irq_disable()
#define HAL_OUT8(p, v)      hal_ops->port_out8((p), (v))
#define HAL_IN8(p)          hal_ops->port_in8((p))
#define HAL_MMIO_W32(a, v)  hal_ops->mmio_write32((a), (v))
#define HAL_MMIO_R32(a)     hal_ops->mmio_read32((a))
#define HAL_MAP(va, pa, f)  hal_ops->mmu_map((va), (pa), (f))
#define HAL_FLUSH(va)       hal_ops->mmu_flush((va))

/* ── MMU flag constants ─────────────────────────────────────────────── */
#define HAL_MMU_PRESENT  BIT(0)
#define HAL_MMU_WRITE    BIT(1)
#define HAL_MMU_USER     BIT(2)
#define HAL_MMU_NOCACHE  BIT(4)
#define HAL_MMU_EXEC     BIT(5)

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAL_H */
