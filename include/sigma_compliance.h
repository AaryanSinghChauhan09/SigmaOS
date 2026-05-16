/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN COMPLIANCE CHECKER (S-COMPLY)
 * =========================================================================
 * Mission: Automated auditing for industrial/government regulatory standards.
 * Inspired by OpenSCAP / Ubuntu Compliance Tools.
 * =========================================================================
 */

#ifndef SIGMA_COMPLIANCE_H
#define SIGMA_COMPLIANCE_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    COMPLIANCE_LEVEL_STANDARD,
    COMPLIANCE_LEVEL_HIPAA,
    COMPLIANCE_LEVEL_SOC2,
    COMPLIANCE_LEVEL_DEFENSE_TOP_SECRET
} sigma_compliance_tier_t;

/* --- Compliance Primitives --- */
void      comply_init(void);
void      comply_run_audit(sigma_compliance_tier_t tier);
void      comply_generate_pqc_report(void);
bool      comply_check_lattice_integrity(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Compliance {

class SovereignComplianceAuditor {
public:
    static SovereignComplianceAuditor& getInstance() {
        static SovereignComplianceAuditor instance;
        return instance;
    }

    void init();
    void runAudit(sigma_compliance_tier_t tier);
    void generateReport();
    bool checkIntegrity();

private:
    SovereignComplianceAuditor() : m_last_audit_ok(true) {}
    bool m_last_audit_ok;
};

} // namespace Compliance
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_COMPLIANCE_H */
