#ifndef PACKAGE_NEXUS_HPP
#define PACKAGE_NEXUS_HPP

#include "../../SigmaOOP.hpp"

namespace SigmaOS {
namespace PackageForge {

class SovereignPackageNexus : public SigmaObject {
public:
    SovereignPackageNexus();
    const char* type_name() const noexcept override { return "SovereignPackageNexus"; }

    void VetHardwareSignature(const char* shard_id);
    void InstallSandboxedShard(const char* shard_id);
};

} // namespace PackageForge
} // namespace SigmaOS

#endif
