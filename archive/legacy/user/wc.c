/* SigmaOS ELF32 user program: wc — line/word/byte counts.
 * Usage: exec WC.ELF [-l] [-w] [-c] [file...]
 * No files = stdin not supported (no stdin fd); requires file args.
 * Flags select which counts print; default all three. Totals with 2+ files. */
#include "lib.h"
#define puts_ uputs
#define putn uputn

static u32 show_l = 1, show_w = 1, show_c = 1;
static u32 tl, tw, tc, files;

static int count_file(const char* path) {
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) return -1;
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
    if (show_l) { putn(lines); puts_(" "); }
    if (show_w) { putn(words); puts_(" "); }
    if (show_c) { putn(bytes); puts_(" "); }
    puts_(path); puts_("\n");
    tl += lines; tw += words; tc += bytes; files++;
    return 0;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    u32 first = 2; /* sp[2] = argv[1] */
    while (first < argc + 1 && first - 1 < argc) {
        const char* a = (const char*)sp[first];
        if (!a || a[0] != '-') break;
        if (a[1] == 'l' && !a[2]) { show_w = 0; show_c = 0; }
        else if (a[1] == 'w' && !a[2]) { show_l = 0; show_c = 0; }
        else if (a[1] == 'c' && !a[2]) { show_l = 0; show_w = 0; }
        else if (a[1] == 'l' && a[2] == 'w' && !a[3]) { show_c = 0; }
        else if (a[1] == 'l' && a[2] == 'c' && !a[3]) { show_w = 0; }
        else if (a[1] == 'w' && a[2] == 'c' && !a[3]) { show_l = 0; }
        else break;
        first++;
    }
    if (first > argc) { puts_("usage: wc [-lwc] <file>...\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int bad = 0;
    for (u32 i = first; i <= argc; i++)
        if (count_file((const char*)sp[i]) != 0) bad = 1;
    if (files > 1) {
        if (show_l) { putn(tl); puts_(" "); }
        if (show_w) { putn(tw); puts_(" "); }
        if (show_c) { putn(tc); puts_(" "); }
        puts_("total\n");
    }
    sc3(SYS_EXIT, bad ? 1 : 0, 0, 0);
    __builtin_unreachable();
}
