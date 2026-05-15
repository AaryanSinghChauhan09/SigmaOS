#ifndef DISPLAY_DRIVER_HPP
#define DISPLAY_DRIVER_HPP

#include "../../../include/core/sigma_types.h"
#include "../../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Drivers {
namespace Graphics {

class SovereignDisplayDriver : public SigmaOS::SigmaObject {
private:
    sigma_u32 m_width;
    sigma_u32 m_height;
    sigma_bool m_accelerated;

public:
    SovereignDisplayDriver() : m_width(1920), m_height(1080), m_accelerated(SIGMA_TRUE) {}

    const char* type_name() const noexcept override { return "SovereignDisplayDriver"; }

    void Initialize();
    void RefreshLattice();
    void Audit();
};

} // namespace Graphics
} // namespace Drivers
} // namespace SigmaOS

#endif
