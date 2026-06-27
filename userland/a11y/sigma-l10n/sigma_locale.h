/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * sigma_locale.h — SigmaOS Localisation (l10n) subsystem
 *
 * Inspired by:
 *   • Android ResourceManager (locale-indexed string tables)
 *   • GNU gettext catalogue format (msgid → msgstr)
 *   • Haiku Locale Kit (BLocaleRoster, BLanguage, BCatalog)
 *   • ICU (International Components for Unicode) locale identifiers
 *
 * Design goals:
 *   1. Kernel-space safe — no heap allocation in core lookup path.
 *   2. Compile-time fallback to en_US if a string is not translated.
 *   3. Support RTL (right-to-left) layout directives.
 *   4. Support three shipped locales: en_US, hi_IN, zh_CN.
 */

#ifndef SIGMA_LOCALE_H
#define SIGMA_LOCALE_H

#include <stddef.h>   /* size_t, NULL */
#include <stdint.h>   /* uint8_t, uint32_t */
#include <stdbool.h>  /* bool */

#ifdef __cplusplus
extern "C" {
#endif

/* ── Locale identifiers ──────────────────────────────────────────────────── */

typedef enum sigma_locale_id {
    SIGMA_LOCALE_UNKNOWN = 0,
    SIGMA_LOCALE_EN_US   = 1,   /* English (United States) — default fallback */
    SIGMA_LOCALE_HI_IN   = 2,   /* Hindi   (India)                            */
    SIGMA_LOCALE_ZH_CN   = 3,   /* Chinese (Simplified, PRC)                  */
    SIGMA_LOCALE__COUNT
} sigma_locale_id_t;

/* Text directionality for layout engine */
typedef enum sigma_text_dir {
    SIGMA_TEXT_LTR = 0,  /* left-to-right (en_US, hi_IN)  */
    SIGMA_TEXT_RTL = 1,  /* right-to-left (future: ar, he) */
} sigma_text_dir_t;

/* ── Locale metadata ─────────────────────────────────────────────────────── */

typedef struct sigma_locale_info {
    sigma_locale_id_t id;
    const char       *bcp47_tag;    /* e.g. "en-US", "hi-IN", "zh-CN" */
    const char       *display_name; /* native script: "English", "हिन्दी", "中文" */
    sigma_text_dir_t  dir;
    const char       *decimal_sep;  /* "." or "," */
    const char       *thousands_sep;
    const char       *date_fmt;     /* strftime format */
    const char       *time_fmt;
} sigma_locale_info_t;

/* Built-in locale table — defined in sigma_locale.c */
extern const sigma_locale_info_t sigma_locale_table[SIGMA_LOCALE__COUNT];

/* ── Message catalogue entry ─────────────────────────────────────────────── */

/**
 * A single translated string mapping.
 * msgid is the canonical English key; msgstr is the translation.
 * All strings are UTF-8.
 */
typedef struct sigma_msg_entry {
    uint32_t    hash;    /* FNV-1a hash of msgid for O(1) lookup */
    const char *msgid;   /* canonical key, always en_US */
    const char *msgstr;  /* translated string */
} sigma_msg_entry_t;

/**
 * A translation catalogue for one locale.
 * Entries must be sorted by hash for binary search.
 */
typedef struct sigma_catalogue {
    sigma_locale_id_t    locale;
    const sigma_msg_entry_t *entries;
    size_t                   count;
} sigma_catalogue_t;

/* ── Built-in catalogues ─────────────────────────────────────────────────── */

/* Defined in sigma_locale_en_US.c / sigma_locale_hi_IN.c / sigma_locale_zh_CN.c */
extern const sigma_catalogue_t sigma_cat_en_US;
extern const sigma_catalogue_t sigma_cat_hi_IN;
extern const sigma_catalogue_t sigma_cat_zh_CN;

/* ── Runtime locale state ────────────────────────────────────────────────── */

/**
 * sigma_locale_init — set the active locale for this process/session.
 *
 * @param id  locale to activate; pass SIGMA_LOCALE_UNKNOWN for auto-detect
 *            from SIGMA_LANG / LANG / LC_ALL environment variables.
 * @return    the effective locale that was activated.
 */
sigma_locale_id_t sigma_locale_init(sigma_locale_id_t id);

/**
 * sigma_locale_get — return the currently active locale id.
 */
sigma_locale_id_t sigma_locale_get(void);

/**
 * sigma_locale_info — return metadata for a locale.
 * Returns en_US info if id is out of range.
 */
const sigma_locale_info_t *sigma_locale_info(sigma_locale_id_t id);

/* ── Translation lookup ──────────────────────────────────────────────────── */

/**
 * sigma_tr — translate a message key in the active locale.
 *
 * Performs a binary search in the active catalogue.
 * Falls back to en_US if the key is missing in the active locale.
 * Falls back to msgid itself if en_US also lacks it.
 *
 * @param msgid  canonical English key string.
 * @return       pointer to translated UTF-8 string (never NULL).
 *
 * Thread-safe: locale id is read atomically; catalogues are read-only.
 */
const char *sigma_tr(const char *msgid);

/**
 * Convenience macro — shorter spelling.
 */
#define _(msgid) sigma_tr(msgid)

/**
 * sigma_tr_locale — translate in a specific locale (not the active one).
 */
const char *sigma_tr_locale(sigma_locale_id_t id, const char *msgid);

/* ── Plural forms ────────────────────────────────────────────────────────── */

/**
 * sigma_tr_plural — pick singular or plural form.
 *
 * Simplified rule: n == 1 → singular, else → plural.
 * (Sufficient for en_US, hi_IN; Chinese has no grammatical plural.)
 *
 * @param msgid_singular  key for singular form
 * @param msgid_plural    key for plural form
 * @param n               count
 */
const char *sigma_tr_plural(const char *msgid_singular,
                             const char *msgid_plural,
                             unsigned long n);

/* ── Number / date formatting ────────────────────────────────────────────── */

/**
 * sigma_fmt_number — format an integer with locale-appropriate thousands sep.
 * Writes into buf[buflen].  Returns bytes written (excl. NUL) or -1 on error.
 */
int sigma_fmt_number(long long value, char *buf, size_t buflen);

/**
 * sigma_fmt_date — format a Unix timestamp using the locale date format.
 * Wraps strftime with the locale's date_fmt.
 */
int sigma_fmt_date(long long unix_ts, char *buf, size_t buflen);

/* ── FNV-1a hash (used internally, exposed for catalogue builders) ──────── */

static inline uint32_t sigma_fnv1a(const char *s) {
    uint32_t h = 2166136261u;
    while (*s) {
        h ^= (uint8_t)*s++;
        h *= 16777619u;
    }
    return h;
}

/* ── Well-known message keys ────────────────────────────────────────────── */
/* Use these constants instead of raw strings to avoid typos. */

#define SIGMA_MSG_OK              "ok"
#define SIGMA_MSG_CANCEL          "cancel"
#define SIGMA_MSG_ERROR           "error"
#define SIGMA_MSG_YES             "yes"
#define SIGMA_MSG_NO              "no"
#define SIGMA_MSG_LOADING         "loading"
#define SIGMA_MSG_DONE            "done"
#define SIGMA_MSG_RETRY           "retry"
#define SIGMA_MSG_PERMISSION_DENIED "permission_denied"
#define SIGMA_MSG_NOT_FOUND       "not_found"
#define SIGMA_MSG_BOOT_OK         "boot_ok"
#define SIGMA_MSG_BOOT_FAIL       "boot_fail"
#define SIGMA_MSG_UPDATE_AVAIL    "update_available"
#define SIGMA_MSG_UPDATE_DONE     "update_done"
#define SIGMA_MSG_SHUTDOWN        "shutdown"
#define SIGMA_MSG_REBOOT          "reboot"

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* SIGMA_LOCALE_H */

/*
 * ── Usage example ────────────────────────────────────────────────────────────
 *
 *   #include "sigma_locale.h"
 *
 *   void boot_message(void) {
 *       sigma_locale_init(SIGMA_LOCALE_UNKNOWN);  // auto-detect from env
 *       printf("%s\n", _(SIGMA_MSG_BOOT_OK));
 *       // en_US → "Boot successful"
 *       // hi_IN → "बूट सफल"
 *       // zh_CN → "启动成功"
 *   }
 *
 * ── Catalogue authoring ──────────────────────────────────────────────────────
 *
 *   Entries must be sorted ascending by hash.  Run:
 *       sigma-locale-gen --sort catalogues/hi_IN.c
 *   from the SDK to regenerate sorted tables.
 */
