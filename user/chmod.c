/* SigmaOS ELF32 user program: chmod — set/clear read-only attribute.
 * Usage: exec CHMOD.ELF <file> [a-w]   (a-w clears RO, a+w sets RO)
 * SYS_CHMOD=27 ebx=name ecx=mode ; SYS_EXIT=0 ; SYS_WRITE_LEN=8 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: chmod <file> [a-w]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    u32 mode = 0;   /* default: clear read-only */
    if (argc > 2) {
        const char* m = (const char*)sp[4];
        if (m[0] == 'u' && m[1] == '+') mode = 1;   /* u+w -> set RO */
    }
    if (sc3(SYS_CHMOD, sp[2], mode, 0) != 0) { uputs("chmod: failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
