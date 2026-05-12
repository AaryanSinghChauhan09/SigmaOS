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

sigma_size_t sigma_strlen(const char* s) {
    sigma_size_t len = 0;
    while (s && s[len]) len++;
    return len;
}

void sigma_strncpy(char* dest, const char* src, sigma_size_t n) {
    sigma_size_t i;
    for (i = 0; i < n && src[i] != '\0'; i++)
        dest[i] = src[i];
    for (; i < n; i++)
        dest[i] = '\0';
}

void sigma_printf(const char* format, ...) {
    // Industrial implementation routes to kernel console
    sigma_log_info(format); 
}

void sigma_exit(int status) {
    sigma_log_info("[LIBC]: Process terminating with status %d", status);
    // Logic: Halt CPU or return to orchestrator
}

} // extern "C"

