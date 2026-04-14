// =============================================================================
// SigmaOS — S10_System — SovereignSystemUpdater.c
// Atomic Transactional System Updater Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows Update    — staged rollout, driver signing verification
//   • macOS SoftwareUpdate — sealed system volume, no partial updates
//   • NixOS             — atomic generations: rollback to any prior state
//   • ChromeOS          — dual A/B partition silent background update
//   • Flatpak/OSTree    — immutable read-only base, delta updates
// Architecture:
//   • A/B partition model: update applied to inactive slot (ChromeOS)
//   • On success → swap boot pointer atomically (NixOS generation model)
//   • On failure → automatic rollback to previous generation
//   • Delta compression: only changed blocks transmitted (OSTree)
//   • Sovereign signature verified via S08_Security before any write
// =============================================================================

#include <stdint.h>
#include <stdbool.h>

#define UPDATER_MAX_GENERATIONS  32
#define UPDATER_DELTA_CHUNK_SIZE (1024 * 1024)  // 1MB delta blocks

// ── Update Slot (A/B partitions) ──────────────────────────────────────────────
typedef enum {
    UPDATE_SLOT_A = 0,
    UPDATE_SLOT_B = 1,
} UpdateSlot;

// ── System Generation ─────────────────────────────────────────────────────────
typedef struct {
    uint32_t    generation_id;
    char        version_str[32];
    UpdateSlot  active_slot;
    uint64_t    timestamp_unix;
    uint8_t     signature[64];   // Ed25519 sovereign signature
    bool        is_booted;       // Currently running this generation
} SigmaGeneration;

static SigmaGeneration generations[UPDATER_MAX_GENERATIONS];
static uint32_t        generation_count = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Check for updates from the sovereign package mirror
bool updater_check_available(char* out_version, uint32_t out_len);

// Download and apply delta blocks to the inactive A/B slot
bool updater_download_and_stage(void);

// Atomically commit the staged update (swap boot pointer on success)
bool updater_commit(void);

// Roll back to a prior generation by generation_id (NixOS model)
bool updater_rollback(uint32_t target_generation_id);

// List all installed generations with their state
uint32_t updater_list_generations(SigmaGeneration* out, uint32_t max);

// Verify sovereign cryptographic signature of a staged update
bool updater_verify_signature(const uint8_t* pkg_data, uint32_t size,
                               const uint8_t* sig, uint32_t sig_len);
