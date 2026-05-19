/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: REAL-TIME CLOCK (RTC) / CMOS
 * =============================================================================
 * Inspired by: Linux kernel drivers/rtc/rtc-cmos.c
 *              FreeBSD sys/dev/acpica/acpi_rtc.c
 * =============================================================================
 * Reads the hardware CMOS clock and converts BCD to UNIX epoch time.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define CMOS_ADDRESS_PORT 0x70
#define CMOS_DATA_PORT    0x71

#define RTC_SECONDS       0x00
#define RTC_MINUTES       0x02
#define RTC_HOURS         0x04
#define RTC_DAY           0x07
#define RTC_MONTH         0x08
#define RTC_YEAR          0x09
#define RTC_CENTURY       0x32
#define RTC_STATUS_B      0x0B

typedef struct {
    sigma_u8 second;
    sigma_u8 minute;
    sigma_u8 hour;
    sigma_u8 day;
    sigma_u8 month;
    sigma_u16 year;
} sigma_rtc_time_t;

/* Simulated CMOS read */
static sigma_u8 cmos_read(sigma_u8 reg) {
    /* For simulation, return fixed BCD values: 2026-05-19 12:30:45 */
    switch (reg) {
        case RTC_SECONDS: return 0x45;
        case RTC_MINUTES: return 0x30;
        case RTC_HOURS:   return 0x12;
        case RTC_DAY:     return 0x19;
        case RTC_MONTH:   return 0x05;
        case RTC_YEAR:    return 0x26;
        case RTC_CENTURY: return 0x20;
        case RTC_STATUS_B:return 0x02; /* 24-hour format, BCD mode */
        default: return 0;
    }
}

static sigma_u8 bcd2bin(sigma_u8 bcd) {
    return ((bcd & 0xF0) >> 1) + ((bcd & 0xF0) >> 3) + (bcd & 0xf);
}

void rtc_init(void) {
    sigma_printf("[rtc] Hardware Real-Time Clock initialized\n");
}

void rtc_read_time(sigma_rtc_time_t* tm) {
    /* In a real kernel, we would wait for the 'Update in Progress' flag to clear */
    
    sigma_u8 sec     = cmos_read(RTC_SECONDS);
    sigma_u8 min     = cmos_read(RTC_MINUTES);
    sigma_u8 hour    = cmos_read(RTC_HOURS);
    sigma_u8 day     = cmos_read(RTC_DAY);
    sigma_u8 month   = cmos_read(RTC_MONTH);
    sigma_u8 year    = cmos_read(RTC_YEAR);
    sigma_u8 century = cmos_read(RTC_CENTURY);
    
    sigma_u8 status = cmos_read(RTC_STATUS_B);
    sigma_bool is_bcd = !(status & 0x04);
    
    if (is_bcd) {
        sec   = bcd2bin(sec);
        min   = bcd2bin(min);
        hour  = bcd2bin(hour);
        day   = bcd2bin(day);
        month = bcd2bin(month);
        year  = bcd2bin(year);
        century = bcd2bin(century);
    }
    
    tm->second = sec;
    tm->minute = min;
    tm->hour   = hour;
    tm->day    = day;
    tm->month  = month;
    tm->year   = (century * 100) + year;
    
    sigma_printf("[rtc] Read CMOS Time: %04u-%02u-%02u %02u:%02u:%02u\n",
                 tm->year, tm->month, tm->day, tm->hour, tm->minute, tm->second);
}

sigma_u64 rtc_to_epoch(const sigma_rtc_time_t* tm) {
    /* Simplified conversion (ignores leap seconds, rough leap year logic) */
    sigma_u32 years = tm->year - 1970;
    sigma_u32 leap_years = years / 4; /* Approximation */
    sigma_u32 days = (years * 365) + leap_years;
    
    const sigma_u32 days_in_month[] = { 0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 };
    
    for (sigma_u32 i = 1; i < tm->month; i++) {
        days += days_in_month[i];
    }
    
    if (tm->month > 2 && (tm->year % 4 == 0)) {
        days++; /* Leap year correction for current year */
    }
    
    days += (tm->day - 1);
    
    sigma_u64 epoch = (days * 86400) + (tm->hour * 3600) + (tm->minute * 60) + tm->second;
    sigma_printf("[rtc] Calculated UNIX Epoch: %llu\n", epoch);
    return epoch;
}
