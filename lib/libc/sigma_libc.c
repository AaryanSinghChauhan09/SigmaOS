/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-LIBC (Zero-Dependency)
 * =============================================================================
 */
#define SIGMA_LIBC_INTERNAL
#include "sigma_kernel_types.h"
#include <stdarg.h>

// Forward declaration of serial_putc for printing
extern void serial_putc(char c);

void sigma_memcpy(void* dest, const void* src, sigma_u32 n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
}

void sigma_memset(void* s, sigma_u8 c, sigma_u32 n) {
    sigma_u8* p = (sigma_u8*)s;
    while (n--) *p++ = c;
}

sigma_u32 sigma_strlen(const char* s) {
    sigma_u32 len = 0;
    while (s[len]) len++;
    return len;
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(sigma_u8*)s1 - *(sigma_u8*)s2;
}

void sigma_strncpy(char* dest, const char* src, sigma_u32 n) {
    sigma_u32 i;
    for (i = 0; i < n - 1 && src[i] != '\0'; i++) dest[i] = src[i];
    dest[i] = '\0';
}

int sigma_atoi(const char* str) {
    int res = 0;
    int sign = 1;
    int i = 0;
    if (str[0] == '-') {
        sign = -1;
        i++;
    }
    for (; str[i] != '\0'; ++i) {
        if (str[i] < '0' || str[i] > '9') break;
        res = res * 10 + str[i] - '0';
    }
    return sign * res;
}

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

void sigma_exit(int status) {
    sigma_printf("[libc] Process terminated with status %d. Halting CPU.\n", status);
    while (1) { __asm__ volatile("cli; hlt"); }
}

sigma_u32 sigma_crc32(const void* data, sigma_size_t n) {
    const sigma_u8* bytes = (const sigma_u8*)data;
    sigma_u32 crc = 0xFFFFFFFF;
    for (sigma_size_t i = 0; i < n; i++) {
        crc ^= bytes[i];
        for (int j = 0; j < 8; j++) {
            if (crc & 1) {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    return ~crc;
}
