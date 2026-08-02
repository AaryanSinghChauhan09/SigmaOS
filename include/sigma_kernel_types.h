#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> origin/jules-4213023701309535613-b11406ba
=======
>>>>>>> origin/jules-4256353901936270903-d30fe5d7
#include <cstddef>

=======
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
=======
>>>>>>> origin/jules-13571719274074749109-6af93541
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

<<<<<<< HEAD
<<<<<<< HEAD
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
<<<<<<< HEAD
<<<<<<< HEAD
=======
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
>>>>>>> origin/improve-installer-script-9830616872725964915
=======
>>>>>>> origin/jules-4213023701309535613-b11406ba
=======
>>>>>>> origin/jules-4256353901936270903-d30fe5d7
=======
typedef unsigned long long sigma_size_t;
typedef unsigned long long sigma_uptr;

typedef bool               sigma_bool;
typedef int                sigma_status;

#define SIGMA_TRUE         true
#define SIGMA_FALSE        false
#define SIGMA_NULL         nullptr

#endif // SIGMA_KERNEL_TYPES_H
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
=======
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
>>>>>>> origin/jules-13571719274074749109-6af93541
