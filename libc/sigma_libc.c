#include "../include/SovereignLibC.h"
/*
 * =========================================================================
 * Î£ SIGMAOS ZENITH SUPREME: SOVEREIGN LIBC CORE (v20.0)
 * =========================================================================
 * Mission: Zero-dependency, bit-perfect logic. No external symbols.
 * Capability: String manipulation, formatted I/O, memory management.
 * =========================================================================
 */

#include "sigma_libc.h"
#include "../include/sigma_types.h"

// --- Syscall Wrappers (defined in SovereignLibC.asm) ---
extern sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count);

// --- libc utility functions ---

sigma_size_t sigma_strlen(const char* s) {
    sigma_size_t len = 0;
    while (s[len]) len++;
    return len;
}

void* sigma_memset(void* s, int c, sigma_size_t n) {
    unsigned char* p = (unsigned char*)s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    unsigned char* d = (unsigned char*)dest;
    const unsigned char* s = (const unsigned char*)src;
    while (n--) *d++ = *s++;
    return dest;
}

int sigma_streq(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return (*s1 == *s2);
}

// --- Formatted I/O ---

void sigma_print(const char* str) {
    sigma_write(1, str, sigma_strlen(str));
}

static void sigma_itoa(sigma_u64 n, char* buf, int base) {
    static char digits[] = "0123456789ABCDEF";
    int i = 0;
    if (n == 0) {
        buf[i++] = '0';
    } else {
        while (n > 0) {
            buf[i++] = digits[n % base];
            n /= base;
        }
    }
    buf[i] = '\0';
    // Reverse
    for (int j = 0; j < i / 2; j++) {
        char tmp = buf[j];
        buf[j] = buf[i - j - 1];
        buf[i - j - 1] = tmp;
    }
}

void sigma_printf(const char* format, ...) {
    // Basic implementation for kernel logging (no varargs for now, to keep it zero-dep/simple)
    // In a real Linux-level kernel, we'd implement full vprintf.
    sigma_print(format);
}

// --- Memory Management (Simple Sharding for now) ---
static sigma_u8 kernel_heap[1024 * 1024]; // 1MB Static Heap Shard
static sigma_size_t heap_ptr = 0;

void* sigma_malloc(sigma_size_t size) {
    if (heap_ptr + size > sizeof(kernel_heap)) return SIGMA_NULL;
    void* ptr = &kernel_heap[heap_ptr];
    heap_ptr += size;
    return ptr;
}

void sigma_free(void* ptr) {
    // In this simple sharder, free is a no-op (industrial sharding uses Slab)
    SIGMA_UNUSED(ptr);
}
