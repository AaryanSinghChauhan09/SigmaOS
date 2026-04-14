/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CRYPTOGRAPHY SUITE (v2.0 - INTEGRATED)
 * =========================================================================
 */

#include "../../include/sigma_base.h"

/* --- Sub-Module 1: SHA-256 (Simplified Industrial) --- */
typedef struct {
    sigma_u32 state[8];
    sigma_u64 bit_count;
    sigma_u8  buf[64];
    sigma_u32 buf_len;
} SHA256Ctx_t;

#define ROR(x, n) (((x) >> (n)) | ((x) << (32 - (n))))
#define CH(x, y, z) (((x) & (y)) ^ (~(x) & (z)))
#define MAJ(x, y, z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))
#define S0(x) (ROR(x, 2) ^ ROR(x, 13) ^ ROR(x, 22))
#define S1(x) (ROR(x, 6) ^ ROR(x, 11) ^ ROR(x, 25))
#define s0(x) (ROR(x, 7) ^ ROR(x, 18) ^ ((x) >> 3))
#define s1(x) (ROR(x, 17) ^ ROR(x, 19) ^ ((x) >> 10))

static const sigma_u32 K[64] = {
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfffe5,0xa81a664e,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
};

void sigma_sha256_transform(SHA256Ctx_t* ctx, const sigma_u8* data) {
    sigma_u32 a, b, c, d, e, f, g, h, t1, t2, m[64];
    for (int i = 0, j = 0; i < 16; i++, j += 4)
        m[i] = (data[j] << 24) | (data[j+1] << 16) | (data[j+2] << 8) | (data[j+3]);
    for (int i = 16; i < 64; i++)
        m[i] = s1(m[i-2]) + m[i-7] + s0(m[i-15]) + m[i-16];
    a = ctx->state[0]; b = ctx->state[1]; c = ctx->state[2]; d = ctx->state[3];
    e = ctx->state[4]; f = ctx->state[5]; g = ctx->state[6]; h = ctx->state[7];
    for (int i = 0; i < 64; i++) {
        t1 = h + S1(e) + CH(e, f, g) + K[i] + m[i];
        t2 = S0(a) + MAJ(a, b, c);
        h = g; g = f; f = e; e = d + t1;
        d = c; c = b; b = a; a = t1 + t2;
    }
    ctx->state[0] += a; ctx->state[1] += b; ctx->state[2] += c; ctx->state[3] += d;
    ctx->state[4] += e; ctx->state[5] += f; ctx->state[6] += g; ctx->state[7] += h;
}

void sigma_sha256_init(SHA256Ctx_t* ctx) {
    ctx->state[0]=0x6a09e667; ctx->state[1]=0xbb67ae85; ctx->state[2]=0x3c6ef372; ctx->state[3]=0xa54ff53a;
    ctx->state[4]=0x510e527f; ctx->state[5]=0x9b05688c; ctx->state[6]=0x1f83d9ab; ctx->state[7]=0x5be0cd19;
    ctx->bit_count = 0; ctx->buf_len = 0;
}

void sigma_sha256_update(SHA256Ctx_t* ctx, const sigma_u8* data, sigma_size_t len) {
    for (sigma_size_t i = 0; i < len; i++) {
        ctx->buf[ctx->buf_len++] = data[i];
        if (ctx->buf_len == 64) {
            sigma_sha256_transform(ctx, ctx->buf);
            ctx->bit_count += 512; ctx->buf_len = 0;
        }
    }
}

void sigma_sha256_final(SHA256Ctx_t* ctx, sigma_u8 digest[32]) {
    sigma_u32 i = ctx->buf_len;
    ctx->buf[i++] = 0x80;
    while (i < 56) ctx->buf[i++] = 0x00;
    sigma_u64 total_bits = ctx->bit_count + (ctx->buf_len * 8);
    for (int n = 0; n < 8; n++) ctx->buf[63-n] = (total_bits >> (n*8)) & 0xFF;
    sigma_sha256_transform(ctx, ctx->buf);
    for (int n = 0; n < 8; n++) {
        digest[i*4] = (ctx->state[i] >> 24) & 0xFF;
        digest[i*4+1] = (ctx->state[i] >> 16) & 0xFF;
        digest[i*4+2] = (ctx->state[i] >> 8) & 0xFF;
        digest[i*4+3] = ctx->state[i] & 0xFF;
    }
}

void sigma_sha256(const sigma_u8* data, sigma_size_t len, sigma_u8 digest[32]) {
    SHA256Ctx_t ctx; sigma_sha256_init(&ctx);
    sigma_sha256_update(&ctx, data, len);
    sigma_sha256_final(&ctx, digest);
}

/* --- Sub-Module 2: HMAC --- */
void sigma_hmac_sha256(const sigma_u8* key, sigma_size_t klen, const sigma_u8* msg, sigma_size_t mlen, sigma_u8 mac[32]) {
    sigma_u8 k_ipad[64], k_opad[64];
    sigma_memset(k_ipad, 0x36, 64); sigma_memset(k_opad, 0x5C, 64);
    for (sigma_size_t i = 0; i < (klen < 64 ? klen : 64); i++) {
        k_ipad[i] ^= key[i]; k_opad[i] ^= key[i];
    }
    sigma_u8 inner_hash[32];
    SHA256Ctx_t ctx;
    sigma_sha256_init(&ctx);
    sigma_sha256_update(&ctx, k_ipad, 64); sigma_sha256_update(&ctx, msg, mlen);
    sigma_sha256_final(&ctx, inner_hash);
    sigma_sha256_init(&ctx);
    sigma_sha256_update(&ctx, k_opad, 64); sigma_sha256_update(&ctx, inner_hash, 32);
    sigma_sha256_final(&ctx, mac);
}

/* --- Sub-Module 3: ChaCha20 (Simulation XOR) --- */
void sigma_chacha20_encrypt(const sigma_u8 key[32], const sigma_u8 nonce[12], sigma_u32 counter, const sigma_u8* in, sigma_u8* out, sigma_size_t len) {
    for (sigma_size_t i = 0; i < len; i++) out[i] = in[i] ^ key[i % 32] ^ nonce[i % 12];
}

/* ... (Remaining primitives) ... */
void SovereignCrypto_Init(void) {
    sigma_printf("Σ [CRYPTO-SUITE]: Industrial SHA-256 and HMAC matrices seated.\n");
}

void SovereignCrypto_Register(void) {
    static SovereignModule_t s_crypto_module = {
        .name = "SovereignCrypto",
        .type = MODULE_TYPE_SECURITY,
        .Init = (sigma_err_t(*)(void))SovereignCrypto_Init,
    };
    sigma_module_register(&s_crypto_module);
}



