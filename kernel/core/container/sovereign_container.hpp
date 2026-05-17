#ifndef SOVEREIGN_CONTAINER_HPP
#define SOVEREIGN_CONTAINER_HPP

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

/*
 * =========================================================================
 * SOVEREIGN CONTAINER (Bare-Metal Isolation)
 * =========================================================================
 * Industrial-grade lightweight isolation without hypervisor overhead.
 * Uses silicon-native shard isolation for zero-latency performance.
 */
class SovereignContainer : public SigmaObject {
private:
    sigma_u32 m_container_id;
    const char* m_image_shard;
    sigma_size_t m_memory_limit;
    sigma_bool m_active;

public:
    SovereignContainer(sigma_u32 id, const char* image, sigma_size_t memory) 
        : m_container_id(id), m_image_shard(image), m_memory_limit(memory), m_active(SIGMA_FALSE) {}

    const char* type_name() const noexcept override { return "SovereignContainer"; }

    void Launch();
    void Terminate();
    void Audit();
};

class ContainerManager : public SigmaObject {
private:
    SovereignContainer* m_active_containers[128];
    sigma_u32 m_count;

public:
    ContainerManager() : m_count(0) {}
    const char* type_name() const noexcept override { return "ContainerManager"; }

    void Deploy(const char* shard_path, sigma_size_t memory_quota);
    void Audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif
 