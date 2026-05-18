#include "sigma_kernel_types.h"
#include "sigma_hal.h"
#ifndef PERSONA_MANAGER_HPP
#define PERSONA_MANAGER_HPP

#include "libc/SovereignLibC.h"

#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

struct PersonaConfig {
    char name[32];
    sigma_u32 accent_color;
    sigma_bool entropy_protection;
    sigma_u8  clearance_level;
};

class SovereignPersonaManager : public SigmaObject {
private:
    PersonaConfig m_current;
    sigma_bool    m_morphic_sync;

public:
    SovereignPersonaManager() : m_morphic_sync(SIGMA_FALSE) {
        sigma_memcpy(m_current.name, "ROOT_SHARD", 11);
        m_current.accent_color = 0x00F2FF; // Neon Cyan
        m_current.entropy_protection = SIGMA_TRUE;
        m_current.clearance_level = 0xFF;
    }

    const char* type_name() const noexcept override { return "SovereignPersonaManager"; }

    void SwitchPersona(const char* name, sigma_u32 color) {
        sigma_log("[PERSONA-ZENITH]: Hot-swapping System State to: %s\n", name);
        sigma_strncpy(m_current.name, name, 31);
        m_current.accent_color = color;
        sigma_log("[PERSONA-ZENITH]: Accent Shard updated to 0x%x\n", color);
    }

    void EnableMorphicSync() {
        m_morphic_sync = SIGMA_TRUE;
        sigma_log("[PERSONA-ZENITH]: Morphic Sync ACTIVE. System adaptation enabled.\n");
    }

    void AuditPersona() {
        sigma_log("\n--- Î£ SOVEREIGN PERSONA AUDIT ---\n");
        sigma_log("| Current Persona: %s\n", m_current.name);
        sigma_log("| Clearance Level: %u\n", m_current.clearance_level);
        sigma_log("| Morphic Sync   : %s\n", m_morphic_sync ? "ENABLED" : "DISABLED");
        sigma_log("----------------------------------\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

 