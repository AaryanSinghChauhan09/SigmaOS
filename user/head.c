/* SigmaOS ELF32 user program: head — print first N lines of a file.
 * Usage: exec HEAD.ELF <file> [n]
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    int max = 10;
    if (argc > 2) {
        const char* a = (const char*)sp[3];
        int v = 0;
        while (*a >= '0' && *a <= '9') { v = v * 10 + (*a - '0'); a++; }
        if (v > 0) max = v;
    }
    if (!path) { uputs("usage: head <file> [n]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("head: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    int lines = 0, done = 0;
    while (!done) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            sc3(SYS_WRITE_LEN, (u32)&buf[i], 1, 0);
            if (buf[i] == '\n') {
                lines++;
                if (lines >= max) { done = 1; break; }
            }
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
