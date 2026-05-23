/*
 * Σ SigmaOS — sigma_date: Sovereign Date/Time Utility
 * Zero-Dependency: No libc time.h.
 * Absorbs: GNU coreutils date behavior.
 * Reads the hardware RTC (Real-Time Clock) via CMOS ports 0x70/0x71.
 */

typedef unsigned char u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

static inline u8 cmos_read(u8 reg) {
    __asm__ volatile ("outb %0, $0x70" : : "a"(reg));
    u8 val;
    __asm__ volatile ("inb $0x71, %0" : "=a"(val));
    return val;
}

/* BCD to binary */
static u8 bcd_to_bin(u8 bcd) {
    return ((bcd >> 4) * 10) + (bcd & 0x0F);
}

extern "C" int sigma_date_main(int argc, char** argv) {
    u8 sec  = bcd_to_bin(cmos_read(0x00));
    u8 min  = bcd_to_bin(cmos_read(0x02));
    u8 hour = bcd_to_bin(cmos_read(0x04));
    u8 day  = bcd_to_bin(cmos_read(0x07));
    u8 mon  = bcd_to_bin(cmos_read(0x08));
    u8 year = bcd_to_bin(cmos_read(0x09));

    sigma_vga_printf("20%u-%u-%u %u:%u:%u UTC\n",
        (unsigned)year, (unsigned)mon, (unsigned)day,
        (unsigned)hour, (unsigned)min, (unsigned)sec);
    return 0;
}
