#ifndef HARDWARE_AUDIT_HPP
#define HARDWARE_AUDIT_HPP

#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Diagnostics {

class IHardwareAudit {
public:
    virtual ~IHardwareAudit() = default;
    virtual void AuditProcessors() = 0;
    virtual void AuditMemory() = 0;
};

class SovereignHardwareAudit : public IHardwareAudit, public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignHardwareAudit"; }
    void AuditProcessors() override;
    void AuditMemory() override;
};

} // namespace Diagnostics
} // namespace SigmaOS

#endif
