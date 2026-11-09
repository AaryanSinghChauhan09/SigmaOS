/* SigmaOS ELF32 user program: column — align whitespace-separated columns.
 * Usage: exec COLUMN.ELF <file> [separator]
 * Reads up to 512 lines x 256 chars; splits on spaces/tabs (or sep char),
 * computes per-column max width, prints aligned table.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"
static char lines[512][256];
static char cells[512][8][64];
static int ncols[512], nlines = 0;
static int colw[8];
static int sepc = ' ';

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
    for (int i = 0; i < total && n < 512; i++) {
        if (big[i] == '\n') { if (col) { lines[n][col] = 0; n++; col = 0; } }
        else if (col < 250) lines[n][col++] = (char)big[i];
    }
    if (col && n < 512) { lines[n][col] = 0; n++; }
    nlines = n;
    /* split cells */
    for (int i = 0; i < nlines; i++) {
        int nc = 0, start = 0, len = u32strlen(lines[i]);
        for (int k = 0; k <= len && nc < 8; k++) {
            if (k == len || lines[i][k] == sepc || lines[i][k] == '\t') {
                int cl = k - start;
                if (cl > 60) cl = 60;
                for (int c = 0; c < cl; c++) cells[i][nc][c] = lines[i][start + c];
                cells[i][nc][cl] = 0;
                if (cl > colw[nc]) colw[nc] = cl;
                nc++;
                start = k + 1;
            }
        }
        ncols[i] = nc;
    }
    return 0;
}
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { uputs("usage: column <file> [sep]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    if (argc > 2) { const char* s = (const char*)sp[4]; if (*s) sepc = *s; }
    if (load(path) != 0) { uputs("column: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    for (int i = 0; i < nlines; i++) {
        for (int c = 0; c < ncols[i]; c++) {
            outs(cells[i][c]);
            if (c + 1 < ncols[i]) {
                int pad = colw[c] - u32strlen(cells[i][c]) + 2;
                for (int p = 0; p < pad; p++) outs(" ");
            }
        }
        outs("\n");
    }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
