// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_locale.h — Internationalization & localization (i18n/l10n)
 *
 * Provides string catalog lookup, plural forms, RTL text direction,
 * number/date formatting, and locale fallback chains.
 *
 * Locale files live at /sigma/share/locale/<lang>/<domain>.scat
 * Binary format: header + sorted key→value string pairs (mmap-friendly).
 *
 * Usage:
 *   sigma_locale_init("de_DE.UTF-8");
 *   const char* s = _("Hello");                    // msgstr from catalog
 *   const char* p = ngettext("%d file", "%d files", n); // plural form
 */

#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Locale handle ───────────────────────────────────────────────────────── */
typedef struct sigma_locale sigma_locale_t;

/* ── Text direction ──────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_TEXT_LTR = 0,   /* Latin, CJK, etc.          */
    SIGMA_TEXT_RTL = 1,   /* Arabic, Hebrew, Farsi      */
    SIGMA_TEXT_TTB = 2,   /* Traditional Mongolian      */
} sigma_text_dir_t;

/* ── Locale descriptor ───────────────────────────────────────────────────── */
typedef struct {
    char              locale_id[32];       /* "en_US.UTF-8"                  */
    char              language[8];         /* "en"                           */
    char              region[8];           /* "US"                           */
    char              charset[16];         /* "UTF-8"                        */
    sigma_text_dir_t  text_direction;
    char              decimal_sep;         /* '.' or ','                     */
    char              thousand_sep;        /* ',' or '.'                     */
    char              date_fmt[32];        /* strftime-style "%m/%d/%Y"      */
    char              time_fmt[32];        /* "%H:%M:%S" or "%I:%M %p"       */
    bool              use_24h;
} sigma_locale_info_t;

/* ── Init / teardown ─────────────────────────────────────────────────────── */

/*
 * sigma_locale_init — initialise locale subsystem from POSIX locale string.
 * Loads catalog for the requested locale, falls back through region→language→C.
 * Returns 0 on success, -SIGMA_ENOENT if no catalog found (C locale used).
 */
int sigma_locale_init(const char* locale_string);

/*
 * sigma_locale_load_domain — load an additional translation domain.
 * Domain name maps to /sigma/share/locale/<lang>/<domain>.scat
 */
int sigma_locale_load_domain(const char* domain);

void sigma_locale_shutdown(void);

/* ── Translation ─────────────────────────────────────────────────────────── */

/* Get translated string for msgid.  Returns msgid if not found. */
const char* sigma_gettext(const char* domain, const char* msgid);

/* Plural-aware translation.  n selects the appropriate plural form. */
const char* sigma_ngettext(const char* domain,
                            const char* msgid_singular,
                            const char* msgid_plural,
                            sigma_u64   n);

/* Convenience macros (set SIGMA_TEXT_DOMAIN before including) */
#ifndef SIGMA_TEXT_DOMAIN
#  define SIGMA_TEXT_DOMAIN "sigma"
#endif

#define _(s)          sigma_gettext(SIGMA_TEXT_DOMAIN, (s))
#define N_(s)         (s)    /* mark-only, no lookup at compile time        */
#define ngettext(s,p,n) sigma_ngettext(SIGMA_TEXT_DOMAIN, (s), (p), (n))

/* ── Locale query ────────────────────────────────────────────────────────── */

/* Fill info for the currently active locale. */
int sigma_locale_query(sigma_locale_info_t* out);

/* Return text direction for the active locale. */
sigma_text_dir_t sigma_locale_text_direction(void);

/* ── Number / date formatting ────────────────────────────────────────────── */

/*
 * sigma_format_number — format integer with locale thousand separator.
 * buf must be at least 32 bytes.
 */
int sigma_format_number(sigma_s64 n, char* buf, int buf_len);

/*
 * sigma_format_bytes — human-readable size (KB/MB/GB) in locale language.
 * e.g., "1,23 GB" (de_DE) or "1.23 GB" (en_US).
 */
int sigma_format_bytes(sigma_u64 bytes, char* buf, int buf_len);

/*
 * sigma_format_date — format Unix timestamp using locale date format.
 */
int sigma_format_date(sigma_u64 timestamp_ns, char* buf, int buf_len);

/* ── Charset conversion ──────────────────────────────────────────────────── */

/*
 * sigma_locale_to_utf8 — convert string from locale charset to UTF-8.
 * Returns bytes written or negative error code.
 */
int sigma_locale_to_utf8(const char* src, int src_len,
                          char* dst,       int dst_len);

int sigma_utf8_to_locale(const char* src, int src_len,
                          char* dst,       int dst_len);

/* ── Catalog compilation (build-time tool) ───────────────────────────────── */
/*
 * sigma-msgfmt converts .po files to .scat binary catalogs:
 *   sigma-msgfmt -d sigma -l de_DE messages.po -o sigma.scat
 *
 * .scat format:
 *   [magic: 4B][version: 1B][n_strings: 4B]
 *   [key_offsets: n×4B][val_offsets: n×4B]
 *   [string_pool: variable, NUL-terminated pairs]
 */
