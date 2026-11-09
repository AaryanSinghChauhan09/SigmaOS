/* SigmaOS ELF32 user program: wc — lines/words/bytes of a file.
 * Usage: exec WC.ELF <path>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"
#define puts_ uputs
#define putn uputn
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { puts_("usage: wc <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { puts_("wc: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    u32 lines = 0, words = 0, bytes = 0, inword = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        bytes += n;
        for (int i = 0; i < n; i++) {
            char c = buf[i];
            if (c == '\n') lines++;
            if (c == ' ' || c == '\t' || c == '\n' || c == '\r') inword = 0;
            else if (!inword) { inword = 1; words++; }
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    putn(lines); puts_(" "); putn(words); puts_(" "); putn(bytes); puts_(" "); puts_(path); puts_("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
