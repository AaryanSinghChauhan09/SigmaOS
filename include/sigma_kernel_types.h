#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

#include <cstddef>

/* === Integer Types === */
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

typedef signed char        sigma_s8;
typedef signed short       sigma_s16;
typedef signed int         sigma_s32;
typedef signed long long   sigma_s64;

/* === Size and Pointer Types === */
typedef std::size_t        sigma_size_t;
typedef unsigned long long sigma_uptr;
typedef unsigned long      sigma_uintptr_t;

/* Convenience alias */
typedef unsigned int       sigma_i32;

/* === Boolean Type === */
typedef bool               sigma_bool;
#define SIGMA_TRUE         true
#define SIGMA_FALSE        false

/* === Status Codes === */
typedef int                sigma_status;
#define SIGMA_OK           0
#define SIGMA_SUCCESS      0
#define SIGMA_ERROR        (-1)
#define K_ERR_INVAL        (-1)

/* === Null Pointer === */
#define SIGMA_NULL         nullptr

#endif // SIGMA_KERNEL_TYPES_H
