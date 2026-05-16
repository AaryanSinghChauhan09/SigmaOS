#ifndef SIGMA_ATTESTATION_H
#define SIGMA_ATTESTATION_H

#include "../sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignAttestation {
public:
    static SovereignAttestation& getInstance();

    void init();
    bool verifyPlatform();
    sigma_u64 getAttestationReport();

private:
    SovereignAttestation() : m_initialized(false), m_trust_score(0) {}
    bool      m_initialized;
    sigma_u32 m_trust_score;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void      attestation_init(void);
bool      attestation_verify(void);
sigma_u64 attestation_get_report(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ATTESTATION_H */
