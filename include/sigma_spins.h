/**
 * @file sigma_spins.h
 * @brief SigmaOS Sovereign Spin Manifest System
 *
 * Defines the spin manifest format and the registry of all
 * profession-based spins. Each spin specifies:
 *   - A package inclusion/exclusion list
 *   - A sandbox capability ring level
 *   - A network policy preset
 *   - Default compositor/DE layout
 */

#ifndef SIGMA_SPINS_H
#define SIGMA_SPINS_H

#include <stdint.h>

namespace sigma {
namespace spins {

/// All supported sovereign spins
enum class SpinId : uint8_t {
    CORE     = 0,   ///< Minimal sovereign base (no spin)
    DEV      = 1,   ///< Developer & Engineer edition
    CREATIVE = 2,   ///< Designer, Artist, Musician edition
    GAMING   = 3,   ///< Gamer edition (SteamOS-style)
    EDU      = 4,   ///< Student & Educator edition
    SCIENCE  = 5,   ///< Researcher & Scientist edition
    BUSINESS = 6,   ///< Enterprise & Productivity edition
    SECURE   = 7,   ///< Security, Forensics & Recovery edition
};

/// Network access policy for a spin
enum class NetworkPolicy : uint8_t {
    OPEN        = 0, ///< Full unrestricted network access
    RESTRICTED  = 1, ///< Outbound only, no raw socket access
    AIRGAPPED   = 2, ///< All interfaces down by default (SECURE spin)
};

/// Spin manifest — describes a sovereign spin's configuration
struct SpinManifest {
    SpinId        id;
    const char*   name;
    const char*   description;
    NetworkPolicy net_policy;
    uint8_t       sandbox_ring_level; ///< 0=kernel, 4=wasm-isolated
    bool          read_only_root;     ///< Boot with tmpfs root (SECURE)
    bool          gaming_optimized;   ///< Enable GameMode/MangoHUD hooks
    bool          audio_low_latency;  ///< Use JACK/PipeWire real-time mode
};

/// Registry of all built-in spins
static constexpr SpinManifest SPIN_REGISTRY[] = {
    { SpinId::CORE,     "sigma-core",     "Minimal sovereign base system",
      NetworkPolicy::RESTRICTED, 3, false, false, false },

    { SpinId::DEV,      "sigma-dev",      "Developer & Engineer edition",
      NetworkPolicy::OPEN,       3, false, false, false },

    { SpinId::CREATIVE, "sigma-creative", "Designer, Artist & Musician edition",
      NetworkPolicy::OPEN,       3, false, false, true  },

    { SpinId::GAMING,   "sigma-gaming",   "Gaming edition (Proton/Vulkan/GameMode)",
      NetworkPolicy::OPEN,       3, false, true,  true  },

    { SpinId::EDU,      "sigma-edu",      "Student & Educator edition (offline-first)",
      NetworkPolicy::RESTRICTED, 3, false, false, false },

    { SpinId::SCIENCE,  "sigma-science",  "Researcher & Scientist edition",
      NetworkPolicy::OPEN,       3, false, false, false },

    { SpinId::BUSINESS, "sigma-business", "Enterprise & Productivity edition",
      NetworkPolicy::RESTRICTED, 3, false, false, false },

    { SpinId::SECURE,   "sigma-secure",   "Security, Forensics & Recovery edition",
      NetworkPolicy::AIRGAPPED,  4, true,  false, false },
};

constexpr uint8_t SPIN_COUNT = sizeof(SPIN_REGISTRY) / sizeof(SpinManifest);

/// Look up a spin manifest by ID
inline const SpinManifest* get_spin(SpinId id) {
    for (uint8_t i = 0; i < SPIN_COUNT; i++) {
        if (SPIN_REGISTRY[i].id == id) return &SPIN_REGISTRY[i];
    }
    return nullptr;
}

} // namespace spins
} // namespace sigma

#endif // SIGMA_SPINS_H
