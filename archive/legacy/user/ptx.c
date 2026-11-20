/* SigmaOS ELF32 user program: ptx — permuted index of words.
 * Usage: exec PTX.ELF <file>
 * For each word in each line, emits "word <tab> N: line".
 * Up to 256 lines, words up to 40 chars, 8 words/line.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"
static char lines[256][160];
static int nl = 0;
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void outn(u32 v) { uputn(v); }

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
    for (int i = 0; i < total && n < 256; i++) {
        if (big[i] == '\n') { if (col) { lines[n][col] = 0; n++; col = 0; } }
        else if (col < 150) lines[n][col++] = (char)big[i];
    }
    if (col && n < 256) { lines[n][col] = 0; n++; }
    nl = n;
    return 0;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { uputs("usage: ptx <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    if (load(path) != 0) { uputs("ptx: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    for (int i = 0; i < nl; i++) {
        char* s = lines[i];
        int len = u32strlen(s);
        int pos = 0;
        while (pos < len) {
            /* skip separators */
            while (pos < len && (s[pos] == ' ' || s[pos] == '\t')) pos++;
            if (pos >= len) break;
            int ws = pos;
            while (pos < len && s[pos] != ' ' && s[pos] != '\t') pos++;
            int wl = pos - ws;
            if (wl > 0) {
                /* emit word + tab + "N: line" */
                for (int k = 0; k < wl; k++) { char c = s[ws+k]; sc3(SYS_WRITE_LEN,(u32)&c,1,0); }
                sc3(SYS_WRITE_LEN, (u32)"\t", 1, 0);
                outn((u32)(i + 1));
                outs(": ");
                outs(s);
                outs("\n");
            }
        }
    }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
