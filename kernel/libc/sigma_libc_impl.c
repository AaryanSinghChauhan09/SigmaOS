/**
 * =========================================================================
 * Σ SIGMAOS SOVEREIGN LIBC — FULL IMPLEMENTATION
 * =========================================================================
 * A minimal, zero-dependency C runtime for SigmaOS kernel and userland.
 * Replaces glibc / musl for kernel-space code entirely.
 *
 * Provided primitives:
 *   Memory  : sigma_malloc, sigma_free, sigma_memset, sigma_memcpy
 *   Strings : sigma_strlen, sigma_strcmp, sigma_strncmp, sigma_strcpy,
 *             sigma_strncpy, sigma_strcat, sigma_strchr, sigma_strstr
 *   I/O     : sys_print (varargs, syscall-backed)
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

#include "../include/sigma_kernel_types.h"

// =========================================================================
// SECTION 1: MEMORY MANAGEMENT
// =========================================================================

/**
 * Sovereign bump allocator — a fixed-size heap carved out at link time.
 * For production: replace with a slab/buddy allocator backed by
 * sigma_page_alloc() from the kernel memory subsystem.
 */
#define SIGMA_HEAP_SIZE (1024 * 1024 * 2)   /* 2 MiB sovereign heap */

static sigma_u8  g_heap[SIGMA_HEAP_SIZE];
static sigma_u32 g_heap_offset = 0;

/* Block header stored before every allocation */
typedef struct sigma_block {
    sigma_u32        magic;   /* 0xSIGMA5A5 — detect corruption */
    sigma_size_t     size;    /* usable bytes requested */
    sigma_bool       free;
    struct sigma_block* next;
} sigma_block_t;

#define SIGMA_HEAP_MAGIC  0x51A4A5A5U
#define SIGMA_BLOCK_HDR   sizeof(sigma_block_t)

static sigma_block_t* g_heap_head = SIGMA_NULL;

/**
 * sigma_malloc — sovereign heap allocator (first-fit free list).
 */
void* sigma_malloc(sigma_size_t size) {
    if (size == 0) return SIGMA_NULL;

    /* Align to 8 bytes */
    size = (size + 7) & ~7UL;

    /* Walk the free list for a reusable block */
    sigma_block_t* blk = g_heap_head;
    while (blk) {
        if (blk->free && blk->size >= size) {
            blk->free = SIGMA_FALSE;
            return (sigma_u8*)blk + SIGMA_BLOCK_HDR;
        }
        blk = blk->next;
    }

    /* Carve a new block from the bump region */
    sigma_size_t total = SIGMA_BLOCK_HDR + size;
    if (g_heap_offset + total > SIGMA_HEAP_SIZE) {
        return SIGMA_NULL; /* Out of sovereign heap memory */
    }

    blk = (sigma_block_t*)(g_heap + g_heap_offset);
    blk->magic = SIGMA_HEAP_MAGIC;
    blk->size  = size;
    blk->free  = SIGMA_FALSE;
    blk->next  = g_heap_head;
    g_heap_head = blk;

    g_heap_offset += total;
    return (sigma_u8*)blk + SIGMA_BLOCK_HDR;
}

/**
 * sigma_free — mark a block as reusable. Does not zero memory (use
 * sigma_memset explicitly before freeing sensitive data).
 */
void sigma_free(void* ptr) {
    if (!ptr) return;
    sigma_block_t* blk = (sigma_block_t*)((sigma_u8*)ptr - SIGMA_BLOCK_HDR);
    if (blk->magic != SIGMA_HEAP_MAGIC) {
        /* Heap corruption detected — signal ZEN_MEM_CORRUPT in production */
        return;
    }
    blk->free = SIGMA_TRUE;
}

/**
 * sigma_memset — fill memory with a byte value.
 * The compiler may replace this with a SIMD intrinsic in O2+ builds.
 */
void* sigma_memset(void* dst, sigma_u8 val, sigma_size_t n) {
    sigma_u8* d = (sigma_u8*)dst;
    while (n--) *d++ = val;
    return dst;
}

/**
 * sigma_memcpy — copy non-overlapping memory regions.
 */
void* sigma_memcpy(void* dst, const void* src, sigma_size_t n) {
    sigma_u8*       d = (sigma_u8*)dst;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
    return dst;
}

/**
 * sigma_memmove — copy potentially overlapping memory regions.
 */
void* sigma_memmove(void* dst, const void* src, sigma_size_t n) {
    sigma_u8*       d = (sigma_u8*)dst;
    const sigma_u8* s = (const sigma_u8*)src;
    if (d < s) {
        while (n--) *d++ = *s++;
    } else {
        d += n; s += n;
        while (n--) *--d = *--s;
    }
    return dst;
}

/**
 * posix_memalign — aligned allocation (required by some C++ placement new paths).
 */
int posix_memalign(void** memptr, sigma_size_t alignment, sigma_size_t size) {
    if (!memptr || alignment == 0 || (alignment & (alignment - 1))) return 1;
    sigma_size_t padded = size + alignment;
    void* raw = sigma_malloc(padded);
    if (!raw) return 2;
    sigma_size_t addr = (sigma_size_t)raw;
    sigma_size_t aligned = (addr + alignment - 1) & ~(alignment - 1);
    *memptr = (void*)aligned;
    return 0;
}

// =========================================================================
// SECTION 2: STRING OPERATIONS
// =========================================================================

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
    while (n-- && *a && *b) {
        if (*a != *b) return (unsigned char)*a - (unsigned char)*b;
        a++; b++;
    }
    return n == (sigma_size_t)-1 ? 0 : (unsigned char)*a - (unsigned char)*b;
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

// =========================================================================
// SECTION 3: NUMBER CONVERSION
// =========================================================================

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
 * sigma_itoa — convert integer to decimal ASCII string.
 * buf must be at least 12 bytes.
 */
char* sigma_itoa(sigma_i32 val, char* buf, sigma_u32 base) {
    static const char digits[] = "0123456789abcdef";
    if (base < 2 || base > 16) { buf[0] = '\0'; return buf; }

    char tmp[32];
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

// =========================================================================
// SECTION 4: OUTPUT — sys_print (varargs, backed by write syscall)
// =========================================================================

/**
 * sigma_vsnprint — sovereign vsnprintf substitute (no FILE*, no glibc).
 * Supports: %s %d %u %x %c %%.
 */
static sigma_size_t sigma_vsnprint(char* out, sigma_size_t max,
                                    const char* fmt, __builtin_va_list args) {
    sigma_size_t pos = 0;
#define EMIT(c) do { if (pos + 1 < max) { out[pos] = (c); } pos++; } while(0)

    while (*fmt) {
        if (*fmt != '%') { EMIT(*fmt++); continue; }
        fmt++; /* skip '%' */

        switch (*fmt++) {
        case 's': {
            const char* s = __builtin_va_arg(args, const char*);
            if (!s) s = "(null)";
            while (*s) EMIT(*s++);
            break;
        }
        case 'd': {
            sigma_i32 v = __builtin_va_arg(args, int);
            char tmp[12]; sigma_itoa(v, tmp, 10);
            for (char* t = tmp; *t; t++) EMIT(*t);
            break;
        }
        case 'u': {
            sigma_u32 v = __builtin_va_arg(args, unsigned int);
            char tmp[12]; sigma_itoa((sigma_i32)v, tmp, 10);
            for (char* t = tmp; *t; t++) EMIT(*t);
            break;
        }
        case 'x': {
            sigma_u32 v = __builtin_va_arg(args, unsigned int);
            char tmp[12]; sigma_itoa((sigma_i32)v, tmp, 16);
            for (char* t = tmp; *t; t++) EMIT(*t);
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
 * sys_print — sovereign console output via raw write syscall.
 * Backed by SIGMA_SYSCALL_WRITE (syscall number 4) on x86_64.
 * On ARM64: same calling convention, different svc number.
 */
void sys_print(const char* fmt, ...) {
    char buf[1024];
    __builtin_va_list args;
    __builtin_va_start(args, fmt);
    sigma_size_t len = sigma_vsnprint(buf, sizeof(buf), fmt, args);
    __builtin_va_end(args);

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

// =========================================================================
// SECTION 5: IPC STUB
// =========================================================================

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
