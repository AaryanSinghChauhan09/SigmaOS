/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S12_Ecosystem/shards/sigma_distro.h
 * =========================================================================
 * Modular Store & Distro Absorption Engine.
 * =========================================================================
 */

#ifndef SIGMA_DISTRO_H
#define SIGMA_DISTRO_H

#include "suites/S01_Genesis/shards/SovereignCommon.h"

typedef sigma_u32   da_u32;
typedef sigma_err_t da_i32;
typedef sigma_bool  da_bool;

#define DA_TRUE  SIGMA_TRUE
#define DA_FALSE SIGMA_FALSE
#define DA_NULL  SIGMA_NULL
#define DA_OK    SIGMA_OK
#define DA_ERR   -1

/* ── Package format ──────────────────────────────────────────────────────── */
typedef enum {
    PKG_DEB     = 0,
    PKG_RPM     = 1,
    PKG_PACMAN  = 2,
    PKG_NIX     = 3,
    PKG_PORTAGE = 4,
    PKG_APK     = 5,
    PKG_FLATPAK = 6,
    PKG_APPIMAGE= 7,
    PKG_APK_AND = 8,
    PKG_HOMEBREW= 9,
    PKG_WINGET  = 10,
    PKG_SIGMA   = 11
} sigma_pkg_fmt_t;

/* ── Micro-Shard Granularity (Selective Store) ───────────────────────────── */
typedef enum {
    SHARD_FULL_BINARY = 0,    /* Complete application bundle            */
    SHARD_CORE_LOGIC  = 1,    /* Minimal required functions only        */
    SHARD_UI_ASSETS   = 2,    /* Web/GUI resources only                 */
    SHARD_AI_WEIGHTS  = 3,    /* Large LLM/ML models                   */
    SHARD_OPTIONAL_FX = 4     /* Non-essential plugins/effects         */
} sigma_shard_type_t;

/* ── Package state ───────────────────────────────────────────────────────── */
typedef enum {
    PKG_AVAILABLE   = 0,
    PKG_DOWNLOADING = 1,
    PKG_INSTALLED   = 2,
    PKG_UPGRADED    = 3,
    PKG_REMOVED     = 4,
    PKG_BROKEN      = 5
} sigma_pkg_state_t;

/* ── Channel/repo type ───────────────────────────────────────────────────── */
typedef enum {
    REPO_STABLE  = 0,
    REPO_TESTING = 1,
    REPO_AUR     = 2,
    REPO_COPR    = 3,
    REPO_NIX_CH  = 4,
    REPO_SIGMA   = 5
} sigma_repo_type_t;

#define DA_NAME_LEN     64
#define DA_VER_LEN      32
#define DA_URL_LEN     256
#define DA_MAX_PKGS    2048
#define DA_MAX_REPOS     32

/* ── Package descriptor (Object-Oriented) ───────────────────────────────── */
typedef struct {
    sigma_obj_t       base;           /* Inheritance from SovereignObject */
    char              version[DA_VER_LEN];
    char              repo_url[DA_URL_LEN];
    sigma_pkg_fmt_t   fmt;
    sigma_pkg_state_t state;
    sigma_repo_type_t channel;
    sigma_shard_type_t granularity;   /* Selective download mode         */
    da_u32            size_kb;
    da_u32            installed_kb;
    da_bool           pinned;
    da_bool           auto_installed;
    char              depends[8][DA_NAME_LEN];
    da_u32            dep_count;
} sigma_package_t;

/* ── Repository descriptor ──────────────────────────────────────────────── */
typedef struct {
    char              name[DA_NAME_LEN];
    char              url[DA_URL_LEN];
    sigma_pkg_fmt_t   fmt;
    sigma_repo_type_t channel;
    da_bool           enabled;
    da_bool           pqc_signed;
    da_u32            pkg_count;
} sigma_repo_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void   sigma_distro_init(void);

da_i32 sigma_repo_add(const char *name, const char *url,
                       sigma_pkg_fmt_t fmt, sigma_repo_type_t ch);
void   sigma_repo_remove(const char *name);
da_i32 sigma_repo_sync(void);
void   sigma_repo_list(void);

/* selective download (on-demand function delivery) */
da_i32 sigma_pkg_install_shard(const char *name, sigma_shard_type_t type);
da_i32 sigma_pkg_install(const char *name, sigma_pkg_fmt_t fmt);
da_i32 sigma_pkg_remove(const char *name, da_bool purge);
da_i32 sigma_pkg_upgrade_all(void);
da_i32 sigma_pkg_search(const char *query);
da_i32 sigma_pkg_show(const char *name);
void   sigma_pkg_list_installed(void);

da_i32 sigma_dal_translate(sigma_package_t *pkg);
void   sigma_dal_generation_snapshot(void);
da_i32 sigma_dal_rollback(da_u32 gen_id);
da_i32 sigma_pkg_run_sandboxed(const char *name);
da_i32 sigma_dal_enable_posix_dominance(void);

void   sigma_distro_report(void);

#endif /* SIGMA_DISTRO_H */
