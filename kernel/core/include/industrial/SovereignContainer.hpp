#pragma once
#include "../../../../include/core/sigma_types.h"
#include "../../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Industrial {

class SovereignPodmanEngine : public SigmaObject {
public:
    static SovereignPodmanEngine& getInstance() {
        static SovereignPodmanEngine instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignPodmanEngine"; }
    void init();
    void runContainer(const char* image);
private:
    SovereignPodmanEngine() = default;
};

class SovereignKubeletEngine : public SigmaObject {
public:
    static SovereignKubeletEngine& getInstance() {
        static SovereignKubeletEngine instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignKubeletEngine"; }
    void init();
    void syncPodStatus();
private:
    SovereignKubeletEngine() = default;
};

} // namespace Industrial
} // namespace Kernel
} // namespace SigmaOS
 