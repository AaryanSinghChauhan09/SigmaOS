/*
 * =========================================================================
 * Σ SIGMAOS: MOCK NIC DRIVER (Sovereign Subsystem)
 * =========================================================================
 * A skeleton driver representing a sovereign hardware abstraction layer
 * for Network Interface Cards.
 * =========================================================================
 */

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_zenithd_log.h"
#include "../../../include/net/sigma_net_internal.h"

/* Simulated MMIO registers */
static sigma_u32 nic_status_reg = 0;

void sigma_nic_init(void) {
    /* Sovereign Error Codes */
    ZENITH_INFO("drv_nic", "Initializing sovereign NIC driver");
    
    /* E.g., read PCI config space, map memory, setup DMA rings */
    nic_status_reg = 1; 
    
    if (nic_status_reg == 0) {
        ZENITH_CRIT(0xD001 /* ZEN_DRV_CRASH */, "drv_nic", "Hardware failed to initialize");
    } else {
        ZENITH_INFO("drv_nic", "Hardware init success. Link UP.");
    }
}

void sigma_nic_transmit(const void* frame, sigma_size_t len) {
    /* Push to physical TX ring */
    ZENITH_TRACE("drv_nic", "Frame transmitted over wire (simulated)");
    (void)frame;
    (void)len;
}

/* Interrupt handler triggered by network card */
void sigma_nic_rx_interrupt(void) {
    /* Simulated RX buffer */
    sigma_u8 dummy_frame[64];
    
    ZENITH_TRACE("drv_nic", "RX Interrupt fired, pulling DMA buffer");
    sigma_eth_receive(dummy_frame, sizeof(dummy_frame));
}
