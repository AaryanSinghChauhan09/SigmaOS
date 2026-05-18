#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// libsovereign_crypto : ChaCha20 Implementation
// Sovereign stack encrypted networking core
// ---------------------------------------------------------

#define ROTL(a,b) (((a) << (b)) | ((a) >> (32 - (b))))
#define QR(a, b, c, d) ( \
    a += b, d ^= a, d = ROTL(d, 16), \
    c += d, b ^= c, b = ROTL(b, 12), \
    a += b, d ^= a, d = ROTL(d, 8),  \
    c += d, b ^= c, b = ROTL(b, 7))

typedef struct {
    uint32_t state[16];
} chacha20_ctx_t;

void chacha20_init(chacha20_ctx_t *ctx, const uint8_t key[32], const uint8_t nonce[12], uint32_t counter) {
    ctx->state[0] = 0x61707865; // "expa"
    ctx->state[1] = 0x3320646e; // "nd 3"
    ctx->state[2] = 0x79622d32; // "2-by"
    ctx->state[3] = 0x6b206574; // "te k"
    
    for (int i = 0; i < 8; i++) {
        ctx->state[4 + i] = (key[i * 4]) | (key[i * 4 + 1] << 8) | (key[i * 4 + 2] << 16) | (key[i * 4 + 3] << 24);
    }
    
    ctx->state[12] = counter;
    for (int i = 0; i < 3; i++) {
        ctx->state[13 + i] = (nonce[i * 4]) | (nonce[i * 4 + 1] << 8) | (nonce[i * 4 + 2] << 16) | (nonce[i * 4 + 3] << 24);
    }
}

static void chacha20_block(chacha20_ctx_t *ctx, uint8_t output[64]) {
    uint32_t x[16];
    for (int i = 0; i < 16; i++) x[i] = ctx->state[i];
    
    for (int i = 0; i < 10; i++) {
        QR(x[0], x[4], x[8], x[12]);
        QR(x[1], x[5], x[9], x[13]);
        QR(x[2], x[6], x[10], x[14]);
        QR(x[3], x[7], x[11], x[15]);
        QR(x[0], x[5], x[10], x[15]);
        QR(x[1], x[6], x[11], x[12]);
        QR(x[2], x[7], x[8], x[13]);
        QR(x[3], x[4], x[9], x[14]);
    }
    
    for (int i = 0; i < 16; i++) {
        uint32_t val = x[i] + ctx->state[i];
        output[i * 4] = val & 0xFF;
        output[i * 4 + 1] = (val >> 8) & 0xFF;
        output[i * 4 + 2] = (val >> 16) & 0xFF;
        output[i * 4 + 3] = (val >> 24) & 0xFF;
    }
    ctx->state[12]++; // increment counter
}

void chacha20_encrypt(chacha20_ctx_t *ctx, const uint8_t *in, uint8_t *out, size_t len) {
    uint8_t block[64];
    while (len > 0) {
        chacha20_block(ctx, block);
        size_t chunk = (len < 64) ? len : 64;
        for (size_t i = 0; i < chunk; i++) {
            out[i] = in[i] ^ block[i];
        }
        in += chunk;
        out += chunk;
        len -= chunk;
    }
}
