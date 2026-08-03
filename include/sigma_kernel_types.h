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
