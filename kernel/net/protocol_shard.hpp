#ifndef PROTOCOL_SHARD_HPP
#define PROTOCOL_SHARD_HPP

#include "../../include/libc/SovereignLibC.h"

#include "../../include/core/sigma_types.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Net {

/*
 * =========================================================================
 * SOVEREIGN PROTOCOL SHARD (Programmable Network Stack)
 * =========================================================================
 * Industrial-grade networking abstraction. Allows developers to extend 
 * the network stack with native, low-level protocol shards.
 */
class SovereignProtocolShard : public SigmaObject {
private:
    char m_protocol_name[32];
    sigma_u16 m_port_nexus;
    sigma_bool m_pqc_enabled;

public:
    SovereignProtocolShard(const char* name, sigma_u16 port) : m_port_nexus(port), m_pqc_enabled(SIGMA_TRUE) {
        sigma_strcpy(m_protocol_name, name);
        sigma_printf("[NET-SHARD]: Programmable Protocol '%s' Orchestrated on Nexus Port %d.\n", m_protocol_name, m_port_nexus);
    }

    const char* type_name() const noexcept override { return "SovereignProtocolShard"; }

    void ProcessPacket(const void* data, sigma_size_t size);
    void Audit();
};

} // namespace Net
} // namespace SigmaOS

#endif
