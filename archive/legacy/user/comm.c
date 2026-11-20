/* SigmaOS ELF32 user program: comm — compare two sorted files.
 * Usage: exec COMM.ELF [-1] [-2] [-3] <file1> <file2>
 * Suppress column 1 (file1-only), 2 (file2-only), or 3 (common).
 * Output: col1=file1-only, col2=file2-only, col3=common (tab-indented).
 * Inputs must be sorted. Line cap: 1024 lines x 160 chars per file.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"
static unsigned char fbuf[32768];
static char la[1024][160], lb[1024][160];
static int na = 0, nb = 0;

static int load(const char* path, char lines[][160], int* cnt) {
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) return -1;
    int total = 0, n = 0;
    /* read whole file */
    static unsigned char big[65536];
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)(big + total), 65536 - total);
        if (r <= 0) break;
        total += r;
        if (total >= 65536 || (u32)r < 65536 - (u32)(total - r)) break;
        if ((u32)r < 65536) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    int col = 0;
    for (int i = 0; i < total && n < 1024; i++) {
        if (big[i] == '\n') {
            if (col) { lines[n][col] = 0; n++; col = 0; }
        } else if (col < 150) lines[n][col++] = (char)big[i];
    }
    if (col && n < 1024) { lines[n][col] = 0; n++; }
    *cnt = n;
    return 0;
}

static int lc(const char* a, const char* b) {
    while (*a && *b) {
        if (*a != *b) return *a - *b;
        a++; b++;
    }
    return *a - *b;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    /* flags may precede or follow files: scan all args for -1/-2/-3 */
    int sup1 = 0, sup2 = 0, sup3 = 0;
    const char* fa = 0, *fb = 0;
    for (u32 i = 2; i <= argc; i++) {
        const char* a = (const char*)sp[i];
        if (!a) continue;
        if (a[0] == '-' && a[1] == '1' && !a[2]) sup1 = 1;
        else if (a[0] == '-' && a[1] == '2' && !a[2]) sup2 = 1;
        else if (a[0] == '-' && a[1] == '3' && !a[2]) sup3 = 1;
        else if (!fa) fa = a;
        else if (!fb) fb = a;
    }
    if (!fa || !fb) { uputs("usage: comm [-123] <file1> <file2>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    if (load(fa, la, &na) != 0) { uputs("comm: cannot open file1\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    if (load(fb, lb, &nb) != 0) { uputs("comm: cannot open file2\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int i = 0, j = 0;
    while (i < na && j < nb) {
        int c = lc(la[i], lb[j]);
        if (c < 0) { if (!sup1) { sc3(SYS_WRITE_LEN,(u32)la[i],(u32)u32strlen(la[i]),0); sc3(SYS_WRITE_LEN,(u32)"\n",1,0); } i++; }
        else if (c > 0) { if (!sup2) { sc3(SYS_WRITE_LEN,(u32)"\t",1,0); sc3(SYS_WRITE_LEN,(u32)lb[j],(u32)u32strlen(lb[j]),0); sc3(SYS_WRITE_LEN,(u32)"\n",1,0); } j++; }
        else { if (!sup3) { sc3(SYS_WRITE_LEN,(u32)"\t\t",2,0); sc3(SYS_WRITE_LEN,(u32)la[i],(u32)u32strlen(la[i]),0); sc3(SYS_WRITE_LEN,(u32)"\n",1,0); } i++; j++; }
    }
    while (i < na) { if (!sup1) { sc3(SYS_WRITE_LEN,(u32)la[i],(u32)u32strlen(la[i]),0); sc3(SYS_WRITE_LEN,(u32)"\n",1,0); } i++; }
    while (j < nb) { if (!sup2) { sc3(SYS_WRITE_LEN,(u32)"\t",1,0); sc3(SYS_WRITE_LEN,(u32)lb[j],(u32)u32strlen(lb[j]),0); sc3(SYS_WRITE_LEN,(u32)"\n",1,0); } j++; }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
