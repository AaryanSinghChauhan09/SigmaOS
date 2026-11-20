/* SigmaOS ELF32 user program: unexpand — leading spaces to tabs.
 * Usage: exec UNEXPAND.ELF <file> [tabstop]   (default 8)
 * Converts runs of spaces at line start into tabs. */
#include "lib.h"
static void emit(const char* s, u32 n) { sc3(SYS_WRITE_LEN, (u32)s, n, 0); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    int ts = 8;
    if (argc > 2) {
        int v = 0;
        for (const char* a = (const char*)sp[3]; *a >= '0' && *a <= '9'; a++) v = v * 10 + (*a - '0');
        if (v > 0) ts = v;
    }
    if (!path) { uputs("usage: unexpand <file> [tabstop]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("unexpand: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static unsigned char buf[4096];
    int at_line_start = 1;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        int i = 0;
        while (i < n) {
            if (at_line_start && buf[i] == ' ') {
                /* count leading spaces */
                int cnt = 0;
                while (i < n && buf[i] == ' ') { cnt++; i++; }
                int tabs = cnt / ts, rem = cnt % ts;
                for (int t = 0; t < tabs; t++) emit("\t", 1);
                for (int t = 0; t < rem; t++) emit(" ", 1);
                at_line_start = 0;
                continue;
            }
            if (buf[i] == '\n') { emit("\n", 1); at_line_start = 1; i++; continue; }
            emit((const char*)(buf + i), 1);
            at_line_start = 0;
            i++;
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
