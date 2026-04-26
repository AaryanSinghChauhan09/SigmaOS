#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

// Standard integer equivalents for Sovereign Silicon
typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;
typedef unsigned long long uint64_t;

// Standard string functions shim
char* strncpy(char* dest, const char* src, unsigned long long n);
int   strncmp(const char* s1, const char* s2, unsigned long long n);
int   strcmp(const char* s1, const char* s2);

// Other shims
void* memcpy(void* dest, const void* src, unsigned long long n);
void* memset(void* s, int c, unsigned long long n);
unsigned long long strlen(const char* s);

#endif // SIGMA_LIBC_H
