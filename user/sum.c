/* SigmaOS ELF32 user program: sum — BSD checksum + 1024-byte blocks.
 * Usage: exec SUM.ELF <file>
 * SYS_OPEN=4 SYS_READ=5 SYS_FSTAT=15 SYS_CLOSE=6 ; EXIT=0 */
#include "lib.h"
static unsigned char buf[8192];
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: sum <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* path = (const char*)sp[2];
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("sum: cannot open "); uputs(path); uputs("\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int sz = sc3(SYS_FSTAT, fd, 0, 0);
    u32 s1 = 0, s2 = 0;
    u32 total = 0;
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (r <= 0) break;
        for (int i = 0; i < r; i++) {
            s1 = (s1 + buf[i]) % 65521;
            s2 = (s2 + s1) % 65521;
        }
        total += (u32)r;
        if ((u32)r < sizeof buf) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    u32 ck = (s2 << 16) | s1;
    u32 blocks = (total + 1023) / 1024;
    uputn(ck); uputs(" "); uputn(blocks); uputs(" "); uputs(path); uputs("\n");
    (void)sz;
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
