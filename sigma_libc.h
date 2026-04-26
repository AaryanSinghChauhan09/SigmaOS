#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

// ---------------------------------------------------------
// SigmaOS Sovereign LibC (Modularised)
// ---------------------------------------------------------

// Standard integer equivalents for Sovereign Silicon
typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;
typedef unsigned long long uint64_t;
typedef long long          int64_t;
typedef unsigned long      size_t;

#define SIGMA_LIBC_VERSION 0x08

// Modular Initialization Declarations
extern const uint32_t SIGMA_CORE_READY;
void sigma_core_init(void);

// Industrial Shard Initialization (Static Inline)
static inline void SIGMA_SHARD_INIT(void) {
    (void)SIGMA_CORE_READY;
    sigma_core_init();
}

#endif // SIGMA_LIBC_H
