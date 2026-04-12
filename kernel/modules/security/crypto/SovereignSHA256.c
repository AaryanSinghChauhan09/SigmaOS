#include "SovereignSHA256.h"

static const sigma_u32 K[64] = {
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,
    0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,
    0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,
    0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,
    0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,
    0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,
    0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,
    0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,
    0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
};

#define ROTR32(x,n) (((x)>>(n))|((x)<<(32-(n))))
#define CH(x,y,z)  (((x)&(y))^(~(x)&(z)))
#define MAJ(x,y,z) (((x)&(y))^((x)&(z))^((y)&(z)))
#define S0(x) (ROTR32(x,2)^ROTR32(x,13)^ROTR32(x,22))
#define S1(x) (ROTR32(x,6)^ROTR32(x,11)^ROTR32(x,25))
#define s0(x) (ROTR32(x,7)^ROTR32(x,18)^((x)>>3))
#define s1(x) (ROTR32(x,17)^ROTR32(x,19)^((x)>>10))

static void sha256_compress(SHA256Ctx_t* ctx, const sigma_u8* block) {
    sigma_u32 W[64], a,b,c,d,e,f,g,h,T1,T2;
    for (int i = 0; i < 16; i++)
        W[i] = ((sigma_u32)block[i*4]<<24)|((sigma_u32)block[i*4+1]<<16)|
               ((sigma_u32)block[i*4+2]<<8)|(sigma_u32)block[i*4+3];
    for (int i = 16; i < 64; i++)
        W[i] = s1(W[i-2]) + W[i-7] + s0(W[i-15]) + W[i-16];
    a=ctx->state[0]; b=ctx->state[1]; c=ctx->state[2]; d=ctx->state[3];
    e=ctx->state[4]; f=ctx->state[5]; g=ctx->state[6]; h=ctx->state[7];
    for (int i = 0; i < 64; i++) {
        T1 = h + S1(e) + CH(e,f,g) + K[i] + W[i];
        T2 = S0(a) + MAJ(a,b,c);
        h=g; g=f; f=e; e=d+T1; d=c; c=b; b=a; a=T1+T2;
    }
    ctx->state[0]+=a; ctx->state[1]+=b; ctx->state[2]+=c; ctx->state[3]+=d;
    ctx->state[4]+=e; ctx->state[5]+=f; ctx->state[6]+=g; ctx->state[7]+=h;
}

void sigma_sha256_init(SHA256Ctx_t* ctx) {
    ctx->state[0]=0x6a09e667; ctx->state[1]=0xbb67ae85;
    ctx->state[2]=0x3c6ef372; ctx->state[3]=0xa54ff53a;
    ctx->state[4]=0x510e527f; ctx->state[5]=0x9b05688c;
    ctx->state[6]=0x1f83d9ab; ctx->state[7]=0x5be0cd19;
    ctx->bit_count = 0; ctx->buf_len = 0;
}

void sigma_sha256_update(SHA256Ctx_t* ctx, const sigma_u8* data, sigma_size_t len) {
    for (sigma_size_t i = 0; i < len; i++) {
        ctx->buf[ctx->buf_len++] = data[i];
        ctx->bit_count += 8;
        if (ctx->buf_len == 64) { sha256_compress(ctx, ctx->buf); ctx->buf_len = 0; }
    }
}

void sigma_sha256_final(SHA256Ctx_t* ctx, sigma_u8 digest[32]) {
    sigma_u64 bc = ctx->bit_count;
    sigma_u8 pad = 0x80;
    sigma_sha256_update(ctx, &pad, 1);
    while (ctx->buf_len != 56) {
        sigma_u8 z = 0x00;
        sigma_sha256_update(ctx, &z, 1);
    }
    for (int i = 7; i >= 0; i--) {
        sigma_u8 b = (sigma_u8)(bc >> (i * 8));
        sigma_sha256_update(ctx, &b, 1);
    }
    for (int i = 0; i < 8; i++) {
        digest[i*4+0] = (sigma_u8)(ctx->state[i] >> 24);
        digest[i*4+1] = (sigma_u8)(ctx->state[i] >> 16);
        digest[i*4+2] = (sigma_u8)(ctx->state[i] >>  8);
        digest[i*4+3] = (sigma_u8)(ctx->state[i]      );
    }
}

void sigma_sha256(const sigma_u8* data, sigma_size_t len, sigma_u8 digest[32]) {
    SHA256Ctx_t ctx;
    sigma_sha256_init(&ctx);
    sigma_sha256_update(&ctx, data, len);
    sigma_sha256_final(&ctx, digest);
}
