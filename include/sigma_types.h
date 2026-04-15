#ifndef SIGMAOS_SIGMA_TYPES_H
#define SIGMAOS_SIGMA_TYPES_H

#include "base/sigma_integers.h"
#include "base/sigma_word.h"
#include "base/sigma_status.h"

/* Boolean */
typedef unsigned char      sigma_bool;
#define SIGMA_TRUE  ((sigma_bool)1)
#define SIGMA_FALSE ((sigma_bool)0)

/* Standard Aliases */
#ifndef SIGMA_EXCLUDE_STD_ALIASES
  #ifndef _STDINT_H
    typedef sigma_u8   uint8_t;
    typedef sigma_u32  uint32_t;
    typedef sigma_u64  uint64_t;
    typedef sigma_i64  int64_t;
  #endif
  #ifndef _STDBOOL_H
    typedef sigma_bool bool;
    #define true   SIGMA_TRUE
    #define false  SIGMA_FALSE
  #endif
  #ifndef _SIZE_T_DEFINED
    typedef sigma_size_t size_t;
  #endif
#endif

/* Architecture Helpers */
#define SIGMA_NULL ((void*)0)

#endif
