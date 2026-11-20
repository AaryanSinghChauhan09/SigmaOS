/* SigmaOS ELF32 user program: pr — paginate text with headers.
 * Usage: exec PR.ELF <file> [lines_per_page]
 * Default 66 lines/page, 5-line header (date, file, page), form feeds. */
#include "lib.h"
static unsigned char buf[32768];
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void outn(u32 v) { uputn(v); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    int lpp = 66;
    if (argc > 2) { int v = 0; for (const char* p = (const char*)sp[4]; *p >= '0' && *p <= '9'; p++) v = v*10 + *p - '0'; if (v > 10) lpp = v; }
    if (!path) { uputs("usage: pr <file> [lines]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("pr: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int total = 0;
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)(buf + total), sizeof buf - total);
        if (r <= 0) break;
        total += r;
        if (total >= (int)sizeof buf || (u32)r < sizeof buf) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    int page = 1, line_in_page = 0;
    /* header: blank lines then centered-ish title */
    for (int i = 0; i < 3; i++) { outs("\n"); line_in_page++; }
    outs("SigmaOS pr — "); outs(path); outs(" — Page "); outn((u32)page); outs("\n");
    line_in_page++;
    outs("\n"); line_in_page++;
    int i = 0;
    while (i < total) {
        if (line_in_page >= lpp - 5) {
            /* form feed to next page */
            outs("\f");
            page++;
            line_in_page = 0;
            for (int k = 0; k < 3; k++) { outs("\n"); line_in_page++; }
            outs("SigmaOS pr — "); outs(path); outs(" — Page "); outn((u32)page); outs("\n");
            line_in_page++;
            outs("\n"); line_in_page++;
            continue;
        }
        /* emit one line */
        int st = i;
        while (i < total && buf[i] != '\n') i++;
        sc3(SYS_WRITE_LEN, (u32)(buf + st), (u32)(i - st), 0);
        outs("\n");
        i++;
        line_in_page++;
    }
    /* pad to page end */
    while (line_in_page < lpp) { outs("\n"); line_in_page++; }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
