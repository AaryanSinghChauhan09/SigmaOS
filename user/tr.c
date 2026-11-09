/* SigmaOS ELF32 user program: tr — char translate/delete/squeeze.
 * Usage:
 *   exec TR.ELF -d <set> <file>     delete chars in set
 *   exec TR.ELF -s <set> <file>     squeeze repeats of set chars
 *   exec TR.ELF <from> <to> <file>  translate chars
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

static int in_set(const char* set, char c) {
    for (const char* p = set; *p; p++) if (*p == c) return 1;
    return 0;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 4) { uputs("usage: tr <-d|-s|from to> ... <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* a1 = (const char*)sp[2];
    const char* a2 = (const char*)sp[3];
    const char* a3 = (argc > 4) ? (const char*)sp[4] : 0;
    int del = 0, sqz = 0, trans = 0;
    const char* set = 0, *toset = 0, *file = 0;
    if (a1[0] == '-' && a1[1] == 'd' && !a1[2]) { del = 1; set = a2; file = a3; }
    else if (a1[0] == '-' && a1[1] == 's' && !a1[2]) { sqz = 1; set = a2; file = a3; }
    else if (a3) { trans = 1; set = a1; toset = a2; file = a3; }
    if (!file) { uputs("tr: bad args\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)file, 0, 0);
    if (fd < 0) { uputs("tr: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    int last = -1;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            char c = buf[i];
            if (del && in_set(set, c)) continue;
            if (sqz && in_set(set, c) && c == last) continue;
            if (trans) {
                int idx = 0;
                while (set[idx]) {
                    if (set[idx] == c) { c = toset[idx] ? toset[idx] : toset[0]; break; }
                    idx++;
                }
            }
            last = c;
            sc3(SYS_WRITE_LEN, (u32)&c, 1, 0);
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
