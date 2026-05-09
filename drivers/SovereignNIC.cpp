#include "../include/SovereignLibC.h"

#include "sigma_hal.h"


/**
 * SigmaOS Sovereign Network Driver (Generic Intel e1000/VirtIO Stub)
 * Pure silicon packet processing.
 */

typedef struct {
    uint8_t mac[6];
    bool link_up;
} nic_device_t;

class SovereignNICEngine {
public:
    static SovereignNICEngine& getInstance() {
        static SovereignNICEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[NIC] Probing Sovereign Network Interface...");

        this->master_nic.mac[0] = 0xDE;
        this->master_nic.mac[1] = 0xAD;
        this->master_nic.mac[2] = 0xBE;
        this->master_nic.mac[3] = 0xEF;
        this->master_nic.mac[4] = 0x00;
        this->master_nic.mac[5] = 0x01;
        this->master_nic.link_up = true;
        this->tx_quota = 0;

        sigma_printf("[NIC] Link established. MAC: %02X:%02X:%02X:%02X:%02X:%02X\n",
                  this->master_nic.mac[0], this->master_nic.mac[1], this->master_nic.mac[2],
                  this->master_nic.mac[3], this->master_nic.mac[4], this->master_nic.mac[5]);
    }

    int transmit(const void* packet, uint32_t len) {
        if (!this->master_nic.link_up) {
            sigma_log("[NIC] [ERROR] Transmit failed: Link is down.");
            return -1;
        }

        if (this->tx_quota > MAX_TX_QUOTA) {
            sigma_log("[NIC] [SECURITY] I/O Quota exceeded. Throttling transmission.");
            return -2;
        }

        /* Inject packet into lattice mesh */
        this->tx_quota++;
        sigma_printf("[NIC] Transmitting packet (%d bytes)... [Quota: %d/%d]\n", len, this->tx_quota, MAX_TX_QUOTA);
        return 0;
    }

    int receive(void* buffer, uint32_t max_len) {
        if (!this->master_nic.link_up) return 0;

        /* Reset quota on receive cycles to simulate time progression */
        if (this->tx_quota > 0) this->tx_quota--;

        /* Pull packet from hardware queue */
        return 0;
    }

private:
    SovereignNICEngine() : tx_quota(0) {
        master_nic.link_up = false;
    }

    nic_device_t master_nic;
    uint32_t tx_quota;
};

extern "C" void nic_init() {
    SovereignNICEngine::init();
}

extern "C" int nic_transmit(const void* packet, uint32_t len) {
    return SovereignNICEngine::transmit(packet, len);
}

extern "C" int nic_receive(void* buffer, uint32_t max_len) {
    return SovereignNICEngine::receive(buffer, max_len);
}
