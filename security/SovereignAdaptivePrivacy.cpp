/**
 * SovereignAdaptivePrivacy.cpp
 * Feature: Adaptive Privacy Profiles (Whonix-style)
 * =====================================================================
 * Absorbs: Whonix stream isolation, Tor transparent proxy,
 *          WireGuard auto-routing, I2P tunnel manager.
 * Mission: Automatically switch between Tor, VPN, or direct networking
 *          based on application domain, threat level, and user context.
 * Branch:  security, kernel-exp
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Security {
namespace Privacy {

static constexpr sigma_u32 MAX_PROFILES  = 16;
static constexpr sigma_u32 MAX_CONTEXTS  = 64;

enum class RoutingMode : sigma_u8 {
    DIRECT    = 0,   // clearnet
    VPN       = 1,   // WireGuard tunnel
    TOR       = 2,   // onion routing
    I2P       = 3,   // garlic routing
    AIRGAPPED = 4    // no network
};

enum class ThreatLevel : sigma_u8 {
    LOW      = 0,
    MODERATE = 1,
    HIGH     = 2,
    CRITICAL = 3
};

struct PrivacyProfile {
    sigma_u32   id;
    char        name[48];
    RoutingMode mode;
    ThreatLevel min_threat;     // auto-activate when threat >= this
    bool        dns_over_tor;
    bool        force_https;
    bool        block_webrtc;
    bool        strip_metadata;
    bool        active;
};

struct ContextBinding {
    sigma_u32 domain_id;   // security domain
    sigma_u32 profile_id;  // privacy profile
};

class SovereignAdaptivePrivacy {
public:
    static SovereignAdaptivePrivacy& getInstance() {
        static SovereignAdaptivePrivacy inst;
        return inst;
    }

    void init() {
        m_profile_count = 0;
        m_context_count = 0;
        m_current_threat = ThreatLevel::LOW;
        m_active_profile = 0;

        // Register default profiles
        registerProfile("ClearNet",  RoutingMode::DIRECT, ThreatLevel::LOW,
                         false, true, false, false);
        registerProfile("VPN-Shield", RoutingMode::VPN, ThreatLevel::MODERATE,
                         false, true, true, true);
        registerProfile("Tor-Anon",  RoutingMode::TOR, ThreatLevel::HIGH,
                         true, true, true, true);
        registerProfile("AirGap",    RoutingMode::AIRGAPPED, ThreatLevel::CRITICAL,
                         false, false, true, true);

        m_active_profile = 1; // start on ClearNet
        sigma_log("[PRIVACY] Sovereign Adaptive Privacy Profiles initialised.");
        sigma_log("[PRIVACY] Mode: Whonix-style auto-routing with 4 default profiles.");
    }

    sigma_u32 registerProfile(const char* name, RoutingMode mode,
                               ThreatLevel min_threat, bool dns_tor,
                               bool https, bool block_rtc, bool strip_meta) {
        if (m_profile_count >= MAX_PROFILES) return 0;
        PrivacyProfile& p = m_profiles[m_profile_count];
        p.id = m_profile_count + 1;
        sigma_u32 i = 0;
        while (i < 47 && name[i]) { p.name[i] = name[i]; i++; }
        p.name[i] = '\0';
        p.mode = mode;
        p.min_threat = min_threat;
        p.dns_over_tor = dns_tor;
        p.force_https = https;
        p.block_webrtc = block_rtc;
        p.strip_metadata = strip_meta;
        p.active = true;
        m_profile_count++;
        return p.id;
    }

    // Bind a security domain to a specific privacy profile
    bool bindDomain(sigma_u32 domain_id, sigma_u32 profile_id) {
        if (m_context_count >= MAX_CONTEXTS) return false;
        m_bindings[m_context_count].domain_id  = domain_id;
        m_bindings[m_context_count].profile_id = profile_id;
        m_context_count++;
        return true;
    }

    // Update threat level — triggers automatic profile escalation
    void setThreatLevel(ThreatLevel level) {
        m_current_threat = level;
        sigma_log_info("[PRIVACY] Threat level updated to %u.\n", (sigma_u32)level);

        // Auto-escalate to highest matching profile
        for (sigma_u32 i = m_profile_count; i > 0; i--) {
            PrivacyProfile& p = m_profiles[i - 1];
            if (p.active && (sigma_u8)p.min_threat <= (sigma_u8)level) {
                m_active_profile = p.id;
                sigma_log_info("[PRIVACY] Auto-switched to profile '%s' (mode=%u).\n",
                               p.name, (sigma_u32)p.mode);
                break;
            }
        }
    }

    // Resolve routing for a given domain
    RoutingMode resolveRouting(sigma_u32 domain_id) {
        // Check domain-specific bindings first
        for (sigma_u32 i = 0; i < m_context_count; i++) {
            if (m_bindings[i].domain_id == domain_id) {
                sigma_u32 pid = m_bindings[i].profile_id;
                if (pid > 0 && pid <= m_profile_count) {
                    return m_profiles[pid - 1].mode;
                }
            }
        }
        // Fall back to active profile
        if (m_active_profile > 0 && m_active_profile <= m_profile_count) {
            return m_profiles[m_active_profile - 1].mode;
        }
        return RoutingMode::DIRECT;
    }

    void printStatus() {
        sigma_log("\n--- ADAPTIVE PRIVACY STATUS ---");
        sigma_log_info("| Profiles    : %u\n", m_profile_count);
        sigma_log_info("| Bindings    : %u\n", m_context_count);
        sigma_log_info("| Threat      : %u\n", (sigma_u32)m_current_threat);
        sigma_log_info("| Active Prof : %u\n", m_active_profile);
        for (sigma_u32 i = 0; i < m_profile_count; i++) {
            PrivacyProfile& p = m_profiles[i];
            const char* mstr = "DIRECT";
            if (p.mode == RoutingMode::VPN) mstr = "VPN";
            else if (p.mode == RoutingMode::TOR) mstr = "TOR";
            else if (p.mode == RoutingMode::I2P) mstr = "I2P";
            else if (p.mode == RoutingMode::AIRGAPPED) mstr = "AIRGAP";
            sigma_log_info("|  [%s] mode=%s dns_tor=%d https=%d%s\n",
                           p.name, mstr, (int)p.dns_over_tor, (int)p.force_https,
                           (p.id == m_active_profile) ? " [ACTIVE]" : "");
        }
        sigma_log("-------------------------------");
    }

private:
    PrivacyProfile m_profiles[MAX_PROFILES];
    ContextBinding m_bindings[MAX_CONTEXTS];
    sigma_u32      m_profile_count = 0;
    sigma_u32      m_context_count = 0;
    ThreatLevel    m_current_threat;
    sigma_u32      m_active_profile;

    SovereignAdaptivePrivacy() = default;
};

} // namespace Privacy
} // namespace Security
} // namespace SigmaOS

extern "C" {

void privacy_init() {
    SigmaOS::Security::Privacy::SovereignAdaptivePrivacy::getInstance().init();
}

void privacy_set_threat(sigma_u8 level) {
    SigmaOS::Security::Privacy::SovereignAdaptivePrivacy::getInstance()
        .setThreatLevel((SigmaOS::Security::Privacy::ThreatLevel)level);
}

sigma_u8 privacy_resolve(sigma_u32 domain_id) {
    return (sigma_u8)SigmaOS::Security::Privacy::SovereignAdaptivePrivacy::getInstance()
               .resolveRouting(domain_id);
}

void privacy_status() {
    SigmaOS::Security::Privacy::SovereignAdaptivePrivacy::getInstance().printStatus();
}

} // extern "C"
