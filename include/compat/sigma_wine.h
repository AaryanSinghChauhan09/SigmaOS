/**
 * =========================================================================
 * Σ SIGMAOS: sigma-wine PUBLIC API
 * =========================================================================
 * Top-level interface to the Windows compatibility layer.
 * Used by sigma-cli, sigma-pod, and any SigmaOS component that needs to
 * launch or interact with Win32 applications.
 *
 * Architecture:
 *   sigma_wine_exec("notepad.exe", ...)
 *     → sigma_wine_loader  — PE detection + DLL wiring
 *       → sigma_pe_loader  — PE32+ parse + VMM map
 *         → sigma_ntdll    — NT syscall translation
 *           → sigma-kernel32 / sigma-user32 / sigma-gdi32
 *             → SigmaOS kernel (sigma-syscall ABI)
 * =========================================================================
 */
#pragma once
#include "../sigma_kernel_types.h"
#include "sigma_pe_types.h"
#include "sigma_nt_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* -----------------------------------------------------------------------
 * sigma-wine configuration
 * ----------------------------------------------------------------------- */
typedef struct {
    /* Path to the Wine prefix directory (holds registry + DLL stubs).
     * Default: /sigma/wine/default */
    const char* prefix_path;

    /* Force a specific D3D backend:
     *   SIGMA_WINE_D3D_AUTO   — detect from app (default)
     *   SIGMA_WINE_D3D_D3D9   — force DXVK D3D9
     *   SIGMA_WINE_D3D_D3D11  — force DXVK D3D11
     *   SIGMA_WINE_D3D_D3D12  — force vkd3d-proton D3D12
     *   SIGMA_WINE_D3D_NONE   — no D3D (console/CLI apps)
     */
    int d3d_backend;
#define SIGMA_WINE_D3D_AUTO   0
#define SIGMA_WINE_D3D_D3D9   1
#define SIGMA_WINE_D3D_D3D11  2
#define SIGMA_WINE_D3D_D3D12  3
#define SIGMA_WINE_D3D_NONE   4

    /* Enable NT syscall trace logging (slow — debug builds only) */
    int trace_syscalls;

    /* Enable sigma-audit ML-DSA attested log for all NT syscall translations */
    int audit_mode;

    /* Screen DPI for Zenith surface creation (96 = 100%, 192 = 200%) */
    int dpi;

    /* sigma-mac policy file; NULL = use /sigma/policy/wine_default.sigma-policy */
    const char* policy_file;
} sigma_wine_config_t;

/* -----------------------------------------------------------------------
 * Lifecycle
 * ----------------------------------------------------------------------- */

/**
 * sigma_wine_init — initialize the compat layer (call once at startup).
 * Opens registry, seeds standard hives, registers sigma-ntdll stubs.
 * @param config  Configuration. Pass NULL for defaults.
 * @return SIGMA_OK on success.
 */
sigma_status sigma_wine_init(const sigma_wine_config_t* config);

/**
 * sigma_wine_shutdown — release all compat-layer resources.
 * Waits for all running Win32 processes to exit first.
 */
void sigma_wine_shutdown(void);

/* -----------------------------------------------------------------------
 * Execution
 * ----------------------------------------------------------------------- */

/**
 * sigma_wine_exec — load and run a Windows PE executable.
 *
 * @param exe_path   Path to .exe file (SigmaOS VFS path)
 * @param argv       NULL-terminated argument array (argv[0] = exe name)
 * @param envp       NULL-terminated environment array, or NULL to inherit
 * @param config     Per-invocation overrides, or NULL to use init config
 * @return Process handle (HANDLE) or INVALID_HANDLE_VALUE on failure
 */
HANDLE sigma_wine_exec(
    const char*              exe_path,
    const char* const*       argv,
    const char* const*       envp,
    const sigma_wine_config_t* config
);

/**
 * sigma_wine_wait — wait for a Win32 process to exit.
 * @param process_handle  Handle from sigma_wine_exec
 * @param timeout_ms      Timeout in milliseconds; -1 = infinite
 * @param exit_code       Output: process exit code
 * @return SIGMA_OK, SIGMA_ERR_TIMEOUT, or error
 */
sigma_status sigma_wine_wait(HANDLE process_handle,
                              sigma_s32 timeout_ms,
                              sigma_u32* exit_code);

/**
 * sigma_wine_kill — terminate a running Win32 process.
 * @param process_handle  Handle from sigma_wine_exec
 * @param exit_code       Exit code to assign
 */
sigma_status sigma_wine_kill(HANDLE process_handle, sigma_u32 exit_code);

/* -----------------------------------------------------------------------
 * PE inspection (no execution — useful for sigma-wine --info)
 * ----------------------------------------------------------------------- */
typedef struct {
    char     machine[16];    /* "x86-64", "ARM64", etc. */
    char     subsystem[16];  /* "Console", "GUI", "DLL" */
    sigma_u32 image_base;
    sigma_u32 entry_point_rva;
    sigma_u32 section_count;
    sigma_u32 import_dll_count;
    int      is_pie;
    int      is_dll;
    char     version[32];   /* from VS_VERSIONINFO if present */
} sigma_wine_pe_info_t;

/**
 * sigma_wine_inspect — parse PE headers without executing.
 * @param exe_path  VFS path to .exe or .dll
 * @param info      Output structure
 * @return SIGMA_OK or error
 */
sigma_status sigma_wine_inspect(const char* exe_path,
                                 sigma_wine_pe_info_t* info);

/* -----------------------------------------------------------------------
 * DLL management
 * ----------------------------------------------------------------------- */

/**
 * sigma_wine_register_dll — register a sigma-wine DLL stub (sigma-kernel32 etc.)
 * so that PE import resolution can find it by name.
 * @param dll_name   e.g. "kernel32.dll"
 * @param sigma_path VFS path to the sigma-wine native implementation
 */
sigma_status sigma_wine_register_dll(const char* dll_name,
                                      const char* sigma_path);

/**
 * sigma_wine_override_dll — override a specific DLL import with a
 * sigma-wine stub (equivalent to WINEDLLOVERRIDES).
 * @param dll_name  e.g. "d3d11.dll"
 * @param mode      SIGMA_WINE_DLL_BUILTIN or SIGMA_WINE_DLL_NATIVE
 */
#define SIGMA_WINE_DLL_BUILTIN 0  /* use sigma-wine implementation */
#define SIGMA_WINE_DLL_NATIVE  1  /* use DLL from prefix (if present) */
sigma_status sigma_wine_override_dll(const char* dll_name, int mode);

/* -----------------------------------------------------------------------
 * Prefix management
 * ----------------------------------------------------------------------- */

/**
 * sigma_wine_create_prefix — create a new isolated Wine prefix.
 * Sets up registry skeleton, sigma-windows directory structure, and
 * registers all sigma-wine DLL stubs.
 * @param prefix_path  VFS path for the new prefix
 * @param arch         "win64" (default) or "win32"
 */
sigma_status sigma_wine_create_prefix(const char* prefix_path,
                                       const char* arch);

/* -----------------------------------------------------------------------
 * Version / capability query
 * ----------------------------------------------------------------------- */
typedef struct {
    sigma_u32 major;          /* sigma-wine major version */
    sigma_u32 minor;
    int  has_dxvk_d3d9;
    int  has_dxvk_d3d11;
    int  has_vkd3d_d3d12;
    int  has_winsock2;
    int  has_com;
    int  has_dotnet;
    char wine_compat_version[16]; /* Wine version we emulate, e.g. "9.0" */
} sigma_wine_caps_t;

void sigma_wine_query_caps(sigma_wine_caps_t* caps);

#ifdef __cplusplus
} /* extern "C" */
#endif
