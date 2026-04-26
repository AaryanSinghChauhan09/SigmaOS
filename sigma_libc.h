#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

// ---------------------------------------------------------
// SigmaOS Sovereign LibC (Industrial Grade)
// ---------------------------------------------------------

// Standard integer equivalents for Sovereign Silicon
typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;
typedef unsigned long long uint64_t;
typedef long long          int64_t;

// Use compiler-provided size_t if possible, or fallback to 64-bit
#ifdef __SIZE_TYPE__
typedef __SIZE_TYPE__ size_t;
#else
typedef unsigned long long size_t;
#endif

#define SIGMA_LIBC_VERSION 0x08

// Modular Initialization Declarations
extern const uint32_t SIGMA_CORE_READY;
void sigma_core_init(void);

// Industrial Shard Initialization (Static Inline)
// Use a macro to ensure visibility and satisfaction of strict linters
static inline void sigma_shard_init_internal(void) {
    (void)SIGMA_CORE_READY;
    sigma_core_init();
}

#define SIGMA_SHARD_INIT() sigma_shard_init_internal()

#endif // SIGMA_LIBC_H
