#pragma once
#include "../../../../include/core/sigma_types.h"
#include "../../../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Observability {

class SovereignDiagEngine : public SigmaObject {
public:
    static SovereignDiagEngine& getInstance();
    const char* type_name() const noexcept override { return "SovereignDiagEngine"; }

    struct ShardID { const char* value; };
    struct AnomalyDesc { const char* value; };

    void init();
    void performScan();
    void reportAnomaly(ShardID shard_id, AnomalyDesc description);
    void autoRepair();
    sigma_u32 getFaultCount() const { return m_fault_count; }

private:
    SovereignDiagEngine() = default;
    sigma_u32 m_initialized{0U};
    sigma_u32 m_fault_count{0U};
};

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS
