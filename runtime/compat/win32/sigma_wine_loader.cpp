/**
 * =========================================================================
 * Σ SIGMAOS: sigma-wine LOADER  — Stage 7
 * =========================================================================
 * Top-level orchestrator for the Windows compatibility layer.
 * Detects PE32+ binaries, wires sigma-ntdll stubs, invokes sigma-pe-loader,
 * and launches the process via sigma-wine-server.
 *
 * This is the code path triggered by:
 *   sigma-wine notepad.exe
 *   sigma-cli wine exec game.exe
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/compat/sigma_pe_types.h"
#include "../../../include/compat/sigma_nt_types.h"
#include "../../../include/compat/sigma_wine.h"

/* Forward declarations */
extern "C" {
    sigma_status sigma_pe_load(const sigma_u8* buf, sigma_usize len, void* out_img);
    void         sigma_pe_inspect(const sigma_u8* buf, sigma_usize len);
    NTSTATUS     NtAllocateVirtualMemory(HANDLE, PVOID*, ULONG, PULONG, ULONG, ULONG);
    /* VFS read — production: sigma_vfs_read(path, &buf, &len) */
    sigma_s32    sigma_vfs_read_file(const char* path, sigma_u8** buf, sigma_usize* len);
    void         sigma_vfs_free_buf(sigma_u8* buf);
    /* sigma-ntdll init */
    void         sigma_ntdll_init(void);
    /* sigma-reg init */
    NTSTATUS     sigma_reg_init(const char* db_path);
}

namespace SigmaOS { namespace Compat { namespace Win32 {

/* -----------------------------------------------------------------------
 * Built-in DLL stub registry
 * Maps "kernel32.dll" → path to sigma-wine native implementation.
 * ----------------------------------------------------------------------- */
struct DllStub { const char* dll_name; const char* sigma_path; };
static const DllStub g_dll_stubs[] = {
    { "ntdll.dll",          "/sigma/wine/lib/sigma_ntdll.so"      },
    { "kernel32.dll",       "/sigma/wine/lib/sigma_kernel32.so"   },
    { "kernelbase.dll",     "/sigma/wine/lib/sigma_kernel32.so"   },
    { "user32.dll",         "/sigma/wine/lib/sigma_user32.so"     },
    { "gdi32.dll",          "/sigma/wine/lib/sigma_gdi32.so"      },
    { "advapi32.dll",       "/sigma/wine/lib/sigma_advapi32.so"   },
    { "shell32.dll",        "/sigma/wine/lib/sigma_shell32.so"    },
    { "comctl32.dll",       "/sigma/wine/lib/sigma_comctl32.so"   },
    { "msvcrt.dll",         "/sigma/wine/lib/sigma_msvcrt.so"     },
    { "vcruntime140.dll",   "/sigma/wine/lib/sigma_msvcrt.so"     },
    { "ucrtbase.dll",       "/sigma/wine/lib/sigma_msvcrt.so"     },
    { "ws2_32.dll",         "/sigma/wine/lib/sigma_winsock2.so"   },
    { "winmm.dll",          "/sigma/wine/lib/sigma_winmm.so"      },
    { "ole32.dll",          "/sigma/wine/lib/sigma_com.so"        },
    { "oleaut32.dll",       "/sigma/wine/lib/sigma_com.so"        },
    { "d3d9.dll",           "/sigma/wine/lib/sigma_dxvk_d3d9.so"  },
    { "d3d11.dll",          "/sigma/wine/lib/sigma_dxvk_d3d11.so" },
    { "d3d12.dll",          "/sigma/wine/lib/sigma_dxvk_d3d12.so" },
    { "dxgi.dll",           "/sigma/wine/lib/sigma_dxgi.so"       },
    { nullptr, nullptr }
};

static const char* find_dll_stub(const char* dll_name) {
    for (const DllStub* s = g_dll_stubs; s->dll_name; s++) {
        /* Case-insensitive compare (ASCII) */
        const char* a = dll_name; const char* b = s->dll_name;
        bool match = true;
        while (*a && *b) {
            char ca = (*a >= 'A' && *a <= 'Z') ? (*a + 32) : *a;
            char cb = (*b >= 'A' && *b <= 'Z') ? (*b + 32) : *b;
            if (ca != cb) { match = false; break; }
            a++; b++;
        }
        if (match && !*a && !*b) return s->sigma_path;
    }
    return nullptr;
}

/* -----------------------------------------------------------------------
 * SigmaWineLoader
 * ----------------------------------------------------------------------- */
class SigmaWineLoader {
public:
    SigmaWineLoader() : m_initialized(false) {}

    sigma_status init(const sigma_wine_config_t* cfg) {
        const char* prefix = cfg ? cfg->prefix_path : "/sigma/wine/default";
        sigma_log("[sigma-wine] Initializing. Prefix: %s", prefix);

        /* 1. Registry */
        char reg_path[512];
        for (int i = 0; prefix[i] && i < 490; i++) reg_path[i] = prefix[i];
        const char* suf = "/registry.db";
        int plen = 0; while(reg_path[plen]) plen++;
        for (int i = 0; suf[i]; i++) reg_path[plen++] = suf[i];
        reg_path[plen] = '\0';

        NTSTATUS rs = sigma_reg_init(reg_path);
        if (!NT_SUCCESS(rs))
            sigma_log("[sigma-wine] WARNING: registry init failed (0x%X) — using in-memory", rs);

        /* 2. NT layer */
        sigma_ntdll_init();

        /* 3. Register built-in DLL stubs */
        sigma_u32 nstubs = 0;
        for (const DllStub* s = g_dll_stubs; s->dll_name; s++) {
            sigma_log_info("[sigma-wine] DLL stub: %-25s → %s", s->dll_name, s->sigma_path);
            nstubs++;
        }
        sigma_log("[sigma-wine] %u DLL stubs registered.", nstubs);

        m_initialized = true;
        return K_OK;
    }

    /**
     * exec — load and launch a Windows PE executable.
     */
    HANDLE exec(const char* exe_path, const char* const* argv,
                const char* const* envp, const sigma_wine_config_t* cfg)
    {
        if (!m_initialized) init(cfg);

        sigma_log("[sigma-wine] exec: %s", exe_path);

        /* 1. Read PE file from sigma-vfs */
        sigma_u8*   buf  = nullptr;
        sigma_usize len  = 0;
        sigma_s32   err  = sigma_vfs_read_file(exe_path, &buf, &len);
        if (err != 0 || !buf) {
            sigma_log_err("[sigma-wine] Cannot read file: %s (err=%d)", exe_path, err);
            return INVALID_HANDLE_VALUE;
        }

        /* 2. Quick MZ check */
        if (len < 2 || buf[0] != 'M' || buf[1] != 'Z') {
            sigma_log_err("[sigma-wine] Not a PE file: %s", exe_path);
            sigma_vfs_free_buf(buf);
            return INVALID_HANDLE_VALUE;
        }

        /* 3. Parse PE */
        /* sizeof(PeLoadedImage) — use opaque buffer; real type from sigma_pe_loader */
        sigma_u8 img_buf[4096] = {};
        sigma_status load_rc = sigma_pe_load(buf, len, img_buf);
        sigma_vfs_free_buf(buf);

        if (load_rc != K_OK) {
            sigma_log_err("[sigma-wine] PE load failed: %s", exe_path);
            return INVALID_HANDLE_VALUE;
        }

        /* 4. TODO: create address space via sigma-vmm, map sections, resolve IAT */
        sigma_log("[sigma-wine] TODO: sigma_vmm_create_aspace() + map PE sections");

        /* 5. TODO: launch via sigma-wine-server IPC */
        sigma_log("[sigma-wine] TODO: sigma_wine_server_spawn(entry_point, stack)");

        /* Return fake process handle for now */
        sigma_log("[sigma-wine] Process launched (stub). HANDLE=0x100");
        return (HANDLE)(sigma_u64)0x100;
    }

    /**
     * inspect — dump PE info without executing.
     */
    sigma_status inspect(const char* exe_path, sigma_wine_pe_info_t* info) {
        sigma_u8*   buf = nullptr;
        sigma_usize len = 0;
        if (sigma_vfs_read_file(exe_path, &buf, &len) != 0 || !buf)
            return K_ERR_INVAL;
        sigma_pe_inspect(buf, len);
        sigma_vfs_free_buf(buf);
        /* TODO: populate info struct from parsed headers */
        if (info) {
            __builtin_memcpy(info->machine, "x86-64", 7);
            __builtin_memcpy(info->subsystem, "Unknown", 8);
        }
        return K_OK;
    }

    void query_caps(sigma_wine_caps_t* caps) {
        if (!caps) return;
        caps->major           = 0;
        caps->minor           = 1;
        caps->has_dxvk_d3d9   = 0;  /* TODO: set when d3d9 built */
        caps->has_dxvk_d3d11  = 0;
        caps->has_vkd3d_d3d12 = 0;
        caps->has_winsock2    = 0;
        caps->has_com         = 0;
        caps->has_dotnet      = 0;
        __builtin_memcpy(caps->wine_compat_version, "9.0", 4);
    }

private:
    bool m_initialized;
};

static SigmaWineLoader g_loader;

} /* Win32 */ } /* Compat */ } /* SigmaOS */

/* -----------------------------------------------------------------------
 * sigma-wine public C API (sigma_wine.h)
 * ----------------------------------------------------------------------- */
extern "C" {

sigma_status sigma_wine_init(const sigma_wine_config_t* config) {
    return SigmaOS::Compat::Win32::g_loader.init(config);
}

void sigma_wine_shutdown(void) {
    sigma_log("[sigma-wine] shutdown.");
}

HANDLE sigma_wine_exec(const char* exe_path, const char* const* argv,
                        const char* const* envp,
                        const sigma_wine_config_t* config) {
    return SigmaOS::Compat::Win32::g_loader.exec(exe_path, argv, envp, config);
}

sigma_status sigma_wine_wait(HANDLE h, sigma_s32 timeout_ms, sigma_u32* exit_code) {
    (void)h; (void)timeout_ms;
    if (exit_code) *exit_code = 0;
    return K_OK;
}

sigma_status sigma_wine_kill(HANDLE h, sigma_u32 exit_code) {
    (void)h; (void)exit_code;
    return K_OK;
}

sigma_status sigma_wine_inspect(const char* exe_path, sigma_wine_pe_info_t* info) {
    return SigmaOS::Compat::Win32::g_loader.inspect(exe_path, info);
}

sigma_status sigma_wine_register_dll(const char* dll_name, const char* sigma_path) {
    sigma_log_info("[sigma-wine] register_dll: %s → %s", dll_name, sigma_path);
    return K_OK;
}

sigma_status sigma_wine_override_dll(const char* dll_name, int mode) {
    sigma_log_info("[sigma-wine] override_dll: %s mode=%d", dll_name, mode);
    return K_OK;
}

sigma_status sigma_wine_create_prefix(const char* prefix_path, const char* arch) {
    sigma_log("[sigma-wine] create_prefix: %s arch=%s", prefix_path, arch ? arch : "win64");
    /* TODO: mkdir -p prefix_path/{drive_c,dosdevices}, seed registry */
    return K_OK;
}

void sigma_wine_query_caps(sigma_wine_caps_t* caps) {
    SigmaOS::Compat::Win32::g_loader.query_caps(caps);
}

/* sigma_ntdll_init stub — real work done inside sigma_ntdll.cpp */
void sigma_ntdll_init(void) {
    sigma_log("[ntdll] NT native API layer ready.");
}

} /* extern "C" */
