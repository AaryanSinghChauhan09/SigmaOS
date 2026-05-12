#include "sigma_libc.h"
#include "sigma_log.h"

extern "C" {

void* sigma_mmap(void* addr, sigma_u64 length, int prot, int flags, int fd, sigma_u64 offset) {
    // Stub for sovereign memory mapping
    return (void*)0xDEADBEEF; 
}

void sigma_print(const char* msg) {
    // Stub for sovereign console output
}

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
    return dest;
}

void* sigma_memset(void* s, int c, sigma_size_t n) {
    sigma_u8* p = (sigma_u8*)s;
    while (n--) *p++ = (sigma_u8)c;
    return s;
}

}
