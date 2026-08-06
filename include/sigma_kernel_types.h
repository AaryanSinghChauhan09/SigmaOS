#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

#include <cstddef>

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_s8;
typedef short              sigma_s16;
typedef int                sigma_s32;
typedef long long          sigma_s64;

typedef std::size_t        sigma_size_t;
typedef bool               sigma_bool;

typedef int                sigma_status;
#define SIGMA_OK           0
#define SIGMA_SUCCESS      0
#define SIGMA_ERROR        (-1)
#define K_ERR_INVAL        (-1)

#endif // SIGMA_KERNEL_TYPES_H
#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

typedef unsigned int sigma_u32;
typedef int sigma_i32;
typedef unsigned char sigma_u8;
typedef unsigned long long sigma_u64;
typedef int sigma_status;

#define SIGMA_SUCCESS 0
#define SIGMA_ERROR -1

typedef enum {
    SIGMA_FALSE = 0,
    SIGMA_TRUE = 1
} sigma_bool;

#endif
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

#endif // SIGMA_KERNEL_TYPES_H
#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

#include <cstddef>

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_s8;
typedef short              sigma_s16;
typedef int                sigma_s32;
typedef long long          sigma_s64;

typedef std::size_t        sigma_size_t;
typedef bool               sigma_bool;

typedef int                sigma_status;
#define SIGMA_OK           0
#define SIGMA_SUCCESS      0
#define SIGMA_ERROR        (-1)
#define K_ERR_INVAL        (-1)

#endif // SIGMA_KERNEL_TYPES_H
#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef unsigned long long sigma_size_t;
typedef unsigned long long sigma_uptr;

typedef bool               sigma_bool;
typedef int                sigma_status;

#define SIGMA_TRUE         true
#define SIGMA_FALSE        false
#define SIGMA_NULL         nullptr

#endif // SIGMA_KERNEL_TYPES_H
