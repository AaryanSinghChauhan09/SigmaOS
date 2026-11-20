/* SigmaOS ELF32 user program: uname — kernel info via SYS_UNAME.
 * Usage: exec UNAME.ELF [-a|-s|-n|-r|-m|-v]
 * SYS_UNAME=22 ebx=buf ecx=len -> string ; SYS_WRITE_LEN=8 ; SYS_EXIT=0 */
#include "lib.h"
enum { SYS_UNAME = 22 };
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* flags = (argc > 1) ? (const char*)sp[2] : "-a";
    static char u[128];
    int n = sc3(SYS_UNAME, (u32)u, sizeof u - 1, 0);
    if (n <= 0) { uputs("uname: failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    /* parse "SigmaOS sigmaos 0.1.0 i386 GNU/Linux" fields */
    char* f[8]; int nf = 0;
    f[nf++] = u;
    for (int i = 0; i < n && nf < 8; i++) if (u[i] == ' ') { u[i] = 0; f[nf++] = u + i + 1; }
    u[n] = 0;
    /* fields: 0=SigmaOS 1=sigmaos 2=0.1.0 3=i386 4=GNU/Linux */
    int all = (flags[0] == '-' && flags[1] == 'a') || argc == 1;
    int first = 1;
    for (int i = 1; i < flags[0] ? 0 : 0; i++) {}
    /* simple: print all or selected */
    int want[5] = {0,0,0,0,0};
    if (all) { for (int i = 0; i < 5; i++) want[i] = 1; }
    else {
        for (const char* p = flags + 1; *p; p++) {
            if (*p == 's') want[0] = 1;
            else if (*p == 'n') want[1] = 1;
            else if (*p == 'r') want[2] = 1;
            else if (*p == 'm') want[3] = 1;
            else if (*p == 'v') want[4] = 1;
        }
    }
    for (int i = 0; i < 5 && i < nf; i++) {
        if (!want[i]) continue;
        if (!first) uputs(" ");
        uputs(f[i]);
        first = 0;
    }
    uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
