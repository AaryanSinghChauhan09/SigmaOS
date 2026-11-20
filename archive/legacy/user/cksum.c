/* SigmaOS ELF32 user program: cksum — POSIX CRC checksum + byte count.
 * Usage: exec CKSUM.ELF <file>
 * POSIX cksum: CRC-32 variant (poly 0x04C11DB7, init 0, no final xor,
 * length appended MSB-first before final complement of length only).
 * Output: "<crc> <bytes> <file>". */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { outs("usage: cksum <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { outs("cksum: open failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    u32 crc = 0;
    u32 nbytes = 0;
    static unsigned char buf[4096];
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            crc ^= (u32)buf[i] << 24;
            for (int b = 0; b < 8; b++)
                crc = (crc & 0x80000000u) ? (crc << 1) ^ 0x04C11DB7u : (crc << 1);
            nbytes++;
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    /* append length (nbytes) as 4 bytes MSB-first into CRC per POSIX */
    u32 len = nbytes;
    for (int i = 0; i < 4; i++) {
        crc ^= (u32)((len >> (24 - i * 8)) & 0xFF) << 24;
        for (int b = 0; b < 8; b++)
            crc = (crc & 0x80000000u) ? (crc << 1) ^ 0x04C11DB7u : (crc << 1);
    }
    uputn(crc); outs(" "); uputn(nbytes); outs(" "); outs(path); outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
