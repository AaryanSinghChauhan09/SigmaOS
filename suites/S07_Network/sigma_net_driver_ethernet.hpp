// SigmaOS — sigma-net-driver-ethernet: OOP Ethernet Driver Interface
// Module: sigma-net-driver-ethernet
// USP: Object-oriented encapsulation of hardware drivers, inheritance for protocols
// Inline assembly used for packet descriptor fetching and DMA kicking

#ifndef SIGMA_NET_DRIVER_ETHERNET_HPP
#define SIGMA_NET_DRIVER_ETHERNET_HPP

#include "sigma_libc.h"
#include "../../sigmaos/core/src/atomic_sigma_oop_base.hpp"

namespace sigma {
namespace net {

struct EthPacketDescriptor {
    unsigned long phys_addr;
    unsigned int  length;
    unsigned int  flags;
};

// Abstract Base Class for all Ethernet NICs
class IEthernetNIC : public sigma::core::ISigmaDriver {
public:
    virtual ~IEthernetNIC() = default;
    
    // Core driver interface
    virtual int  probe_hardware() override = 0;
    virtual void enable_dma() override = 0;

    // Networking specific
    virtual int transmit(const unsigned char* data, unsigned int len) = 0;
    virtual int receive(unsigned char* buffer, unsigned int max_len) = 0;
    virtual void get_mac_address(unsigned char mac[6]) const = 0;
};

// Generic PCIe Ethernet Driver Implementation
class GenericPCIeEthernet : public IEthernetNIC {
private:
    unsigned long mmio_base;
    unsigned char mac[6];

    // Inline ASM for triggering DMA hardware doorbell
    inline void kick_dma_doorbell(unsigned int tx_tail) {
#if defined(__x86_64__)
        // Write to MMIO doorbell register (offset 0x1000) using direct ASM store
        unsigned long doorbell_addr = mmio_base + 0x1000;
        __asm__ __volatile__(
            "movl %1, (%0)\n\t"
            :
            : "r"(doorbell_addr), "r"(tx_tail)
            : "memory"
        );
#else
        (void)tx_tail;
#endif
    }

public:
    GenericPCIeEthernet() : mmio_base(0) {}

    int probe_hardware() override {
        sigma_kprint("[ETH-NIC] Probing PCIe bus for generic Ethernet...\n");
        mmio_base = 0xFEE00000; // Mock MMIO
        mac[0] = 0xDE; mac[1] = 0xAD; mac[2] = 0xBE; 
        mac[3] = 0xEF; mac[4] = 0x00; mac[5] = 0x01;
        return 0;
    }

    void enable_dma() override {
        sigma_kprint("[ETH-NIC] Configuring Ring Buffers & Enabling Bus Mastering...\n");
    }

    int transmit(const unsigned char* data, unsigned int len) override {
        if (len > 1500) return -1;
        // In real code: copy to DMA ring, then ring doorbell
        kick_dma_doorbell(1);
        return (int)len;
    }

    int receive(unsigned char* buffer, unsigned int max_len) override {
        (void)buffer; (void)max_len;
        return 0; // No packets in mock
    }

    void get_mac_address(unsigned char out_mac[6]) const override {
        for(int i=0; i<6; i++) out_mac[i] = mac[i];
    }
};

} // namespace net
} // namespace sigma

#endif /* SIGMA_NET_DRIVER_ETHERNET_HPP */
