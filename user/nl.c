/* SigmaOS ELF32 user program: nl — number lines of a file.
 * Usage: exec NL.ELF <file>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { uputs("usage: nl <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("nl: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    u32 lineno = 1, col = 0;
    int at_line_start = 1;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (at_line_start) {
                uputn(lineno); uputs("\t");
                lineno++; at_line_start = 0;
            }
            sc3(SYS_WRITE_LEN, (u32)&buf[i], 1, 0);
            if (buf[i] == '\n') at_line_start = 1;
        }
        (void)col;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
