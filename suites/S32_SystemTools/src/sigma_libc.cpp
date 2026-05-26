#include "../../include/sigma_kernel_types.h"
#include "../../include/libc/sigma_libc.h"

extern "C" {

sigma_usize sigma_strlen(const char* str) {
    sigma_usize len = 0;
    while (str[len]) len++;
    return len;
}

void sigma_strcpy(char* dest, const char* src, sigma_usize max_len) {
    if (!dest || !src || max_len == 0) return;
    sigma_usize i = 0;
    for (i = 0; i < max_len - 1 && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    dest[i] = '\0';
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

void sigma_strcat(char* dest, const char* src, sigma_usize dest_size) {
    if (!dest || !src || dest_size == 0) return;
    sigma_usize dlen = sigma_strlen(dest);
    if (dlen >= dest_size) return; /* Already full */
    sigma_usize i = 0;
    for (i = 0; dlen + i < dest_size - 1 && src[i] != '\0'; i++) {
        dest[dlen + i] = src[i];
    }
    dest[dlen + i] = '\0';
}

void* sigma_memset(void* s, int c, sigma_usize n) {
    unsigned char* p = (unsigned char*)s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void* sigma_memcpy(void* dest, const void* src, sigma_usize n) {
    unsigned char* d = (unsigned char*)dest;
    const unsigned char* s = (const unsigned char*)src;
    while (n--) *d++ = *s++;
    return dest;
}

// Stub for kernel print (in real OS, this writes to VGA or UART)
void sigma_kprint(const char* str) {
    // For now, use an internal mechanism to log without stdio
}

void sigma_kprint_int(int val) {
    // Minimalist int to string printer
}

} // extern "C"
