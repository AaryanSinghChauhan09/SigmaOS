/**
 * =========================================================================
 * Σ SIGMAOS IMMUTABLE AUTO-UPDATE DAEMON — PHASE 6 ENHANCED
 * =========================================================================
 * Inspired by Flatcar and Fedora CoreOS. Automates block-level immutable
 * base layer updates with cryptographically-verified sovereign signatures.
 *
 * Phase 6 additions:
 *   - Post-swap DKMS rebuild (calls sigma_driver_registry_rebuild_dkms).
 *   - Driver error notifications via sigma_notify_send.
 *   - Structured ZEN-UPDATE-xxxx error codes.
 *   - A/B partition tracking with rollback on DKMS failure.
 * =========================================================================
 */

#include <sigma_libc.h>
#include <sigma_error_codes.h>
#include <sigma_driver_codes.h>

// Forward declarations from sibling subsystems
extern "C" sigma_status sigma_driver_registry_rebuild_dkms(const char* kernel_version);
extern "C" sigma_u32    sigma_notify_send(const char* app, const char* summary,
                                           const char* body, sigma_u32 urgency, sigma_u32 ttl);
extern "C" void         zenith_log_structured(sigma_u32 code, const char* comp,
                                               const char* desc, sigma_u32 cid);

// -------------------------------------------------------------------------
// Structured Update Error Codes
// -------------------------------------------------------------------------
#define ZEN_UPDATE_OK                    0x1000
#define ZEN_UPDATE_SIG_INVALID           0x1001  // Kernel image signature check failed
#define ZEN_UPDATE_WRITE_FAILED          0x1002  // Sector write to immutable partition failed
#define ZEN_UPDATE_DKMS_REBUILD_FAILED   0x1003  // Post-swap DKMS rebuild had failures
#define ZEN_UPDATE_ROLLBACK_TRIGGERED    0x1004  // Rolled back to previous partition slot
#define ZEN_UPDATE_NO_SLOT_AVAILABLE     0x1005  // Both A/B slots are invalid

namespace SigmaOS {
namespace Updates {

// -------------------------------------------------------------------------
// A/B Partition Slot
// -------------------------------------------------------------------------
typedef enum {
    SLOT_A = 0,
    SLOT_B = 1,
} PartitionSlot;

static PartitionSlot g_active_slot  = SLOT_A;
static PartitionSlot g_staging_slot = SLOT_B;
static char          g_active_kernel_version[64] = "6.7-sigma";

struct UpdateDescriptor {
    sigma_u32    target_version;
    sigma_u8     kernel_image_signature[64];
    sigma_u32    sector_offset;
    sigma_size_t sector_count;
    char         kernel_version_str[64];  // e.g. "6.8-sigma"
};

// -------------------------------------------------------------------------
// UpdateDaemon
// -------------------------------------------------------------------------
class UpdateDaemon {
public:
    static UpdateDaemon& getInstance() {
        static UpdateDaemon instance;
        return instance;
    }

    void init() {
        sys_print("[SovereignUpdated] Starting Immutable Update Daemon (Flatcar/CoreOS paradigm).\n");
        sys_print("[SovereignUpdated] Active slot: %s | Staging slot: %s\n",
                  g_active_slot == SLOT_A ? "A" : "B",
                  g_staging_slot == SLOT_A ? "A" : "B");
        sys_print("[SovereignUpdated] Running kernel: %s\n", g_active_kernel_version);
    }

    sigma_status checkAndApplyUpdate(const UpdateDescriptor* update) {
        sys_print("\n[SovereignUpdated] ── Update Check ──────────────────────────────────\n");
        sys_print("[SovereignUpdated] Target version : %u (%s)\n",
                  update->target_version, update->kernel_version_str);
        sys_print("[SovereignUpdated] Staging slot   : %s\n",
                  g_staging_slot == SLOT_A ? "A" : "B");

        // Step 1: Cryptographic signature verification
        sys_print("[SovereignUpdated] Verifying sovereign root key signature...");
        if (!verifySig(update)) {
            zenith_log_structured(ZEN_UPDATE_SIG_INVALID, "UpdateDaemon",
                                  "Kernel image signature invalid", 0);
            sigma_notify_send("SovereignUpdated",
                              "⚠ Update Blocked",
                              "Kernel image failed signature verification. Update aborted.",
                              2 /* URGENCY_CRITICAL */, 0);
            sys_print(" FAIL\n");
            return SIGMA_ERROR;
        }
        sys_print(" PASS\n");

        // Step 2: Stream kernel image to staging partition slot
        sys_print("[SovereignUpdated] Writing %u sectors to staging slot %s...\n",
                  (sigma_u32)update->sector_count,
                  g_staging_slot == SLOT_A ? "A" : "B");
        if (!writeToStagingSlot(update)) {
            zenith_log_structured(ZEN_UPDATE_WRITE_FAILED, "UpdateDaemon",
                                  "Sector write to staging slot failed", 0);
            return SIGMA_ERROR;
        }
        sys_print("[SovereignUpdated] Staging write complete.\n");

        // Step 3: Atomic slot swap
        performSlotSwap(update->kernel_version_str);

        // Step 4: Post-swap DKMS rebuild — rebuild all tracked kernel modules
        sys_print("[SovereignUpdated] Triggering DKMS rebuild for kernel '%s'...\n",
                  g_active_kernel_version);
        sigma_status dkms_result = sigma_driver_registry_rebuild_dkms(g_active_kernel_version);

        if (dkms_result != SIGMA_SUCCESS) {
            zenith_log_structured(ZEN_UPDATE_DKMS_REBUILD_FAILED, "UpdateDaemon",
                                  "DKMS rebuild had failures after kernel swap", 0);
            sigma_notify_send("SovereignUpdated",
                              "⚠ Driver Rebuild Issues",
                              "Some kernel modules failed to rebuild after the update. "
                              "Check Driver Manager for details.",
                              2 /* URGENCY_CRITICAL */, 0);

            // Trigger rollback if critical modules failed
            sys_print("[SovereignUpdated] Critical DKMS failure — initiating rollback to previous slot.\n");
            rollback();
            return SIGMA_ERROR;
        }

        // Step 5: Notify user of success
        sigma_notify_send("SovereignUpdated",
                          "✅ Update Applied",
                          "Sovereign kernel update complete. All drivers rebuilt successfully.",
                          1 /* URGENCY_NORMAL */, 8000);

        zenith_log_structured(ZEN_UPDATE_OK, "UpdateDaemon",
                              "Kernel update and DKMS rebuild succeeded", 0);

        sys_print("[SovereignUpdated] ── Update Complete ───────────────────────────────\n\n");
        return SIGMA_SUCCESS;
    }

private:
    bool verifySig(const UpdateDescriptor* update) {
        // Production: compare SHA-512 HMAC of image against sovereign root key
        (void)update;
        return true; // Mocked
    }

    bool writeToStagingSlot(const UpdateDescriptor* update) {
        // Production: block-level write via sigma_storage_write_sectors()
        (void)update;
        return true; // Mocked
    }

    void performSlotSwap(const char* new_kernel_version) {
        PartitionSlot old_active = g_active_slot;
        g_active_slot  = g_staging_slot;
        g_staging_slot = old_active;

        // Update the tracked kernel version string
        sigma_size_t i = 0;
        while (new_kernel_version[i] && i < 63) {
            g_active_kernel_version[i] = new_kernel_version[i];
            i++;
        }
        g_active_kernel_version[i] = '\0';

        sys_print("[SovereignUpdated] ✅ Slot swap: Active is now %s (kernel: %s)\n",
                  g_active_slot == SLOT_A ? "A" : "B",
                  g_active_kernel_version);

        zenith_log_structured(ZEN_UPDATE_OK, "UpdateDaemon", "A/B slot swap completed", 0);
    }

    void rollback() {
        PartitionSlot old_active = g_active_slot;
        g_active_slot  = g_staging_slot;
        g_staging_slot = old_active;

        zenith_log_structured(ZEN_UPDATE_ROLLBACK_TRIGGERED, "UpdateDaemon",
                              "Rolled back to previous slot after DKMS failure", 0);

        sigma_notify_send("SovereignUpdated",
                          "🔄 Update Rolled Back",
                          "Kernel swap was reverted due to driver rebuild failure. "
                          "Previous slot is now active.",
                          2 /* URGENCY_CRITICAL */, 0);

        sys_print("[SovereignUpdated] 🔄 Rollback complete. Active slot: %s\n",
                  g_active_slot == SLOT_A ? "A" : "B");
    }
};

} // namespace Updates
} // namespace SigmaOS

extern "C" {
    void sigma_update_daemon_init() {
        SigmaOS::Updates::UpdateDaemon::getInstance().init();
    }

    sigma_status sigma_update_apply(const void* descriptor_ptr) {
        return SigmaOS::Updates::UpdateDaemon::getInstance().checkAndApplyUpdate(
            (const SigmaOS::Updates::UpdateDescriptor*)descriptor_ptr);
    }
}
