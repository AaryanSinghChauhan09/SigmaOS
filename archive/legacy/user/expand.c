/* SigmaOS ELF32 user program: expand — tabs to spaces.
 * Usage: exec EXPAND.ELF <file> [tabstop]   (default 8)
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    int ts = 8;
    if (argc > 2) {
        const char* a = (const char*)sp[3];
        int v = 0;
        while (*a >= '0' && *a <= '9') { v = v * 10 + (*a - '0'); a++; }
        if (v > 0) ts = v;
    }
    if (!path) { uputs("usage: expand <file> [tabstop]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("expand: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    int col = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            char c = buf[i];
            if (c == '\t') {
                int sp = ts - (col % ts);
                for (int k = 0; k < sp; k++) { sc3(SYS_WRITE_LEN, (u32)" ", 1, 0); }
                col += sp;
            } else {
                sc3(SYS_WRITE_LEN, (u32)&c, 1, 0);
                col = (c == '\n') ? 0 : col + 1;
            }
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
