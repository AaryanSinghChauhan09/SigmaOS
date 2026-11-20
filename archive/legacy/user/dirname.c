/* SigmaOS ELF32 user program: dirname — strip last component.
 * Usage: exec DIRNAME.ELF <path>
 * SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* p = (argc > 1) ? (const char*)sp[2] : 0;
    if (!p) { uputs("usage: dirname <path>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int n = 0; while (p[n]) n++;
    while (n > 1 && p[n-1] == '/') n--;
    int s = n - 1;
    while (s >= 0 && p[s] != '/') s--;
    if (s < 0) { uputs(".\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    /* dir part = p[0..s], strip trailing slashes */
    int e = s;
    while (e > 0 && p[e-1] == '/') e--;
    if (e == 0) { uputs("/\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    sc3(SYS_WRITE_LEN, (u32)p, e, 0);
    uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
