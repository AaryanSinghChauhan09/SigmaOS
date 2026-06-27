// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_extension.h — Safe sandboxed plugin/extension framework
 *
 * Allows apps to be extended without modifying SigmaOS core.
 * Every extension runs inside sigma-jail with minimal pledge/unveil
 * — a crash in an extension cannot bring down the host app.
 *
 * Extension manifest: extension.toml
 *   [extension]
 *   name       = "Maharashtra VAT Forms"
 *   version    = "1.0.0"
 *   target     = "sigma-legal"       # which app this extends
 *   pqc_signed = true                # Dilithium3 signature required
 *   author_did = "did:sigma:dev:..."
 *
 *   [permissions]
 *   bus_interfaces = ["sigma.Legal.Forms"]  # sigma-bus access
 *   unveil_read    = ["/sigma/share/legal"]
 *   unveil_write   = ["/home/user/.sigma/legal-forms"]
 *
 *   [entrypoints]
 *   menu_items = ["sigma-legal-forms/mh_vat_menu.js"]
 *   templates  = ["sigma-legal-forms/mh_form_3.html"]
 *
 * Distribution:
 *   sigma-pkg install sigma-extension-mh-vat
 *   sigma-pkg search extension --target sigma-legal
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_EXT_MAX_PERMISSIONS  16
#define SIGMA_EXT_MAX_ENTRYPOINTS  32

/* ── Extension manifest (parsed from extension.toml) ────────────────────── */
typedef struct {
    char     name[64];
    char     version[16];
    char     target_app[64];      /* which app this plugin extends          */
    char     author_did[128];     /* DID URI of publisher                   */
    bool     pqc_signed;          /* true = Dilithium3 signature present    */
    sigma_u8 signature[4628];     /* Dilithium3 signature (max size)        */
    sigma_u32 sig_len;

    /* Permissions declared in manifest */
    char     bus_interfaces[SIGMA_EXT_MAX_PERMISSIONS][64];
    int      n_bus_interfaces;
    char     unveil_read[SIGMA_EXT_MAX_PERMISSIONS][256];
    int      n_unveil_read;
    char     unveil_write[SIGMA_EXT_MAX_PERMISSIONS][256];
    int      n_unveil_write;

    /* Entrypoints */
    char     menu_items[SIGMA_EXT_MAX_ENTRYPOINTS][256];
    int      n_menu_items;
    char     templates[SIGMA_EXT_MAX_ENTRYPOINTS][256];
    int      n_templates;
    char     report_formats[SIGMA_EXT_MAX_ENTRYPOINTS][256];
    int      n_report_formats;
} sigma_extension_manifest_t;

/* ── Extension runtime state ─────────────────────────────────────────────── */
typedef enum {
    SIGMA_EXT_STATE_UNLOADED  = 0,
    SIGMA_EXT_STATE_LOADED    = 1,
    SIGMA_EXT_STATE_ACTIVE    = 2,
    SIGMA_EXT_STATE_CRASHED   = 3,  /* sandboxed — host app continues      */
    SIGMA_EXT_STATE_DISABLED  = 4,
} sigma_extension_state_t;

typedef struct {
    sigma_extension_manifest_t manifest;
    sigma_extension_state_t    state;
    sigma_u32                  jail_pid;       /* jail process PID          */
    sigma_u64                  load_time_ns;
    sigma_u32                  crash_count;
    char                       install_path[256];
} sigma_extension_t;

/* ── Extension API ───────────────────────────────────────────────────────── */

/* Load and verify an extension from its install directory. */
int sigma_extension_load(const char *install_dir,
                          sigma_extension_t *out);

/* Activate extension for the specified host app. */
int sigma_extension_activate(sigma_extension_t *ext,
                               const char        *host_app_id);

/* Deactivate (stop sandbox process, unregister menu items). */
int sigma_extension_deactivate(sigma_extension_t *ext);

/* Unload and remove extension. */
int sigma_extension_unload(sigma_extension_t *ext);

/* List installed extensions for a specific app. */
int sigma_extension_list(const char *target_app,
                          sigma_extension_t *out, int max,
                          int *count_out);

/*
 * Verify Dilithium3 signature on an extension.
 * pubkey: author's public key retrieved from their DID document.
 * Returns 0 if valid, -EBADMSG if invalid/unsigned.
 */
int sigma_extension_verify(const sigma_extension_t *ext,
                             const sigma_u8 *pubkey);

/* Send a sigma-bus message to an active extension's sandbox. */
int sigma_extension_send(sigma_extension_t *ext,
                          const char *interface,
                          const char *method,
                          const char *json_body);

/* Receive a sigma-bus message from an active extension. */
int sigma_extension_recv(sigma_extension_t *ext,
                          char *json_out, sigma_size_t max_len,
                          sigma_u32 timeout_ms);
