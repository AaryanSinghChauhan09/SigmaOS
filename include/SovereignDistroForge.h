#ifndef SOVEREIGN_DISTRO_FORGE_H
#define SOVEREIGN_DISTRO_FORGE_H

#include "./SigmaOOP.hpp"

namespace SigmaOS {
namespace DistroForge {

class SovereignDistroForge : public SigmaObject {
public:
    const char* type_name() const noexcept override;
    void AbsorbLinux();
    void ForgeNewDistro(const char* name);
};

} // namespace DistroForge
} // namespace SigmaOS

#endif
