#include "SovereignLibC.h"
#include <stdarg.h>

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

// --- sigma_printf (v1.0 ZENITH) ---
void sigma_printf(const char* format, ...) {
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
                default:
                    sigma_write(1, p, 1);
            }
        } else {
            sigma_write(1, p, 1);
        }
    }
    va_end(args);
}

// --- Memory Management Shard ---
// Primitive slab: just bump pointer on mmap'd region.
static void* g_heap_start = SIGMA_NULL;
static sigma_size_t g_heap_used = 0;
static const sigma_size_t HEAP_SIZE = 1024 * 1024 * 128; // 128MB Shard

void* sigma_slab_alloc_raw(sigma_size_t size) {
    if (g_heap_start == SIGMA_NULL) {
        // mmap(NULL, size, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)
        // Linux: PROT_READ=1, PROT_WRITE=2 -> 3
        // MAP_PRIVATE=0x02, MAP_ANONYMOUS=0x20 -> 0x22
        g_heap_start = sigma_mmap(SIGMA_NULL, HEAP_SIZE, 3, 0x22, -1, 0);
    }
    
    if (g_heap_used + size > HEAP_SIZE) return SIGMA_NULL;
    
    void* ptr = (sigma_u8*)g_heap_start + g_heap_used;
    g_heap_used += size;
    return ptr;
}

void* sigma_malloc(sigma_size_t size) {
    return sigma_slab_alloc_raw(size);
}

void sigma_free(void* ptr) {
    // In this zero-latency shard, we do not reclaim small blocks yet.
    // Genuine SigmaOS memory management is per-process shard cleanup.
}
