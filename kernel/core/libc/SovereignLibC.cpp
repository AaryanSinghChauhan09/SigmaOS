#include "sigma_libc.h"
#include "sigma_log.h"

extern "C" {

void* sigma_mmap(void* addr, sigma_u64 length, int prot, int flags, int fd, sigma_u64 offset) {
    (void)addr; (void)length; (void)prot; (void)flags; (void)fd; (void)offset;
    // Stub for sovereign memory mapping
    return (void*)0xDEADBEEF; 
}

void sigma_print(const char* msg) {
    (void)msg;
    // Stub for sovereign console output
}

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
    return dest;
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

int sigma_atoi(const char* str) {
    int res = 0;
    while (*str >= '0' && *str <= '9') {
        res = res * 10 + (*str - '0');
        str++;
    }
    return res;
}

}
