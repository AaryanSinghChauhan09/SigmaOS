// =============================================================================
// SigmaOS — S10_System — SovereignAppBundleManager.c
// Industrial-Grade Application Packaging & Bundle Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • macOS .app Bundles — Self-contained, multi-architecture binaries
//   • Windows AppX/MSIX — Secure, sandboxed application identity
//   • Flatpak (Linux)    — Runtime isolation and dependency bundling
//   • Android APK/AAB    — Manifest-driven permission system
// Architecture:
//   • .sab (Sovereign App Bundle) = VFS-mounted SquashFS-style archive
//   • Manifest-First: syscall-level permission enforcement based on signature
//   • Multi-Arch Fat Binaries: Support for x86_64, ARM64, and RISC-V in one bundle
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_types.h"


#define SAB_MAX_PERMISSIONS 32
#define SAB_MAGIC           "SIGMABND"

// ── SAB Manifest ──────────────────────────────────────────────────────────────
typedef struct {
    char     app_id[128];
    char     version[32];
    char     author[64];
    uint64_t permissions_bitmask; // S08 Capability mapping
    uint32_t entry_point_offset;
    uint8_t  signature[64];       // Sovereign-signed verification
} SabManifest;

// ── Bundle Handle ─────────────────────────────────────────────────────────────
typedef struct {
    char     bundle_path[256];
    bool     is_mounted;
    uint32_t mount_pid;           // PID of the running instance
    SabManifest manifest;
} SabBundle;

// ── Public API ────────────────────────────────────────────────────────────────

// Mount and verify a .sab bundle into the Sovereign VFS
SabBundle* app_bundle_mount(const char* sab_path);

// Launch an application from a bundle with strict sandboxing (S08)
uint32_t app_bundle_launch(SabBundle* bundle);

// Verify SAB signature against SigmaOS Root Certificate
bool app_bundle_verify(SabBundle* bundle);

// Resolve internal resources (icons, sounds) from bundle VFS
void* app_bundle_get_resource(SabBundle* bundle, const char* resource_name);

// Unmount and clean up bundle resources
void app_bundle_unmount(SabBundle* bundle);

// Query installed bundles in /apps/
uint32_t app_bundle_list_installed(SabBundle* out, uint32_t max);



