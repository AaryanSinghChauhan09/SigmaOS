#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

typedef unsigned short sigma_u16;
typedef unsigned int sigma_u32;
typedef int sigma_i32;
typedef unsigned char sigma_u8;
typedef unsigned long long sigma_u64;
typedef unsigned long long sigma_uptr;
typedef unsigned long long sigma_size_t;
typedef int sigma_status;

#ifndef SIGMA_NULL
#ifdef __cplusplus
#define SIGMA_NULL nullptr
#else
#define SIGMA_NULL ((void*)0)
#endif
#endif

#define SIGMA_SUCCESS 0
#define SIGMA_ERROR -1

typedef enum {
    SIGMA_FALSE = 0,
    SIGMA_TRUE = 1
} sigma_bool;

#endif