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

typedef unsigned long long sigma_uptr;

#define SIGMA_TRUE         true
#define SIGMA_FALSE        false
#define SIGMA_NULL         nullptr

#endif // SIGMA_KERNEL_TYPES_H
