#pragma once
#include "../../../include/sigma_types.h"
#include "../../../include/SigmaOOP.hpp"

typedef struct {
    sigma_u32 memory_limit_mb;
    sigma_u32 cpu_weight;
    sigma_bool allow_network;
    sigma_bool allow_io;
} sigma_sandbox_config_t;

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignSandboxEngine : public SigmaObject {
public:
    static SovereignSandboxEngine& getInstance() {
        static SovereignSandboxEngine instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignSandboxEngine"; }

    void init();
    sigma_u32 createContainer(const sigma_sandbox_config_t* config);
    bool execute(sigma_u32 container_id, const char* binary_path);
    void destroyContainer(sigma_u32 container_id);
    bool checkSyscall(sigma_u32 syscall_id);
    bool validateMACPolicy(const char* sub, const char* obj, const char* act);
    bool hasCapability(const char* shard_name, const char* capability);

private:
    SovereignSandboxEngine() = default;
    sigma_u32 initialized{0};
    sigma_u32 next_container_id{1};
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
