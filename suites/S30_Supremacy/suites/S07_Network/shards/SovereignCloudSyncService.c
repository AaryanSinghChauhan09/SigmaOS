// =============================================================================
// SigmaOS — S07_Network — SovereignCloudSyncService.c
// Sovereign Cloud Sync & Remote Desktop Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • iCloud (Apple)    — transparent file sync, delta compression
//   • OneDrive (Windows)— on-demand placeholders, selective sync
//   • Dropbox           — block-level dedup, LAN sync peer discovery
//   • Google Drive      — real-time collaborative document streaming
//   • Microsoft RDP     — hardware-encoded GPU frame streaming
// Architecture:
//   • File watcher hooks into VFS change notifications (S06)
//   • Block-dedup: only changed 4MB chunks uploaded (Dropbox model)
//   • On-demand placeholders: file appears in VFS without download (OneDrive)
//   • LAN peer discovery via mDNS shard for zero-latency local sync
//   • Remote Desktop: GPU H.265 encode → RTP stream (RDP hardware model)
// =============================================================================

#include "../../../../../include/core/sigma_types.h"


#define SIGMA_CLOUD_MAX_PROVIDERS   8
#define SIGMA_SYNC_CHUNK_SIZE      (4 * 1024 * 1024)  // 4MB block dedup

// ── Sync Modes ────────────────────────────────────────────────────────────────
typedef enum {
    SYNC_MODE_FULL_MIRROR    = 0,  // All files local (iCloud default)
    SYNC_MODE_ON_DEMAND      = 1,  // Placeholders only (OneDrive model)
    SYNC_MODE_SELECTIVE      = 2,  // User-chosen folder subset
} CloudSyncMode;

// ── Cloud Provider Descriptor ─────────────────────────────────────────────────
typedef struct {
    const char*  provider_name;
    const char*  endpoint_url;
    CloudSyncMode mode;
    bool         lan_sync_enabled;  // Dropbox LAN sync via mDNS
    void       (*on_conflict)(const char* path); // Conflict resolution callback
} CloudProvider;

// ── Remote Desktop Session ────────────────────────────────────────────────────
typedef struct {
    uint32_t session_id;
    uint32_t client_ip;
    uint16_t port;
    uint8_t  codec;   // 0=H.264, 1=H.265, 2=AV1
    bool     is_active;
} RemoteDesktopSession;

// ── Public API ────────────────────────────────────────────────────────────────

// Register a cloud sync provider and begin monitoring the VFS
bool cloud_sync_register_provider(CloudProvider* provider, const char* local_root);

// Force a manual sync cycle for a registered provider
void cloud_sync_trigger(const char* provider_name);

// Create an on-demand file placeholder in the VFS (OneDrive model)
void cloud_sync_create_placeholder(const char* vfs_path, uint64_t remote_size);

// Hydrate (download) a placeholder when the user first opens it
bool cloud_sync_hydrate_file(const char* vfs_path);

// Start a hardware-encoded Remote Desktop listener (RDP H.265 GPU model)
RemoteDesktopSession* rdp_start_session(uint16_t port, uint8_t codec);

// Stop and clean up a Remote Desktop session
void rdp_terminate_session(uint32_t session_id);



