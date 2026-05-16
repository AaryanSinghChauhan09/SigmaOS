#pragma once
#include "../../../../include/core/sigma_types.h"
#include "../../../../include/SigmaOOP.hpp"
#include "../../../../include/observability/sigma_monitor.h"

namespace SigmaOS {
namespace Kernel {
namespace Observability {

class SovereignObservabilityMonitor : public SigmaObject {
public:
    static SovereignObservabilityMonitor& getInstance();

    const char* type_name() const noexcept override;

    void init();
    sigma_system_load_t getLoadMatrix();
    void executeEbpfProgram(const void* bytecode, sigma_usize size);
    void rebalanceLattice();

private:
    SovereignObservabilityMonitor() : m_initialized(false) {}
    bool m_initialized;
};

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS
