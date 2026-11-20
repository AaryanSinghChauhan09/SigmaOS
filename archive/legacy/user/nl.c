/* SigmaOS ELF32 user program: nl — number lines of a file.
 * Usage: exec NL.ELF [-bt] <file>
 * Default numbers every line; -bt numbers only lines with a
 * non-blank character. Bounded: 512 lines x 255 chars. */
#include "lib.h"
#define MAXL 512
#define MAXLL 255
static char lines[MAXL][MAXLL + 1];
static int nlines;

static int is_blank(const char* s) {
    while (*s) {
        if (*s != ' ' && *s != '\t' && *s != '\r') return 0;
        s++;
    }
    return 1;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    int nonblank_only = 0;
    u32 ai = 2;
    while (ai <= argc) {
        const char* a = (const char*)sp[ai];
        if (!a || a[0] != '-' || a[1] != 'b' || a[2] != 't' || a[3]) break;
        nonblank_only = 1; ai++;
    }
    const char* path = (ai <= argc) ? (const char*)sp[ai] : 0;
    if (!path) { uputs("usage: nl [-bt] <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("nl: open failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    static char buf[4096];
    static char cur[MAXLL];
    int cl = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (buf[i] == '\n') {
                if (nlines < MAXL) {
                    cur[cl < MAXLL ? cl : MAXLL] = 0;
                    int l = 0; while (cur[l] && l <= MAXLL) { lines[nlines][l] = cur[l]; l++; }
                    lines[nlines][l] = 0; nlines++;
                }
                cl = 0;
            } else if (cl < MAXLL - 1) cur[cl++] = buf[i];
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (cl > 0 && nlines < MAXL) { cur[cl] = 0; int l = 0; while (cur[l] && l <= MAXLL) { lines[nlines][l] = cur[l]; l++; } lines[nlines][l] = 0; nlines++; }
    u32 lineno = 1;
    for (int i = 0; i < nlines; i++) {
        if (!nonblank_only || !is_blank(lines[i])) {
            uputn(lineno); uputs("\t");
            lineno++;
        }
        uputs(lines[i]); uputs("\n");
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
