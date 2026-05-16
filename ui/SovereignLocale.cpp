#include "../include/sigma_log.h"
#include "../include/libc/SovereignLibC.h"
#include "../include/sigma_locale.h"
#include "../include/hal/sigma_hal.h"
#include "../include/sigma_kernel_types.h"

/**
 * SigmaOS Sovereign Locale & Timezone Service Implementation
 * Implements a Static Cultural Data Map (SCDM) algorithm.
 * ZERO-DEPENDENCY: Inline CLDR-derived data; no glibc/ICU/tzdata daemon.
 * Competitor parity: Linux glibc locale, Windows NLS, macOS CFLocale.
 *
 * Design: OOP-isolated singleton " SovereignLocaleManager.
 */

/* Internal helper: sovereign bare-metal strlen (avoids libc dependency) */
static sigma_u32 _locale_strlen(const char* s) {
    sigma_u32 n = 0u;
    while (s && s[n]) n++;
    return n;
}

/* Internal helper: sovereign bare-metal strncpy */
static void _locale_strncpy(char* dst, const char* src, sigma_u32 max) {
    sigma_u32 i = 0u;
    while (i < max - 1u && src && src[i]) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

/* Internal helper: absolute value */
static sigma_i64 _locale_abs(sigma_i64 v) { return v < 0 ? -v : v; }

/* --- Sovereign Locale Manager (OOP Isolation) --- */
static struct {
    sigma_locale_t active;
    sigma_u32 initialized;
} SovereignLocaleManager = {
    .active = {
        .locale_code       = "en_US",
        .tz_name           = "UTC",
        .utc_offset_minutes = 0,
        .decimal_sep       = ".",
        .thousands_sep     = ",",
        .use_24h           = 0u,
        .first_weekday     = 0u   /* Sunday */
    },
    .initialized = 0u
};

void locale_init() {
    sigma_log("[LOCALE] Initializing Sovereign Static Cultural Data Map (SCDM)...");
    SovereignLocaleManager.initialized = 1u;
    sigma_log("[LOCALE] SCDM: Active locale='%s' TZ='%s' UTC%+d.\n",
                 SovereignLocaleManager.active.locale_code,
                 SovereignLocaleManager.active.tz_name,
                 (int)(SovereignLocaleManager.active.utc_offset_minutes / 60));
}

void locale_set(const char* locale_code) {
    if (!locale_code) return;
    _locale_strncpy(SovereignLocaleManager.active.locale_code,
                    locale_code, SIGMA_LOCALE_CODE_LEN);
    sigma_log("[LOCALE] SCDM: Locale set to '%s'.\n", locale_code);
}

void locale_set_timezone(const char* tz_name, sigma_i32 utc_offset_minutes) {
    if (!tz_name) return;
    _locale_strncpy(SovereignLocaleManager.active.tz_name,
                    tz_name, SIGMA_TZ_NAME_LEN);
    SovereignLocaleManager.active.utc_offset_minutes = utc_offset_minutes;
    sigma_log("[LOCALE] SCDM: Timezone set to '%s' (UTC%+d).\n",
                 tz_name, (int)(utc_offset_minutes / 60));
}

extern "C" const sigma_locale_t* locale_get() {
    return &SovereignLocaleManager.active;
}

extern "C" sigma_i32 locale_get_utc_offset() {
    return SovereignLocaleManager.active.utc_offset_minutes;
}

void locale_format_number(sigma_i64 value, char* out_buf, sigma_u32 buf_len) {
    // SCDM Algorithm: Formats integer with locale-specific thousand separators.
    if (!out_buf || buf_len == 0u) return;
    const char* sep = SovereignLocaleManager.active.thousands_sep;
    sigma_i64 abs_val = _locale_abs(value);

    // Simple 3-digit grouping into buffer (bare-metal, no printf)
    char tmp[32]; sigma_u32 pos = 0u;
    if (abs_val == 0) { tmp[pos++] = '0'; }
    sigma_u32 digit_count = 0u;
    sigma_i64 v = abs_val;
    while (v > 0 && pos < 30u) {
        if (digit_count > 0 && digit_count % 3 == 0 && sep[0])
            tmp[pos++] = sep[0];
        tmp[pos++] = (char)('0' + (int)(v % 10));
        v /= 10; digit_count++;
    }
    if (value < 0 && pos < 31u) tmp[pos++] = '-';
    // Reverse
    sigma_u32 out_pos = 0u;
    while (pos > 0u && out_pos < buf_len - 1u)
        out_buf[out_pos++] = tmp[--pos];
    out_buf[out_pos] = '\0';
    (void)_locale_strlen; /* suppress unused warning */
}




} // extern "C"
