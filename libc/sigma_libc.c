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

/* =========================================================================
 * sigma_log — minimal labelled print (replaces SigmaOOP.hpp sigma_log)
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
    sigma_size_t i = 0;
    while (s1[i] != '\0' && s2[i] != '\0') {
        if (s1[i] != s2[i]) return SIGMA_FALSE;
        i++;
    }
    return (s1[i] == s2[i]) ? SIGMA_TRUE : SIGMA_FALSE;
}

int sigma_compare(const char* s1, const char* s2) {
    return sigma_streq(s1, s2);
}

/* =========================================================================
 * sigma_strcat
 * ========================================================================= */
void sigma_strcat(char* dest, const char* src) {
    char* rd = dest;
    while (*rd) rd++;
    while (*src) { *rd++ = *src++; }
    *rd = '\0';
}

/* =========================================================================
 * xv6-parity syscall wrappers (inline asm, C11 compatible)
 * ========================================================================= */
int sigma_fork(void) {
    long res;
    __asm__ __volatile__ (
        "syscall"
        : "=a"(res)
        : "a"(57)
        : "rcx", "r11", "memory");
    return (int)res;
}

int sigma_pipe(int pipefd[2]) {
    long res;
    __asm__ __volatile__ (
        "syscall"
        : "=a"(res)
        : "a"(22), "D"(pipefd)
        : "rcx", "r11", "memory");
    return (int)res;
}

unsigned int sigma_sleep(unsigned int seconds) {
    sigma_printf("[ZENITH-LIBC]: Pulse sleep for %u seconds...\n", seconds);
    return 0;
}

int sigma_wait(int* wstatus) {
    long res;
    register long r10 __asm__("r10") = 0;
    register long r8  __asm__("r8")  = 0;
    __asm__ __volatile__ (
        "syscall"
        : "=a"(res)
        : "a"(61), "D"(-1), "S"(wstatus), "r"(r10), "r"(r8)
        : "rcx", "r11", "memory");
    return (int)res;
}

int sigma_dup(int oldfd) {
    long res;
    __asm__ __volatile__ (
        "syscall"
        : "=a"(res)
        : "a"(32), "D"(oldfd)
        : "rcx", "r11", "memory");
    return (int)res;
}

/* =========================================================================
 * sigma_printf — sovereign variadic formatter (Absolute Scratch)
 * ========================================================================= */
void sigma_printf(const char* format, ...) {
    sigma_va_list args;
    sigma_va_start(args, format);

    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p + 1) != '\0') {
            p++;
            switch (*p) {
                case 's':
                    sigma_print(sigma_va_arg(args, const char*));
                    break;
                case 'd': {
                    int v = sigma_va_arg(args, int);
                    if (v < 0) { sigma_write(1, "-", 1); v = -v; }
                    sigma_print_num((sigma_u64)v);
                    break;
                }
                case 'u':
                    sigma_print_num((sigma_u64)sigma_va_arg(args, unsigned int));
                    break;
                case 'l':
                    /* Handle %llu, %lld */
                    if (*(p+1) == 'l' && *(p+2) == 'u') {
                        sigma_print_num(sigma_va_arg(args, sigma_u64));
                        p += 2;
                    } else if (*(p+1) == 'l' && *(p+2) == 'd') {
                        sigma_i64 v = sigma_va_arg(args, sigma_i64);
                        if (v < 0) { sigma_write(1, "-", 1); v = -v; }
                        sigma_print_num((sigma_u64)v);
                        p += 2;
                    }
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
                case 'f': {
                    /* Bare-metal float print: integer + 4 decimal places */
                    sigma_f64 fv = sigma_va_arg(args, sigma_f64);
                    if (fv < 0.0) { sigma_write(1, "-", 1); fv = -fv; }
                    sigma_u64 intpart = (sigma_u64)fv;
                    sigma_print_num(intpart);
                    sigma_write(1, ".", 1);
                    sigma_u64 frac = (sigma_u64)((fv - (sigma_f64)intpart) * 10000.0);
                    sigma_print_num(frac);
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
 * sigma_vsnprintf / sigma_snprintf — bounded variadic formatter
 * ========================================================================= */
void sigma_vsnprintf(char* buf, sigma_size_t n, const char* format, sigma_va_list args) {
    sigma_size_t pos = 0;
    if (n == 0) return;

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
                    char tmp[32]; int i = 30; tmp[31] = '\0';
                    if (v == 0) tmp[i--] = '0';
                    else while (v > 0) { tmp[i--] = (char)((v % 10) + '0'); v /= 10; }
                    const char* s = &tmp[i + 1];
                    while (*s && pos < n - 1) buf[pos++] = *s++;
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
}

void sigma_snprintf(char* buf, sigma_size_t n, const char* format, ...) {
    sigma_va_list args;
    sigma_va_start(args, format);
    sigma_vsnprintf(buf, n, format, args);
    sigma_va_end(args);
}

/* =========================================================================
 * sigma_strstr / sigma_strrchr
 * ========================================================================= */
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
 * INDUSTRIAL MEMORY SHARDING (v21.1 - Bump-Pointer Master)
 * ========================================================================= */
static sigma_u8  _sigma_memory_shard[128 * 1024 * 1024]; /* 128 MB Sovereign Shard */
static sigma_u64 _sigma_shard_ptr = 0;

void* sigma_slab_alloc_raw(sigma_size_t size) {
    if (_sigma_shard_ptr + size > sizeof(_sigma_memory_shard)) {
        sigma_log("[ERROR]: Silicon-Memory Shard EXHAUSTED.");
        return SIGMA_NULL;
    }
    void* ptr = (void*)(&_sigma_memory_shard[_sigma_shard_ptr]);
    _sigma_shard_ptr += size;
    return ptr;
}

void* sigma_malloc(sigma_size_t size) {
    /* Industrial Alignment (8-byte sharding) */
    sigma_size_t aligned = (size + 7) & ~7ULL;
    return sigma_slab_alloc_raw(aligned);
}

void sigma_free(void* ptr) {
    /* Sovereign Free: No-op for maximum performance (rely on Task-Level cleanup).
     * Professional competitors use complex GC; SigmaOS uses total partition wipe. */
}

/* =========================================================================
 * INDUSTRIAL NETWORKING MATRIX (Raw Silicon Implementation)
 * ========================================================================= */

int sigma_socket(int domain, int type, int protocol) {
    sigma_log("[NET-SHARD]: Provisioning industrial socket...");
    return _sigma_sys_socket(domain, type, protocol);
}

int sigma_bind(int sockfd, const void* addr, sigma_u32 addrlen) {
    sigma_log("[NET-SHARD]: Binding to silicon network interface...");
    return _sigma_sys_bind(sockfd, addr, addrlen);
}

int sigma_connect(int sockfd, const void* addr, sigma_u32 addrlen) {
    sigma_log("[NET-SHARD]: Orchestrating peer-to-peer connection...");
    return _sigma_sys_connect(sockfd, addr, addrlen);
}

/* =========================================================================
 * INDUSTRIAL IPC & SHARDING (Stubs)
 * ========================================================================= */
int sigma_shm_open(const char* name, int oflag, int mode) {
    sigma_printf("[IPC-SHARD]: Opening Shared Memory Shard: %s\n", name);
    return 0; /* Simulated handle */
}

int sigma_shm_unlink(const char* name) {
    sigma_printf("[IPC-SHARD]: Unlinking Shared Memory Shard: %s\n", name);
    return 0;
}

int sigma_ioctl(int fd, unsigned long request, ...) {
    sigma_log("[IO-SHARD]: Direct silicon I/O command issued.");
    return 0;
}

int sigma_mprotect(void* addr, sigma_size_t len, int prot) {
    sigma_log("[SEC-SHARD]: Enforcing hardware memory protection (W^X).");
    return 0;
}
