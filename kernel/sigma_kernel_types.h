/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN KERNEL TYPES (v1.0 - C11 ZERO-DEPENDENCY)
 * =============================================================================
 * All kernel-internal types, constants, and primitive definitions.
 * Standard: C11 (ISO/IEC 9899:2011) — no external headers.
 * =============================================================================
 */

#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

/* ---- primitive types ---- */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef signed char        i8;
typedef signed short       i16;
typedef signed int         i32;
typedef signed long long   i64;
typedef unsigned long long usize;
typedef long long          isize;
typedef u64                paddr_t;   /* physical address */
typedef u64                vaddr_t;   /* virtual  address */
typedef int                bool_t;
typedef volatile u32       spinlock_t;

/* ---- standard aliases (Zero-Dependency compliant) ---- */
typedef u8  uint8_t;
typedef u16 uint16_t;
typedef u32 uint32_t;
typedef unsigned long long uint64_t;
typedef i8  int8_t;
typedef i16 int16_t;
typedef i32 int32_t;
typedef long long int64_t;
typedef usize size_t;
typedef isize ssize_t;

#ifndef __cplusplus
typedef bool_t bool;
#define true  TRUE
#define false FALSE
#endif

extern void hal_spinlock_acquire(spinlock_t* lock);
extern void hal_spinlock_release(spinlock_t* lock);

#define spinlock_init(l)    (*(l) = 0)
#define spinlock_acquire(l) hal_spinlock_acquire(l)
#define spinlock_release(l) hal_spinlock_release(l)

#define TRUE   1
#define FALSE  0

#ifndef NULL
#define NULL   ((void*)0)
#endif

/* ---- page constants ---- */
#define PAGE_SIZE       4096ULL
#define PAGE_SHIFT      12u
#define HUGE_PAGE_SIZE  (2ULL * 1024ULL * 1024ULL)

/* ---- kernel virtual base ---- */
#define KERNEL_VMA      0xFFFFFFFF80000000ULL   /* -2GB top */

/* ---- alignment helpers ---- */
#define ALIGN_DOWN(v, a)  ((v) & ~((a)-1))
#define ALIGN_UP(v, a)    (((v) + (a)-1) & ~((a)-1))

/* ---- bit helpers ---- */
#define BIT(n)            (1ULL << (n))
#define IS_SET(v, n)      (!!((v) & BIT(n)))

/* ---- status codes ---- */
#define K_OK              0
#define K_ERR_NOMEM      -1
#define K_ERR_INVAL      -2
#define K_ERR_BUSY       -3
#define K_ERR_NOTFOUND   -4
#define K_ERR_PERM       -5

typedef i32 k_status;

/* ---- intrinsics ---- */
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

static inline u64 cpu_read_cr3(void) {
    u64 v;
    __asm__ __volatile__("mov %%cr3, %0" : "=r"(v));
    return v;
}

static inline void cpu_write_cr3(u64 v) {
    __asm__ __volatile__("mov %0, %%cr3" :: "r"(v) : "memory");
}

static inline void cpu_invlpg(vaddr_t va) {
    __asm__ __volatile__("invlpg (%0)" :: "r"(va) : "memory");
}

static inline u8 port_inb(u16 port) {
    u8 v;
    __asm__ __volatile__("inb %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

static inline void port_outb(u16 port, u8 val) {
    __asm__ __volatile__("outb %0, %1" :: "a"(val), "dN"(port));
}

static inline void port_outw(u16 port, u16 val) {
    __asm__ __volatile__("outw %0, %1" :: "a"(val), "dN"(port));
}

/* =========================================================================
 * SOVEREIGN-ASM: Silicon-Direct Memory Orchestration (No Pre-Defined Functions)
 * ========================================================================= */

static inline void sigma_memcpy(void* dst, const void* src, usize n) {
    __asm__ __volatile__ (
        "rep movsb"
        : "+D"(dst), "+S"(src), "+c"(n)
        : : "memory"
    );
}

static inline void* sigma_memset(void* s, int c, usize n) {
    __asm__ __volatile__ (
        "rep stosb"
        : "+D"(s), "+c"(n)
        : "a"((u8)c)
        : "memory"
    );
    return s;
}

static inline usize sigma_strlen(const char* s) {
    usize len = 0;
    while (s[len]) len++;
    return len;
}

static inline int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return (unsigned char)*s1 - (unsigned char)*s2;
}

static inline char* sigma_strcpy_safe(char* dst, const char* src, usize max) {
    usize i;
    for (i = 0; i < max - 1 && src[i]; i++) dst[i] = src[i];
    dst[i] = '\0';
    return dst;
}

static inline const char* sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return haystack;
    const char* h = haystack;
    while (*h) {
        const char* p = h, *n = needle;
        while (*p && *n && *p == *n) { p++; n++; }
        if (!*n) return h;
        h++;
    }
    return (const char*)0;
}

/* =========================================================================
 * SOVEREIGN-FAULT: Industrial Recovery & Assertion
 * ========================================================================= */
void sigma_panic(const char* msg, u64 rip, u64 rsp);
void sigma_kprintf(const char* fmt, ...);

#define SIGMA_ASSERT(cond, msg) \
    do { if (!(cond)) sigma_panic(msg, 0, 0); } while (0)

/* ---- prng ---- */
#define SIGMA_RAND_MAX 0xFFFFFFFFU
extern u32 sigma_rand32(void);
extern u64 sigma_rand64(void);

#endif /* SIGMA_KERNEL_TYPES_H */
