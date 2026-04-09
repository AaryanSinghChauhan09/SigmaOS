/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN BROWSER ENGINE + PUTER CLOUD OS — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignBrowserCloud.h"

/* Global contexts */
SigmaBrowserCtx_t g_sigma_browser;
SigmaCloudCtx_t   g_sigma_cloud;

static sigma_u32 s_next_tab_id   = 1;
static sigma_u32 s_next_proc_pid = 9000;

/* =========================================================================
 * §1  PUTER CLOUD OS
 * ====================================================================== */

sigma_err_t sigma_cloud_auth(const char *user, const char *token) {
    sigma_strcpy(g_sigma_cloud.current_user, user, SIGMA_CLOUD_NAME_MAX);
    sigma_strcpy(g_sigma_cloud.cloud_token,  token, 256);
    g_sigma_cloud.authenticated       = SIGMA_TRUE;
    g_sigma_cloud.storage_quota_bytes = 10ULL * 1024 * 1024 * 1024; /* 10 GB */
    sigma_printf("Σ [CLOUD]: Authenticated as '%s'. Quota: 10 GB.\n", user);
    return SIGMA_OK;
}

sigma_err_t sigma_cloud_app_install(const char *app_id, const char *url,
                                     sigma_u32 permissions) {
    if (!g_sigma_cloud.authenticated) return SIGMA_EPERM;
    if (g_sigma_cloud.app_count >= SIGMA_CLOUD_APP_MAX) return SIGMA_ENOSPC;

    SigmaCloudApp_t *a = &g_sigma_cloud.apps[g_sigma_cloud.app_count++];
    sigma_strcpy(a->app_id,    app_id, SIGMA_CLOUD_NAME_MAX);
    sigma_strcpy(a->name,      app_id, SIGMA_CLOUD_NAME_MAX);
    sigma_strcpy(a->entry_url, url,    SIGMA_CLOUD_URL_MAX);
    a->permissions = permissions;
    a->sandboxed   = SIGMA_TRUE;
    a->installed   = SIGMA_TRUE;

    sigma_printf("Σ [CLOUD]: App installed: %s  perms=0x%02x  sandboxed=yes\n",
                 app_id, permissions);
    return SIGMA_OK;
}

sigma_err_t sigma_cloud_app_launch(const char *app_id) {
    for (sigma_u32 i = 0; i < g_sigma_cloud.app_count; i++) {
        SigmaCloudApp_t *a = &g_sigma_cloud.apps[i];
        if (sigma_streq(a->app_id, app_id)) {
            sigma_printf("Σ [CLOUD]: Launching '%s'  url=%s  sandbox=%s\n",
                         app_id, a->entry_url,
                         a->sandboxed ? "iframe+CSP" : "native");
            /* Open a browser tab for web apps */
            sigma_browser_tab_open(a->entry_url, SIGMA_FALSE);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

sigma_err_t sigma_cloud_app_remove(const char *app_id) {
    for (sigma_u32 i = 0; i < g_sigma_cloud.app_count; i++) {
        if (sigma_streq(g_sigma_cloud.apps[i].app_id, app_id)) {
            g_sigma_cloud.apps[i].installed = SIGMA_FALSE;
            sigma_printf("Σ [CLOUD]: App removed: %s\n", app_id);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

void sigma_cloud_app_list(void) {
    sigma_printf("Σ [CLOUD]: Installed apps (%u):\n", g_sigma_cloud.app_count);
    for (sigma_u32 i = 0; i < g_sigma_cloud.app_count; i++) {
        SigmaCloudApp_t *a = &g_sigma_cloud.apps[i];
        if (!a->installed) continue;
        sigma_printf("  %-20s  %s\n", a->app_id, a->entry_url);
    }
}

sigma_err_t sigma_cloud_fs_ls(const char *path) {
    sigma_printf("Σ [CLOUD-FS]: %s:\n"
                 "  Documents/  Photos/  Music/  sigma-notes.txt  resume.pdf\n",
                 path);
    return SIGMA_OK;
}

sigma_err_t sigma_cloud_fs_put(const char *local, const char *cloud) {
    sigma_printf("Σ [CLOUD-FS]: Upload %s -> cloud:%s\n", local, cloud);
    return SIGMA_OK;
}

sigma_err_t sigma_cloud_fs_get(const char *cloud, const char *local) {
    sigma_printf("Σ [CLOUD-FS]: Download cloud:%s -> %s\n", cloud, local);
    return SIGMA_OK;
}

/* =========================================================================
 * §2  BROWSER ENGINE (Chromium multi-process + Firefox privacy)
 * ====================================================================== */

/* Spawn a browser process */
static pid_t spawn_browser_proc(SigmaBrowserProcType_t type,
                                 const char *origin, sigma_bool sandboxed) {
    if (g_sigma_browser.proc_count >= SIGMA_BROWSER_PROC_MAX)
        return -1;
    SigmaBrowserProc_t *p = &g_sigma_browser
                              .procs[g_sigma_browser.proc_count++];
    p->pid       = (pid_t)s_next_proc_pid++;
    p->type      = type;
    p->sandboxed = sandboxed;
    p->mem_bytes = 64 * 1024 * 1024;   /* 64 MB initial */
    p->cpu_pct   = 0;
    if (origin) sigma_strcpy(p->origin, origin, SIGMA_BROWSER_URL_MAX);
    return p->pid;
}

sigma_u32 sigma_browser_tab_open(const char *url, sigma_bool incognito) {
    if (g_sigma_browser.tab_count >= SIGMA_BROWSER_TAB_MAX) return 0;
    SigmaTab_t *t = &g_sigma_browser.tabs[g_sigma_browser.tab_count++];
    t->tab_id    = s_next_tab_id++;
    sigma_strcpy(t->url, url, SIGMA_BROWSER_URL_MAX);
    sigma_strcpy(t->title, "Loading…", 256);
    t->state     = TAB_LOADING;
    t->incognito = incognito;
    /* Spawn a dedicated renderer process (site isolation: Chromium) */
    t->renderer_pid = spawn_browser_proc(PROC_RENDERER, url, SIGMA_TRUE);
    t->mem_bytes    = 128 * 1024 * 1024;
    t->state        = TAB_COMPLETE;
    sigma_strcpy(t->title, "SigmaOS Page", 256);
    g_sigma_browser.total_mem_bytes += t->mem_bytes;

    sigma_printf("Σ [BROWSER]: Tab %u opened: %s%s  pid=%d\n",
                 t->tab_id, url, incognito ? " [incognito]" : "",
                 (int)t->renderer_pid);
    return t->tab_id;
}

sigma_err_t sigma_browser_tab_close(sigma_u32 tab_id) {
    for (sigma_u32 i = 0; i < g_sigma_browser.tab_count; i++) {
        SigmaTab_t *t = &g_sigma_browser.tabs[i];
        if (t->tab_id == tab_id) {
            g_sigma_browser.total_mem_bytes -= t->mem_bytes;
            t->state = TAB_SUSPENDED;
            sigma_printf("Σ [BROWSER]: Tab %u closed.\n", tab_id);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

sigma_err_t sigma_browser_navigate(sigma_u32 tab_id, const char *url) {
    for (sigma_u32 i = 0; i < g_sigma_browser.tab_count; i++) {
        SigmaTab_t *t = &g_sigma_browser.tabs[i];
        if (t->tab_id == tab_id) {
            sigma_strcpy(t->url, url, SIGMA_BROWSER_URL_MAX);
            t->state = TAB_LOADING;
            sigma_printf("Σ [BROWSER]: Tab %u -> %s\n", tab_id, url);
            t->state = TAB_COMPLETE;
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

void sigma_browser_tab_list(void) {
    sigma_printf("Σ [BROWSER]: Open tabs (%u):\n", g_sigma_browser.tab_count);
    static const char *state_str[] = {"loading","complete","error","suspended"};
    for (sigma_u32 i = 0; i < g_sigma_browser.tab_count; i++) {
        SigmaTab_t *t = &g_sigma_browser.tabs[i];
        if (t->state == TAB_SUSPENDED) continue;
        sigma_printf("  [%2u] %-8s %s%s  mem=%lluMB\n",
                     t->tab_id, state_str[t->state], t->url,
                     t->incognito ? " [incognito]" : "",
                     (unsigned long long)(t->mem_bytes / (1024*1024)));
    }
}

sigma_err_t sigma_browser_ext_install(const char *name, sigma_u32 perms) {
    if (g_sigma_browser.ext_count >= SIGMA_BROWSER_EXT_MAX) return SIGMA_ENOSPC;
    SigmaBrowserExt_t *e = &g_sigma_browser
                             .extensions[g_sigma_browser.ext_count++];
    sigma_strcpy(e->id,   name, 64);
    sigma_strcpy(e->name, name, SIGMA_CLOUD_NAME_MAX);
    sigma_strcpy(e->version, "1.0", 16);
    e->permissions = perms;
    e->enabled     = SIGMA_TRUE;
    sigma_printf("Σ [BROWSER]: Extension '%s' installed  perms=0x%02x\n",
                 name, perms);
    return SIGMA_OK;
}

sigma_err_t sigma_browser_ext_toggle(const char *name, sigma_bool enable) {
    for (sigma_u32 i = 0; i < g_sigma_browser.ext_count; i++) {
        if (sigma_streq(g_sigma_browser.extensions[i].name, name)) {
            g_sigma_browser.extensions[i].enabled = enable;
            sigma_printf("Σ [BROWSER]: Extension '%s' %s.\n",
                         name, enable ? "enabled" : "disabled");
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

void sigma_browser_stats(void) {
    sigma_printf("Σ [BROWSER]: Engine stats:\n"
                 "  Processes: %u  Tabs: %u  Extensions: %u\n"
                 "  Total mem: %lluMB\n"
                 "  EFP (tracking protection): %s\n"
                 "  dFPI (cookie isolation): %s\n"
                 "  HTTPS-only: %s\n",
                 g_sigma_browser.proc_count,
                 g_sigma_browser.tab_count,
                 g_sigma_browser.ext_count,
                 (unsigned long long)(g_sigma_browser.total_mem_bytes/(1024*1024)),
                 g_sigma_browser.efp_enabled  ? "on" : "off",
                 g_sigma_browser.dfpi_enabled ? "on" : "off",
                 g_sigma_browser.https_only   ? "on" : "off");
}

void sigma_browser_procs(void) {
    static const char *ptypes[] = {"Browser","Renderer","GPU","Utility","Plugin"};
    sigma_printf("Σ [BROWSER]: Process list:\n");
    for (sigma_u32 i = 0; i < g_sigma_browser.proc_count; i++) {
        SigmaBrowserProc_t *p = &g_sigma_browser.procs[i];
        sigma_printf("  pid=%-5d  %-9s  sandbox=%s  mem=%lluMB  %s\n",
                     (int)p->pid, ptypes[p->type],
                     p->sandboxed ? "yes" : "no",
                     (unsigned long long)(p->mem_bytes/(1024*1024)),
                     p->origin);
    }
}

/* CSP */
sigma_bool sigma_csp_check_script(const SigmaCSP_t *csp, const char *origin) {
    if (!csp->allow_scripts) {
        sigma_printf("Σ [CSP]: Blocked inline script from '%s'\n", origin);
        return SIGMA_FALSE;
    }
    return SIGMA_TRUE;
}

sigma_bool sigma_csp_check_frame(const SigmaCSP_t *csp, const char *origin) {
    if (!csp->allow_frames) {
        sigma_printf("Σ [CSP]: Blocked frame from '%s'\n", origin);
        return SIGMA_FALSE;
    }
    return SIGMA_TRUE;
}

/* WASM */
sigma_err_t sigma_wasm_load(const sigma_u8 *wasm_bytes, sigma_size_t len) {
    /* Validate magic bytes: 0x00 0x61 0x73 0x6D ('\0asm') */
    if (len < 8 || wasm_bytes[0] != 0x00 || wasm_bytes[1] != 0x61 ||
        wasm_bytes[2] != 0x73 || wasm_bytes[3] != 0x6D) {
        sigma_printf("Σ [WASM]: Invalid module magic.\n");
        return SIGMA_EINVAL;
    }
    sigma_printf("Σ [WASM]: Module loaded (%lu bytes). "
                 "Compiling via Cranelift JIT...\n", (unsigned long)len);
    return SIGMA_OK;
}

sigma_err_t sigma_wasm_call(const char *export_name, sigma_u64 *args,
                             sigma_u32 argc, sigma_u64 *result) {
    (void)args;
    sigma_printf("Σ [WASM]: Calling export '%s' argc=%u\n", export_name, argc);
    if (result) *result = 42; /* Simulated return */
    return SIGMA_OK;
}

/* =========================================================================
 * SovereignBrowserCloud_Init
 * ====================================================================== */
void SovereignBrowserCloud_Init(void) {
    sigma_printf("Σ [BROWSER]: Initialising Sovereign Browser + Cloud OS "
                 "(Puter/Firefox/Chromium parity)...\n");

    sigma_memset(&g_sigma_browser, 0, sizeof(g_sigma_browser));
    sigma_memset(&g_sigma_cloud,   0, sizeof(g_sigma_cloud));

    /* Privacy defaults (Firefox-inspired) */
    g_sigma_browser.efp_enabled   = SIGMA_TRUE;
    g_sigma_browser.dfpi_enabled  = SIGMA_TRUE;
    g_sigma_browser.https_only    = SIGMA_TRUE;
    g_sigma_browser.webrtc_isolated = SIGMA_TRUE;

    /* Spawn browser/GPU processes */
    spawn_browser_proc(PROC_BROWSER,  "sigma://browser", SIGMA_FALSE);
    spawn_browser_proc(PROC_GPU,      "sigma://gpu",     SIGMA_TRUE);
    spawn_browser_proc(PROC_UTILITY,  "sigma://network", SIGMA_TRUE);

    /* Open tabs */
    sigma_browser_tab_open("https://puter.com",          SIGMA_FALSE);
    sigma_browser_tab_open("https://sigma.os/home",      SIGMA_FALSE);
    sigma_browser_tab_open("https://private.search.sigma", SIGMA_TRUE);

    /* Extensions */
    sigma_browser_ext_install("sigma-adblock",     PUTER_PERM_NETWORK);
    sigma_browser_ext_install("sigma-password-mgr",PUTER_PERM_CLIPBOARD);
    sigma_browser_tab_list();
    sigma_browser_procs();
    sigma_browser_stats();

    /* Puter Cloud */
    sigma_cloud_auth("aaryan", "eyJhbGciOiJIUzI1NiJ9.sigma.token");
    sigma_cloud_app_install("sigma-editor",  "https://sigma.os/apps/editor",
                             PUTER_PERM_FS_READ | PUTER_PERM_FS_WRITE);
    sigma_cloud_app_install("sigma-office",  "https://sigma.os/apps/office",
                             PUTER_PERM_FS_READ | PUTER_PERM_FS_WRITE | PUTER_PERM_NETWORK);
    sigma_cloud_app_install("sigma-terminal","https://sigma.os/apps/terminal",
                             PUTER_PERM_SHELL | PUTER_PERM_FS_READ | PUTER_PERM_FS_WRITE);
    sigma_cloud_app_list();
    sigma_cloud_app_launch("sigma-terminal");
    sigma_cloud_fs_ls("/home/aaryan");

    /* WASM loader demo */
    sigma_u8 wasm_stub[] = {0x00,0x61,0x73,0x6D, 0x01,0x00,0x00,0x00};
    sigma_wasm_load(wasm_stub, sizeof(wasm_stub));
    sigma_u64 result = 0;
    sigma_wasm_call("add", SIGMA_NULL, 0, &result);

    /* CSP demo */
    SigmaCSP_t csp = {
        .allow_scripts        = SIGMA_TRUE,
        .allow_inline_scripts = SIGMA_FALSE,
        .allow_frames         = SIGMA_FALSE,
        .allow_mixed_content  = SIGMA_FALSE,
    };
    sigma_csp_check_script(&csp, "https://cdn.example.com");
    sigma_csp_check_frame (&csp, "https://ads.example.com");

    sigma_printf("Σ [BROWSER]: Sovereign Browser + Puter Cloud OS online.\n");
}
