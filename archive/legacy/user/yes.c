/* SigmaOS ELF32 user program: yes — print string repeatedly (bounded 10000).
 * Usage: exec YES.ELF [string]
 * SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* s = (argc > 1) ? (const char*)sp[2] : "y";
    for (int i = 0; i < 10000; i++) {
        uputs(s);
        uputs("\n");
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
