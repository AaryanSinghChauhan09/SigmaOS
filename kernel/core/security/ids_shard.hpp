#ifndef IDS_SHARD_HPP
#define IDS_SHARD_HPP

#include "../../include/sigma_types.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Security {

/*
 * =========================================================================
 * SOVEREIGN IDS (Silicon-Native Intrusion Detection)
 * =========================================================================
 * Industrial-grade intrusion detection shard. Monitors silicon cycles and 
 * lattice throughput for side-channel attacks and unauthorized access.
 */
class SovereignIDS : public SigmaObject {
private:
    sigma_u32 m_threat_level;
    sigma_u64 m_alerts_triaged;
    sigma_bool m_auto_mitigation;

public:
    SovereignIDS() : m_threat_level(0), m_alerts_triaged(0), m_auto_mitigation(SIGMA_TRUE) {
        sigma_printf("[S-IDS]: Sovereign Intrusion Detection [READY].\n");
    }

    const char* type_name() const noexcept override { return "SovereignIDS"; }

    void AnalyzeSiliconPatterns();
    void TriageAnomaly(const char* shard_id, sigma_u32 risk_score);
    void Audit();
};

} // namespace Security
} // namespace SigmaOS

#endif
