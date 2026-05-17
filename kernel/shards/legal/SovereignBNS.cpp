#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign BNS Legal Shard (S-BNS)
 * Implementation: Bharatiya Nyaya Sanhita (BNS) 2023 lookup and mapping.
 * Mission: Assist Indian Legal Professionals in transitioning from IPC to BNS.
 */

namespace SigmaOS {
namespace Kernel {
namespace Legal {

class SovereignBNS : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignBNS> {
    friend class SigmaOS::SigmaSingleton<SovereignBNS>;
public:
    const char* type_name() const noexcept override { return "SovereignBNS"; }

    struct LegalMapping {
        sigma_u32 ipc_section;
        sigma_u32 bns_section;
        const char* description;
    };

    // Sample Mappings (BNS 2023)
    static constexpr LegalMapping s_mappings[] = {
        {302, 101, "Punishment for Murder"},
        {307, 109, "Attempt to Murder"},
        {376, 64,  "Punishment for Rape"},
        {420, 318, "Cheating and dishonestly inducing delivery of property"},
        {124, 150, "Sedition (Now Acts endangering sovereignty, unity and integrity of India)"}
    };

    void lookupByIPC(sigma_u32 ipc_sec) {
        sigma_log_info("[S-BNS] Looking up BNS equivalent for IPC Section %u...", ipc_sec);
        for (const auto& m : s_mappings) {
            if (m.ipc_section == ipc_sec) {
                sigma_log_info("[S-BNS] Match Found: IPC %u -> BNS %u (%s)", m.ipc_section, m.bns_section, m.description);
                return;
            }
        }
        sigma_log_warn("[S-BNS] No direct mapping found for IPC %u in local shard database.", ipc_sec);
    }

    void lookupByBNS(sigma_u32 bns_sec) {
        sigma_log_info("[S-BNS] Looking up IPC equivalent for BNS Section %u...", bns_sec);
        for (const auto& m : s_mappings) {
            if (m.bns_section == bns_sec) {
                sigma_log_info("[S-BNS] Match Found: BNS %u -> IPC %u (%s)", m.bns_section, m.ipc_section, m.description);
                return;
            }
        }
        sigma_log_warn("[S-BNS] No direct mapping found for BNS %u in local shard database.", bns_sec);
    }

private:
    SovereignBNS() = default;
};

} // namespace Legal
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void bns_lookup_ipc(sigma_u32 sec) { SigmaOS::Kernel::Legal::SovereignBNS::getInstance().lookupByIPC(sec); }
    void bns_lookup_bns(sigma_u32 sec) { SigmaOS::Kernel::Legal::SovereignBNS::getInstance().lookupByBNS(sec); }
}
 