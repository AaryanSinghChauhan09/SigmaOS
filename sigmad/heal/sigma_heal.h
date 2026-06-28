// SPDX-License-Identifier: GPL-2.0-only
// sigma_heal.h — SigmaOS Self-Heal Subsystem
// Purpose: Autonomous OS repair daemon — filesystem corruption, kernel panic
//          recovery, package conflicts, network self-heal, security self-heal,
//          hardware self-heal. No IT support required.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// ---------------------------------------------------------------------------
// Heal Event Categories
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_HEAL_CAT_FILESYSTEM   = 1,   // Bad sectors, corrupted inodes, orphaned files
    SIGMA_HEAL_CAT_KERNEL       = 2,   // Kernel panic recovery, hotfix, rollback
    SIGMA_HEAL_CAT_PACKAGE      = 3,   // Broken deps, failed upgrades, conflicts
    SIGMA_HEAL_CAT_NETWORK      = 4,   // DNS, routes, Wi-Fi driver, DHCP
    SIGMA_HEAL_CAT_SECURITY     = 5,   // Intrusion isolation, rootkit, PQC key compromise
    SIGMA_HEAL_CAT_HARDWARE     = 6,   // GPU crash, sound fail, USB disconnect
    SIGMA_HEAL_CAT_SERVICE      = 7,   // Crashed or stuck daemon restart
    SIGMA_HEAL_CAT_MEMORY       = 8,   // OOM pressure relief, swap recovery
} sigma_heal_category_t;

typedef enum {
    SIGMA_HEAL_SEVERITY_INFO    = 0,   // Proactive: minor cleanup
    SIGMA_HEAL_SEVERITY_WARN    = 1,   // Detected issue, repaired without user impact
    SIGMA_HEAL_SEVERITY_ERROR   = 2,   // Repair required user-invisible fallback
    SIGMA_HEAL_SEVERITY_CRITICAL = 3,  // Kernel panic / security breach — logged prominently
} sigma_heal_severity_t;

typedef enum {
    SIGMA_HEAL_RESULT_FIXED     = 0,   // Fully repaired
    SIGMA_HEAL_RESULT_MITIGATED = 1,   // Partial repair, degraded mode
    SIGMA_HEAL_RESULT_ROLLED_BACK = 2, // Could not fix — rolled back to last good state
    SIGMA_HEAL_RESULT_FAILED    = 3,   // Could not repair — admin alert sent
    SIGMA_HEAL_RESULT_PENDING   = 4,   // Repair in progress
} sigma_heal_result_t;

// ---------------------------------------------------------------------------
// Heal Event Record
// ---------------------------------------------------------------------------

typedef struct {
    uint64_t             event_id;
    sigma_heal_category_t category;
    sigma_heal_severity_t severity;
    sigma_heal_result_t  result;
    time_t               detected_at;
    time_t               repaired_at;
    uint32_t             repair_duration_ms;
    char                 component[64];    // e.g. "btrfs:/dev/sda1", "wlan0", "nvidia.ko"
    char                 description[256]; // Human-readable description of the fault
    char                 action_taken[256];// What sigma-heal did
    char                 rollback_target[64]; // If rolled back, what generation
    bool                 user_notified;
    bool                 admin_alerted;
    char                 did_signature[128]; // DID-signed heal event (immutable audit)
} sigma_heal_event_t;

// ---------------------------------------------------------------------------
// Filesystem Self-Heal
// ---------------------------------------------------------------------------

typedef struct {
    char    device[64];           // e.g. /dev/sda1
    char    mountpoint[128];
    char    fstype[16];           // btrfs, ext4, xfs, f2fs
    bool    bad_sectors_found;
    uint32_t bad_sector_count;
    bool    corrupted_inodes;
    uint32_t corrupted_inode_count;
    uint32_t orphaned_files;
    bool    scrub_ran;            // btrfs scrub / fsck equivalent
    bool    repaired;
    bool    mirror_restore_needed; // If unfixable locally
    char    mirror_restore_url[256];
    time_t  last_scrub;
    time_t  next_scrub_due;
} sigma_heal_fs_status_t;

// Filesystem heal actions:
// 1. btrfs: btrfs scrub start → btrfs scrub status → btrfs device stats
// 2. ext4:  e2fsck -f -y
// 3. xfs:   xfs_repair
// 4. If repair fails: restore from sigma-mirror snapshot

int sigma_heal_fs_check(const char *device, sigma_heal_fs_status_t *out);
int sigma_heal_fs_repair(const char *device, sigma_heal_event_t *event_out);
int sigma_heal_fs_schedule_scrub(const char *device, int interval_days);

// ---------------------------------------------------------------------------
// Kernel Panic Recovery
// ---------------------------------------------------------------------------

typedef struct {
    char    panic_reason[256];     // Oops message summary
    char    faulting_module[64];   // Module that caused panic
    char    stack_trace_hash[64];  // SHA-256 of stack trace (for dedup)
    char    dump_path[256];        // Path to memory dump
    char    ai_diagnosis[512];     // sigma-ai analysis result
    char    suggested_fix[256];    // Hotfix or rollback recommendation
    bool    hotfix_applied;
    bool    rolled_back;
    char    rollback_generation[32];
    time_t  panic_time;
    uint32_t uptime_before_panic_s;
    bool    recovery_kernel_used;  // Booted to 2nd recovery kernel
} sigma_heal_kernel_panic_t;

// Kernel panic recovery flow:
// ON CRASH:  → capture full kdump (kernel crash dump)
//            → boot to recovery kernel (always slot-B)
//            → sigma-ai analyzes dump → identifies likely cause
//            → apply sigma-livepatch hotfix OR rollback to last-known-good
int sigma_heal_panic_analyze(const char *dump_path,
                              sigma_heal_kernel_panic_t *out);
int sigma_heal_panic_apply_hotfix(const char *patch_id,
                                   sigma_heal_event_t *event_out);
int sigma_heal_panic_rollback(sigma_heal_event_t *event_out);

// ---------------------------------------------------------------------------
// Package Self-Heal
// ---------------------------------------------------------------------------

typedef struct {
    char    failed_package[64];
    char    failure_reason[256];   // "Missing dep: libfoo.so.3"
    bool    dependency_resolved;
    bool    package_rolled_back;
    char    rolled_back_version[32];
    uint32_t packages_affected;
    char    packages_list[2048];   // Newline-separated list
} sigma_heal_pkg_status_t;

// Detects: broken dependencies after failed upgrade
// Fixes:   dependency solver + rollback broken packages
// Logs:    what was broken, what was done
int sigma_heal_pkg_check_broken(sigma_heal_pkg_status_t *out);
int sigma_heal_pkg_fix(sigma_heal_pkg_status_t *status,
                        sigma_heal_event_t *event_out);

// ---------------------------------------------------------------------------
// Network Self-Heal
// ---------------------------------------------------------------------------

typedef struct {
    bool    dns_resolving;
    char    dns_primary[16];
    char    dns_fallback[16];      // "1.1.1.1" or "8.8.8.8"
    bool    default_route_present;
    bool    dhcp_renewed;
    bool    wifi_driver_reloaded;
    char    wifi_module[32];       // e.g. "iwlwifi"
    char    failed_interface[16];
    char    action_taken[128];
} sigma_heal_net_status_t;

// Heal actions:
// DNS not resolving → try alternate DNS (1.1.1.1, 8.8.8.8, DoT)
// Default route gone → DHCP renew (sigma-netd --renew)
// Wi-Fi driver crashed → rmmod + modprobe (reload module)
// No IP → bring interface down+up → DHCP
int sigma_heal_net_check(const char *interface,
                          sigma_heal_net_status_t *out);
int sigma_heal_net_repair(sigma_heal_net_status_t *status,
                           sigma_heal_event_t *event_out);

// ---------------------------------------------------------------------------
// Security Self-Heal
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_HEAL_SEC_INTRUSION        = 1, // sigma-ids detected intrusion
    SIGMA_HEAL_SEC_ROOTKIT          = 2, // Rootkit / integrity violation
    SIGMA_HEAL_SEC_PQC_KEY_COMPROMISE = 3, // DID keypair compromised
    SIGMA_HEAL_SEC_UNAUTHORIZED_EXEC  = 4, // Unexpected privileged execution
} sigma_heal_sec_event_type_t;

typedef struct {
    sigma_heal_sec_event_type_t type;
    char    affected_process[64];  // PID + name of compromised process
    bool    process_isolated;      // sigma-jail --isolate applied
    bool    integrity_restored;    // Restored from PQ-signed verified baseline
    bool    did_rekeyed;           // New DID keypair generated
    char    new_did_key[128];      // New public key after rekey
    char    baseline_restore_gen[32]; // Which generation used for restore
    time_t  detected_at;
    time_t  remediated_at;
} sigma_heal_sec_status_t;

// sigma-ids detects intrusion → auto-isolate compromised process
// Rootkit detected → integrity restore from PQ-signed verified backup
// PQC key compromise → auto-generate new DID keypair
int sigma_heal_sec_isolate_process(pid_t pid, sigma_heal_event_t *event_out);
int sigma_heal_sec_restore_integrity(const char *path,
                                      sigma_heal_event_t *event_out);
int sigma_heal_sec_rekey_did(sigma_heal_event_t *event_out);

// ---------------------------------------------------------------------------
// Hardware Self-Heal
// ---------------------------------------------------------------------------

typedef struct {
    bool    gpu_driver_crashed;
    bool    gpu_software_fallback;  // Switched to llvmpipe/softpipe
    bool    sound_card_failed;
    bool    sound_muted_gracefully; // No kernel panic
    bool    usb_disconnect_safe;    // Safe state preserved on USB disconnect
    char    gpu_driver[32];         // "nvidia", "amdgpu", "i915"
    char    fallback_driver[32];    // "llvmpipe", "softpipe"
    char    failed_device[64];      // USB device path
} sigma_heal_hw_status_t;

// GPU driver crash → switch to software rendering (no black screen)
// Sound card failure → mute gracefully (no kernel panic, no freeze)
// USB disconnect during write → safe state, no data loss (write barrier)
int sigma_heal_hw_gpu_fallback(sigma_heal_event_t *event_out);
int sigma_heal_hw_sound_mute(sigma_heal_event_t *event_out);
int sigma_heal_hw_usb_safe_state(const char *device,
                                  sigma_heal_event_t *event_out);

// ---------------------------------------------------------------------------
// Simulation Mode
// ---------------------------------------------------------------------------

typedef struct {
    char    component[64];         // What to simulate failing
    char    predicted_impact[512]; // What sigma-heal predicts would happen
    char    predicted_action[256]; // What sigma-heal would do
    int     estimated_recovery_s;  // Estimated time to recover
    bool    user_impact;           // Would user notice?
    char    fallback_mode[64];     // Degraded mode description
} sigma_heal_simulation_t;

// sigma-heal simulate --component <name>
// Runs without affecting system — shows what WOULD happen
int sigma_heal_simulate(const char *component,
                         sigma_heal_simulation_t *out);

// ---------------------------------------------------------------------------
// Status & Reporting API
// ---------------------------------------------------------------------------

// sigma-heal status   → healed events in last 30 days summary
// sigma-heal log      → full repair history (paginated)
// sigma-heal stats    → MTTR, event counts by category

int sigma_heal_status(sigma_heal_event_t *events, int *count, int days);
int sigma_heal_log_query(sigma_heal_category_t cat, time_t from, time_t to,
                          sigma_heal_event_t *events, int *count);
int sigma_heal_stats_export(const char *output_json);

// ---------------------------------------------------------------------------
// Daemon Entry Point (sigmad/heal/main.go calls these via CGO)
// ---------------------------------------------------------------------------

// sigma-heald: background daemon
// - Polls every 60s for filesystem health
// - Subscribes to kernel panic notifications via sigma-bus
// - Subscribes to sigma-ids security events via sigma-bus
// - Subscribes to hardware events (driver crashes) via sigma-bus
// - Runs all repairs autonomously
// - Sends sigma-notify notification on WARN+ events

void sigma_heal_daemon_init(void);
void sigma_heal_daemon_run(void);  // Blocking event loop
void sigma_heal_daemon_stop(void);
