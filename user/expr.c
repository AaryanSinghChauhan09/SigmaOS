/* SigmaOS ELF32 user program: expr — integer arithmetic + comparisons.
 * Usage: exec EXPR.ELF <a> <op> <b>   op: + - * / % = != < <= > >=
 * SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

static int atoi_(const char* s) {
    int neg = 0, v = 0;
    if (*s == '-') { neg = 1; s++; }
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return neg ? -v : v;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 4) { uputs("usage: expr <a> <op> <b>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int a = atoi_((const char*)sp[2]);
    const char* op = (const char*)sp[3];
    int b = atoi_((const char*)sp[4]);
    int r = 0;
    if (op[0] == '+' && !op[1]) r = a + b;
    else if (op[0] == '-' && !op[1]) r = a - b;
    else if (op[0] == '*' && !op[1]) r = a * b;
    else if (op[0] == '/' && !op[1]) r = b ? a / b : 0;
    else if (op[0] == '%' && !op[1]) r = b ? a % b : 0;
    else if (op[0] == '=' && !op[1]) r = (a == b);
    else if (op[0] == '!' && op[1] == '=') r = (a != b);
    else if (op[0] == '<' && op[1] == '=') r = (a <= b);
    else if (op[0] == '>' && op[1] == '=') r = (a >= b);
    else if (op[0] == '<' && !op[1]) r = (a < b);
    else if (op[0] == '>' && !op[1]) r = (a > b);
    else { uputs("expr: bad op\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    uputn((u32)r);
    uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
