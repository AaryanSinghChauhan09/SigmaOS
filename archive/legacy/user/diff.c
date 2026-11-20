/* SigmaOS ELF32 user program: diff — line comparison of two files.
 * Usage: exec DIFF.ELF <file1> <file2>
 * Simple Myers-lite: reports "< line" (file1 only), "> line" (file2 only).
 * Uses LCS via dynamic programming on line hashes is heavy; instead
 * does a greedy two-pointer with lookahead-1 (works for typical edits).
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 ; EXIT=0
 * Exit 0 = identical, 1 = differ, 2 = error. */
#include "lib.h"
static char la[512][160], lb[512][160];
static int na = 0, nb = 0;

static int load(const char* path, char lines[][160], int* cnt) {
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
        else if (col < 150) lines[n][col++] = (char)big[i];
    }
    if (col && n < 512) { lines[n][col] = 0; n++; }
    *cnt = n;
    return 0;
}
static int eq(const char* a, const char* b) {
    while (*a && *a == *b) { a++; b++; }
    return *a == *b;
}
static void emit(const char* tag, const char* s) {
    sc3(SYS_WRITE_LEN, (u32)tag, u32strlen(tag), 0);
    sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0);
    sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0);
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 3) { uputs("usage: diff <file1> <file2>\n"); sc3(SYS_EXIT,2,0,0); __builtin_unreachable(); }
    if (load((const char*)sp[2], la, &na) != 0) { uputs("diff: cannot open file1\n"); sc3(SYS_EXIT,2,0,0); __builtin_unreachable(); }
    if (load((const char*)sp[3], lb, &nb) != 0) { uputs("diff: cannot open file2\n"); sc3(SYS_EXIT,2,0,0); __builtin_unreachable(); }
    int i = 0, j = 0, diffs = 0;
    while (i < na && j < nb) {
        if (eq(la[i], lb[j])) { i++; j++; continue; }
        /* lookahead: is la[i] == lb[j+1]? (deletion) or la[i+1]==lb[j]? (insertion) */
        if (j + 1 < nb && eq(la[i], lb[j+1])) {
            emit("> ", lb[j]); j++; diffs++;
        } else if (i + 1 < na && eq(la[i+1], lb[j])) {
            emit("< ", la[i]); i++; diffs++;
        } else {
            emit("< ", la[i]); emit("> ", lb[j]); i++; j++; diffs += 2;
        }
    }
    while (i < na) { emit("< ", la[i]); i++; diffs++; }
    while (j < nb) { emit("> ", lb[j]); j++; diffs++; }
    sc3(SYS_EXIT, diffs ? 1 : 0, 0, 0); __builtin_unreachable();
}
