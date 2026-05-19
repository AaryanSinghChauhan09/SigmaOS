#ifndef SIGMA_TYPES_H
#define SIGMA_TYPES_H

#include "sigma_kernel_types.h"

/* Standard type compatibility mapping */
typedef sigma_u8   uint8_t;
typedef sigma_u16  uint16_t;
typedef sigma_u32  uint32_t;
typedef sigma_u64  uint64_t;
typedef sigma_i8   int8_t;
typedef sigma_i16  int16_t;
typedef sigma_i32  int32_t;
typedef sigma_i64  int64_t;

#ifndef _SIZE_T_DEFINED
#define _SIZE_T_DEFINED
typedef sigma_usize size_t;
#endif

#ifndef _SSIZE_T_DEFINED
#define _SSIZE_T_DEFINED
typedef sigma_isize ssize_t;
#endif

#endif /* SIGMA_TYPES_H */
