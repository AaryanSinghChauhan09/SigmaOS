#ifndef SIGMA_VPN_STACK_H
#define SIGMA_VPN_STACK_H

#include <sigma_types.h>

// SigmaOS Modular VPN Stack
// Absorbs WireGuard's speed and OpenVPN's robustness natively into S07_Network

// Initialize securely encrypted VPN tunnel mapping
void net_vpn_create_tunnel(const char* endpoint_ip, const char* local_pub_key);

// Route all active OS traffic through the tunnel interface
void net_vpn_force_route(uint32_t tunnel_id);

// Establish seamless split-tunneling per application namespace
void net_vpn_split_namespace(uint32_t container_id, uint32_t tunnel_id);

#endif // SIGMA_VPN_STACK_H

