// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_netgw.h — Two-VM network gateway isolation (Whonix-inspired)
 *
 * sigma-netgw runs as a minimal Type-1 hypervisor guest between the physical NIC
 * and the SigmaOS workload VM. All traffic MUST pass through it.
 * A compromised browser/app in the workload VM cannot bypass the gateway.
 *
 * Architecture:
 *   Physical NIC
 *        │
 *   sigma-netgw VM  (Tor + eBPF + ZT policy at wire level)
 *        │  virtio NIC (sigma-tap0)
 *   SigmaOS Workload VM  (all user apps)
 *
 * The gateway enforces:
 *   - tor_only: all outbound → Tor SOCKS5 (no direct clearnet)
 *   - zerotrust_enforce: eBPF at wire validates ZT policy
 *   - dns_over_tor: no plaintext UDP/53 (DNS leaks blocked)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    char     external_iface[16];      /* physical NIC: "eth0"                   */
    char     external_ip[16];
    char     external_gw[16];

    char     internal_iface[16];      /* virtio tap to workload: "sigma-tap0"   */
    char     internal_ip[16];         /* 10.152.152.10  (Whonix default)        */
    char     workload_ip[16];         /* 10.152.152.11                          */

    bool     tor_only;                /* block all direct outbound              */
    bool     zerotrust_enforce;       /* apply ZT policy at wire                */
    bool     dns_over_tor;            /* block UDP/53 from workload             */

    sigma_u16 allowed_direct_ports[16]; /* bypass Tor for e.g. NTP port 123    */
    int       n_allowed_direct;
} sigma_netgw_config_t;

/* Startup — configures iptables rules and starts transparent Tor proxy */
int sigma_netgw_main(const sigma_netgw_config_t* cfg);

/* Default config — maximum isolation (tor_only + dns_over_tor) */
static inline sigma_netgw_config_t sigma_netgw_default(void) {
    sigma_netgw_config_t c = {};
    /* Standard Whonix network layout */
    __builtin_memcpy(c.external_iface, "eth0",         5);
    __builtin_memcpy(c.internal_iface, "sigma-tap0",  10);
    __builtin_memcpy(c.internal_ip,    "10.152.152.10", 13);
    __builtin_memcpy(c.workload_ip,    "10.152.152.11", 13);
    c.tor_only          = true;
    c.zerotrust_enforce = true;
    c.dns_over_tor      = true;
    c.n_allowed_direct  = 0;
    return c;
}
