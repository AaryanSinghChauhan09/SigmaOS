/**
 * SovereignConfigRollbackCLI.cpp
 * Feature #53 – Config Rollback CLI
 * =====================================================================
 * Absorbs: NixOS generations, BTRFS send/receive, OSTree.
 * Mission: Atomic snapshot + rollback for any SigmaOS config blob.
 *          Every change is versioned; any generation can be restored.
 * Branch:  tools-dev, fs-dev
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Tools {

static constexpr sigma_u32 MAX_GENERATIONS = 32;
static constexpr sigma_u32 MAX_KEY_LEN     = 64;
static constexpr sigma_u32 MAX_VALUE_LEN   = 256;
static constexpr sigma_u32 MAX_ENTRIES     = 64;

// ── Data Structures ────────────────────────────────────────────────
struct ConfigEntry {
    char key[MAX_KEY_LEN];
    char value[MAX_VALUE_LEN];
};

struct Generation {
    sigma_u32    gen_id;
    sigma_u64    timestamp;      // logical clock tick
    sigma_u32    entry_count;
    ConfigEntry  entries[MAX_ENTRIES];
    char         label[64];
};

// ── Utility ────────────────────────────────────────────────────────
static void safe_copy(char* dst, const char* src, sigma_u32 max) {
    sigma_u32 i = 0;
    while (i + 1 < max && src[i]) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

static bool key_eq(const char* a, const char* b) {
    sigma_u32 i = 0;
    while (a[i] && b[i] && a[i] == b[i]) i++;
    return a[i] == '\0' && b[i] == '\0';
}

// ── Manager ────────────────────────────────────────────────────────
class SovereignConfigRollback {
public:
    static SovereignConfigRollback& getInstance() {
        static SovereignConfigRollback inst;
        return inst;
    }

    void init() {
        m_gen_count   = 0;
        m_active_gen  = 0;
        m_clock       = 0;
        // Seed an empty "generation 0"
        snapshotCurrent("initial-boot");
        sigma_log("[ROLLBACK] Sovereign Config Rollback CLI initialised.");
        sigma_log("[ROLLBACK] Mode: NixOS-style atomic generations — rollback at any time.");
    }

    // Set a key in the *live* working config
    void set(const char* key, const char* value) {
        for (sigma_u32 i = 0; i < m_live_count; i++) {
            if (key_eq(m_live[i].key, key)) {
                safe_copy(m_live[i].value, value, MAX_VALUE_LEN);
                sigma_log_info("[ROLLBACK] Updated key '%s'.\n", key);
                return;
            }
        }
        if (m_live_count < MAX_ENTRIES) {
            safe_copy(m_live[m_live_count].key,   key,   MAX_KEY_LEN);
            safe_copy(m_live[m_live_count].value, value, MAX_VALUE_LEN);
            m_live_count++;
            sigma_log_info("[ROLLBACK] Set key '%s'.\n", key);
        } else {
            sigma_log("[ROLLBACK] ERROR: Live config full.");
        }
    }

    // Commit live config as a new generation
    sigma_u32 snapshotCurrent(const char* label) {
        if (m_gen_count >= MAX_GENERATIONS) {
            sigma_log("[ROLLBACK] Generation ring full — evicting oldest.");
            // Evict oldest (shift down)
            for (sigma_u32 i = 0; i + 1 < MAX_GENERATIONS; i++)
                m_gens[i] = m_gens[i + 1];
            m_gen_count = MAX_GENERATIONS - 1;
        }
        Generation& g    = m_gens[m_gen_count];
        g.gen_id         = ++m_next_id;
        g.timestamp      = ++m_clock;
        g.entry_count    = m_live_count;
        safe_copy(g.label, label, 64);
        for (sigma_u32 i = 0; i < m_live_count; i++) g.entries[i] = m_live[i];
        m_active_gen = m_gen_count;
        m_gen_count++;
        sigma_log_info("[ROLLBACK] Snapshot #%u created: '%s'.\n", g.gen_id, label);
        return g.gen_id;
    }

    // Roll back to a specific generation id
    bool rollback(sigma_u32 gen_id) {
        for (sigma_u32 i = 0; i < m_gen_count; i++) {
            if (m_gens[i].gen_id == gen_id) {
                // Restore live config
                m_live_count = m_gens[i].entry_count;
                for (sigma_u32 j = 0; j < m_live_count; j++)
                    m_live[j] = m_gens[i].entries[j];
                m_active_gen = i;
                sigma_log_info("[ROLLBACK] ✓ Rolled back to generation #%u ('%s').\n",
                               gen_id, m_gens[i].label);
                return true;
            }
        }
        sigma_log_info("[ROLLBACK] ERROR: Generation #%u not found.\n", gen_id);
        return false;
    }

    // Roll back one step
    bool rollbackPrevious() {
        if (m_active_gen == 0) {
            sigma_log("[ROLLBACK] Already at oldest generation.");
            return false;
        }
        return rollback(m_gens[m_active_gen - 1].gen_id);
    }

    // Print generation history (like `nixos-rebuild list-generations`)
    void listGenerations() {
        sigma_log("\n--- SOVEREIGN CONFIG GENERATIONS ---");
        for (sigma_u32 i = 0; i < m_gen_count; i++) {
            const char* marker = (i == m_active_gen) ? " <-- ACTIVE" : "";
            sigma_log_info("  #%02u  clock=%llu  '%s'%s\n",
                           m_gens[i].gen_id,
                           (unsigned long long)m_gens[i].timestamp,
                           m_gens[i].label,
                           marker);
        }
        sigma_log("------------------------------------");
    }

    // Get a live value
    const char* get(const char* key) {
        for (sigma_u32 i = 0; i < m_live_count; i++)
            if (key_eq(m_live[i].key, key)) return m_live[i].value;
        return nullptr;
    }

private:
    Generation  m_gens[MAX_GENERATIONS];
    ConfigEntry m_live[MAX_ENTRIES];
    sigma_u32   m_gen_count   = 0;
    sigma_u32   m_live_count  = 0;
    sigma_u32   m_active_gen  = 0;
    sigma_u32   m_next_id     = 0;
    sigma_u64   m_clock       = 0;

    SovereignConfigRollback() = default;
};

} // namespace Tools
} // namespace SigmaOS

// ── C API ──────────────────────────────────────────────────────────
extern "C" {

void rollback_init() {
    SigmaOS::Tools::SovereignConfigRollback::getInstance().init();
}

void rollback_set(const char* key, const char* value) {
    SigmaOS::Tools::SovereignConfigRollback::getInstance().set(key, value);
}

sigma_u32 rollback_snapshot(const char* label) {
    return SigmaOS::Tools::SovereignConfigRollback::getInstance().snapshotCurrent(label);
}

bool rollback_to(sigma_u32 gen_id) {
    return SigmaOS::Tools::SovereignConfigRollback::getInstance().rollback(gen_id);
}

bool rollback_previous() {
    return SigmaOS::Tools::SovereignConfigRollback::getInstance().rollbackPrevious();
}

void rollback_list() {
    SigmaOS::Tools::SovereignConfigRollback::getInstance().listGenerations();
}

} // extern "C"
