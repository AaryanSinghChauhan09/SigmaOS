#include "SovereignChaCha20.h"

#define ROTR32(x,n) (((x)>>(n))|((x)<<(32-(n))))

#define CHACHA20_QUARTERROUND(a,b,c,d) \
    a+=b; d^=a; d=ROTR32(d,16);       \
    c+=d; b^=c; b=ROTR32(b,12);       \
    a+=b; d^=a; d=ROTR32(d, 8);       \
    c+=d; b^=c; b=ROTR32(b, 7)

static void chacha20_block(sigma_u32 state[16], sigma_u8 out[64]) {
    sigma_u32 x[16];
    sigma_memcpy(x, state, 64);
    for (int i = 0; i < 10; i++) {
        CHACHA20_QUARTERROUND(x[0],x[4],x[ 8],x[12]);
        CHACHA20_QUARTERROUND(x[1],x[5],x[ 9],x[13]);
        CHACHA20_QUARTERROUND(x[2],x[6],x[10],x[14]);
        CHACHA20_QUARTERROUND(x[3],x[7],x[11],x[15]);
        CHACHA20_QUARTERROUND(x[0],x[5],x[10],x[15]);
        CHACHA20_QUARTERROUND(x[1],x[6],x[11],x[12]);
        CHACHA20_QUARTERROUND(x[2],x[7],x[ 8],x[13]);
        CHACHA20_QUARTERROUND(x[3],x[4],x[ 9],x[14]);
    }
    for (int i = 0; i < 16; i++) {
        sigma_u32 v = x[i] + state[i];
        out[i*4+0]=(sigma_u8)(v      ); out[i*4+1]=(sigma_u8)(v>> 8);
        out[i*4+2]=(sigma_u8)(v>>16  ); out[i*4+3]=(sigma_u8)(v>>24);
    }
    state[12]++;
}

void sigma_chacha20_encrypt(const sigma_u8 key[32], const sigma_u8 nonce[12],
                             sigma_u32 counter,
                             const sigma_u8* in, sigma_u8* out, sigma_size_t len) {
    sigma_u32 state[16] = {
        0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
        (sigma_u32)key[ 0]|(sigma_u32)key[ 1]<<8|(sigma_u32)key[ 2]<<16|(sigma_u32)key[ 3]<<24,
        (sigma_u32)key[ 4]|(sigma_u32)key[ 5]<<8|(sigma_u32)key[ 6]<<16|(sigma_u32)key[ 7]<<24,
        (sigma_u32)key[ 8]|(sigma_u32)key[ 9]<<8|(sigma_u32)key[10]<<16|(sigma_u32)key[11]<<24,
        (sigma_u32)key[12]|(sigma_u32)key[13]<<8|(sigma_u32)key[14]<<16|(sigma_u32)key[15]<<24,
        (sigma_u32)key[16]|(sigma_u32)key[17]<<8|(sigma_u32)key[18]<<16|(sigma_u32)key[19]<<24,
        (sigma_u32)key[20]|(sigma_u32)key[21]<<8|(sigma_u32)key[22]<<16|(sigma_u32)key[23]<<24,
        (sigma_u32)key[24]|(sigma_u32)key[25]<<8|(sigma_u32)key[26]<<16|(sigma_u32)key[27]<<24,
        (sigma_u32)key[28]|(sigma_u32)key[29]<<8|(sigma_u32)key[30]<<16|(sigma_u32)key[31]<<24,
        counter,
        (sigma_u32)nonce[0]|(sigma_u32)nonce[1]<<8|(sigma_u32)nonce[2]<<16|(sigma_u32)nonce[3]<<24,
        (sigma_u32)nonce[4]|(sigma_u32)nonce[5]<<8|(sigma_u32)nonce[6]<<16|(sigma_u32)nonce[7]<<24,
        (sigma_u32)nonce[8]|(sigma_u32)nonce[9]<<8|(sigma_u32)nonce[10]<<16|(sigma_u32)nonce[11]<<24,
    };
    sigma_u8 keystream[64];
    sigma_size_t pos = 0;
    while (pos < len) {
        chacha20_block(state, keystream);
        sigma_size_t block_len = (len - pos < 64) ? (len - pos) : 64;
        for (sigma_size_t i = 0; i < block_len; i++) out[pos+i] = in[pos+i] ^ keystream[i];
        pos += block_len;
    }
}
