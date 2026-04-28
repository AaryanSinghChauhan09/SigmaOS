#ifndef SILICON_AUDIT_HPP
#define SILICON_AUDIT_HPP

#include "../../SigmaOOP.hpp"

namespace SigmaOS {
namespace Build {

class ICPUDector {
public:
    virtual ~ICPUDector() = default;
    virtual void DetectFeatures() = 0;
    virtual SigmaString GetOptimizationFlags() = 0;
};

class SovereignSiliconAudit : public ICPUDector {
private:
    bool m_has_avx512 = false;
    bool m_has_avx2 = false;
    bool m_has_sse42 = false;

public:
    void DetectFeatures() override;
    SigmaString GetOptimizationFlags() override;
};

} // namespace Build
} // namespace SigmaOS

#endif
