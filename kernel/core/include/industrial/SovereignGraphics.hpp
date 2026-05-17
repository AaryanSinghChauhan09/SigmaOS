#pragma once
#include "../../../../include/core/sigma_types.h"
#include "../../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Industrial {

class SovereignVulkanLoader : public SigmaObject {
public:
    static SovereignVulkanLoader& getInstance() {
        static SovereignVulkanLoader instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignVulkanLoader"; }
    void init();
    void loadDriver(const char* driver_path);
private:
    SovereignVulkanLoader() = default;
    bool m_initialized{false};
};

class SovereignDXVKEngine : public SigmaObject {
public:
    static SovereignDXVKEngine& getInstance() {
        static SovereignDXVKEngine instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignDXVKEngine"; }
    void init();
    void translateDirect3D();
private:
    SovereignDXVKEngine() = default;
};

} // namespace Industrial
} // namespace Kernel
} // namespace SigmaOS
 