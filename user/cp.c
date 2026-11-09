/* SigmaOS ELF32 user program: cp — copy file.
 * Usage: exec CP.ELF <src> <dst>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_CREAT=11 SYS_APPEND=14 ; EXIT=0 */
#include "lib.h"
static unsigned char buf[16384];
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 3) { uputs("usage: cp <src> <dst>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* src = (const char*)sp[2];
    const char* dst = (const char*)sp[3];
    int fd = sc3(SYS_OPEN, (u32)src, 0, 0);
    if (fd < 0) { uputs("cp: cannot open "); uputs(src); uputs("\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    sc3(SYS_CREAT, (u32)dst, 0, 0);   /* create/truncate dst */
    u32 total = 0;
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (r <= 0) break;
        if (sc3(SYS_APPEND, (u32)dst, (u32)buf, (u32)r) != 0) {
            uputs("cp: write failed\n"); sc3(SYS_CLOSE,fd,0,0); sc3(SYS_EXIT,1,0,0); __builtin_unreachable();
        }
        total += (u32)r;
        if ((u32)r < sizeof buf) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    uputs("cp: "); uputn(total); uputs(" bytes -> "); uputs(dst); uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
