// SigmaOS — sigma-hal-driver-network: Network Hardware Abstraction
// Module: sigma-hal-driver-network
// USP: Universal abstraction for Ethernet, Wi-Fi, and 5G cellular modems.

#ifndef SIGMA_HAL_DRIVER_NETWORK_HPP
#define SIGMA_HAL_DRIVER_NETWORK_HPP

#include "atomic_sigma_oop_base.hpp"

namespace sigma {
namespace hal {

enum class NetworkMedium {
    ETHERNET,
    WIFI,
    CELLULAR_5G
};

class INetworkDriver : public sigma::core::ISigmaDriver {
public:
    virtual bool initialize() override = 0;
    virtual bool transmit_packet(const void* buffer, unsigned int size) = 0;
    virtual bool receive_packet(void* buffer, unsigned int max_size, unsigned int* bytes_read) = 0;
    virtual NetworkMedium get_medium() const = 0;
    virtual ~INetworkDriver() = default;
};

} // namespace hal
} // namespace sigma

#endif /* SIGMA_HAL_DRIVER_NETWORK_HPP */
