/* SigmaOS ELF32 user program: look — binary-search prefix lookup.
 * Usage: exec LOOK.ELF <prefix> <sorted_file>
 * File must be sorted; finds first line starting with prefix, prints
 * all consecutive matches (cap 64). Real binary search on line offsets. */
#include "lib.h"
static char lines[1024][160];
static int nl = 0;
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }

static int load(const char* path) {
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) return -1;
    static unsigned char big[65536];
    int total = 0;
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)(big + total), 65536 - total);
        if (r <= 0) break;
        total += r;
        if (total >= 65536 || (u32)r < 65536) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    int n = 0, col = 0;
    for (int i = 0; i < total && n < 1024; i++) {
        if (big[i] == '\n') { if (col) { lines[n][col] = 0; n++; col = 0; } }
        else if (col < 150) lines[n][col++] = (char)big[i];
    }
    if (col && n < 1024) { lines[n][col] = 0; n++; }
    nl = n;
    return 0;
}
static int cmp_pre(const char* line, const char* pre, int plen) {
    for (int i = 0; i < plen; i++) {
        if (line[i] != pre[i]) return line[i] - pre[i];
    }
    return 0;
}
static int cmp_str(const char* a, const char* b) {
    while (*a && *a == *b) { a++; b++; }
    return *a - *b;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 3) { uputs("usage: look <prefix> <sorted_file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* pre = (const char*)sp[2];
    const char* path = (const char*)sp[3];
    int plen = u32strlen(pre);
    if (load(path) != 0) { uputs("look: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    /* binary search for first line >= prefix */
    int lo = 0, hi = nl;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (cmp_pre(lines[mid], pre, plen) < 0) lo = mid + 1;
        else hi = mid;
    }
    /* print consecutive prefix matches */
    int shown = 0;
    for (int i = lo; i < nl && shown < 64; i++) {
        if (cmp_pre(lines[i], pre, plen) != 0) break;
        outs(lines[i]); outs("\n");
        shown++;
    }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
