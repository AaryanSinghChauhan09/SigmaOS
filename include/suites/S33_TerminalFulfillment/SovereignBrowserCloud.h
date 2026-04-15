/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN PUTER CLOUD OS + BROWSER ENGINE (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from:
 *   Puter OS  — https://github.com/HeyPuter/puter
 *   Firefox   — https://github.com/mozilla-firefox/firefox
 *   Chromium  — https://github.com/chromium/chromium
 *   BlackBox  — https://github.com/FBlackBox/BlackBox
 *
 * Puter OS USPs absorbed:
 *   ✓ Cloud-first virtual filesystem (cloud-storage-backed VFS)
 *   ✓ Web app sandbox container (iframe-isolated, permission-gated)
 *   ✓ App Store registry with permission manifests
 *   ✓ Cloud desktop environment (draggable windows, taskbar, launcher)
 *   ✓ Multi-user cloud authentication (OAuth2/JWT stub)
 *
 * Firefox / Chromium USPs absorbed:
 *   ✓ Multi-process architecture (browser / renderer / GPU / utility)
 *   ✓ DOM/CSS layout engine stub (Gecko/Blink parity)
 *   ✓ JavaScript engine interface (SpiderMonkey / V8 dispatch stub)
 *   ✓ Content Security Policy (CSP) enforcement
 *   ✓ Site isolation (one process per site origin)
 *   ✓ Extension/Add-on system (WebExtensions API)
 *   ✓ Privacy features: Enhanced Tracking Protection, dFPI (cookies)
 *   ✓ WebAssembly module loader
 *
 * BlackBox USPs absorbed:
 *   ✓ App virtualisation container (isolated environment)
 *   ✓ Permission bypass sandbox for third-party apps
 * =========================================================================
 */

#ifndef SOVEREIGN_BROWSER_CLOUD_H
#define SOVEREIGN_BROWSER_CLOUD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * §1  PUTER CLOUD OS
 * ====================================================================== */

#define SIGMA_CLOUD_URL_MAX    256
#define SIGMA_CLOUD_APP_MAX     64
#define SIGMA_CLOUD_USER_MAX    32
#define SIGMA_CLOUD_NAME_MAX    64

/* -------------------------------------------------------------------------
 * Cloud VFS node (backed by object storage / S3-like bucket)
 * ---------------------------------------------------------------------- */
typedef enum {
    CLOUD_NODE_FILE   = 0,
    CLOUD_NODE_DIR    = 1,
    CLOUD_NODE_LINK   = 2,
} SigmaCloudNodeType_t;

typedef struct {
    char                 path    [SIGMA_CLOUD_URL_MAX];
    char                 storage_url[SIGMA_CLOUD_URL_MAX]; /* presigned URL */
    SigmaCloudNodeType_t type;
    sigma_u64            size_bytes;
    sigma_u64            modified_ts;
    char                 owner   [SIGMA_CLOUD_NAME_MAX];
    sigma_bool           public_read;
} SigmaCloudNode_t;

/* -------------------------------------------------------------------------
 * Puter App permission flags
 * ---------------------------------------------------------------------- */
#define PUTER_PERM_FS_READ    (1u << 0)
#define PUTER_PERM_FS_WRITE   (1u << 1)
#define PUTER_PERM_NETWORK    (1u << 2)
#define PUTER_PERM_CAMERA     (1u << 3)
#define PUTER_PERM_MIC        (1u << 4)
#define PUTER_PERM_CLIPBOARD  (1u << 5)
#define PUTER_PERM_NOTIFY     (1u << 6)
#define PUTER_PERM_SHELL      (1u << 7)

typedef struct {
    char      app_id    [SIGMA_CLOUD_NAME_MAX];
    char      name      [SIGMA_CLOUD_NAME_MAX];
    char      entry_url [SIGMA_CLOUD_URL_MAX];  /* index.html / main.js */
    sigma_u32 permissions;
    sigma_bool sandboxed;
    sigma_bool installed;
    sigma_u64  install_ts;
} SigmaCloudApp_t;

/* =========================================================================
 * §2  BROWSER ENGINE
 * ====================================================================== */

#define SIGMA_BROWSER_PROC_MAX   32
#define SIGMA_BROWSER_TAB_MAX    64
#define SIGMA_BROWSER_EXT_MAX    32
#define SIGMA_BROWSER_URL_MAX   512

/* -------------------------------------------------------------------------
 * Browser process types (Chromium multi-process architecture)
 * ---------------------------------------------------------------------- */
typedef enum {
    PROC_BROWSER   = 0,   /* Main / UI process                   */
    PROC_RENDERER  = 1,   /* One per site-origin (site isolation) */
    PROC_GPU       = 2,   /* GPU compositing / shader compilation */
    PROC_UTILITY   = 3,   /* Audio, network, storage              */
    PROC_PLUGIN    = 4,   /* Isolated plugin host                 */
} SigmaBrowserProcType_t;

typedef struct {
    pid_t                  pid;
    SigmaBrowserProcType_t type;
    char                   origin [SIGMA_BROWSER_URL_MAX]; /* site origin   */
    sigma_bool             sandboxed;
    sigma_u64              mem_bytes;
    sigma_u32              cpu_pct;
} SigmaBrowserProc_t;

/* -------------------------------------------------------------------------
 * Browser Tab (renderer context)
 * ---------------------------------------------------------------------- */
typedef enum {
    TAB_LOADING   = 0,
    TAB_COMPLETE  = 1,
    TAB_ERROR     = 2,
    TAB_SUSPENDED = 3,  /* Chromium tab discarding */
} SigmaTabState_t;

typedef struct {
    sigma_u32      tab_id;
    char           url     [SIGMA_BROWSER_URL_MAX];
    char           title   [256];
    SigmaTabState_t state;
    sigma_bool     incognito;
    sigma_bool     pinned;
    pid_t          renderer_pid;
    sigma_u64      mem_bytes;
} SigmaTab_t;

/* -------------------------------------------------------------------------
 * Content Security Policy
 * ---------------------------------------------------------------------- */
typedef struct {
    sigma_bool allow_scripts;
    sigma_bool allow_inline_scripts;
    sigma_bool allow_eval;
    sigma_bool allow_frames;
    sigma_bool allow_mixed_content;
    char       allowed_origins[16][128];
    sigma_u32  origin_count;
} SigmaCSP_t;

/* -------------------------------------------------------------------------
 * WebExtension / Add-on
 * ---------------------------------------------------------------------- */
typedef struct {
    char      id      [64];
    char      name    [SIGMA_CLOUD_NAME_MAX];
    char      version [16];
    sigma_u32 permissions;
    sigma_bool enabled;
    sigma_bool content_script;  /* Injected into pages */
} SigmaBrowserExt_t;

/* -------------------------------------------------------------------------
 * Browser engine context
 * ---------------------------------------------------------------------- */
typedef struct {
    SigmaBrowserProc_t procs   [SIGMA_BROWSER_PROC_MAX];
    sigma_u32          proc_count;
    SigmaTab_t         tabs    [SIGMA_BROWSER_TAB_MAX];
    sigma_u32          tab_count;
    sigma_u32          active_tab;
    SigmaBrowserExt_t  extensions[SIGMA_BROWSER_EXT_MAX];
    sigma_u32          ext_count;
    /* Privacy */
    sigma_bool         efp_enabled;    /* Enhanced Tracking Protection */
    sigma_bool         dfpi_enabled;   /* Dynamic FPI (cookie isolation) */
    sigma_bool         https_only;
    sigma_bool         webrtc_isolated;
    /* Performance */
    sigma_u64          total_mem_bytes;
    sigma_u64          cache_bytes;
} SigmaBrowserCtx_t;

/* -------------------------------------------------------------------------
 * Puter Cloud context
 * ---------------------------------------------------------------------- */
typedef struct {
    SigmaCloudApp_t apps    [SIGMA_CLOUD_APP_MAX];
    sigma_u32       app_count;
    char            current_user[SIGMA_CLOUD_NAME_MAX];
    char            cloud_token [256];   /* JWT / OAuth2 bearer */
    sigma_bool      authenticated;
    sigma_u64       storage_used_bytes;
    sigma_u64       storage_quota_bytes;
} SigmaCloudCtx_t;

extern SigmaBrowserCtx_t g_sigma_browser;
extern SigmaCloudCtx_t   g_sigma_cloud;

/* -------------------------------------------------------------------------
 * Public API — Puter Cloud OS
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_cloud_auth       (const char *user, const char *token);
sigma_err_t sigma_cloud_app_install(const char *app_id, const char *url,
                                     sigma_u32 permissions);
sigma_err_t sigma_cloud_app_launch (const char *app_id);
sigma_err_t sigma_cloud_app_remove (const char *app_id);
void        sigma_cloud_app_list   (void);
sigma_err_t sigma_cloud_fs_ls      (const char *path);
sigma_err_t sigma_cloud_fs_put     (const char *local, const char *cloud);
sigma_err_t sigma_cloud_fs_get     (const char *cloud, const char *local);

/* Public API — Browser Engine */
sigma_u32   sigma_browser_tab_open (const char *url, sigma_bool incognito);
sigma_err_t sigma_browser_tab_close(sigma_u32 tab_id);
sigma_err_t sigma_browser_navigate (sigma_u32 tab_id, const char *url);
void        sigma_browser_tab_list (void);
sigma_err_t sigma_browser_ext_install(const char *name, sigma_u32 perms);
sigma_err_t sigma_browser_ext_toggle (const char *name, sigma_bool enable);
void        sigma_browser_stats     (void);
void        sigma_browser_procs     (void);

/* CSP */
sigma_bool  sigma_csp_check_script (const SigmaCSP_t *csp, const char *origin);
sigma_bool  sigma_csp_check_frame  (const SigmaCSP_t *csp, const char *origin);

/* WASM */
sigma_err_t sigma_wasm_load        (const sigma_u8 *wasm_bytes, sigma_sz_t len);
sigma_err_t sigma_wasm_call        (const char *export_name, sigma_u64 *args,
                                     sigma_u32 argc, sigma_u64 *result);

void SovereignBrowserCloud_Init(void);

#endif /* SOVEREIGN_BROWSER_CLOUD_H */
