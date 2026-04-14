// =============================================================================
// SigmaOS — S10_System — SovereignApiBridge.c
// Multi-OS API Compatibility & Syscall Bridge
// =============================================================================
// Competitor USPs Absorbed:
//   • Wine / Proton    — Standard for running Windows apps on Linux
//   • WSL (Original)   — Linux syscall translation on Windows
//   • Darling (Linux)  — macOS API translation on Linux
// SigmaOS API Bridge:
//   • Direct syscall re-mapping: Windows NT syscalls -> SigmaOS shards
//   • Userland Library Thunking: Redirects User32/Cocoa calls to ZenithUI
//   • 100% Zero-Dependency: No external glibc or wine-runtime required.
// =============================================================================

#include <sigma_types.h>


typedef enum {
    API_WIN_NT    = 0,
    API_LINUX_SYSC = 1,
    API_DARWIN_MACH= 2
} ApiType;

// ── Call Descriptor ──────────────────────────────────────────────────────────
typedef struct {
    uint32_t syscall_num;
    ApiType  source_api;
    uintptr_t args[6];
} ApiCall;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the API Bridge nexus
void api_bridge_init(void);

// Intercept and route a foreign syscall to a Sovereign shard
long api_bridge_dispatch(ApiCall* call);

// Map a Windows "DLL" to a Sovereign "Shard" (Registry v2 hook)
bool api_bridge_map_library(const char* dll_name, uint16_t shard_id);

// Emulate a Linux /proc or /sys entry via SigmaOS VFS (S06)
void api_bridge_emulate_vfs(const char* guest_path, const char* host_path);

// Log failed translations to the Neural Oracle (S13) for future evolution
void api_bridge_audit_miss(ApiCall* call);

// Formal Verification: Check if a translation is memory-safe (S08)
bool api_bridge_verify_safety(ApiCall* call);



