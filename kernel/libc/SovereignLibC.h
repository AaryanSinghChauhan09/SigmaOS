#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H

typedef unsigned long long sigma_u64;
typedef unsigned int       sigma_u32;
typedef unsigned short     sigma_u16;
typedef unsigned char      sigma_u8;

extern void sigma_printf(const char* fmt, ...);

#endif
