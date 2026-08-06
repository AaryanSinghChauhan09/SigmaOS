#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_s8;
typedef signed short       sigma_s16;
typedef signed int         sigma_s32;
typedef signed long long   sigma_s64;

typedef unsigned long      sigma_size_t;
typedef unsigned long      sigma_uintptr_t;

typedef int                sigma_bool;
#define SIGMA_TRUE         1
#define SIGMA_FALSE        0

#define SIGMA_NULL         0
#define SIGMA_TRUE         true
#define SIGMA_FALSE        false
typedef unsigned long long sigma_uptr;

#endif // SIGMA_KERNEL_TYPES_H
