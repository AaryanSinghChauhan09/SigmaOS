/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LOCALE & TIMEZONE SERVICE (S-LOCALE)
 * =========================================================================
 * Mission: Kernel-native locale, timezone, and i18n without glibc/ICU.
 * Competitor parity: Linux glibc locale / Windows NLS / macOS CFLocale.
 * ZERO-DEPENDENCY: Static CLDR-derived data; no runtime library required.
 * =========================================================================
 */

#ifndef SIGMA_LOCALE_H
#define SIGMA_LOCALE_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_LOCALE_CODE_LEN   8u   /* e.g. "en_US\0"             */
#define SIGMA_TZ_NAME_LEN       48u  /* e.g. "America/New_York\0"  */
#define SIGMA_DECIMAL_SEP_LEN   4u
#define SIGMA_THOUSANDS_SEP_LEN 4u

typedef struct {
    char locale_code[SIGMA_LOCALE_CODE_LEN];  /* POSIX locale identifier  */
    char tz_name[SIGMA_TZ_NAME_LEN];          /* IANA timezone name       */
    sigma_i32 utc_offset_minutes;             /* UTC offset in minutes    */
    char decimal_sep[SIGMA_DECIMAL_SEP_LEN];  /* '.' or ','               */
    char thousands_sep[SIGMA_THOUSANDS_SEP_LEN];
    sigma_u32 use_24h;                        /* 1 = 24h clock, 0 = 12h   */
    sigma_u32 first_weekday;                  /* 0 = Sunday, 1 = Monday   */
} sigma_locale_t;

/* --- Locale Primitives --- */
void locale_init(void);
void locale_set(const char* locale_code);
void locale_set_timezone(const char* tz_name, sigma_i32 utc_offset_minutes);
const sigma_locale_t* locale_get(void);
sigma_i32 locale_get_utc_offset(void);
void locale_format_number(sigma_i64 value, char* out_buf, sigma_u32 buf_len);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LOCALE_H */
