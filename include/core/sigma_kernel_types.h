/*
 * =============================================================================
 * SIGMAOS: SOVEREIGN KERNEL TYPES (v1.0 - C11 ZERO-DEPENDENCY)
 * =============================================================================
 * All kernel-internal types, constants, and primitive definitions.
 * Standard: C11 (ISO/IEC 9899:2011) - no external headers.
 * =============================================================================
 */

#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

/* ---- primitive types ---- */
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;
typedef signed char        sigma_i8;
typedef signed short       sigma_i16;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;
typedef unsigned long long sigma_usize;
typedef long long          sigma_isize;
typedef sigma_u64          sigma_paddr_t;   /* physical address */
typedef sigma_u64          sigma_vaddr_t;   /* virtual  address */
typedef int                sigma_bool;

#ifndef SIGMA_TRUE
#define SIGMA_TRUE   1
#endif
#ifndef SIGMA_FALSE
#define SIGMA_FALSE  0
#endif
#ifndef SIGMA_NULL
#ifdef __cplusplus
#define SIGMA_NULL  nullptr
#else
#define SIGMA_NULL  ((void*)0)
#endif
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

#ifndef SIGMA_STATUS_DEFINED
#define SIGMA_STATUS_DEFINED
typedef sigma_i32 sigma_status;
#endif

/* ---- intrinsics ---- */
static inline void cpu_halt(void)  { __asm__ __volatile__("cli; hlt"); }
static inline void cpu_pause(void) { __asm__ __volatile__("pause" ::: "memory"); }
static inline void cpu_fence(void) { __asm__ __volatile__("mfence" ::: "memory"); }
static inline void cpu_sti(void)   { __asm__ __volatile__("sti"); }
static inline void cpu_cli(void)   { __asm__ __volatile__("cli"); }

#ifndef SIGMA_HOST
static inline sigma_u64 cpu_rdtsc(void) {
    sigma_u64 v;
    __asm__ __volatile__(
        "rdtsc\n\t shl $32,%%rdx\n\t or %%rdx,%%rax"
        : "=a"(v) :: "rdx");
    return v;
}
#else
/* Mock version for host-mode tests */
sigma_u64 cpu_rdtsc(void);
#endif


static inline sigma_u64 cpu_read_cr3(void) {
    sigma_u64 v;
    __asm__ __volatile__("mov %%cr3, %0" : "=r"(v));
    return v;
}

static inline void cpu_write_cr3(sigma_u64 v) {
    __asm__ __volatile__("mov %0, %%cr3" :: "r"(v) : "memory");
}

static inline void cpu_invlpg(sigma_vaddr_t va) {
    __asm__ __volatile__("invlpg (%0)" :: "r"(va) : "memory");
}

static inline sigma_u8 port_inb(sigma_u16 port) {
    sigma_u8 v;
    __asm__ __volatile__("inb %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

static inline void port_outb(sigma_u16 port, sigma_u8 val) {
    __asm__ __volatile__("outb %0, %1" :: "a"(val), "dN"(port));
}

static inline void port_outw(sigma_u16 port, sigma_u16 val) {
    __asm__ __volatile__("outw %0, %1" :: "a"(val), "dN"(port));
}

/* =========================================================================
 * SOVEREIGN-ASM: Silicon-Direct Memory Orchestration (Unified with SovereignLibC)
 * ========================================================================= */

static inline void* sigma_memcpy(void* dst, const void* src, sigma_usize n) {
    void* d = dst;
    __asm__ __volatile__ (
        "rep movsb"
        : "+D"(dst), "+S"(src), "+c"(n)
        : : "memory"
    );
    return d;
}

static inline void* sigma_memset(void* s, int c, sigma_usize n) {
    void* d = s;
    __asm__ __volatile__ (
        "rep stosb"
        : "+D"(s), "+c"(n)
        : "a"((sigma_u8)c)
        : "memory"
    );
    return d;
}

static inline sigma_usize sigma_strlen(const char* s) {
    sigma_usize len = 0;
    while (s && s[len]) len++;
    return len;
}

#ifndef SIGMA_STRCMP_DEFINED
#define SIGMA_STRCMP_DEFINED
static inline int sigma_strcmp(const char* s1, const char* s2) {
    if (!s1 || !s2) return 0;
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(sigma_u8*)s1 - *(sigma_u8*)s2;
}
#endif

static inline void sigma_strncpy(char* dest, const char* src, sigma_usize n) {
    if (!dest || !src) return;
    sigma_usize i;
    for (i = 0; i < n - 1 && src[i] != '\0'; i++) dest[i] = src[i];
    dest[i] = '\0';
}

/* =========================================================================
 * SOVEREIGN-FAULT: Industrial Recovery & Assertion
 * ========================================================================= */
void sigma_panic(const char* msg, sigma_u64 rip, sigma_u64 rsp);

#define SIGMA_ASSERT(cond, msg) \
    do { if (!(cond)) sigma_panic(msg, 0, 0); } while (0)

#ifdef __cplusplus
extern "C" {
#endif

/* ---- Common Kernel Function Declarations ---- */
void kprintf(const char* fmt, ...);
sigma_u32  cpu_get_id(void);

void vga_clear(sigma_u8 color);
void vga_putc_at(sigma_u8 x, sigma_u8 y, char c, sigma_u8 color);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_KERNEL_TYPES_H */
