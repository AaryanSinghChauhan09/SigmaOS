/**
 * SigmaOS Zenith CLI Bridge — programmatic access to Zenith Desktop GUI operations.
 * Maps GUI apps (settings, files, browser, panel, WM, dashboard) to stable CLI tools
 * so sigma-agent and sigma-sh can perform any GUI task without mouse/keyboard.
 */
#ifndef SIGMA_SOVEREIGN_ZENITH_CLI_H
#define SIGMA_SOVEREIGN_ZENITH_CLI_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_ZENITH_APP_ID_MAX     64
#define SIGMA_ZENITH_PATH_MAX      256
#define SIGMA_ZENITH_QUERY_MAX     512
#define SIGMA_ZENITH_OUTPUT_MAX   4096
#define SIGMA_ZENITH_APP_LIST_MAX   32

typedef enum {
    SIGMA_ZENITH_LAYOUT_MOSAIC = 0,
    SIGMA_ZENITH_LAYOUT_TILE,
    SIGMA_ZENITH_LAYOUT_STACK,
    SIGMA_ZENITH_LAYOUT_FLOAT
} sigma_zenith_layout_t;

typedef enum {
    SIGMA_ZENITH_THEME_OBSIDIAN = 0,
    SIGMA_ZENITH_THEME_CYBER,
    SIGMA_ZENITH_THEME_PAPER,
    SIGMA_ZENITH_THEME_HIGH_CONTRAST
} sigma_zenith_theme_t;

typedef enum {
    SIGMA_ZENITH_APP_SETTINGS = 0,
    SIGMA_ZENITH_APP_FILES,
    SIGMA_ZENITH_APP_BROWSER,
    SIGMA_ZENITH_APP_TERMINAL,
    SIGMA_ZENITH_APP_DASHBOARD,
    SIGMA_ZENITH_APP_PANEL,
    SIGMA_ZENITH_APP_DATALAB,
    SIGMA_ZENITH_APP_RECOVERY
} sigma_zenith_app_t;

typedef struct {
    char app_id[SIGMA_ZENITH_APP_ID_MAX];
    char title[128];
    bool running;
    sigma_i32 workspace;
} sigma_zenith_app_info_t;

int sigma_zenith_cli_init(void);

/* Compositor & session */
int sigma_zenith_start(char* status_out, sigma_u32 cap);
int sigma_zenith_stop(char* status_out, sigma_u32 cap);
int sigma_zenith_status(char* json_out, sigma_u32 cap);

/* Layout & theme (replaces zenith-layout / zenith-theme GUI) */
int sigma_zenith_set_layout(sigma_zenith_layout_t layout, char* status_out, sigma_u32 cap);
int sigma_zenith_set_theme(sigma_zenith_theme_t theme, char* status_out, sigma_u32 cap);
int sigma_zenith_get_layout(char* name_out, sigma_u32 cap);
int sigma_zenith_get_theme(char* name_out, sigma_u32 cap);

/* Window manager */
int sigma_zenith_switch_workspace(sigma_i32 index, char* status_out, sigma_u32 cap);
int sigma_zenith_arrange_tiling(char* status_out, sigma_u32 cap);
int sigma_zenith_focus_app(const char* app_id, char* status_out, sigma_u32 cap);

/* App lifecycle (replaces clicking Applications menu) */
int sigma_zenith_app_launch(sigma_zenith_app_t app, char* status_out, sigma_u32 cap);
int sigma_zenith_app_close(const char* app_id, char* status_out, sigma_u32 cap);
int sigma_zenith_app_list(sigma_zenith_app_info_t* out, sigma_u32* count);

/* zenith-settings — declarative control center */
int sigma_zenith_settings_get(const char* key, char* value_out, sigma_u32 cap);
int sigma_zenith_settings_set(const char* key, const char* value, char* status_out, sigma_u32 cap);
int sigma_zenith_settings_list(char* json_out, sigma_u32 cap);

/* zenith-files — semantic + tree file manager */
int sigma_zenith_files_search(const char* query, char* results_out, sigma_u32 cap);
int sigma_zenith_files_tree(const char* path, char* tree_out, sigma_u32 cap);
int sigma_zenith_files_open(const char* path, char* status_out, sigma_u32 cap);

/* zenith-browser */
int sigma_zenith_browser_navigate(const char* url, char* status_out, sigma_u32 cap);

/* sigma-dashboard — AI observability */
int sigma_zenith_dashboard_metrics(char* panel_out, sigma_u32 cap);
int sigma_zenith_dashboard_query(const char* prompt, char* diagnosis_out, sigma_u32 cap);

/* Unified dispatch — parse "zenith theme cyber" style commands */
int sigma_zenith_cli_exec(const char* command_line, char* output_out, sigma_u32 cap);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SOVEREIGN_ZENITH_CLI_H */
