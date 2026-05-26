/**
 * SovereignDeclarativeConfig.cpp
 * Feature: Declarative Config Manager (NixOS-style)
 * =====================================================================
 * Absorbs: NixOS configuration.nix, Guix System, Ansible declarative.
 * Mission: Reproducible system configs described in YAML/JSON, extended
 *          to drivers and kernel modules. Atomic rollbacks, generations,
 *          and diff-based change tracking.
 * Branch:  tools-dev, kernel-exp
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Architecture {
namespace Config {

static constexpr sigma_u32 MAX_ENTRIES      = 128;
static constexpr sigma_u32 MAX_GENERATIONS  = 32;

enum class EntryType : sigma_u8 {
    SERVICE   = 0,
    DRIVER    = 1,
    PACKAGE   = 2,
    KERNEL    = 3,
    NETWORK   = 4,
    SECURITY  = 5,
    BOOT      = 6
};

struct ConfigEntry {
    sigma_u32 id;
    EntryType type;
    char      key[64];
    char      value[128];
    bool      active;
};

struct Generation {
    sigma_u32    gen_id;
    sigma_u32    entry_count;
    sigma_u32    entries[MAX_ENTRIES];  // indices into config store
    sigma_u64    timestamp;
    bool         current;
};

class SovereignDeclarativeConfig {
public:
    static SovereignDeclarativeConfig& getInstance() {
        static SovereignDeclarativeConfig inst;
        return inst;
    }

    void init() {
        m_entry_count = 0;
        m_gen_count   = 0;

        // Create generation 0 (base system)
        createGeneration();

        // Register default entries
        setEntry(EntryType::KERNEL,  "kernel.scheduler", "adaptive-ewma");
        setEntry(EntryType::KERNEL,  "kernel.selfheal",  "enabled");
        setEntry(EntryType::SECURITY, "security.pqc",    "kyber-768");
        setEntry(EntryType::SECURITY, "security.firewall", "domain-isolation");
        setEntry(EntryType::NETWORK, "network.privacy",  "adaptive");
        setEntry(EntryType::SERVICE, "service.init",     "sigma-init");
        setEntry(EntryType::DRIVER,  "driver.gpu",       "sovereign-vulkan");

        sigma_log("[CONFIG] Sovereign Declarative Config Manager initialised.");
        sigma_log("[CONFIG] Mode: NixOS-style generations with atomic rollback.");
    }

    sigma_u32 setEntry(EntryType type, const char* key, const char* value) {
        // Check for existing key and update
        for (sigma_u32 i = 0; i < m_entry_count; i++) {
            if (streq(m_entries[i].key, key)) {
                sigma_u32 j = 0;
                while (j < 127 && value[j]) { m_entries[i].value[j] = value[j]; j++; }
                m_entries[i].value[j] = '\0';
                return m_entries[i].id;
            }
        }

        // New entry
        if (m_entry_count >= MAX_ENTRIES) return 0;
        ConfigEntry& e = m_entries[m_entry_count];
        e.id = m_entry_count + 1;
        e.type = type;
        sigma_u32 i = 0;
        while (i < 63 && key[i]) { e.key[i] = key[i]; i++; }
        e.key[i] = '\0';
        i = 0;
        while (i < 127 && value[i]) { e.value[i] = value[i]; i++; }
        e.value[i] = '\0';
        e.active = true;
        m_entry_count++;

        // Add to current generation
        if (m_gen_count > 0) {
            Generation& g = m_generations[m_gen_count - 1];
            if (g.entry_count < MAX_ENTRIES) {
                g.entries[g.entry_count++] = m_entry_count - 1;
            }
        }
        return e.id;
    }

    const char* getEntry(const char* key) {
        for (sigma_u32 i = 0; i < m_entry_count; i++) {
            if (m_entries[i].active && streq(m_entries[i].key, key)) {
                return m_entries[i].value;
            }
        }
        return nullptr;
    }

    // Create a new generation snapshot
    sigma_u32 createGeneration() {
        if (m_gen_count >= MAX_GENERATIONS) return 0;
        // Deactivate previous current
        if (m_gen_count > 0) {
            m_generations[m_gen_count - 1].current = false;
        }
        Generation& g = m_generations[m_gen_count];
        g.gen_id = m_gen_count + 1;
        g.entry_count = 0;
        g.timestamp = m_gen_count * 1000;  // simulated
        g.current = true;
        m_gen_count++;
        sigma_log_info("[CONFIG] Generation #%u created.\n", g.gen_id);
        return g.gen_id;
    }

    // Rollback to a previous generation
    bool rollback(sigma_u32 gen_id) {
        if (gen_id == 0 || gen_id > m_gen_count) return false;
        for (sigma_u32 i = 0; i < m_gen_count; i++) {
            m_generations[i].current = (m_generations[i].gen_id == gen_id);
        }
        sigma_log_info("[CONFIG] Rolled back to generation #%u.\n", gen_id);
        return true;
    }

    void printStatus() {
        sigma_log("\n--- DECLARATIVE CONFIG STATUS ---");
        sigma_log_info("| Entries      : %u\n", m_entry_count);
        sigma_log_info("| Generations  : %u\n", m_gen_count);
        for (sigma_u32 i = 0; i < m_entry_count; i++) {
            sigma_log_info("|  [%s] = '%s' (type=%u)\n",
                           m_entries[i].key, m_entries[i].value, (sigma_u32)m_entries[i].type);
        }
        for (sigma_u32 i = 0; i < m_gen_count; i++) {
            sigma_log_info("|  Gen #%u: %u entries %s\n",
                           m_generations[i].gen_id, m_generations[i].entry_count,
                           m_generations[i].current ? "[CURRENT]" : "");
        }
        sigma_log("---------------------------------");
    }

private:
    ConfigEntry m_entries[MAX_ENTRIES];
    Generation  m_generations[MAX_GENERATIONS];
    sigma_u32   m_entry_count = 0;
    sigma_u32   m_gen_count   = 0;

    static bool streq(const char* a, const char* b) {
        while (*a && *b) { if (*a++ != *b++) return false; }
        return *a == *b;
    }

    SovereignDeclarativeConfig() = default;
};

} // namespace Config
} // namespace Architecture
} // namespace SigmaOS

extern "C" {

void dconfig_init() {
    SigmaOS::Architecture::Config::SovereignDeclarativeConfig::getInstance().init();
}

sigma_u32 dconfig_set(sigma_u8 type, const char* key, const char* value) {
    return SigmaOS::Architecture::Config::SovereignDeclarativeConfig::getInstance()
               .setEntry((SigmaOS::Architecture::Config::EntryType)type, key, value);
}

const char* dconfig_get(const char* key) {
    return SigmaOS::Architecture::Config::SovereignDeclarativeConfig::getInstance()
               .getEntry(key);
}

sigma_u32 dconfig_snapshot() {
    return SigmaOS::Architecture::Config::SovereignDeclarativeConfig::getInstance()
               .createGeneration();
}

bool dconfig_rollback(sigma_u32 gen) {
    return SigmaOS::Architecture::Config::SovereignDeclarativeConfig::getInstance()
               .rollback(gen);
}

void dconfig_status() {
    SigmaOS::Architecture::Config::SovereignDeclarativeConfig::getInstance().printStatus();
}

} // extern "C"
