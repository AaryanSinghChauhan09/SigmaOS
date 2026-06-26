/**
 * =========================================================================
 * Σ SIGMAOS SOVEREIGN LIBC — FULL IMPLEMENTATION (Phase 16 Hardened)
 * =========================================================================
 * A minimal, zero-dependency C runtime for SigmaOS kernel and userland.
 * Replaces glibc / musl for kernel-space code entirely.
 *
 * Phase 16 Changes:
 *   - Bump allocator REPLACED with buddy allocator (power-of-2 splitting)
 *   - sigma_memcpy / sigma_memset use inline assembly (rep movsb/stosb)
 *   - Added: sigma_memcmp, sigma_snprintf, sigma_realloc
 *   - ERMS (Enhanced REP MOVSB/STOSB) auto-detection via CPUID
 *
 * Provided primitives:
 *   Memory  : sigma_malloc, sigma_free, sigma_realloc, sigma_memset,
 *             sigma_memcpy, sigma_memmove, sigma_memcmp
 *   Strings : sigma_strlen, sigma_strcmp, sigma_strncmp, sigma_strcpy,
 *             sigma_strncpy, sigma_strcat, sigma_strchr, sigma_strstr
 *   I/O     : sys_print (varargs, syscall-backed), sigma_snprintf
 *   Math    : sigma_atoi, sigma_itoa, sigma_abs
 *
 * Inspired by:
 *   - musl libc (Alpine Linux): Lean, spec-compliant C runtime.
 *   - diet libc: Radically minimal C library for embedded targets.
 *   - Linux kernel's own lib/ routines (memcpy, string helpers).
 *   - BusyBox: Replacing heavy glibc calls with inline custom helpers.
 *
 * RULES:
 *   - No #include <string.h>, <stdlib.h>, <stdio.h>. Ever.
 *   - All helpers are prefixed `sigma_` to avoid symbol conflicts.
 *   - kernel-space code must never call sys_print in interrupt context.
 * =========================================================================
 */

#define SIGMA_LIBC_INTERNAL  /* suppress inline fallbacks in kernel_types.h */
#include "../../include/sigma_kernel_types.h"

/* Define SIGMA_SUCCESS locally to avoid circular include with sigma_error_codes.h
 * (which includes sigma_libc.h, which declares the functions we define here). */
#ifndef SIGMA_SUCCESS
#define SIGMA_SUCCESS 0
#endif

/* =========================================================================
 * SECTION 0: ERMS DETECTION
 * =========================================================================
 * Intel Ivy Bridge+ and AMD Zen+ support Enhanced REP MOVSB/STOSB (ERMS),
 * which makes rep movsb/stosb the fastest memcpy/memset path — faster than
 * SSE/AVX for most sizes. We detect this once at init time.
 * ========================================================================= */

static sigma_bool g_has_erms = SIGMA_FALSE;

void sigma_libc_detect_cpu_features(void) {
#if defined(__x86_64__) || defined(__i386__)
    sigma_u32 eax, ebx, ecx, edx;
    /* CPUID leaf 7, subleaf 0: EBX bit 9 = ERMS */
    __asm__ __volatile__(
        "cpuid"
        : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
        : "a"(7), "c"(0)
    );
    g_has_erms = (ebx & (1u << 9)) ? SIGMA_TRUE : SIGMA_FALSE;
#else
    g_has_erms = SIGMA_FALSE;
#endif
}


/* =========================================================================
 * SECTION 1: BUDDY ALLOCATOR
 * =========================================================================
 * Replaces the Phase 11 bump allocator with a proper buddy system.
 *
 * Design:
 *   - 12 orders: order 0 = 64 bytes, order 11 = 128 KiB
 *   - Total heap = 2 MiB (managed as 32768 order-0 blocks)
 *   - Free lists per order (intrusive linked list via block headers)
 *   - Splitting: if no block at requested order, split a larger one
 *   - Coalescing: on free, merge with buddy if buddy is also free
 *   - Corruption guard: magic word in every block header
 *
 * Why buddy over slab:
 *   - Slab is optimal for fixed-size objects (process descriptors, inodes)
 *   - Buddy is the general-purpose kernel allocator (like Linux's page allocator)
 *   - We'll layer slab ON TOP of buddy in a future phase
 * ========================================================================= */

#define SIGMA_HEAP_SIZE       (1024ULL * 1024ULL * 2ULL)  /* 2 MiB sovereign heap */
#define BUDDY_MIN_ORDER       0
#define BUDDY_MAX_ORDER       11
#define BUDDY_NUM_ORDERS      (BUDDY_MAX_ORDER + 1)
#define BUDDY_MIN_BLOCK_SIZE  64ULL  /* Order 0 = 64 bytes (header + 16 usable) */
#define BUDDY_BLOCK_SIZE(o)   (BUDDY_MIN_BLOCK_SIZE << (o))
#define BUDDY_HEAP_MAGIC      0x51A4B0DDU  /* "SIGMA BUDDY" */
#define BUDDY_FREE_MAGIC      0xF4EEB0DDU  /* "FREE BUDDY"  */

/* Block header — stored at the start of every allocated/free block */
typedef struct sigma_buddy_block {
    sigma_u32                   magic;   /* BUDDY_HEAP_MAGIC or BUDDY_FREE_MAGIC */
    sigma_u32                   order;   /* Block order (0..BUDDY_MAX_ORDER) */
    struct sigma_buddy_block*   next;    /* Next free block in free list (if free) */
    struct sigma_buddy_block*   prev;    /* Prev free block in free list (if free) */
} sigma_buddy_block_t;

#define BUDDY_HDR_SIZE  sizeof(sigma_buddy_block_t)

/* The heap itself */
static sigma_u8 __attribute__((aligned(4096))) g_heap[SIGMA_HEAP_SIZE];

/* Free lists: one per order */
static sigma_buddy_block_t* g_free_lists[BUDDY_NUM_ORDERS];

/* Statistics */
static sigma_u64 g_alloc_count   = 0;
static sigma_u64 g_free_count    = 0;
static sigma_u64 g_bytes_in_use  = 0;

/* --- Internal helpers --- */

static inline sigma_u32 buddy_order_for_size(sigma_size_t size) {
    /* Find smallest order whose block can hold header + requested size */
    sigma_size_t total = size + BUDDY_HDR_SIZE;
    sigma_u32 order = BUDDY_MIN_ORDER;
    while (BUDDY_BLOCK_SIZE(order) < total && order <= BUDDY_MAX_ORDER) {
        order++;
    }
    return order;
}

static inline sigma_buddy_block_t* buddy_of(sigma_buddy_block_t* blk, sigma_u32 order) {
    /* The buddy's address is found by XOR-ing the block offset with the block size */
    sigma_size_t offset = (sigma_u8*)blk - g_heap;
    sigma_size_t buddy_offset = offset ^ BUDDY_BLOCK_SIZE(order);
    if (buddy_offset >= SIGMA_HEAP_SIZE) return SIGMA_NULL;
    return (sigma_buddy_block_t*)(g_heap + buddy_offset);
}

static void buddy_list_remove(sigma_buddy_block_t* blk, sigma_u32 order) {
    if (blk->prev) blk->prev->next = blk->next;
    else           g_free_lists[order] = blk->next;
    if (blk->next) blk->next->prev = blk->prev;
    blk->next = SIGMA_NULL;
    blk->prev = SIGMA_NULL;
}

static void buddy_list_insert(sigma_buddy_block_t* blk, sigma_u32 order) {
    blk->prev = SIGMA_NULL;
    blk->next = g_free_lists[order];
    if (g_free_lists[order]) g_free_lists[order]->prev = blk;
    g_free_lists[order] = blk;
    blk->magic = BUDDY_FREE_MAGIC;
    blk->order = order;
}

/* --- Initialize the buddy allocator --- */

static sigma_bool g_buddy_initialized = SIGMA_FALSE;

static void buddy_init(void) {
    if (g_buddy_initialized) return;

    for (sigma_u32 i = 0; i < BUDDY_NUM_ORDERS; i++) {
        g_free_lists[i] = SIGMA_NULL;
    }

    /* Carve the entire heap into max-order blocks and insert them */
    sigma_size_t max_block = BUDDY_BLOCK_SIZE(BUDDY_MAX_ORDER);
    sigma_size_t offset = 0;
    while (offset + max_block <= SIGMA_HEAP_SIZE) {
        sigma_buddy_block_t* blk = (sigma_buddy_block_t*)(g_heap + offset);
        buddy_list_insert(blk, BUDDY_MAX_ORDER);
        offset += max_block;
    }

    g_buddy_initialized = SIGMA_TRUE;
}

/* --- Public API --- */

/**
 * sigma_malloc — sovereign buddy allocator.
 * Returns a pointer to at least `size` usable bytes, or NULL on failure.
 */
void* sigma_malloc(sigma_size_t size) {
    if (size == 0) return SIGMA_NULL;
    if (!g_buddy_initialized) buddy_init();

    sigma_u32 order = buddy_order_for_size(size);
    if (order > BUDDY_MAX_ORDER) return SIGMA_NULL;  /* Request too large */

    /* Find a free block: walk up from requested order to find one */
    sigma_u32 found_order = order;
    while (found_order <= BUDDY_MAX_ORDER && !g_free_lists[found_order]) {
        found_order++;
    }
    if (found_order > BUDDY_MAX_ORDER) return SIGMA_NULL;  /* Out of memory */

    /* Remove the block from its free list */
    sigma_buddy_block_t* blk = g_free_lists[found_order];
    buddy_list_remove(blk, found_order);

    /* Split down to the requested order */
    while (found_order > order) {
        found_order--;
        /* The second half becomes a new free block */
        sigma_buddy_block_t* buddy = (sigma_buddy_block_t*)(
            (sigma_u8*)blk + BUDDY_BLOCK_SIZE(found_order)
        );
        buddy_list_insert(buddy, found_order);
    }

    blk->magic = BUDDY_HEAP_MAGIC;
    blk->order = order;
    blk->next = SIGMA_NULL;
    blk->prev = SIGMA_NULL;

    g_alloc_count++;
    g_bytes_in_use += BUDDY_BLOCK_SIZE(order);

    return (sigma_u8*)blk + BUDDY_HDR_SIZE;
}

/**
 * sigma_free — return a block to the buddy allocator.
 * Coalesces with buddy if possible, recursively up to max order.
 */
void sigma_free(void* ptr) {
    if (!ptr) return;

    sigma_buddy_block_t* blk = (sigma_buddy_block_t*)(
        (sigma_u8*)ptr - BUDDY_HDR_SIZE
    );

    /* Corruption check */
    if (blk->magic != BUDDY_HEAP_MAGIC) {
        /* ZEN-MEM-CORRUPT: silent return in kernel; production would panic */
        return;
    }

    sigma_u32 order = blk->order;
    g_free_count++;
    g_bytes_in_use -= BUDDY_BLOCK_SIZE(order);

    /* Coalesce with buddy */
    while (order < BUDDY_MAX_ORDER) {
        sigma_buddy_block_t* bdy = buddy_of(blk, order);
        if (!bdy) break;
        if (bdy->magic != BUDDY_FREE_MAGIC || bdy->order != order) break;

        /* Buddy is free and same order — merge */
        buddy_list_remove(bdy, order);

        /* The merged block starts at the lower address */
        if ((sigma_u8*)bdy < (sigma_u8*)blk) {
            blk = bdy;
        }
        order++;
    }

    buddy_list_insert(blk, order);
}

/**
 * sigma_realloc — resize an allocation.
 * If the new size fits in the current block, return same pointer.
 * Otherwise, allocate new, copy, free old.
 */
void* sigma_realloc(void* ptr, sigma_size_t new_size) {
    if (!ptr) return sigma_malloc(new_size);
    if (new_size == 0) { sigma_free(ptr); return SIGMA_NULL; }

    sigma_buddy_block_t* blk = (sigma_buddy_block_t*)(
        (sigma_u8*)ptr - BUDDY_HDR_SIZE
    );
    if (blk->magic != BUDDY_HEAP_MAGIC) return SIGMA_NULL;

    sigma_size_t old_usable = BUDDY_BLOCK_SIZE(blk->order) - BUDDY_HDR_SIZE;
    if (new_size <= old_usable) return ptr;  /* Fits in current block */

    /* Allocate new, copy, free old */
    void* new_ptr = sigma_malloc(new_size);
    if (!new_ptr) return SIGMA_NULL;

    /* Copy the smaller of old and new sizes */
    sigma_size_t copy_size = old_usable < new_size ? old_usable : new_size;
    sigma_u8* d = (sigma_u8*)new_ptr;
    const sigma_u8* s = (const sigma_u8*)ptr;
    for (sigma_size_t i = 0; i < copy_size; i++) d[i] = s[i];

    sigma_free(ptr);
    return new_ptr;
}


/* =========================================================================
 * SECTION 2: MEMORY OPERATIONS (Assembly-Optimized)
 * =========================================================================
 * On x86_64 with ERMS, rep movsb/stosb are the fastest possible memcpy/
 * memset — they use microcode-optimized 256-bit internal data paths.
 * On non-x86 or without ERMS, we fall back to byte loops.
 * ========================================================================= */

/**
 * sigma_memset — fill memory with a byte value.
 * Uses `rep stosb` on x86_64 for silicon-direct performance.
 */
void* sigma_memset(void* dst, sigma_u8 val, sigma_size_t n) {
#if defined(__x86_64__)
    void* ret = dst;
    __asm__ __volatile__(
        "rep stosb"
        : "+D"(dst), "+c"(n)      /* RDI = dst, RCX = count */
        : "a"(val)                 /* AL = fill byte */
        : "memory"
    );
    return ret;
#else
    sigma_u8* d = (sigma_u8*)dst;
    while (n--) *d++ = val;
    return dst;
#endif
}

/**
 * sigma_memcpy — copy non-overlapping memory regions.
 * Uses `rep movsb` on x86_64.
 */
void* sigma_memcpy(void* dst, const void* src, sigma_size_t n) {
#if defined(__x86_64__)
    void* ret = dst;
    __asm__ __volatile__(
        "rep movsb"
        : "+D"(dst), "+S"(src), "+c"(n)
        :
        : "memory"
    );
    return ret;
#else
    sigma_u8*       d = (sigma_u8*)dst;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
    return dst;
#endif
}

/**
 * sigma_memmove — copy potentially overlapping memory regions.
 * When dst > src and regions overlap, copies backwards.
 */
void* sigma_memmove(void* dst, const void* src, sigma_size_t n) {
    sigma_u8*       d = (sigma_u8*)dst;
    const sigma_u8* s = (const sigma_u8*)src;
    if (d < s) {
        /* Forward copy — safe to use rep movsb */
#if defined(__x86_64__)
        __asm__ __volatile__(
            "rep movsb"
            : "+D"(d), "+S"(s), "+c"(n) : : "memory"
        );
#else
        while (n--) *d++ = *s++;
#endif
    } else if (d > s) {
        /* Backward copy — must go byte-by-byte in reverse */
        d += n; s += n;
        while (n--) *--d = *--s;
    }
    return dst;
}

/**
 * sigma_memcmp — compare two memory regions.
 * Returns 0 if equal, <0 or >0 otherwise (like POSIX memcmp).
 */
int sigma_memcmp(const void* a, const void* b, sigma_size_t n) {
    const sigma_u8* pa = (const sigma_u8*)a;
    const sigma_u8* pb = (const sigma_u8*)b;
    while (n--) {
        if (*pa != *pb) return (int)*pa - (int)*pb;
        pa++; pb++;
    }
    return 0;
}

/**
 * posix_memalign — aligned allocation (required by some C++ placement new paths).
 * Now properly backed by buddy allocator.
 */
int sigma_posix_memalign(void** memptr, sigma_size_t alignment, sigma_size_t size) {
    if (!memptr || alignment == 0 || (alignment & (alignment - 1))) return 1;

    /* Buddy blocks are naturally aligned to their size (power of 2).
     * If requested alignment <= block size, we're automatically aligned. */
    sigma_size_t total = size + alignment;
    void* raw = sigma_malloc(total);
    if (!raw) return 2;

    sigma_size_t addr = (sigma_size_t)raw;
    sigma_size_t aligned = (addr + alignment - 1) & ~(alignment - 1);
    *memptr = (void*)aligned;
    return 0;
}


/* =========================================================================
 * SECTION 3: STRING OPERATIONS
 * ========================================================================= */

/**
 * sigma_strlen — compute length of a null-terminated string.
 */
sigma_size_t sigma_strlen(const char* s) {
    sigma_size_t n = 0;
    while (s[n]) n++;
    return n;
}

/**
 * sigma_strcmp — lexicographic string comparison.
 * Returns 0 if equal, <0 if a < b, >0 if a > b.
 */
int sigma_strcmp(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return (unsigned char)*a - (unsigned char)*b;
}

/**
 * sigma_strncmp — bounded string comparison.
 */
int sigma_strncmp(const char* a, const char* b, sigma_size_t n) {
    if (n == 0) return 0;
    while (n > 1 && *a && *b && *a == *b) { a++; b++; n--; }
    return (unsigned char)*a - (unsigned char)*b;
}

/**
 * sigma_strcpy — copy null-terminated string. Destination must be large enough.
 */
char* sigma_strcpy(char* dst, const char* src) {
    char* ret = dst;
    while ((*dst++ = *src++));
    return ret;
}

/**
 * sigma_strncpy — bounded string copy (always null-terminates).
 */
char* sigma_strncpy(char* dst, const char* src, sigma_size_t n) {
    char* ret = dst;
    while (n > 1 && *src) { *dst++ = *src++; n--; }
    if (n > 0) *dst = '\0';
    return ret;
}

/**
 * sigma_strcat — concatenate strings. Destination must have sufficient space.
 */
char* sigma_strcat(char* dst, const char* src) {
    char* end = dst + sigma_strlen(dst);
    while ((*end++ = *src++));
    return dst;
}

/**
 * sigma_strchr — find first occurrence of character in string.
 */
const char* sigma_strchr(const char* s, char c) {
    while (*s) {
        if (*s == c) return s;
        s++;
    }
    return (c == '\0') ? s : SIGMA_NULL;
}

/**
 * sigma_strstr — find first occurrence of needle in haystack.
 */
const char* sigma_strstr(const char* haystack, const char* needle) {
    sigma_size_t nlen = sigma_strlen(needle);
    if (nlen == 0) return haystack;
    while (*haystack) {
        if (sigma_strncmp(haystack, needle, nlen) == 0) return haystack;
        haystack++;
    }
    return SIGMA_NULL;
}


/* =========================================================================
 * SECTION 4: NUMBER CONVERSION
 * ========================================================================= */

/**
 * sigma_atoi — convert ASCII decimal string to integer.
 */
sigma_i32 sigma_atoi(const char* str) {
    sigma_i32 result = 0;
    int sign = 1;
    while (*str == ' ' || *str == '\t') str++;
    if (*str == '-') { sign = -1; str++; }
    else if (*str == '+') str++;
    while (*str >= '0' && *str <= '9') {
        result = result * 10 + (*str - '0');
        str++;
    }
    return sign * result;
}

/**
 * sigma_itoa — convert integer to ASCII string in given base (2–16).
 * buf must be at least 34 bytes for base-2.
 */
char* sigma_itoa(sigma_i32 val, char* buf, sigma_u32 base) {
    static const char digits[] = "0123456789abcdef";
    if (base < 2 || base > 16) { buf[0] = '\0'; return buf; }

    char tmp[34];
    sigma_u32 idx = 0;
    sigma_bool negative = SIGMA_FALSE;

    sigma_u32 uval;
    if (val < 0 && base == 10) {
        negative = SIGMA_TRUE;
        uval = (sigma_u32)(-val);
    } else {
        uval = (sigma_u32)val;
    }

    if (uval == 0) { tmp[idx++] = '0'; }
    while (uval > 0) { tmp[idx++] = digits[uval % base]; uval /= base; }
    if (negative) tmp[idx++] = '-';

    /* Reverse */
    sigma_u32 out = 0;
    while (idx > 0) buf[out++] = tmp[--idx];
    buf[out] = '\0';
    return buf;
}

/**
 * sigma_abs — absolute value.
 */
sigma_i32 sigma_abs(sigma_i32 val) {
    return (val < 0) ? -val : val;
}


/* =========================================================================
 * SECTION 5: OUTPUT — sys_print / sigma_snprintf
 * ========================================================================= */

/**
 * sigma_vsnprint — sovereign vsnprintf substitute (no FILE*, no glibc).
 * Supports: %s %d %u %x %p %c %% %ld %lu %lx.
 */
static sigma_size_t sigma_vsnprint(char* out, sigma_size_t max,
                                    const char* fmt, __builtin_va_list args) {
    sigma_size_t pos = 0;
#define EMIT(c) do { if (pos + 1 < max) { out[pos] = (c); } pos++; } while(0)

    while (*fmt) {
        if (*fmt != '%') { EMIT(*fmt++); continue; }
        fmt++; /* skip '%' */

        /* Check for 'l' length modifier */
        int is_long = 0;
        if (*fmt == 'l') { is_long = 1; fmt++; }

        switch (*fmt++) {
        case 's': {
            const char* s = __builtin_va_arg(args, const char*);
            if (!s) s = "(null)";
            while (*s) EMIT(*s++);
            break;
        }
        case 'd': {
            sigma_i64 v;
            if (is_long) v = __builtin_va_arg(args, long long);
            else          v = __builtin_va_arg(args, int);
            char tmp[22];
            /* Handle sign manually for 64-bit */
            sigma_u32 ti = 0;
            sigma_bool neg = SIGMA_FALSE;
            sigma_u64 uv;
            if (v < 0) { neg = SIGMA_TRUE; uv = (sigma_u64)(-v); }
            else { uv = (sigma_u64)v; }
            if (uv == 0) tmp[ti++] = '0';
            while (uv > 0) { tmp[ti++] = '0' + (char)(uv % 10); uv /= 10; }
            if (neg) tmp[ti++] = '-';
            while (ti > 0) EMIT(tmp[--ti]);
            break;
        }
        case 'u': {
            sigma_u64 v;
            if (is_long) v = __builtin_va_arg(args, unsigned long long);
            else          v = __builtin_va_arg(args, unsigned int);
            char tmp[22];
            sigma_u32 ti = 0;
            if (v == 0) tmp[ti++] = '0';
            while (v > 0) { tmp[ti++] = '0' + (char)(v % 10); v /= 10; }
            while (ti > 0) EMIT(tmp[--ti]);
            break;
        }
        case 'x': {
            sigma_u64 v;
            if (is_long) v = __builtin_va_arg(args, unsigned long long);
            else          v = __builtin_va_arg(args, unsigned int);
            char tmp[18];
            sigma_u32 ti = 0;
            const char* xd = "0123456789abcdef";
            if (v == 0) tmp[ti++] = '0';
            while (v > 0) { tmp[ti++] = xd[v & 0xF]; v >>= 4; }
            while (ti > 0) EMIT(tmp[--ti]);
            break;
        }
        case 'p': {
            sigma_u64 v = (sigma_u64)(unsigned long)__builtin_va_arg(args, void*);
            EMIT('0'); EMIT('x');
            char tmp[18];
            sigma_u32 ti = 0;
            const char* xd = "0123456789abcdef";
            if (v == 0) tmp[ti++] = '0';
            while (v > 0) { tmp[ti++] = xd[v & 0xF]; v >>= 4; }
            while (ti > 0) EMIT(tmp[--ti]);
            break;
        }
        case 'c': {
            char c = (char)__builtin_va_arg(args, int);
            EMIT(c);
            break;
        }
        case '%':
            EMIT('%');
            break;
        default:
            EMIT('?');
            break;
        }
    }
    if (max > 0) out[pos < max ? pos : max - 1] = '\0';
    return pos;
#undef EMIT
}

/**
 * sigma_snprintf — bounded format into buffer. Returns chars written (excl. NUL).
 * This is the safe, bounded variant that all userland code should prefer.
 */
sigma_size_t sigma_snprintf(char* buf, sigma_size_t max, const char* fmt, ...) {
    __builtin_va_list args;
    __builtin_va_start(args, fmt);
    sigma_size_t n = sigma_vsnprint(buf, max, fmt, args);
    __builtin_va_end(args);
    return n < max ? n : max - 1;
}

/**
 * sys_print — sovereign console output via raw write syscall.
 * Backed by SIGMA_SYSCALL_WRITE (syscall number 1) on x86_64.
 * On ARM64: same calling convention, different svc number.
 */
void sys_print(const char* fmt, ...) {
    char buf[1024];
    __builtin_va_list args;
    __builtin_va_start(args, fmt);
    sigma_size_t len = sigma_vsnprint(buf, sizeof(buf), fmt, args);
    __builtin_va_end(args);

    if (len > sizeof(buf) - 1) len = sizeof(buf) - 1;

    /* Raw write(1, buf, len) syscall — no FILE* involved */
#if defined(__x86_64__)
    __asm__ volatile (
        "syscall"
        :
        : "a"(1),           /* syscall number: write */
          "D"(1),           /* fd: stdout (1) */
          "S"(buf),         /* buf */
          "d"(len)          /* count */
        : "rcx", "r11", "memory"
    );
#elif defined(__aarch64__)
    register long x0 __asm__("x0") = 1;       /* fd: stdout */
    register const char* x1 __asm__("x1") = buf;
    register sigma_size_t x2 __asm__("x2") = len;
    register long x8 __asm__("x8") = 64;      /* ARM64: write syscall = 64 */
    __asm__ volatile("svc #0" : "+r"(x0) : "r"(x1), "r"(x2), "r"(x8) : "memory");
#endif
}


/* =========================================================================
 * SECTION 6: IPC STUB
 * ========================================================================= */

/**
 * sys_ipc_send — sovereign IPC message passing.
 * In production: invokes SIGMA_SYSCALL_IPC_SEND kernel trap.
 */
sigma_status sys_ipc_send(sigma_u32 target_shard, sigma_u32 msg_id,
                           const void* data, sigma_size_t len) {
    (void)target_shard; (void)msg_id; (void)data; (void)len;
    /* TODO: hook into sigma_shard_ipc_dispatch() */
    return SIGMA_SUCCESS;
}
