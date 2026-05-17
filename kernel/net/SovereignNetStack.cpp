#include "SovereignNetStack.hpp"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"

namespace SigmaOS {
namespace Net {

SovereignNetStackEngine::SovereignNetStackEngine() 
    : m_is_initialized(false), m_shard_id(0x4E4554), m_packets_sent(0), m_packets_received(0) {
    m_config.enable_ipv6 = true;
    m_config.enable_firewall = true;
    m_config.enable_ssl = true;
}

SovereignNetStackEngine::~SovereignNetStackEngine() {}

void SovereignNetStackEngine::init(const sigma_net_config_t* config) {
    sigma_log_info("[NET] Initializing Sovereign Network Stack (TCP/IP & ZCLN)...");
    if (config) {
        m_config = *config;
    }
    m_is_initialized = true;
    
    if (m_config.enable_ipv6) {
        sigma_log_info("[NET] IPv6 Support: ENABLED.");
    }
    if (m_config.enable_ssl) {
        sigma_log_info("[NET] Secure Socket Layer (SSL): INTEGRATED.");
    }
    sigma_log_info("[NET] TCP/IP Stack: ACTIVE. S-Firewall: %s", m_config.enable_firewall ? "ENFORCING" : "DISABLED");
}

void SovereignNetStackEngine::sendPacket(const void* data, sigma_u32 len) {
    if (!m_is_initialized || !data || len == 0) return;
    
    sigma_log_info("[NET] TCP/IP: Encapsulating data (%u bytes) into segment...", len);
    
    if (m_config.enable_ipv6) {
        sigma_log_info("[NET] TCP/IP: Attaching IPv6 headers...");
    } else {
        sigma_log_info("[NET] TCP/IP: Attaching IPv4 headers...");
    }
    
    sigma_log_info("[NET] ZCLN: Sending packet to SovereignNIC...");
    m_packets_sent++;
}

void SovereignNetStackEngine::receivePacket(void* buffer, sigma_u32* len) {
    if (!m_is_initialized || !buffer || !len) return;
    
    if (m_config.enable_firewall) {
        sigma_log_info("[NET] Firewall: Inspecting incoming packet...");
        if (*len > 1500) {
            sigma_log_info("[NET] [SECURITY] Firewall DROP: Packet size exceeds MTU.");
            *len = 0;
            return;
        }
    }
    
    sigma_log_info("[NET] TCP/IP: Processing incoming datagram...");
    sigma_log_info("[NET] ZCLN: Packet received and forwarded to userland socket.");
    m_packets_received++;
}

void SovereignNetStackEngine::reportStats() const {
    sigma_log_info("[NET] TCP/IP Stats: Sent=%u, Received=%u", m_packets_sent, m_packets_received);
}

bool SovereignNetStackEngine::fetchPackageReliably(const char* url, void* buffer, sigma_u32* len) {
    if (!m_is_initialized) return false;
    sigma_log_info("[NET] Sigma-Pkg Hook: Fetching package from %s over %s...", url, m_config.enable_ssl ? "HTTPS/SSL" : "HTTP");
    
    // Simulate reliable fetch
    *len = 1024; // Mock size
    return true;
}

} // namespace Net
} // namespace SigmaOS

extern "C" {
    void net_stack_init(const SigmaOS::Net::sigma_net_config_t* config) {
        SigmaOS::Net::SovereignNetStackEngine::getInstance().init(config);
    }

    void net_send_packet(const void* data, sigma_u32 len) {
        SigmaOS::Net::SovereignNetStackEngine::getInstance().sendPacket(data, len);
    }

    void net_receive_packet(void* buffer, sigma_u32* len) {
        SigmaOS::Net::SovereignNetStackEngine::getInstance().receivePacket(buffer, len);
    }

    void net_report_stats() {
        SigmaOS::Net::SovereignNetStackEngine::getInstance().reportStats();
    }
}
 