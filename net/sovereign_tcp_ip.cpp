#include "../include/sigma_kernel_types.h"

extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
}

namespace SigmaOS {
namespace Net {

struct NetworkInterface {
    char name[16];
    bool is_up;
    char ipv4_address[16];
    char ipv6_address[40];
    char mac_address[18];
};

NetworkInterface active_interface = {"eth0", false, "0.0.0.0", "::", "00:00:00:00:00:00"};

sigma_status bring_interface_up(const char* iface_name) {
    sigma_log_info("[TCP/IP] Bringing up interface %s...", iface_name);
    active_interface.is_up = true;
    sigma_log_info("[TCP/IP] Interface %s is UP. Carrier detected.", iface_name);
    return K_OK;
}

sigma_status bring_interface_down(const char* iface_name) {
    sigma_log_info("[TCP/IP] Bringing down interface %s...", iface_name);
    active_interface.is_up = false;
    sigma_log_info("[TCP/IP] Interface %s is DOWN.", iface_name);
    return K_OK;
}

sigma_status request_dhcp_lease(const char* iface_name) {
    if (!active_interface.is_up) {
        sigma_log_error("[DHCP] Cannot request lease. Interface %s is DOWN.", iface_name);
        return K_ERR_INVAL;
    }
    
    sigma_log_info("[DHCP] Broadcasting DHCPDISCOVER on %s...", iface_name);
    // Mock DHCP negotiation
    sigma_log_info("[DHCP] DHCPOFFER received from Sovereign Relay.");
    sigma_log_info("[DHCP] DHCPACK received. Lease established.");
    
    // Assign mock IPs
    const char* mock_ip = "192.168.100.42";
    const char* mock_ipv6 = "fe80::1ff:fe23:4567:890a";
    
    // Copy safely (mock logic)
    for (int i = 0; mock_ip[i] != '\0' && i < 15; i++) {
        active_interface.ipv4_address[i] = mock_ip[i];
        active_interface.ipv4_address[i+1] = '\0';
    }
    for (int i = 0; mock_ipv6[i] != '\0' && i < 39; i++) {
        active_interface.ipv6_address[i] = mock_ipv6[i];
        active_interface.ipv6_address[i+1] = '\0';
    }
    
    sigma_log_info("[TCP/IP] Assigned IPv4: %s | IPv6: %s", active_interface.ipv4_address, active_interface.ipv6_address);
    return K_OK;
}

NetworkInterface get_interface_status() {
    return active_interface;
}

} // namespace Net
} // namespace SigmaOS
