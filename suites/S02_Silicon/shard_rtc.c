#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: RTC DRIVER (v1.0 â€ PURE C11)
 * =============================================================================
 * Hardware: CMOS RTC (ports 0x70/0x71) â€ i8254-compatible
 * Reads: year, month, day, hour, minute, second
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "core/sigma_kernel_types.h"

/* =========================================================================
 * CMOS RTC Ports
 * ========================================================================= */
#define RTC_PORT_CMD   0x70u
#define RTC_PORT_DATA  0x71u

#define RTC_REG_SEC    0x00u
#define RTC_REG_MIN    0x02u
#define RTC_REG_HOUR   0x04u
#define RTC_REG_DAY    0x07u
#define RTC_REG_MONTH  0x08u
#define RTC_REG_YEAR   0x09u
#define RTC_REG_STATUS 0x0Au  /* Status register A */
#define RTC_REG_B      0x0Bu  /* Status register B */

extern void ksigma_printf(const char *fmt, ...);

/* =========================================================================
 * RTC State
 * ========================================================================= */
typedef struct SigmaRTCTime {
    sigma_u8 sec, min, hour;
    sigma_u8 day, month;
    sigma_u16 year;
} SigmaRTCTime;

static SigmaRTCTime g_rtc_time;

/* =========================================================================
 * Helpers
 * ========================================================================= */
static sigma_u8 rtc_read(sigma_u8 reg) {
    port_outb(RTC_PORT_CMD, reg);
    return port_inb(RTC_PORT_DATA);
}

static sigma_bool rtc_is_updating(void) {
    port_outb(RTC_PORT_CMD, RTC_REG_STATUS);
    return !!(port_inb(RTC_PORT_DATA) & 0x80u);
}

static sigma_u8 bcd_to_bin(sigma_u8 bcd) {
    return ((bcd >> 4) * 10) + (bcd & 0x0Fu);
}

/* =========================================================================
 * rtc_read_time â€ reads date/time from CMOS RTC
 * Returns: pointer to static RTC state (valid until next call)
 * ========================================================================= */
const SigmaRTCTime *rtc_read_time(void) {
    /* Wait for RTC update to complete */
    while (rtc_is_updating());

    sigma_u8 sec   = rtc_read(RTC_REG_SEC);
    sigma_u8 min   = rtc_read(RTC_REG_MIN);
    sigma_u8 hour  = rtc_read(RTC_REG_HOUR);
    sigma_u8 day   = rtc_read(RTC_REG_DAY);
    sigma_u8 month = rtc_read(RTC_REG_MONTH);
    sigma_u8 year  = rtc_read(RTC_REG_YEAR);

    /* Check if BCD mode or binary mode */
    sigma_u8 regb = rtc_read(RTC_REG_B);
    if (!(regb & 0x04u)) {
        /* BCD mode */
        sec   = bcd_to_bin(sec);
        min   = bcd_to_bin(min);
        hour  = bcd_to_bin(hour);
        day   = bcd_to_bin(day);
        month = bcd_to_bin(month);
        year  = bcd_to_bin(year);
    }

    /* Handle 12/24-hour mode */
    if (!(regb & 0x02u) && (hour & 0x80u)) {
        hour = ((hour & 0x7Fu) + 12u) % 24u;
    }

    g_rtc_time.sec   = sec;
    g_rtc_time.min   = min;
    g_rtc_time.hour  = hour;
    g_rtc_time.day   = day;
    g_rtc_time.month = month;
    g_rtc_time.year  = (sigma_u16)(2000u + year);  /* Y2K correction */

    return &g_rtc_time;
}

/* =========================================================================
 * rtc_init
 * ========================================================================= */
void rtc_init(void) {
    const SigmaRTCTime *t = rtc_read_time();
<<<<<<< HEAD:suites/S02_Silicon/shard_rtc.c
    ksigma_printf("[RTC]: CMOS RTC online. Date: %04u-%02u-%02u  Time: %02u:%02u:%02u\n",
            (u32)t->year, (u32)t->month, (u32)t->day,
            (u32)t->hour, (u32)t->min,   (u32)t->sec);
=======
    kprintf("[RTC]: CMOS RTC online. Date: %04u-%02u-%02u  Time: %02u:%02u:%02u\n",
            (sigma_u32)t->year, (sigma_u32)t->month, (sigma_u32)t->day,
            (sigma_u32)t->hour, (sigma_u32)t->min,   (sigma_u32)t->sec);
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/drivers/rtc.c
}

/* =========================================================================
 * rtc_uptime_seconds â€ estimated uptime using jiffy counter
 * ========================================================================= */
extern sigma_u64 pit_get_jiffies(void);

sigma_u64 rtc_uptime_seconds(void) {
    return pit_get_jiffies() / 1000ULL;  /* jiffies are 1ms each */
}

/* =========================================================================
 * rtc_audit
 * ========================================================================= */
void rtc_audit(void) {
    const SigmaRTCTime *t = rtc_read_time();
<<<<<<< HEAD:suites/S02_Silicon/shard_rtc.c
    ksigma_printf("[RTC]: Current time: %04u-%02u-%02u %02u:%02u:%02u | Uptime: %llu s\n",
            (u32)t->year, (u32)t->month, (u32)t->day,
            (u32)t->hour, (u32)t->min,   (u32)t->sec,
=======
    kprintf("[RTC]: Current time: %04u-%02u-%02u %02u:%02u:%02u | Uptime: %llu s\n",
            (sigma_u32)t->year, (sigma_u32)t->month, (sigma_u32)t->day,
            (sigma_u32)t->hour, (sigma_u32)t->min,   (sigma_u32)t->sec,
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/drivers/rtc.c
            rtc_uptime_seconds());
}
