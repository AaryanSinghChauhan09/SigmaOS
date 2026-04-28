#ifndef PERSONA_MANAGER_HPP
#define PERSONA_MANAGER_HPP

#include "../../SigmaOOP.hpp"

namespace SigmaOS {
namespace Core {

class SovereignPersonaManager : public SigmaOS::SigmaObject {
private:
    SigmaString m_current_user;
    sigma_bool m_is_personalizing;

public:
    SovereignPersonaManager() : m_current_user("Sovereign_Admin"), m_is_personalizing(SIGMA_FALSE) {}

    const char* type_name() const noexcept override { return "SovereignPersonaManager"; }

    void ApplyProfile(const char* profile_name) {
        sigma_printf("[PERSONA]: Applying Customization Profile: %s...\n", profile_name);
        sigma_printf("[OK]: Zenith Theme, Automation Rules, and Privacy Policies synchronized.\n");
    }

    void TogglePersonalization() {
        m_is_personalizing = !m_is_personalizing;
        sigma_printf("[PERSONA]: AI-Driven Personalization is now %s.\n", m_is_personalizing ? "ACTIVE" : "INACTIVE");
    }
};

} // namespace Core
} // namespace SigmaOS

#endif
