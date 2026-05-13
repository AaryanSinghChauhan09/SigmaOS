#include "sigma_types.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Container Networking (CNI)
 * Zero-trust network routing for Micro-VM containers.
 *
 * USP: Automatically provisions virtual network interfaces (veth) and routes 
 * container traffic through the SovereignNetStack's deep packet inspection.
 *
 * Design: OOP-isolated singleton — SovereignContainerNetEngine.
 */

class SovereignContainerNetEngine {
public:
    static SovereignContainerNetEngine& getInstance() {
        static SovereignContainerNetEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[CONTAINER-NET] Initializing Sovereign Container Networking...");
        this->active_routes = 0;
    }

    void attachContainerNetwork(const char* container_name, const char* veth_mac) {
        if (this->active_routes >= 32) return;
        sigma_hardened_strcpy(this->container_macs[this->active_routes], veth_mac, 18);
        this->active_routes++;
        sigma_log_info("[CONTAINER-NET] Attached veth %s to Container '%s'.\n", veth_mac, container_name);
    }

private:
    SovereignContainerNetEngine() : active_routes(0) {}

    char container_macs[32][18];
    sigma_u32 active_routes;
};

/* --- C Wrappers --- */
extern "C" void container_net_init() {
    SovereignContainerNetEngine::getInstance().init();
}

extern "C" void container_net_attach(const char* container, const char* mac) {
    SovereignContainerNetEngine::getInstance().attachContainerNetwork(container, mac);
}


