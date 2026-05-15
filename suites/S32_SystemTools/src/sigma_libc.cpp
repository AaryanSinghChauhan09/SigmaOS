#include "../../../include/libc/sigma_libc.h"

extern "C" {

sigma_size_t sigma_strlen(const char* str) {
    sigma_size_t len = 0;
    while (str[len]) len++;
    return len;
}

void sigma_strcpy(char* dest, const char* src) {
    sigma_size_t i = 0;
    while ((dest[i] = src[i])) i++;
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

void sigma_strcat(char* dest, const char* src) {
    sigma_size_t dlen = sigma_strlen(dest);
    sigma_size_t i = 0;
    while ((dest[dlen + i] = src[i])) i++;
}

void* sigma_memset(void* s, int c, sigma_size_t n) {
    unsigned char* p = (unsigned char*)s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
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

}

} // extern "C"
