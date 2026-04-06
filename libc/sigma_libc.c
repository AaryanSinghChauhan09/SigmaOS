#include "SovereignLibC.h"

typedef __builtin_va_list sigma_va_list;
#define sigma_va_start(ap, last) __builtin_va_start(ap, last)
#define sigma_va_arg(ap, type)   __builtin_va_arg(ap, type)
#define sigma_va_end(ap)         __builtin_va_end(ap)

extern sigma_size_t _sigma_asm_strlen(const char* s);
extern int _sigma_sys_open(const char* path, int flags, int mode);
extern int _sigma_sys_close(int fd);

void sigma_log(const char* msg) { sigma_print("[LOG]: "); sigma_print(msg); sigma_print("\n"); }

void sigma_print(const char* str) { if (str) sigma_write(1, str, sigma_strlen(str)); }

static void _sigma_u64_to_str(sigma_u64 val, char* buf, int base, sigma_bool caps) {
    const char* hex = caps ? "0123456789ABCDEF" : "0123456789abcdef";
    int i = 0;
    if (val == 0) buf[i++] = '0';
    else { while (val > 0) { buf[i++] = hex[val % base]; val /= base; } }
    buf[i] = '\0';
    for (int j = 0; j < i / 2; j++) { char t = buf[j]; buf[j] = buf[i - 1 - j]; buf[i - 1 - j] = t; }
}

void sigma_print_num(sigma_u64 val) { char buf[32]; _sigma_u64_to_str(val, buf, 10, 0); sigma_print(buf); }
void sigma_print_hex(sigma_u64 val) { char buf[32]; _sigma_u64_to_str(val, buf, 16, 1); sigma_print("0x"); sigma_print(buf); }

int sigma_atoi(const char* s) {
    int res = 0;
    if (!s) return 0;
    while (*s >= '0' && *s <= '9') { res = res * 10 + (*s - '0'); s++; }
    return res;
}

int sigma_streq(const char* s1, const char* s2) {
    if (!s1 || !s2) return s1 == s2;
    while (*s1 && *s1 == *s2) { s1++; s2++; }
    return *s1 == *s2;
}

int sigma_compare(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

void* sigma_memset(void* s, int c, sigma_size_t n) {
    unsigned char* p = s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void* sigma_memcpy(void* d, const void* s, sigma_size_t n) {
    unsigned char* dst = d; const unsigned char* src = s;
    while (n--) *dst++ = *src++;
    return d;
}

void sigma_printf(const char* fmt, ...) {
    sigma_va_list args; sigma_va_start(args, fmt);
    for (const char* p = fmt; *p; p++) {
        if (*p == '%' && *(p+1)) {
            p++;
            if (*p == 's') sigma_print(sigma_va_arg(args, char*));
            else if (*p == 'd') { 
                long v = sigma_va_arg(args, int); 
                if (v < 0) { sigma_print("-"); v = -v; }
                sigma_print_num((sigma_u64)v);
            }
            else if (*p == 'x') sigma_print_hex(sigma_va_arg(args, sigma_u64));
            else if (*p == 'c') { char c = (char)sigma_va_arg(args, int); sigma_write(1, &c, 1); }
        } else sigma_write(1, p, 1);
    }
    sigma_va_end(args);
}

void* sigma_malloc(sigma_size_t size) {
    static sigma_u8 arena[1024*1024]; static sigma_u64 ptr = 0;
    sigma_size_t a = (size + 7) & ~7;
    if (ptr + a > sizeof(arena)) return SIGMA_NULL;
    void* r = &arena[ptr]; ptr += a; return r;
}
void sigma_free(void* p) { (void)p; }
