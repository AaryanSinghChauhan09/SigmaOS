#ifndef SIGMA_UEFI_SUPPORT_H
#define SIGMA_UEFI_SUPPORT_H

#include <stdint.h>

// UEFI System Table abstraction
typedef struct {
    uint64_t Signature;
    uint32_t Revision;
    uint32_t HeaderSize;
    uint32_t CRC32;
    uint32_t Reserved;
} UefiTableHeader;

// Initializes UEFI Runtime Services
int init_uefi_support(void);

// Get UEFI memory map
int uefi_get_memory_map(void);

#endif // SIGMA_UEFI_SUPPORT_H
