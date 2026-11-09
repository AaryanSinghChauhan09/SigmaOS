/* SigmaOS ELF32 user program: bc — integer RPN calculator.
 * Usage: exec BC.ELF [file]   (reads expressions; no args = stdin)
 * Grammar per line: numbers and + - * / % operators, = prints result.
 * Stack depth 32. Integer arithmetic only.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_GETCHAR=2 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"

static int st[32];
static int sp_ = 0;
static int fd_ = -1;
static unsigned char fbuf[4096];
static int fpos = 0, flen = 0, fdone = 0;

static int next_ch(void) {
    if (fd_ >= 0) {
        if (fpos >= flen) {
            if (fdone) return -1;
            int r = sc3(SYS_READ, fd_, (u32)fbuf, sizeof fbuf);
            if (r <= 0) { fdone = 1; return -1; }
            fpos = 0; flen = r;
            if ((u32)r < sizeof fbuf) fdone = 1;
        }
        return fbuf[fpos++];
    }
    int c = sc3(SYS_GETCHAR, 0, 0, 0);
    return (c < 0 || c == 0xFF) ? -1 : c;
}
static void push(int v) { if (sp_ < 32) st[sp_++] = v; }
static int pop(void) { return sp_ > 0 ? st[--sp_] : 0; }
static void emit_i(int v) {
    if (v < 0) { sc3(SYS_WRITE_LEN, (u32)"-", 1, 0); v = -v; }
    uputn((u32)v);
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc > 1) {
        fd_ = sc3(SYS_OPEN, sp[2], 0, 0);
        if (fd_ < 0) { uputs("bc: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    }
    int c, val = 0, innum = 0;
    while ((c = next_ch()) >= 0) {
        if (c >= '0' && c <= '9') { val = val * 10 + (c - '0'); innum = 1; continue; }
        if (innum) { push(val); val = 0; innum = 0; }
        if (c == '+') { int b = pop(), a = pop(); push(a + b); }
        else if (c == '-') { int b = pop(), a = pop(); push(a - b); }
        else if (c == '*') { int b = pop(), a = pop(); push(a * b); }
        else if (c == '/') { int b = pop(), a = pop(); push(b ? a / b : 0); }
        else if (c == '%') { int b = pop(), a = pop(); push(b ? a % b : 0); }
        else if (c == '=') { emit_i(pop()); sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0); }
        else if (c == 'p') { emit_i(st[sp_-1]); sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0); }
        else if (c == 'c') { sp_ = 0; }
        else if (c == 'q') break;
        /* other chars ignored */
    }
    if (fd_ >= 0) sc3(SYS_CLOSE, fd_, 0, 0);
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
