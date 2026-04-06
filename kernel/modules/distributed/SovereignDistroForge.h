#ifndef SOVEREIGN_DISTRO_FORGE_H
#define SOVEREIGN_DISTRO_FORGE_H

#include "../../SovereignOSBasicsZenith.h"

namespace SigmaOS {
namespace DistroForge {

class SovereignDistroForge {
public:
    const char* type_name() const noexcept;
    void AbsorbLinux();
    void ForgeNewDistro(const char* name);
};

} // namespace DistroForge
} // namespace SigmaOS

extern "C" void sigma_distro_forge_init(void);

#endif
