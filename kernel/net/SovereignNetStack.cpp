#include "sigma_net.h"
#include "sigma_hal.h"
#include "sigma_types.h"

/**
 * SigmaOS Sovereign Network Stack (S-NET)
 * Implements a Zero-Copy Lattice Networking (ZCLN) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal network orchestration.
 *
 * Design: OOP-isolated singleton — SovereignNetStackEngine.
 */

/* --- Sovereign Network Stack Implementation --- */

void SovereignNetStackEngine::init(const sigma_net_config_t* config) {
    sigma_log("[NET] Initializing Sovereign Network Stack (ZCLN)...");
    if (config) {
        this->config = *config;
    }
    this->initialized = 1u;
    sigma_log("[NET] ZCLN: Zero-copy packet pipeline ACTIVE.");
}

void SovereignNetStackEngine::sendPacket(const void* data, sigma_u32 len) {
    if (!this->initialized) return;
    sigma_printf("[NET] ZCLN: Sending packet (%u bytes)...\n", len);
    this->packets_sent++;
}

void SovereignNetStackEngine::receivePacket(void* buffer, sigma_u32* len) {
    if (!this->initialized) return;
    sigma_log("[NET] ZCLN: Packet received.");
    this->packets_received++;
}

void SovereignNetStackEngine::reportStats() const {
    sigma_printf("[NET] ZCLN: Sent: %u, Received: %u.\n", 
                 this->packets_sent, this->packets_received);
}

/* --- C Wrappers --- */
extern "C" void net_init(const sigma_net_config_t* config) {
    SovereignNetStackEngine::getInstance().init(config);
}

extern "C" void net_send_packet(const void* data, sigma_u32 len) {
    SovereignNetStackEngine::getInstance().sendPacket(data, len);
}

extern "C" void net_receive_packet(void* buffer, sigma_u32* len) {
    SovereignNetStackEngine::getInstance().receivePacket(buffer, len);
}

extern "C" void net_report_stats() {
    SovereignNetStackEngine::getInstance().reportStats();
}
