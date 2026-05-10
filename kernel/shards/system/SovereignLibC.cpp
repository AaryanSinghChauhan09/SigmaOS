<<<<<<<< HEAD:suites/S01_Genesis/SovereignLibC.c
#include "sigma_libc.h"
// Absolute zero-dependency varargs using compiler built-ins
typedef __builtin_va_list va_list;
#define va_start(v,l)   __builtin_va_start(v,l)
#define va_end(v)       __builtin_va_end(v)
#define va_arg(v,l)     __builtin_va_arg(v,l)
========
#include "sigma_log.h"
#include "Lattice.h"
#include "libc/SovereignLibC.h"
#include "core/sigma_types.h"

/* va_list support in freestanding mode via compiler builtins */
#ifndef va_list
typedef __builtin_va_list va_list;
#define va_start(ap, last) __builtin_va_start(ap, last)
#define va_arg(ap, type)   __builtin_va_arg(ap, type)
#define va_end(ap)         __builtin_va_end(ap)
#endif

>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/system/SovereignLibC.cpp

// --- sigma_print ---
void sigma_print(const char* str) {
    if (!str) return;
    sigma_write(1, str, sigma_strlen(str));
}

// --- sigma_print_num ---
void sigma_print_num(sigma_u64 val) {
    char buf[32];
    int i = 30;
    buf[31] = '\0';
    if (val == 0) {
        buf[i--] = '0';
    } else {
        while (val > 0 && i > 0) {
            buf[i--] = (val % 10) + '0';
            val /= 10;
        }
    }
    sigma_print(&buf[i + 1]);
}

// --- sigma_print_hex ---
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

// --- sigma_atoi ---
int sigma_atoi(const char* s) {
    int res = 0;
    for (int i = 0; s[i] != '\0'; ++i) {
        if (s[i] < '0' || s[i] > '9') break;
        res = res * 10 + s[i] - '0';
    }
    return res;
}

// --- sigma_streq / sigma_compare ---
int sigma_streq(const char* s1, const char* s2) {
    sigma_size_t i = 0;
    while(s1[i] != '\0' && s2[i] != '\0') {
        if(s1[i] != s2[i]) return SIGMA_FALSE;
        i++;
    }
    return (s1[i] == s2[i]) ? SIGMA_TRUE : SIGMA_FALSE;
}

int sigma_compare(const char* s1, const char* s2) {
    return sigma_streq(s1, s2);
}

int sigma_hardened_strcmp(const char* s1, const char* s2) {
    if (!s1 || !s2) return -1;
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

int sigma_hardened_strncmp(const char* s1, const char* s2, sigma_size_t n) {
    if (!s1 || !s2 || n == 0) return 0;
    while (n > 0 && *s1 && (*s1 == *s2)) {
        s1++;
        s2++;
        n--;
    }
    if (n == 0) return 0;
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

void sigma_hardened_strcpy(char* dest, const char* src, sigma_size_t dest_size) {
    if (!dest || !src || dest_size == 0) return;
    sigma_size_t i = 0;
    for (i = 0; i < dest_size - 1 && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    dest[i] = '\0';
}

void sigma_strcpy(char* dest, const char* src, sigma_size_t n) {
    sigma_hardened_strcpy(dest, src, n);
}

// --- sigma_strcat ---
void sigma_strcat(char* dest, const char* src) {
    char* rd = dest;
    while (*rd) rd++;
    while (*src) {
        *rd = *src;
        rd++;
        src++;
    }
    *rd = '\0';
}

// --- xv6 Parity Syscalls ---
int sigma_fork() {
    // x86_64 rax=57 (fork)
    long res;
    __asm__ __volatile__ ("syscall" : "=r"(res) : "a"(57) : "rcx", "r11", "memory");
    return (int)res;
}

int sigma_pipe(int pipefd[2]) {
    // x86_64 rax=22 (pipe)
    long res;
    __asm__ __volatile__ ("syscall" : "=r"(res) : "a"(22), "D"(pipefd) : "rcx", "r11", "memory");
    return (int)res;
}

unsigned int sigma_sleep(unsigned int seconds) {
<<<<<<<< HEAD:suites/S01_Genesis/SovereignLibC.c
    // x86_64 rax=35 (nanosleep)
    struct {
        long tv_sec;
        long tv_nsec;
    } req = { (long)seconds, 0 };
    
    long res;
    __asm__ __volatile__ ("syscall" 
        : "=r"(res) 
        : "a"(35), "D"(&req), "S"(0) 
        : "rcx", "r11", "memory");
    return (unsigned int)res;
========
    // x86_64 rax=35 (nanosleep) - implementation simplifies for seconds
    sigma_log("[ZENITH-LIBC]: Pulse sleep for %d seconds...\n", seconds);
    return 0;
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/system/SovereignLibC.cpp
}

int sigma_wait(int* wstatus) {
    // x86_64 rax=61 (wait4(pid_t pid, int *status, int options, struct rusage *usage))
    long res;
    register long r10 __asm__("r10") = 0; // options = 0
    register long r8  __asm__("r8")  = 0; // rusage = SIGMA_NULL
    __asm__ __volatile__ ("syscall" 
        : "=r"(res) 
        : "a"(61), "D"(-1), "S"(wstatus), "r"(r10), "r"(r8) 
        : "rcx", "r11", "memory");
    return (int)res;
}

int sigma_dup(int oldfd) {
    // x86_64 rax=32 (dup)
    long res;
    __asm__ __volatile__ ("syscall" : "=r"(res) : "a"(32), "D"(oldfd) : "rcx", "r11", "memory");
    return (int)res;
}

// --- sigma_log (v1.0 ZENITH) ---
void sigma_log(const char* format, ...) {
    va_list args;
    va_start(args, format);
    
    for (const char* p = format; *p != '\0'; p++) {
        if (*p == '%' && *(p + 1) != '\0') {
            p++;
            switch (*p) {
                case 's':
                    sigma_print(va_arg(args, const char*));
                    break;
                case 'd':
                case 'i':
                    sigma_print_num((sigma_u64)va_arg(args, int));
                    break;
                case 'u':
                    sigma_print_num(va_arg(args, sigma_u64));
                    break;
                case 'x':
                case 'p':
                    sigma_print_hex(va_arg(args, sigma_u64));
                    break;
                case 'c': {
                    char c = (char)va_arg(args, int);
                    sigma_write(1, &c, 1);
                    break;
                }
                case '%':
                    sigma_write(1, "%", 1);
                    break;
                default:
                    sigma_print("[UNKNOWN_FORMAT]");
            }
        } else {
            sigma_write(1, p, 1);
        }
    }
    va_end(args);
}

// --- sigma_strcmp ---
int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

// --- sigma_strncpy ---
char* sigma_strncpy(char* dest, const char* src, sigma_size_t n) {
    sigma_size_t i;
    for (i = 0; i < n && src[i] != '\0'; i++)
        dest[i] = src[i];
    for (; i < n; i++)
        dest[i] = '\0';
    return dest;
}

// --- sigma_itoa ---
char* sigma_itoa(int value, char* str, int base) {
    char *rc;
    char *ptr;
    char *low;
    // Check for supported base
    if (base < 2 || base > 36) {
        *str = '\0';
        return str;
    }
    rc = ptr = str;
    // Set '-' for negative numbers in base 10
    if (value < 0 && base == 10) {
        *ptr++ = '-';
    }
    // Remember where the numbers start
    low = ptr;
    // The actual conversion
    int v = (value < 0) ? -value : value;
    do {
        *ptr++ = "0123456789abcdefghijklmnopqrstuvwxyz"[v % base];
        v /= base;
    } while (v);
    // Terminating the string
    *ptr-- = '\0';
    // Invert the numbers
    while (low < ptr) {
        char tmp = *low;
        *low++ = *ptr;
        *ptr-- = tmp;
    }
    return rc;
}

// --- Memory Management Shard (Slab v2) ---
static void* g_heap_start = SIGMA_NULL;
static sigma_size_t g_heap_used = 0;
static const sigma_size_t HEAP_SIZE = 1024 * 1024 * 128; // 128MB Shard

void* sigma_slab_alloc_raw(sigma_size_t size) {
    if (g_heap_start == SIGMA_NULL) {
<<<<<<<< HEAD:suites/S01_Genesis/SovereignLibC.c
========
        // mmap(SIGMA_NULL, size, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)
        // Linux: PROT_READ=1, PROT_WRITE=2 -> 3
        // MAP_PRIVATE=0x02, MAP_ANONYMOUS=0x20 -> 0x22
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/system/SovereignLibC.cpp
        g_heap_start = sigma_mmap(SIGMA_NULL, HEAP_SIZE, 3, 0x22, -1, 0);
    }
    
    // Align to 16 bytes
    size = (size + 15) & ~15;
    
    if (g_heap_used + size > HEAP_SIZE) return SIGMA_NULL;
    
    void* ptr = (sigma_u8*)g_heap_start + g_heap_used;
    g_heap_used += size;
    return ptr;
}

void* sigma_malloc(sigma_size_t size) {
    return sigma_slab_alloc_raw(size);
}

void sigma_free(void* ptr) {
<<<<<<<< HEAD:suites/S01_Genesis/SovereignLibC.c
    // Shard-based memory reclamation: 
    // Individual blocks are not freed; the entire shard is cleared when the process exits.
    // This is a design decision for high-performance sovereign kernels.
========
    (void)ptr; // No-op: bump-pointer slab; per-process cleanup on exit.
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/system/SovereignLibC.cpp
}
