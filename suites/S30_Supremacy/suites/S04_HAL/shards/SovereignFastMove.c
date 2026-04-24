/*
 * =========================================================================
 * S SIGMAOS: S04_HAL — SovereignFastMove.c
 * =========================================================================
 * Mission: High-Performance Memory Operations Shard.
 * Design: Branch-Prediction Aware, Unrolled loops for industrial throughput.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"

/**
 * @brief High-performance block move with manual loop unrolling for efficiency.
 */
void Sovereign_Fastsigma_memcpy(void* dest, const void* src, sigma_sz_t size) {
    sigma_u64* d64 = (sigma_u64*)dest;
    const sigma_u64* s64 = (const sigma_u64*)src;
    
    // Unroll 8x to saturate the pipeline
    while (size >= 64) {
        d64[0] = s64[0];
        d64[1] = s64[1];
        d64[2] = s64[2];
        d64[3] = s64[3];
        d64[4] = s64[4];
        d64[5] = s64[5];
        d64[6] = s64[6];
        d64[7] = s64[7];
        d64 += 8;
        s64 += 8;
        size -= 64;
    }
    
    // Remaining bytes
    sigma_u8* d8 = (sigma_u8*)d64;
    const sigma_u8* s8 = (const sigma_u8*)s64;
    while (size--) {
        *d8++ = *s8++;
    }
}

void Sovereign_Fastsigma_memset(void* dest, sigma_u8 val, sigma_sz_t size) {
    sigma_u64 v64 = (sigma_u64)val | ((sigma_u64)val << 8) | ((sigma_u64)val << 16) | ((sigma_u64)val << 24);
    v64 |= (v64 << 32);
    
    sigma_u64* d64 = (sigma_u64*)dest;
    while (size >= 64) {
        d64[0] = v64; d64[1] = v64; d64[2] = v64; d64[3] = v64;
        d64[4] = v64; d64[5] = v64; d64[6] = v64; d64[7] = v64;
        d64 += 8; size -= 64;
    }
    
    sigma_u8* d8 = (sigma_u8*)d64;
    while (size--) {
        *d8++ = val;
    }
}
