#ifndef SIGMAOS_SUPREME_TYPES_H
#define SIGMAOS_SUPREME_TYPES_H

typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;
typedef signed int         sigma_i32;
typedef signed long long   sigma_i64;

typedef sigma_u64          sigma_uptr;
typedef sigma_u64          sigma_size_t;
typedef sigma_i64          sigma_ssize_t;
typedef signed int         sigma_err_t;

#ifndef SIGMA_EXCLUDE_STD_ALIASES
  #if !defined(_STDINT_H) && !defined(_STDINT_H_)
    typedef sigma_u8   uint8_t;
    typedef sigma_u32  uint32_t;
    typedef sigma_u64  uint64_t;
  #endif
  #if !defined(_SIZE_T) && !defined(_SIZE_T_DEFINED)
    typedef sigma_size_t size_t;
  #endif
#endif

#define SIGMA_NULL ((void*)0)

#endif
