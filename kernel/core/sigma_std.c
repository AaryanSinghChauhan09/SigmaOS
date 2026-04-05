/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SIGMA-STANDARD-SHARD (v2.0 - SOVEREIGN SILICON DIRECT)
 * =============================================================================
 * Algorithm: Freestanding Silicon Logic (FSL) — Zero external dependency
 * Principles:
 *   - Absolute reduction of predefined library functions.
 *   - Direct x86_64 RDTSC / MSR / I/O register orchestration.
 *   - Bit-perfect memory, string, arithmetic, and I/O management.
 *   - OOP-style: each function is a reusable silicon-native primitive.
 * =============================================================================
 */

#include "../libc/SovereignLibC.h"

void* sigma_memset(void* s, int c, usize n);
void* sigma_memcpy(void* dest, const void* src, usize n);

/* =========================================================================
 * SILICON-DIRECT SYSCALL BRIDGE (x86_64)
 * Used only when running on Linux host for testing (not in bare-metal kernel)
 * ========================================================================= */
static inline i64 sigma_syscall(i64 n, i64 a1, i64 a2, i64 a3) {
    i64 ret;
    __asm__ __volatile__ (
        "syscall"
        : "=a"(ret)
        : "a"(n), "D"(a1), "S"(a2), "d"(a3)
        : "rcx", "r11", "memory"
    );
    return ret;
}

/* =========================================================================
 * TIMESTAMP — x86_64 RDTSC-based nanosecond clock
 * Uses RDTSC + assumed 3GHz base (calibrated at boot in production)
 * ========================================================================= */
static u64 g_tsc_freq_mhz = 3000;  /* 3 GHz default — updated by PIT calibration */

void sigma_set_tsc_freq_mhz(u64 mhz) {
    g_tsc_freq_mhz = mhz;
}

/* Returns nanoseconds since boot */
u64 os_get_timestamp_ns(void) {
    u64 tsc = cpu_rdtsc();
    /* ns = tsc / (freq_mhz / 1000) = tsc * 1000 / freq_mhz */
    if (g_tsc_freq_mhz == 0) return tsc;
    return (tsc * 1000ULL) / g_tsc_freq_mhz;
}

/* Returns milliseconds since boot */
u64 os_get_timestamp_ms(void) {
    return os_get_timestamp_ns() / 1000000ULL;
}

/* =========================================================================
 * CORE MEMORY OPERATIONS (zero-dep, rep-string accelerated)
 * ========================================================================= */

void* sigma_memset32(void* s, u32 val, usize count) {
    u32* p = (u32*)s;
    while (count--) *p++ = val;
    return s;
}

void* sigma_memset(void* s, int c, usize n) {
    return k_memset(s, c, n);
}

void* sigma_memcpy(void* dest, const void* src, usize n) {
    return k_memcpy(dest, src, n);
}

/* Zero a buffer */
void sigma_bzero(void* s, usize n) {
    sigma_memset(s, 0, n);
}

/* Byte-by-byte compare — returns 0 if equal */
int sigma_memcmp(const void* a, const void* b, usize n) {
    const u8* p = (const u8*)a;
    const u8* q = (const u8*)b;
    usize i;
    for (i = 0; i < n; i++) {
        if (p[i] < q[i]) return -1;
        if (p[i] > q[i]) return  1;
    }
    return 0;
}

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

/* Convert integer to decimal string (zero-dep itoa) */
usize sigma_itoa(i64 val, char* buf, usize buflen) {
    if (buflen == 0) return 0;
    char tmp[24];
    usize i = 0, j = 0;
    bool_t neg = FALSE;
    if (val < 0) { neg = TRUE; val = -val; }
    if (val == 0) { tmp[i++] = '0'; }
    while (val > 0) { tmp[i++] = '0' + (char)(val % 10); val /= 10; }
    if (neg && j < buflen - 1) buf[j++] = '-';
    while (i > 0 && j < buflen - 1) buf[j++] = tmp[--i];
    buf[j] = '\0';
    return j;
}

/* Convert unsigned 64-bit to hex string */
usize sigma_utohex(u64 val, char* buf, usize buflen) {
    static const char hx[] = "0123456789abcdef";
    if (buflen < 3) return 0;
    buf[0] = '0'; buf[1] = 'x';
    usize i = 2;
    bool_t lead = TRUE;
    int shift;
    for (shift = 60; shift >= 0 && i < buflen - 1; shift -= 4) {
        u8 nibble = (u8)((val >> shift) & 0xF);
        if (lead && nibble == 0) continue;
        lead = FALSE;
        buf[i++] = hx[nibble];
    }
    if (i == 2) buf[i++] = '0';  /* value was zero */
    buf[i] = '\0';
    return i;
}

/* Parse decimal string to integer */
i64 sigma_atoi64(const char* s) {
    i64 v = 0;
    bool_t neg = FALSE;
    if (!s) return 0;
    if (*s == '-') { neg = TRUE; s++; }
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return neg ? -v : v;
}

/* =========================================================================
 * HASH FUNCTIONS (zero-dep — no <openssl>, no <crypto>)
 * ========================================================================= */

/* FNV-1a 32-bit — forensic integrity hash */
u32 sigma_fnv1a_32(const u8* data, usize len) {
    u32 hash = 0x811c9dc5u;
    usize i;
    for (i = 0; i < len; i++) {
        hash ^= (u32)data[i];
        hash *= 0x01000193u;
    }
    return hash;
}

/* FNV-1a 64-bit — extended forensic hash */
u64 sigma_fnv1a_64(const u8* data, usize len) {
    u64 hash = 0xcbf29ce484222325ULL;
    usize i;
    for (i = 0; i < len; i++) {
        hash ^= (u64)data[i];
        hash *= 0x00000100000001b3ULL;
    }
    return hash;
}

/* DJB2 string hash — fast for command lookup */
u32 sigma_djb2(const char* s) {
    u32 hash = 5381u;
    while (*s) { hash = ((hash << 5) + hash) ^ (u32)(u8)*s++; }
    return hash;
}

/* =========================================================================
 * I/O PRIMITIVES (x86_64 port I/O — inline where possible)
 * ========================================================================= */

/* Short IOWAIT delay (used for PIC/PIT programming) */
void sigma_io_wait(void) {
    port_outb(0x80, 0);
}

/* Read 16-bit from port */
u16 port_inw(u16 port) {
    u16 v;
    __asm__ __volatile__("inw %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

/* Write 16-bit to port */
void port_outw_fn(u16 port, u16 val) {
    __asm__ __volatile__("outw %0, %1" :: "a"(val), "dN"(port));
}

/* Write 32-bit to port */
void port_outl(u16 port, u32 val) {
    __asm__ __volatile__("outl %0, %1" :: "a"(val), "dN"(port));
}

/* Read 32-bit from port */
u32 port_inl(u16 port) {
    u32 v;
    __asm__ __volatile__("inl %1, %0" : "=a"(v) : "dN"(port));
    return v;
}

/* =========================================================================
 * MSR READ/WRITE (x86_64 — needed for SYSCALL, LAPIC)
 * ========================================================================= */

u64 sigma_rdmsr(u32 msr) {
    u32 lo, hi;
    __asm__ __volatile__("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
    return ((u64)hi << 32) | lo;
}

void sigma_wrmsr(u32 msr, u64 val) {
    u32 lo = (u32)val, hi = (u32)(val >> 32);
    __asm__ __volatile__("wrmsr" :: "c"(msr), "a"(lo), "d"(hi));
}

/* =========================================================================
 * CPUID wrapper (silicon discovery)
 * ========================================================================= */
typedef struct CPUIDResult {
    u32 eax, ebx, ecx, edx;
} CPUIDResult;

CPUIDResult sigma_cpuid(u32 leaf) {
    CPUIDResult r;
    __asm__ __volatile__(
        "cpuid"
        : "=a"(r.eax), "=b"(r.ebx), "=c"(r.ecx), "=d"(r.edx)
        : "a"(leaf)
    );
    return r;
}

/* =========================================================================
 * DIRECT CONSOLE OUTPUT (raw VGA / serial — no buffering)
 * ========================================================================= */

/* Write to serial COM1 (Linux testing path) */
void k_print_raw(const char* s) {
    sigma_syscall(1, 1, (i64)(usize)s, (i64)sigma_strlen(s));
}

/* =========================================================================
 * FORMATTED OUTPUT (SOVEREIGN KPRINTF)
 * ========================================================================= */
typedef __builtin_va_list va_list;
#define va_start(v,l)   __builtin_va_start(v,l)
#define va_end(v)       __builtin_va_end(v)
#define va_arg(v,l)     __builtin_va_arg(v,l)
#define va_copy(d,s)    __builtin_va_copy(d,s)

extern void sigma_putchar(char c);

void sigma_kprintf(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    for (const char* p = fmt; *p != '\0'; p++) {
        if (*p != '%') {
            sigma_putchar(*p);
            continue;
        }

        p++; // move past %
        switch (*p) {
            case 's': {
                const char* s = va_arg(args, const char*);
                while (*s) sigma_putchar(*s++);
                break;
            }
            case 'd': {
                long d = va_arg(args, long);
                char buf[32];
                sigma_itoa(d, buf, sizeof(buf));
                const char* s = buf;
                while (*s) sigma_putchar(*s++);
                break;
            }
            case 'x': 
            case 'p': {
                unsigned long x = va_arg(args, unsigned long);
                char buf[32];
                sigma_utohex(x, buf, sizeof(buf));
                const char* s = buf;
                while (*s) sigma_putchar(*s++);
                break;
            }
            case '%': {
                sigma_putchar('%');
                break;
            }
            default: {
                sigma_putchar('%');
                sigma_putchar(*p);
                break;
            }
        }
    }
    va_end(args);
}

/* =========================================================================
 * LIGHTWEIGHT RING BUFFER (generic, used by event busses)
 * ========================================================================= */
typedef struct RingBuffer {
    u8*  buf;
    u32  capacity;
    u32  head;
    u32  tail;
    u32  count;
} RingBuffer;

void rb_init(RingBuffer* rb, u8* storage, u32 cap) {
    rb->buf = storage; rb->capacity = cap;
    rb->head = rb->tail = rb->count = 0;
}

bool_t rb_push(RingBuffer* rb, u8 val) {
    if (rb->count >= rb->capacity) return FALSE;
    rb->buf[rb->tail] = val;
    rb->tail = (rb->tail + 1) % rb->capacity;
    rb->count++;
    return TRUE;
}

bool_t rb_pop(RingBuffer* rb, u8* out) {
    if (rb->count == 0) return FALSE;
    *out = rb->buf[rb->head];
    rb->head = (rb->head + 1) % rb->capacity;
    rb->count--;
    return TRUE;
}

/* =========================================================================
 * ENTROPY GENERATOR (RDTSC-seeded XorShift64 — zero-dep PRNG)
 * ========================================================================= */
static u64 g_entropy_state = 0xDEADC0DE5164AULL;

u64 sigma_rand64(void) {
    u64 x = g_entropy_state ^ cpu_rdtsc();
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    g_entropy_state = x;
    return x;
}

u32 sigma_rand32(void) {
    return (u32)(sigma_rand64() >> 32);
}

/* =========================================================================
 * ATOMIC OPERATIONS (x86_64 LOCK prefix)
 * ========================================================================= */
static inline u32 sigma_atomic_add(volatile u32* ptr, u32 val) {
    return __sync_fetch_and_add(ptr, val);
}

static inline u32 sigma_atomic_sub(volatile u32* ptr, u32 val) {
    return __sync_fetch_and_sub(ptr, val);
}

static inline bool_t sigma_atomic_cas(volatile u32* ptr, u32 expected, u32 desired) {
    return __sync_bool_compare_and_swap(ptr, expected, desired) ? TRUE : FALSE;
}
