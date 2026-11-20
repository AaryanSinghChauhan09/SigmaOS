/* SigmaOS ELF32 user program: uptime — seconds since boot.
 * SYS_TICKS=3 -> PIT ticks (100 Hz assumed); prints s/m/h.
 * SYS_WRITE_LEN=8 ; SYS_EXIT=0 */
#include "lib.h"
void _start(void) {
    u32 t = (u32)sc3(SYS_TICKS, 0, 0, 0);
    u32 sec = t / 100;
    u32 h = sec / 3600, m = (sec / 60) % 60, s = sec % 60;
    uputs("up "); uputn(h); uputs("h "); uputn(m); uputs("m "); uputn(s); uputs("s\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
