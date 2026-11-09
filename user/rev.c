/* SigmaOS ELF32 user program: rev — reverse characters per line.
 * Usage: exec REV.ELF <file>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 1024
static char line[MAXL + 1];
static int ll;

static void flush_line(void) {
    if (ll > 0) {
        for (int i = ll - 1; i >= 0; i--)
            sc3(SYS_WRITE_LEN, (u32)&line[i], 1, 0);
    }
    sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0);
    ll = 0;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { uputs("usage: rev <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("rev: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (buf[i] == '\n') flush_line();
            else if (ll < MAXL) line[ll++] = buf[i];
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (ll > 0) flush_line();
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
