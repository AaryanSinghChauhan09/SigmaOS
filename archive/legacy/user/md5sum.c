/* SigmaOS ELF32 user program: md5sum — RFC 1321 MD5 hex digest.
 * Usage: exec MD5SUM.ELF <file>
 * Table-driven MD5, little-endian, standard padding. */
#include "lib.h"
typedef unsigned int u32;
typedef unsigned char u8;

static const u32 K[64] = {
0xd76aa478,0xe8c7b756,0x242070db,0xc1bdceee,0xf57c0faf,0x4787c62a,0xa8304613,0xfd469501,
0x698098d8,0x8b44f7af,0xffff5bb1,0x895cd7be,0x6b901122,0xfd987193,0xa679438e,0x49b40821,
0xf61e2562,0xc040b340,0x265e5a51,0xe9b6c7aa,0xd62f105d,0x02441453,0xd8a1e681,0xe7d3fbc8,
0x21e1cde6,0xc33707d6,0xf4d50d87,0x455a14ed,0xa9e3e905,0xfcefa3f8,0x676f02d9,0x8d2a4c8a,
0xfffa3942,0x8771f681,0x6d9d6122,0xfde5380c,0xa4beea44,0x4bdecfa9,0xf6bb4b60,0xbebfbc70,
0x289b7ec6,0xeaa127fa,0xd4ef3085,0x04881d05,0xd9d4d039,0xe6db99e5,0x1fa27cf8,0xc4ac5665,
0xf4292244,0x432aff97,0xab9423a7,0xfc93a039,0x655b59c3,0x8f0ccc92,0xffeff47d,0x85845dd1,
0x6fa87e4f,0xfe2ce6e0,0xa3014314,0x4e0811a1,0xf7537e82,0xbd3af235,0x2ad7d2bb,0xeb86d391};
static const int S[64] = {
7,12,17,22,7,12,17,22,7,12,17,22,7,12,17,22,
5,9,14,20,5,9,14,20,5,9,14,20,5,9,14,20,
4,11,16,23,4,11,16,23,4,11,16,23,4,11,16,23,
6,10,15,21,6,10,15,21,6,10,15,21,6,10,15,21};

static u32 rol(u32 x, int n) { return (x << n) | (x >> (32 - n)); }

static void md5(const u8* msg, u32 len, u8 out[16]) {
    u32 a0 = 0x67452301, b0 = 0xefcdab89, c0 = 0x98badcfe, d0 = 0x10325476;
    /* process 64-byte blocks; msg includes padding already */
    u32 total = len;
    for (u32 off = 0; off < total; off += 64) {
        u32 M[16];
        for (int i = 0; i < 16; i++) {
            u32 v = 0;
            for (int j = 0; j < 4; j++) v |= (u32)msg[off + i * 4 + j] << (j * 8);
            M[i] = v;
        }
        u32 A = a0, B = b0, C = c0, D = d0;
        for (int i = 0; i < 64; i++) {
            u32 F; int g;
            if (i < 16) { F = (B & C) | (~B & D); g = i; }
            else if (i < 32) { F = (D & B) | (~D & C); g = (5 * i + 1) & 15; }
            else if (i < 48) { F = B ^ C ^ D; g = (3 * i + 5) & 15; }
            else { F = C ^ (B | ~D); g = (7 * i) & 15; }
            u32 tmp = D;
            D = C; C = B;
            B = B + rol(A + F + K[i] + M[g], S[i]);
            A = tmp;
        }
        a0 += A; b0 += B; c0 += C; d0 += D;
    }
    u32 st[4] = {a0, b0, c0, d0};
    for (int i = 0; i < 4; i++)
        for (int j = 0; j < 4; j++)
            out[i * 4 + j] = (u8)(st[i] >> (j * 8));
}

static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static const char* HEX = "0123456789abcdef";

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { outs("usage: md5sum <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { outs("md5sum: open failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    /* bounded: files up to ~56 KiB fit one padded buffer */
    int fsz = sc3(SYS_FSTAT, fd, 0, 0);
    if (fsz > 57344 - 128) { outs("md5sum: file too large\n"); sc3(SYS_CLOSE,fd,0,0); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    static u8 buf[57344];
    u32 total = 0;
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)(buf + total), 57344 - total - 64);
        if (r <= 0) break;
        total += r;
        if (total > 57344 - 64) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    /* MD5 padding: 0x80, zeros, 8-byte LE bit-length */
    u64 bits = (u64)total * 8;
    buf[total++] = 0x80;
    while (total % 64 != 56) buf[total++] = 0;
    for (int i = 0; i < 8; i++) buf[total++] = (u8)(bits >> (i * 8));
    u8 digest[16];
    md5(buf, total, digest);
    for (int i = 0; i < 16; i++) {
        char c[2] = { HEX[digest[i] >> 4], HEX[digest[i] & 15] };
        outs(c);
    }
    outs("  "); outs(path); outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
