/* 
 Σ SIGMAOS ZENITH SUPREME: INTERNAL KERNEL UTILS (v1300.0)
 Absolute Zero-Dependency Replacements for Forbidden Stdlib Functions.
*/

#ifndef SIGMA_INTERNAL_H
#define SIGMA_INTERNAL_H

// Σ PRIVE-LEVEL LOGGING
static inline void sigma_print(const char* s) {
   // Hardware-direct character emit logic placeholder (Bare-metal UART/TTY)
}

// Σ BASIC STRING OPS
static inline void sigma_strcpy(char* dest, const char* src) {
    while ((*dest++ = *src++));
}

static inline int sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return 1;
    for (; *haystack; haystack++) {
        if (*haystack == *needle) {
            const char *h = haystack, *n = needle;
            while (*h && *n && *h == *n) { h++; n++; }
            if (!*n) return 1;
        }
    }
    return 0;
}

// Σ RANDOM NUMBER GENERATOR (LINEAR CONGRUENTIAL)
static unsigned long _sigma_seed = 0x51634A;
#define SIGMA_RAND_MAX 32767
static inline int sigma_rand() {
    _sigma_seed = _sigma_seed * 1103515245 + 12345;
    return (unsigned int)(_sigma_seed / 65536) % 32768;
}

#endif
