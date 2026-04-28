#ifndef SILICON_AUDIT_HPP
#define SILICON_AUDIT_HPP

#include "../../SigmaOOP.hpp"

namespace SigmaOS {
namespace Build {

class SovereignSiliconAudit : public SigmaOS::SigmaObject {
private:
    sigma_bool m_has_sse42;
    sigma_bool m_has_avx2;
    sigma_bool m_has_avx512;

public:
    SovereignSiliconAudit() : m_has_sse42(SIGMA_FALSE), m_has_avx2(SIGMA_FALSE), m_has_avx512(SIGMA_FALSE) {}
    
    const char* type_name() const noexcept override { return "SovereignSiliconAudit"; }

    void DetectFeatures();
    SigmaString GetOptimizationFlags();
};

} // namespace Build
} // namespace SigmaOS

#endif
