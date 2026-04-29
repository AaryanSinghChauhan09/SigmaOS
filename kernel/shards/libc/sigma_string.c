/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIBC - STRING SHARD (v20.0)
 * =========================================================================
 */

#include "../../../include/SovereignLibC.h"

int sigma_atoi(const char* s) {
    int res = 0;
    int sign = 1;
    if (*s == '-') { sign = -1; s++; }
    for (int i = 0; s[i] != '\0'; ++i) {
        if (s[i] < '0' || s[i] > '9') break;
        res = res * 10 + s[i] - '0';
    }
    return res * sign;
}

int sigma_streq(const char* s1, const char* s2) {
    if (!s1 || !s2) return SIGMA_FALSE;
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return (*s1 == *s2) ? SIGMA_TRUE : SIGMA_FALSE;
}

int sigma_compare(const char* s1, const char* s2) {
    return sigma_streq(s1, s2);
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return (unsigned char)*s1 - (unsigned char)*s2;
}

void sigma_strcat(char* dest, const char* src) {
    while (*dest) dest++;
    while (*src) *dest++ = *src++;
    *dest = '\0';
}

void sigma_strcpy(char* dest, const char* src) {
    while (*src) *dest++ = *src++;
    *dest = '\0';
}

void sigma_hardened_strcpy(char* dest, const char* src, sigma_size_t dest_size) {
    if (dest_size == 0) return;
    sigma_size_t i;
    for (i = 0; i < dest_size - 1 && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    dest[i] = '\0';
}

sigma_usize sigma_itoa(sigma_i64 val, char* buf, sigma_usize buflen) {
    if (buflen == 0) return 0;
    char tmp[24];
    sigma_usize i = 0, j = 0;
    sigma_bool neg = SIGMA_FALSE;
    if (val < 0) { neg = SIGMA_TRUE; val = -val; }
    if (val == 0) { tmp[i++] = '0'; }
    while (val > 0) { tmp[i++] = '0' + (char)(val % 10); val /= 10; }
    if (neg && j < buflen - 1) buf[j++] = '-';
    while (i > 0 && j < buflen - 1) buf[j++] = tmp[--i];
    buf[j] = '\0';
    return j;
}
