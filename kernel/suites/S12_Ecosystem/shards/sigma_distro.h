/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S12_Ecosystem/shards/sigma_distro.h
 * =========================================================================
 * Distro Absorption Engine — closes compatibility gaps vs:
 *   Ubuntu   : APT package database, dpkg format, PPA repositories
 *   Arch     : pacman, AUR, rolling-release model
 *   Fedora   : RPM/DNF, COPR repos, flatpak integration
 *   Debian   : stable/testing/unstable channels, pinning
 *   NixOS    : functional package management, generations, flakes
 *   Gentoo   : portage, USE flags, ebuild compilation
 *   Alpine   : musl libc, apk, minimal footprint model
 *   Android  : APK sideload, ADB push, system image flashing
 *   macOS    : Homebrew, MacPorts, pkg drag-install
 *   Windows  : winget, chocolatey, MSI/NSIS install
 * =========================================================================
 * SigmaOS absorbs all package formats natively, translating them to
 * Sovereign Shards through the Distro Absorption Layer (DAL).
 * =========================================================================
 */

#ifndef SIGMA_DISTRO_H
#define SIGMA_DISTRO_H

typedef unsigned int  da_u32;
typedef signed   int  da_i32;
typedef unsigned char da_bool;
#define DA_TRUE  ((da_bool)1)
#define DA_FALSE ((da_bool)0)
#define DA_NULL  ((void*)0)
#define DA_OK    ((da_i32) 0)
#define DA_ERR   ((da_i32)-1)

/* ── Package format ──────────────────────────────────────────────────────── */
typedef enum {
    PKG_DEB     = 0,   /* Ubuntu / Debian .deb                         */
    PKG_RPM     = 1,   /* Fedora / RHEL .rpm                           */
    PKG_PACMAN  = 2,   /* Arch .pkg.tar.zst                            */
    PKG_NIX     = 3,   /* NixOS /nix/store derivation                  */
    PKG_PORTAGE = 4,   /* Gentoo ebuild                                */
    PKG_APK     = 5,   /* Alpine .apk                                  */
    PKG_FLATPAK = 6,   /* Flatpak OCI bundle                           */
    PKG_APPIMAGE= 7,   /* AppImage portable binary                     */
    PKG_APK_AND = 8,   /* Android .apk                                 */
    PKG_HOMEBREW= 9,   /* macOS Homebrew formula                       */
    PKG_WINGET  = 10,  /* Windows winget manifest                      */
    PKG_SIGMA   = 11   /* Native SigmaOS Sovereign Shard               */
} sigma_pkg_fmt_t;

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
    REPO_SIGMA   = 5    /* sovereign shard registry                    */
} sigma_repo_type_t;

#define DA_NAME_LEN     64
#define DA_VER_LEN      32
#define DA_URL_LEN     256
#define DA_MAX_PKGS    2048
#define DA_MAX_REPOS     32

/* ── Package descriptor ─────────────────────────────────────────────────── */
typedef struct {
    char              name[DA_NAME_LEN];
    char              version[DA_VER_LEN];
    char              repo_url[DA_URL_LEN];
    sigma_pkg_fmt_t   fmt;
    sigma_pkg_state_t state;
    sigma_repo_type_t channel;
    da_u32            size_kb;
    da_u32            installed_kb;
    da_bool           pinned;
    da_bool           auto_installed;
    char              depends[8][DA_NAME_LEN];  /* up to 8 deps        */
    da_u32            dep_count;
} sigma_package_t;

/* ── Repository descriptor ──────────────────────────────────────────────── */
typedef struct {
    char              name[DA_NAME_LEN];
    char              url[DA_URL_LEN];
    sigma_pkg_fmt_t   fmt;
    sigma_repo_type_t channel;
    da_bool           enabled;
    da_bool           pqc_signed;  /* repo signature uses ML-DSA        */
    da_u32            pkg_count;
} sigma_repo_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void   sigma_distro_init(void);

/* Repository management */
da_i32 sigma_repo_add(const char *name, const char *url,
                       sigma_pkg_fmt_t fmt, sigma_repo_type_t ch);
void   sigma_repo_remove(const char *name);
da_i32 sigma_repo_sync(void);       /* apt update / pacman -Sy         */
void   sigma_repo_list(void);

/* Package operations */
da_i32 sigma_pkg_install(const char *name, sigma_pkg_fmt_t fmt);
da_i32 sigma_pkg_remove(const char *name, da_bool purge);
da_i32 sigma_pkg_upgrade_all(void);
da_i32 sigma_pkg_search(const char *query);
da_i32 sigma_pkg_show(const char *name);
void   sigma_pkg_list_installed(void);

/* Format translation (DAL core) */
da_i32 sigma_dal_translate(sigma_package_t *pkg);  /* any fmt -> sigma  */
void   sigma_dal_generation_snapshot(void);        /* NixOS-style rollback */
da_i32 sigma_dal_rollback(da_u32 gen_id);

/* Flatpak/AppImage/WASM sandbox integration */
da_i32 sigma_pkg_run_sandboxed(const char *name);

void   sigma_distro_report(void);

#endif /* SIGMA_DISTRO_H */
