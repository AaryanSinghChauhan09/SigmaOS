#include "SovereignLibC.h"

int sigma_atoi(const char* s) {
    int res = 0;
    int i   = 0;
    while (s[i] != '\0') {
        if (s[i] < '0' || s[i] > '9') break;
        res = res * 10 + (s[i] - '0');
        i++;
    }
    return res;
}

int sigma_streq(const char* s1, const char* s2) {
    sigma_size_t i = 0;
    while (s1[i] != '\0' && s2[i] != '\0') {
        if (s1[i] != s2[i]) return SIGMA_FALSE;
        i++;
    }
    return (s1[i] == s2[i]) ? SIGMA_TRUE : SIGMA_FALSE;
}


void sigma_strcat(char* dest, const char* src) {
    char* rd = dest;
    while (*rd) rd++;
    while (*src) { *rd++ = *src++; }
    *rd = '\0';
}

void sigma_strcpy(char* dest, const char* src) {
    while (*src) { *dest++ = *src++; }
    *dest = '\0';
}

int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

int sigma_compare(const char* s1, const char* s2) {
    return sigma_strcmp(s1, s2);
}
