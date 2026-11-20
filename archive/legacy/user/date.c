/* SigmaOS ELF32 user program: date — prints system uptime via SYS_GETTIME(44).
 * Format: "uptime HH:MM:SS.mmm". (Wall-clock RTC is not yet implemented;
 * this reports monotonic uptime — same semantic as `uptime -s` boot time.) */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void outc(char c) { sc3(SYS_WRITE_LEN, (u32)&c, 1, 0); }
static void outd(u32 v) {
    char t[12]; int i = 11; t[i] = 0;
    if (!v) { outc('0'); return; }
    while (v && i) { t[--i] = (char)('0' + v % 10); v /= 10; }
    outs(t + i);
}
void _start(void) {
    u32 sec, ms;
    __asm__ volatile(
        "movl $44, %%eax\n\tint $0x80\n\t"
        "movl %%eax, %0\n\tmovl %%edx, %1\n\t"
        : "=m"(sec), "=m"(ms) :: "eax", "edx", "memory");
    outs("uptime ");
    u32 h = sec / 3600, m = (sec / 60) % 60, s = sec % 60;
    if (h < 10) outc('0'); outd(h); outc(':');
    if (m < 10) outc('0'); outd(m); outc(':');
    if (s < 10) outc('0'); outd(s); outc('.');
    if (ms < 100) outc('0'); if (ms < 10) outc('0'); outd(ms);
    outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
