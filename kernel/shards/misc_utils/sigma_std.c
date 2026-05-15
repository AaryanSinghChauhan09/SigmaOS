#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SIGMA-STANDARD-SHARD (v2.0 - SOVEREIGN SILICON DIRECT)
 * =============================================================================
 * Algorithm: Freestanding Silicon Logic (FSL) â€ Zero external dependency
 * Principles:
 *   - Absolute reduction of predefined library functions.
 *   - Direct x86_64 RDTSC / MSR / I/O register orchestration.
 *   - Bit-perfect memory, string, arithmetic, and I/O management.
 *   - OOP-style: each function is a reusable silicon-native primitive.
 * =============================================================================
 */

#include "../../../include/core/sigma_kernel_types.h"

/* =========================================================================
 * SILICON-DIRECT SYSCALL BRIDGE (x86_64)
 * Used only when running on Linux host for testing (not in bare-metal kernel)
 * ========================================================================= */
static inline sigma_i64 sigma_syscall(sigma_i64 n, sigma_i64 a1, sigma_i64 a2, sigma_i64 a3) {
    sigma_i64 ret;
    __asm__ __volatile__ (
        "syscall"
        : "=a"(ret)
        : "a"(n), "D"(a1), "S"(a2), "d"(a3)
        : "rcx", "r11", "memory"
    );
    return ret;
}

/* =========================================================================
 * TIMESTAMP â€ x86_64 RDTSC-based nanosecond clock
 * Uses RDTSC + assumed 3GHz base (calibrated at boot in production)
 * ========================================================================= */
static sigma_u64 g_tsc_freq_mhz = 3000;  /* 3 GHz default â€ updated by PIT calibration */

void sigma_set_tsc_freq_mhz(sigma_u64 mhz) {
    g_tsc_freq_mhz = mhz;
}

/* Returns nanoseconds since boot */
sigma_u64 os_get_timestamp_ns(void) {
    sigma_u64 tsc = cpu_rdtsc();
    /* ns = tsc / (freq_mhz / 1000) = tsc * 1000 / freq_mhz */
    if (g_tsc_freq_mhz == 0) return tsc;
    return (tsc * 1000ULL) / g_tsc_freq_mhz;
}

/* Returns milliseconds since boot */
sigma_u64 os_get_timestamp_ms(void) {
    return os_get_timestamp_ns() / 1000000ULL;
}

/* =========================================================================
 * CORE MEMORY OPERATIONS (zero-dep, rep-string accelerated)
 * ========================================================================= */

void* sigma_memset32(void* s, sigma_u32 val, sigma_usize count) {
    sigma_u32* p = (sigma_u32*)s;
    while (count--) *p++ = val;
    return s;
}

/* Zero a buffer */
void sigma_bzero(void* s, sigma_usize n) {
    sigma_memset(s, 0, n);
}

/* Byte-by-byte compare â€ returns 0 if equal */
int sigma_memcmp(const void* a, const void* b, sigma_usize n) {
    const sigma_u8* p = (const sigma_u8*)a;
    const sigma_u8* q = (const sigma_u8*)b;
    sigma_usize i;
    for (i = 0; i < n; i++) {
        if (p[i] < q[i]) return -1;
        if (p[i] > q[i]) return  1;
    }
    return 0;
}

/* Safe bounded string copy â€ always SIGMA_NULL-terminates destination */
void sigma_strcpy_safe(char* dst, const char* src, sigma_usize max) {
    sigma_usize i;
    for (i = 0; i < max - 1 && src[i]; i++) dst[i] = src[i];
    dst[i] = '\0';
}

/* String comparison — returns 0 if equal
 * Guard prevents redefinition against sigma_kernel_types.h static inline version */
#ifndef SIGMA_STRCMP_DEFINED
#define SIGMA_STRCMP_DEFINED
int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return (unsigned char)*s1 - (unsigned char)*s2;
}
#endif

/* Case-insensitive compare */
int sigma_strcasecmp(const char* s1, const char* s2) {
    while (*s1 && *s2) {
        char c1 = (*s1 >= 'A' && *s1 <= 'Z') ? (*s1 + 32) : *s1;
        char c2 = (*s2 >= 'A' && *s2 <= 'Z') ? (*s2 + 32) : *s2;
        if (c1 != c2) return (unsigned char)c1 - (unsigned char)c2;
        s1++; s2++;
    }
    return (unsigned char)*s1 - (unsigned char)*s2;
}

/* String search â€ returns pointer to first occurrence of needle */
const char* sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return haystack;
    const char* h = haystack;
    while (*h) {
        const char* p = h, *n = needle;
        while (*p && *n && *p == *n) { p++; n++; }
        if (!*n) return h;
        h++;
    }
    return SIGMA_NULL;
}

/* Convert integer to decimal string (zero-dep itoa) */
sigma_usize sigma_itoa(sigma_i64 val, char* buf, sigma_usize buflen) {
    if (buflen == 0) return 0;
    char tmp[24];
    sigma_usize i = 0, j = 0;
    sigma_bool neg = SIGMA_FALSE;
    if (val < 0) { neg = SIGMA_TRUE; val = -val; }
    if (val == 0) { tmp[i++] = '0'; }
    while (val > 0) { tmp[i++] = '0' + (char)(val % 10); val /= 10; }
    if (neg && j < buflen - 1) buf[j++] = '-';
    while (i > 0 && j < buflen - 1) buf[j++] = tmp[--i];
    buf[j] = '\0';
    return j;
}

/* Convert unsigned 64-bit to hex string */
sigma_usize sigma_utohex(sigma_u64 val, char* buf, sigma_usize buflen) {
    static const char hx[] = "0123456789abcdef";
    if (buflen < 3) return 0;
    buf[0] = '0'; buf[1] = 'x';
    sigma_usize i = 2;
    sigma_bool lead = SIGMA_TRUE;
    int shift;
    for (shift = 60; shift >= 0 && i < buflen - 1; shift -= 4) {
        sigma_u8 nibble = (sigma_u8)((val >> shift) & 0xF);
        if (lead && nibble == 0) continue;
        lead = SIGMA_FALSE;
        buf[i++] = hx[nibble];
    }
    if (i == 2) buf[i++] = '0';  /* value was zero */
    buf[i] = '\0';
    return i;
}

/* Parse decimal string to integer */
sigma_i64 sigma_atoi64(const char* s) {
    sigma_i64 v = 0;
    sigma_bool neg = SIGMA_FALSE;
    if (!s) return 0;
    if (*s == '-') { neg = SIGMA_TRUE; s++; }
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return neg ? -v : v;
}

/* =========================================================================
 * HASH FUNCTIONS (zero-dep â€ no <openssl>, no <crypto>)
 * ========================================================================= */

/* FNV-1a 32-bit â€ forensic integrity hash */
sigma_u32 sigma_fnv1a_32(const sigma_u8* data, sigma_usize len) {
    sigma_u32 hash = 0x811c9dc5u;
    sigma_usize i;
    for (i = 0; i < len; i++) {
        hash ^= (sigma_u32)data[i];
        hash *= 0x01000193u;
    }
    return hash;
}

/* FNV-1a 64-bit â€ extended forensic hash */
sigma_u64 sigma_fnv1a_64(const sigma_u8* data, sigma_usize len) {
    sigma_u64 hash = 0xcbf29ce484222325ULL;
    sigma_usize i;
    for (i = 0; i < len; i++) {
        hash ^= (sigma_u64)data[i];
        hash *= 0x00000100000001b3ULL;
    }
    return hash;
}

/* DJB2 string hash â€ fast for command lookup */
sigma_u32 sigma_djb2(const char* s) {
    sigma_u32 hash = 5381u;
    while (*s) { hash = ((hash << 5) + hash) ^ (sigma_u32)(sigma_u8)*s++; }
    return hash;
}

/* =========================================================================
 * I/O PRIMITIVES (x86_64 port I/O â€ inline where possible)
 * ========================================================================= */

/* Short IOWAIT delay (used for PIC/PIT programming) */
void sigma_io_wait(void) {
    port_outb(0x80, 0);
}

/* Read 16-bit from port */
sigma_u16 port_inw(sigma_u16 port) {
    sigma_u16 v;
    __asm__ __volatile__("inw %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

/* Write 16-bit to port */
void port_outw_fn(sigma_u16 port, sigma_u16 val) {
    __asm__ __volatile__("outw %0, %1" :: "a"(val), "dN"(port));
}

/* Write 32-bit to port */
void port_outl(sigma_u16 port, sigma_u32 val) {
    __asm__ __volatile__("outl %0, %1" :: "a"(val), "dN"(port));
}

/* Read 32-bit from port */
sigma_u32 port_inl(sigma_u16 port) {
    sigma_u32 v;
    __asm__ __volatile__("inl %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

/* =========================================================================
 * MSR READ/WRITE (x86_64 â€ needed for SYSCALL, LAPIC)
 * ========================================================================= */

sigma_u64 sigma_rdmsr(sigma_u32 msr) {
    sigma_u32 lo, hi;
    __asm__ __volatile__("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
    return ((sigma_u64)hi << 32) | lo;
}

void sigma_wrmsr(sigma_u32 msr, sigma_u64 val) {
    sigma_u32 lo = (sigma_u32)val, hi = (sigma_u32)(val >> 32);
    __asm__ __volatile__("wrmsr" :: "c"(msr), "a"(lo), "d"(hi));
}

/* =========================================================================
 * CPUID wrapper (silicon discovery)
 * ========================================================================= */
typedef struct CPUIDResult {
    sigma_u32 eax, ebx, ecx, edx;
} CPUIDResult;

CPUIDResult sigma_cpuid(sigma_u32 leaf) {
    CPUIDResult r;
    __asm__ __volatile__(
        "cpuid"
        : "=a"(r.eax), "=b"(r.ebx), "=c"(r.ecx), "=d"(r.edx)
        : "a"(leaf)
    );
    return r;
}

/* =========================================================================
 * DIRECT CONSOLE OUTPUT (raw VGA / serial â€ no buffering)
 * ========================================================================= */

/* Write to serial COM1 (Linux testing path) */
void k_print_raw(const char* s) {
    sigma_syscall(1, 1, (sigma_i64)s, (sigma_i64)sigma_strlen(s));
}

/* =========================================================================
 * LIGHTWEIGHT RING BUFFER (generic, used by event busses)
 * ========================================================================= */
typedef struct RingBuffer {
    sigma_u8*  buf;
    sigma_u32  capacity;
    sigma_u32  head;
    sigma_u32  tail;
    sigma_u32  count;
} RingBuffer;

void rb_init(RingBuffer* rb, sigma_u8* storage, sigma_u32 cap) {
    rb->buf = storage; rb->capacity = cap;
    rb->head = rb->tail = rb->count = 0;
}

sigma_bool rb_push(RingBuffer* rb, sigma_u8 val) {
    if (rb->count >= rb->capacity) return SIGMA_FALSE;
    rb->buf[rb->tail] = val;
    rb->tail = (rb->tail + 1) % rb->capacity;
    rb->count++;
    return SIGMA_TRUE;
}

sigma_bool rb_pop(RingBuffer* rb, sigma_u8* out) {
    if (rb->count == 0) return SIGMA_FALSE;
    *out = rb->buf[rb->head];
    rb->head = (rb->head + 1) % rb->capacity;
    rb->count--;
    return SIGMA_TRUE;
}

/* =========================================================================
 * SPINLOCK (x86_64 CMPXCHG-based â€ proper SMP locking primitive)
 * ========================================================================= */
typedef volatile sigma_u32 spinlock_t;

static inline void spinlock_init(spinlock_t* l)    __attribute__((unused));
static inline void spinlock_acquire(spinlock_t* l) __attribute__((unused));
static inline void spinlock_release(spinlock_t* l) __attribute__((unused));

static inline void spinlock_init(spinlock_t* l)    { *l = 0; }
static inline void spinlock_acquire(spinlock_t* l) {
    while (__sync_lock_test_and_set(l, 1)) { cpu_pause(); }
}
static inline void spinlock_release(spinlock_t* l) {
    __sync_lock_release(l);
}

/* =========================================================================
 * ENTROPY GENERATOR (RDTSC-seeded XorShift64 â€ zero-dep PRNG)
 * ========================================================================= */
static sigma_u64 g_entropy_state = 0xDEADC0DEULL;

sigma_u64 sigma_rand64(void) {
    sigma_u64 x = g_entropy_state ^ cpu_rdtsc();
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    g_entropy_state = x;
    return x;
}

sigma_u32 sigma_rand32(void) {
    return (sigma_u32)(sigma_rand64() >> 32);
}

/* =========================================================================
 * ATOMIC OPERATIONS (x86_64 LOCK prefix)
 * ========================================================================= */
static inline sigma_u32   sigma_atomic_add(volatile sigma_u32* ptr, sigma_u32 val)                           __attribute__((unused));
static inline sigma_u32   sigma_atomic_sub(volatile sigma_u32* ptr, sigma_u32 val)                           __attribute__((unused));
static inline sigma_bool  sigma_atomic_cas(volatile sigma_u32* ptr, sigma_u32 expected, sigma_u32 desired)   __attribute__((unused));

static inline sigma_u32 sigma_atomic_add(volatile sigma_u32* ptr, sigma_u32 val) {
    return __sync_fetch_and_add(ptr, val);
}

static inline sigma_u32 sigma_atomic_sub(volatile sigma_u32* ptr, sigma_u32 val) {
    return __sync_fetch_and_sub(ptr, val);
}

static inline sigma_bool sigma_atomic_cas(volatile sigma_u32* ptr, sigma_u32 expected, sigma_u32 desired) {
    return __sync_bool_compare_and_swap(ptr, expected, desired) ? SIGMA_TRUE : SIGMA_FALSE;
}
