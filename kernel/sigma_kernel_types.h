/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN KERNEL TYPES (v2.0 - ZENITH CONSOLIDATED)
 * =============================================================================
 * Inherits universal foundations from sigma_types.h.
 * Adds kernel-specific silicon primitives and synchronization types.
 * =============================================================================
 */

#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

#include "../libc/sigma_types.h"

/* ---- Kernel-Specific Address Types ---- */
typedef u64 paddr_t;   /* physical address */
typedef u64 vaddr_t;   /* virtual  address */
typedef int bool_t;

#define TRUE   SIGMA_TRUE
#define FALSE  SIGMA_FALSE
#define NULL   SIGMA_NULL

/* ---- Synchronization ---- */
typedef volatile u32 spinlock_t;
extern void hal_spinlock_acquire(spinlock_t* lock);
extern void hal_spinlock_release(spinlock_t* lock);

#define spinlock_init(l)    (*(l) = 0)
#define spinlock_acquire(l) hal_spinlock_acquire(l)
#define spinlock_release(l) hal_spinlock_release(l)

/* ---- page constants ---- */
#define PAGE_SIZE       4096ULL
#define PAGE_SHIFT      12u
#define HUGE_PAGE_SIZE  (2ULL * 1024ULL * 1024ULL)

/* ---- kernel virtual base ---- */
#define KERNEL_VMA      0xFFFFFFFF80000000ULL   /* -2GB top */

/* ---- alignment helpers ---- */
#define ALIGN_DOWN(v, a)  ((v) & ~((a)-1))
#define ALIGN_UP(v, a)    (((v) + (a)-1) & ~((a)-1))

/* ---- status codes ---- */
#define K_OK              0
#define K_ERR_NOMEM      -1
#define K_ERR_INVAL      -2
#define K_ERR_BUSY       -3
#define K_ERR_NOTFOUND   -4
#define K_ERR_PERM       -5
typedef i32 k_status;

/* ---- CPU intrinsics (x86_64) ---- */
static inline void cpu_halt(void)  { __asm__ __volatile__("cli; hlt"); }
static inline void cpu_pause(void) { __asm__ __volatile__("pause"); }
static inline void cpu_fence(void) { __asm__ __volatile__("mfence" ::: "memory"); }
static inline void cpu_sti(void)   { __asm__ __volatile__("sti"); }
static inline void cpu_cli(void)   { __asm__ __volatile__("cli"); }

static inline u64 cpu_rdtsc(void) {
    u64 v;
    __asm__ __volatile__(
        "rdtsc\n\t shl $32,%%rdx\n\t or %%rdx,%%rax"
        : "=a"(v) :: "rdx");
    return v;
}

static inline u8 port_inb(u16 port) {
    u8 v;
    __asm__ __volatile__("inb %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

static inline void port_outb(u16 port, u8 val) {
    __asm__ __volatile__("outb %0, %1" :: "a"(val), "dN"(port));
}

/* =========================================================================
 * SOVEREIGN-ASM: Silicon-Direct Memory Orchestration (Rep-String)
 * ========================================================================= */
static inline void* k_memcpy(void* dst, const void* src, usize n) {
    __asm__ __volatile__ (
        "rep movsb"
        : "+D"(dst), "+S"(src), "+c"(n)
        : : "memory"
    );
    return dst;
}

static inline void* k_memset(void* s, int c, usize n) {
    __asm__ __volatile__ (
        "rep stosb"
        : "+D"(s), "+c"(n)
        : "a"((u8)c)
        : "memory"
    );
    return s;
}

/* ---- prng ---- */
extern u32 sigma_rand32(void);
extern u64 sigma_rand64(void);

#endif
