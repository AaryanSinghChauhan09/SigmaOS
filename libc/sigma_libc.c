#include "SovereignLibC.h"

/* =========================================================================
 * ABSOLUTE SCRATCH: COMPILER-NATIVE VARIADIC HANDLER (Zero-Header)
 * =========================================================================
 * No <stdarg.h>. Using compiler-intrinsic types for absolute sovereignty.
 * ========================================================================= */
typedef __builtin_va_list sigma_va_list;

#define sigma_va_start(ap, last) __builtin_va_start(ap, last)
#define sigma_va_arg(ap, type)   __builtin_va_arg(ap, type)
#define sigma_va_end(ap)         __builtin_va_end(ap)

/* External ASM Shards */
extern sigma_size_t _sigma_asm_strlen(const char* s);
extern int _sigma_sys_open(const char* path, int flags, int mode);
extern int _sigma_sys_close(int fd);
extern int _sigma_sys_socket(int domain, int type, int protocol);
extern int _sigma_sys_bind(int sockfd, const void* addr, sigma_u32 addrlen);
extern int _sigma_sys_connect(int sockfd, const void* addr, sigma_u32 addrlen);

/* =========================================================================
 * sigma_log
 * ========================================================================= */
void sigma_log(const char* msg) {
    sigma_print("[SIGMA_LOG]: ");
    sigma_print(msg);
    sigma_print("\n");
}

/* =========================================================================
 * sigma_print / sigma_print_num / sigma_print_hex
 * ========================================================================= */
void sigma_print(const char* str) {
    if (!str) return;
    sigma_write(1, str, sigma_strlen(str));
}

void sigma_print_num(sigma_u64 val) {
    char buf[32];
    int i = 30;
    buf[31] = '\0';
    if (val == 0) {
        buf[i--] = '0';
    } else {
        while (val > 0 && i > 0) {
            buf[i--] = (char)((val % 10) + '0');
            val /= 10;
        }
    }
    sigma_print(&buf[i + 1]);
}

void sigma_print_hex(sigma_u64 val) {
    char buf[32];
    int i = 30;
    const char* hex = "0123456789ABCDEF";
    buf[31] = '\0';
    if (val == 0) {
        buf[i--] = '0';
    } else {
        while (val > 0 && i > 0) {
            buf[i--] = hex[val % 16];
            val /= 16;
        }
    }
    sigma_print("0x");
    sigma_print(&buf[i + 1]);
}

/* =========================================================================
 * sigma_atoi
 * ========================================================================= */
int sigma_atoi(const char* s) {
    int res = 0;
    int i   = 0;
    if (!s) return 0;
    while (s[i] != '\0') {
        if (s[i] < '0' || s[i] > '9') break;
        res = res * 10 + (s[i] - '0');
        i++;
    }
    return res;
}

/* =========================================================================
 * sigma_streq / sigma_compare
 * ========================================================================= */
int sigma_streq(const char* s1, const char* s2) {
    if (!s1 || !s2) return SIGMA_FALSE;
    sigma_size_t i = 0;
    while (s1[i] != '\0' && s2[i] != '\0') {
        if (s1[i] != s2[i]) return SIGMA_FALSE;
        i++;
    }
    return (s1[i] == s2[i]) ? SIGMA_TRUE : SIGMA_FALSE;
}

int sigma_compare(const char* s1, const char* s2) {
    if (!s1 || !s2) return (s1 == s2) ? 0 : (s1 ? 1 : -1);
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

void *sigma_memset(void *s, int c, sigma_size_t n) {
    unsigned char *p = (unsigned char *)s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void *sigma_memcpy(void *dest, const void *src, sigma_size_t n) {
    unsigned char *d = (unsigned char *)dest;
    const unsigned char *s = (const unsigned char *)src;
    while (n--) *d++ = *s++;
    return dest;
}

void sigma_strlcat(char* dest, const char* src, sigma_size_t dstsize) {
    if (!dest || !src || dstsize == 0) return;
    sigma_size_t dlen = 0;
    while (dlen < dstsize && dest[dlen]) dlen++;
    if (dlen == dstsize) return;
    sigma_size_t i = 0;
    while (src[i] && (dlen + i < dstsize - 1)) {
        dest[dlen + i] = src[i];
        i++;
    }
    dest[dlen + i] = '\0';
}

char* sigma_strcpy(char* dest, const char* src, sigma_size_t maxlen) {
    if (!dest || !src || maxlen == 0) return dest;
    sigma_size_t i;
    for (i = 0; i < maxlen - 1 && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    dest[i] = '\0';
    return dest;
}

/* =========================================================================
 * sigma_printf
 * ========================================================================= */
void sigma_printf(const char* format, ...) {
    sigma_va_list args;
    sigma_va_start(args, format);

    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p + 1) != '\0') {
            p++;
            switch (*p) {
                case '%':
                    sigma_write(1, "%", 1);
                    break;
                case 's':
                    sigma_print(sigma_va_arg(args, const char*));
                    break;
                case 'd':
                case 'i': {
                    long long v = (long long)sigma_va_arg(args, int);
                    if (v < 0) { sigma_write(1, "-", 1); v = -v; }
                    sigma_print_num((sigma_u64)v);
                    break;
                }
                case 'u':
                    sigma_print_num((sigma_u64)sigma_va_arg(args, unsigned int));
                    break;
                case 'x':
                case 'p':
                    sigma_print_hex((sigma_u64)sigma_va_arg(args, sigma_u64));
                    break;
                case 'c': {
                    char c = (char)sigma_va_arg(args, int);
                    sigma_write(1, &c, 1);
                    break;
                }
                default:
                    sigma_write(1, p, 1);
                    break;
            }
        } else {
            sigma_write(1, p, 1);
        }
    }
    sigma_va_end(args);
}

/* =========================================================================
 * sigma_sleep (unsigned int seconds)
 * ========================================================================= */
extern int sigma_nanosleep(const void* req, void* rem);

unsigned int sigma_sleep(unsigned int seconds) {
    long long timing[2] = { (long long)seconds, 0 };
    sigma_nanosleep(timing, SIGMA_NULL);
    return 0; // Simplified
}

int sigma_snprintf(char* buf, sigma_size_t n, const char* format, ...) {
    if (n == 0) return 0;
    sigma_va_list args;
    sigma_va_start(args, format);
    
    sigma_size_t pos = 0;
    for (const char* p = format; *p != '\0' && pos < n - 1; p++) {
        if (*p == '%' && *(p + 1) != '\0') {
            p++;
            switch (*p) {
                case 's': {
                    const char* s = sigma_va_arg(args, const char*);
                    while (*s && pos < n - 1) buf[pos++] = *s++;
                    break;
                }
                case 'd': {
                    int v = sigma_va_arg(args, int);
                    if (v < 0) { if (pos < n - 1) buf[pos++] = '-'; v = -v; }
                    char tmp[20]; int i = 0;
                    if (v == 0) tmp[i++] = '0';
                    while (v > 0) { tmp[i++] = (v % 10) + '0'; v /= 10; }
                    while (i > 0 && pos < n - 1) buf[pos++] = tmp[--i];
                    break;
                }
                case 'x': {
                    sigma_u64 v = (sigma_u64)sigma_va_arg(args, sigma_u64);
                    const char* hex = "0123456789abcdef";
                    char tmp[16]; int i = 0;
                    if (v == 0) tmp[i++] = '0';
                    while (v > 0) { tmp[i++] = hex[v % 16]; v /= 16; }
                    while (i > 0 && pos < n - 1) buf[pos++] = tmp[--i];
                    break;
                }
                default:
                    buf[pos++] = *p;
                    break;
            }
        } else {
            buf[pos++] = *p;
        }
    }
    buf[pos] = '\0';
    sigma_va_end(args);
    return (int)pos;
}

const char* sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return haystack;
    for (; *haystack; haystack++) {
        if (*haystack == *needle) {
            const char *h = haystack, *n = needle;
            while (*h && *n && *h == *n) { h++; n++; }
            if (!*n) return haystack;
        }
    }
    return SIGMA_NULL;
}

const char* sigma_strrchr(const char* s, int c) {
    const char* last = SIGMA_NULL;
    while (*s) {
        if (*s == (char)c) last = s;
        s++;
    }
    return last;
}

/* =========================================================================
 * SLAB ALLOCATOR (Industrial Version)
 * ========================================================================= */
static sigma_u8  _sigma_memory_shard[1024 * 1024]; /* 1 MB */
static sigma_u64 _sigma_shard_ptr = 0;

void* sigma_malloc(sigma_size_t size) {
    sigma_size_t aligned = (size + 7) & ~7ULL;
    if (_sigma_shard_ptr + aligned > sizeof(_sigma_memory_shard)) return SIGMA_NULL;
    void* ptr = (void*)(&_sigma_memory_shard[_sigma_shard_ptr]);
    _sigma_shard_ptr += aligned;
    return ptr;
}

void sigma_free(void* ptr) {
    (void)ptr; /* Bump allocator cannot free */
}
