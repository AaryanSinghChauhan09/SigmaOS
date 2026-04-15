/**
 * @file Sovereign_API_v1.h
 * @brief Phase 65: Architectural Contract.
 */

#ifndef SOVEREIGN_API_V1_H
#define SOVEREIGN_API_V1_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* Sovereign LibC Contract */
typedef struct {
    void (*print)(const char*);
    int (*atoi)(const char*);
    void* (*malloc)(sigma_sz_t);
} sigma_libc_api_v1_t;

/* Sovereign HAL Contract */
typedef struct {
    long (*syscall)(long num, ...);
} sigma_hal_api_v1_t;

#endif // SOVEREIGN_API_V1_H
