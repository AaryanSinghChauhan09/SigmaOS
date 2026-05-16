#ifndef SOVEREIGN_NET_STACK_HPP
#define SOVEREIGN_NET_STACK_HPP

#include "../../include/sigma_kernel_types.h"

namespace SigmaOS {
namespace Net {

struct sigma_net_config_t {
    bool enable_ipv6;
    bool enable_firewall;
    bool enable_ssl;
};

/**
 * @class SovereignNetStackEngine
 * @brief Modular TCP/IP Stack (S-NET) for SigmaOS
 * 
 * Provides an isolated, sovereign shard for network processing.
 * Designed to handle zero-copy packet dispatching, IPv6, SSL, and modular
 * protocol loading without monolithic kernel dependencies.
 */
class SovereignNetStackEngine {
public:
    static SovereignNetStackEngine& getInstance() {
        static SovereignNetStackEngine instance;
        return instance;
    }

    void init(const sigma_net_config_t* config);
    void sendPacket(const void* data, sigma_u32 len);
    void receivePacket(void* buffer, sigma_u32* len);
    void reportStats() const;

    // Hooks for sigma-pkg
    bool fetchPackageReliably(const char* url, void* buffer, sigma_u32* len);

private:
    SovereignNetStackEngine();
    ~SovereignNetStackEngine();

    SovereignNetStackEngine(const SovereignNetStackEngine&) = delete;
    SovereignNetStackEngine& operator=(const SovereignNetStackEngine&) = delete;

    bool m_is_initialized;
    sigma_u32 m_shard_id;
    sigma_net_config_t m_config;
    sigma_u32 m_packets_sent;
    sigma_u32 m_packets_received;
};

} // namespace Net
} // namespace SigmaOS

extern "C" {
    void net_init(const SigmaOS::Net::sigma_net_config_t* config);
    void net_send_packet(const void* data, sigma_u32 len);
    void net_receive_packet(void* buffer, sigma_u32* len);
    void net_report_stats();
}

#endif // SOVEREIGN_NET_STACK_HPP
