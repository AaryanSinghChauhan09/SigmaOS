#include "sigma_libc.h"
#include "sigma_log.h"

extern "C" {

void* sigma_mmap(void* addr, sigma_u64 length, int prot, int flags, int fd, sigma_u64 offset) {
    (void)addr; (void)length; (void)prot; (void)flags; (void)fd; (void)offset;
    // Stub for sovereign memory mapping - Industrial Implementation Pending
    return (void*)0xDEADBEEF; 
}

void sigma_print(const char* msg) {
    if (msg) {
        // Direct call to kernel log system
        sigma_log_info(msg);
    }
}

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    return (void*)sigma_mem_copy(dest, src, n);
}

void* sigma_memset(void* s, int c, sigma_size_t n) {
    return (void*)sigma_mem_set(s, c, (sigma_usize)n);
}

int sigma_strcmp(const char* s1, const char* s2) {
    // Uses the kernel-native strcmp primitive
    if (!s1 || !s2) return 0;
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(const sigma_u8*)s1 - *(const sigma_u8*)s2;
}

int sigma_atoi(const char* str) {
    int res = 0;
    if (!str) return 0;
    while (*str >= '0' && *str <= '9') {
        res = res * 10 + (*str - '0');
        str++;
    }
    return res;
}

}

