/* 
 Σ SIGMAOS ZENITH: SOVEREIGN STRING UTILS (v2600.0)
 Mission: Total Standard Library Obsolescence.
*/

#include <stdint.h>
#include <stddef.h>

// Σ SOVEREIGN STRLEN
size_t sigma_strlen(const char* s) {
    size_t len = 0;
    while (s[len]) len++;
    return len;
}

// Σ SOVEREIGN STRCMP
int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

// Σ SOVEREIGN MEMSET
void* sigma_memset(void* s, int c, size_t n) {
    unsigned char* p = (unsigned char*)s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}
