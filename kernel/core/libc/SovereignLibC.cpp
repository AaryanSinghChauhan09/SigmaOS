#define SIGMA_LIBC_INTERNAL
#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "libc/SovereignLibC.h"
#include "sigma_log.h"
#include <stdarg.h>
#include "sigma_log.h"

extern "C" {

/**
 * SIGMAOS: SOVEREIGN LIBC IMPLEMENTATION (v15.0 Zenith)
 * Mission: Zero-dependency silicon-direct primitives.
 */

// Serial I/O primitive (implemented in HAL)
void serial_putc(char c);

// Assembly primitives (from SovereignLibC_x64.S)
extern "C" void* sigma_memcpy_asm(void* dest, const void* src, sigma_size_t n);
extern "C" void* sigma_memset_asm(void* s, int c, sigma_size_t n);
extern "C" sigma_size_t sigma_strlen_asm(const char* s);

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    return sigma_memcpy_asm(dest, src, n);
}

void* sigma_memset(void* s, int c, sigma_size_t n) {
    return sigma_memset_asm(s, c, n);
}

void* sigma_secure_memset(void* ptr, int ch, sigma_size_t n) {
    sigma_memset(ptr, ch, n);
    __asm__ volatile("" : : "g"(ptr) : "memory");
    return ptr;
}

sigma_size_t sigma_strlen(const char* s) {
    if (!s) return 0;
    return sigma_strlen_asm(s);
}

void sigma_strcpy(char* dest, const char* src, sigma_size_t n) {
    if (!dest || !src) return;
    sigma_size_t i;
    for (i = 0; i < n - 1 && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    dest[i] = '\0';
}

void sigma_hardened_strcpy(char* dest, const char* src, sigma_size_t dest_size) {
    sigma_strcpy(dest, src, dest_size);
}

int sigma_hardened_strcmp(const char* str1, const char* str2) {
    if (!str1 || !str2) return (str1 == str2) ? 0 : (str1 ? 1 : -1);
    while (*str1 && (*str1 == *str2)) {
        str1++;
        str2++;
    }
    return *(unsigned char*)str1 - *(unsigned char*)str2;
}

// Simple integer to string conversion
static void sigma_itoa(sigma_u64 n, char* buf, int base) {
    char tmp[64];
    int i = 0;
    if (n == 0) {
        buf[0] = '0';
        buf[1] = '\0';
        return;
    }
    while (n > 0) {
        sigma_u64 rem = n % base;
        tmp[i++] = (rem < 10) ? (char)('0' + rem) : (char)('A' + (rem - 10));
        n /= base;
    }
    int j;
    for (j = 0; j < i; j++) {
        buf[j] = tmp[i - j - 1];
    }
    buf[j] = '\0';
}

void sigma_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);

    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p+1) != '\0') {
            p++;
            switch (*p) {
                case 's': {
                    const char* s = va_arg(args, const char*);
                    if (!s) s = "(null)";
                    while (*s) { serial_putc(*s++); }
                    break;
                }
                case 'd':
                case 'u': {
                    sigma_u64 val = va_arg(args, sigma_u64);
                    char buf[64];
                    sigma_itoa(val, buf, 10);
                    char* b = buf;
                    while (*b) { serial_putc(*b++); }
                    break;
                }
                case 'x':
                case 'X': {
                    sigma_u64 val = va_arg(args, sigma_u64);
                    char buf[64];
                    sigma_itoa(val, buf, 16);
                    char* b = buf;
                    while (*b) { serial_putc(*b++); }
                    break;
                }
                case 'p': {
                    sigma_u64 val = (sigma_u64)va_arg(args, void*);
                    serial_putc('0'); serial_putc('x');
                    char buf[64];
                    sigma_itoa(val, buf, 16);
                    char* b = buf;
                    while (*b) { serial_putc(*b++); }
                    break;
                }
                case 'c': {
                    char c = (char)va_arg(args, int);
                    serial_putc(c);
                    break;
                }
                case '%': {
                    serial_putc('%');
                    break;
                }
                default:
                    serial_putc('%');
                    serial_putc(*p);
            }
        } else {
            serial_putc(*p);
        }
    }

    va_end(args);
}

void sigma_log_industrial(const char* msg) {
    sigma_log_info("[SIGMA] %s\n", msg);
}

} // extern "C"
 

