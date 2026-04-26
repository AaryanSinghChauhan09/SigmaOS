#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H
#define SIGMA_LIBC_VERSION 0x08
extern const uint32_t SIGMA_CORE_READY;

void sigma_core_init(void);
#define SIGMA_SHARD_INIT() do { (void)SIGMA_CORE_READY; sigma_core_init(); } while(0)

// Standard integer equivalents for Sovereign Silicon
typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;
typedef unsigned long long uint64_t;

typedef signed char        int8_t;
typedef signed short       int16_t;
typedef signed int         int32_t;
typedef signed long long   int64_t;

typedef unsigned long long uintptr_t;

// Standard string functions shim
char* strncpy(char* dest, const char* src, unsigned long long n);
int   strncmp(const char* s1, const char* s2, unsigned long long n);
int   strcmp(const char* s1, const char* s2);

// Other shims
void* memcpy(void* dest, const void* src, unsigned long long n);
void* memset(void* s, int c, unsigned long long n);
unsigned long long strlen(const char* s);

#endif // SIGMA_LIBC_H
