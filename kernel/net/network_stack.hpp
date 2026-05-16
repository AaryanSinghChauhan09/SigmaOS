#ifndef NETWORK_STACK_HPP
#define NETWORK_STACK_HPP

#include "../../include/core/sigma_types.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Net {

/*
 * =========================================================================
 * SOVEREIGN NETWORK MESH (Low-Level Zero-Trust Stack)
 * =========================================================================
 */
class SovereignNetMesh : public SigmaOS::SigmaObject {
private:
    sigma_u64 m_packets_sent;
    sigma_u64 m_packets_received;
    sigma_bool m_encryption_active;

public:
    SovereignNetMesh() : m_packets_sent(0), m_packets_received(0), m_encryption_active(SIGMA_TRUE) {}

    const char* type_name() const noexcept override { return "SovereignNetMesh"; }

    void Initialize();
    void SendShard(const void* data, sigma_size_t size, const char* target_lattice);
    void ReceiveShard();
    void Audit();
};

} // namespace Net
} // namespace SigmaOS

#endif
