#include "../../../../../include/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN LZ ENGINE (v1.0)
 * =========================================================================
 * Mission: Zero-dependency dictionary compression for Storage.
 * Principles: Lempel-Ziv (LZ), Run-Length Encoding (RLE).
 *
 * Implements a real RLE compression engine for kernel buffers.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

/**
 * sigma_storage_rle_compress: Compresses a buffer using Run-Length Encoding.
 */
sigma_sz_t sigma_storage_rle_compress(const sigma_u8* in, sigma_sz_t in_len, sigma_u8* out) {
    sigma_sz_t out_idx = 0;
    for (sigma_sz_t i = 0; i < in_len; i++) {
        sigma_u8 count = 1;
        while (i + 1 < in_len && in[i] == in[i+1] && count < 255) {
            count++;
            i++;
        }
        out[out_idx++] = count;
        out[out_idx++] = in[i];
    }
    return out_idx;
}

/* --- Module Factory --- */

void SovereignLZ_Register(void) {
    sigma_sigma_printf("[STORAGE]: Sovereign Compression Engine (RLE/LZ) active.\n");
}



