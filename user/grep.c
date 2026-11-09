/* SigmaOS ELF32 user program: grep — print lines containing substring.
 * Usage: exec GREP.ELF <pattern> <file>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

static int lower(int c) { return ulower(c); }
static int contains(const char* h, int hlen, const char* n) {
    if (!*n) return 1;
    for (int i = 0; i <= hlen; i++) {
        int j = 0;
        while (n[j] && i + j < hlen && lower(h[i+j]) == lower(n[j])) j++;
        if (!n[j]) return 1;
    }
    return 0;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* pat = (argc > 1) ? (const char*)sp[2] : 0;
    const char* path = (argc > 2) ? (const char*)sp[3] : 0;
    if (!pat || !path) {
        uputs("usage: grep <pattern> <file>\n");
        sc3(SYS_EXIT, 0, 0, 0);
        __builtin_unreachable();
    }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("grep: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    static char line[1024];
    int ll = 0, matched = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            char c = buf[i];
            if (c == '\n' || ll >= 1000) {
                line[ll] = 0;
                if (contains(line, ll, pat)) {
                    matched++;
                    uputs(line); uputs("\n");
                }
                ll = 0;
                if (c != '\n') { /* line too long: skip rest handled by ll cap */ }
            } else line[ll++] = c;
        }
    }
    if (ll > 0) {
        line[ll] = 0;
        if (contains(line, ll, pat)) { matched++; uputs(line); uputs("\n"); }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    (void)matched;
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
