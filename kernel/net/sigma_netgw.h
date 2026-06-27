// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_netgw.h — Two-VM network gateway architecture
 *
 * Provides Whonix-level network isolation without requiring Tor.
 *
 * Architecture:
 *
 *   ┌────────────────────┐     virtio-net veth pair    ┌───────────────────┐
 *   │   WORKLOAD VM      │ ──────────────────────────► │   GATEWAY VM      │
 *   │  (untrusted code)  │  10.152.0.2 ↔ 10.152.0.1  │  (network access) │
 *   │  No direct NIC     │                             │  Firewall + NAT   │
 *   │  All traffic MUST  │                             │  DNS resolver     │
 *   │  go via gateway    │                             │  VPN/Tor exit     │
 *   └────────────────────┘                             └───────────────────┘
 *                                                              │
 *                                                         Physical NIC
 *                                                       (host network)
 *
 * Integration: sigma_hypervisor.cpp creates both VMs and wires the veth pair.
 * The workload VM has NO direct network access — all packets must traverse
 * the gateway, which enforces the firewall ruleset before forwarding.
 *
 * This is stronger than a normal firewall:
 *   - Even a kernel exploit in the workload VM cannot reach the network
 *     directly because the virtual NIC is connected only to the gateway.
 *   - The gateway VM runs a minimal, hardened kernel with no user sessions.
 *   - The gateway's firewall is evaluated BEFORE packets reach the internet.
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── VM addressing ───────────────────────────────────────────────────────── */
#define SIGMA_NETGW_GATEWAY_IP4      "10.152.0.1"   /* gateway side of veth */
#define SIGMA_NETGW_WORKLOAD_IP4     "10.152.0.2"   /* workload side        */
#define SIGMA_NETGW_SUBNET_MASK      "255.255.255.0"
#define SIGMA_NETGW_DNS_PORT         53
#define SIGMA_NETGW_GATEWAY_VM_NAME  "sigma-netgw"
#define SIGMA_NETGW_VETH_PREFIX      "sgw"           /* sgw0, sgw1, etc.    */

/* ── Gateway mode ────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_NETGW_MODE_CLEARNET = 0,  /* NAT to host network (default)       */
    SIGMA_NETGW_MODE_VPN      = 1,  /* All traffic through WireGuard VPN   */
    SIGMA_NETGW_MODE_TOR      = 2,  /* All traffic through Tor (optional)  */
    SIGMA_NETGW_MODE_AIRGAP   = 3,  /* No outbound — inbound LAN only      */
} sigma_netgw_mode_t;

/* ── Firewall ruleset for the gateway ────────────────────────────────────── */
typedef struct {
    /* Inbound from workload VM → internet */
    bool   allow_http;             /* TCP 80  */
    bool   allow_https;            /* TCP 443 */
    bool   allow_dns;              /* UDP/TCP 53 (forwarded to gateway resolver) */
    bool   allow_ssh_out;          /* TCP 22 outbound */
    bool   allow_smtp;             /* TCP 25/587/465 */
    bool   allow_ntp;              /* UDP 123 */
    bool   block_all_other;        /* default DROP for unlisted ports */

    /* DNS policy */
    bool   dns_intercept;          /* redirect all DNS to gateway's resolver */
    bool   dns_dnssec;             /* require DNSSEC validation */
    bool   dns_doh;                /* use DoH upstream (Cloudflare/Quad9) */
    char   dns_upstream[64];       /* upstream DoH URL */
    bool   dns_sinkhole_malware;   /* block known malware domains */

    /* Bandwidth limits (0 = unlimited) */
    sigma_u32 rate_limit_kbps;     /* aggregate rate limit from workload */
    sigma_u32 burst_kbps;

    /* Connection tracking */
    sigma_u32 conntrack_max;       /* max tracked connections */
    sigma_u32 conntrack_timeout_s; /* idle timeout */
} sigma_netgw_ruleset_t;

/* ── Gateway VM configuration ────────────────────────────────────────────── */
typedef struct {
    sigma_netgw_mode_t    mode;
    sigma_netgw_ruleset_t rules;

    /* VPN configuration (mode = SIGMA_NETGW_MODE_VPN) */
    char   vpn_server[64];
    sigma_u16 vpn_port;
    char   vpn_pubkey[64];         /* WireGuard peer public key */

    /* Tor configuration (mode = SIGMA_NETGW_MODE_TOR) */
    char   tor_socks_addr[32];     /* default: 127.0.0.1:9050 */

    /* Gateway VM resource limits */
    sigma_u32 memory_mb;           /* RAM for gateway VM (default 256 MB) */
    sigma_u32 vcpus;               /* vCPU count (default 1) */

    /* Logging */
    bool   log_all_flows;          /* log every forwarded connection */
    bool   log_blocked;            /* log dropped packets */
    char   log_socket[64];         /* unix socket for flow logs */
} sigma_netgw_config_t;

/* ── Runtime state ───────────────────────────────────────────────────────── */
typedef struct {
    sigma_netgw_config_t config;
    sigma_u32            gateway_vm_id;
    sigma_u32            workload_vm_id;
    char                 veth_gw[16];       /* e.g. "sgw0" on gateway side  */
    char                 veth_wl[16];       /* e.g. "sgw1" on workload side */
    bool                 active;
    sigma_u64            bytes_forwarded;
    sigma_u64            packets_dropped;
    sigma_u64            dns_queries;
    sigma_u64            dns_blocked;
} sigma_netgw_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/*
 * sigma_netgw_create — provision a two-VM gateway pair.
 * Creates the gateway VM, attaches a virtio-net veth pair,
 * configures the firewall ruleset, and starts the gateway.
 */
int sigma_netgw_create(const sigma_netgw_config_t *config,
                        sigma_netgw_t              *out);

/*
 * sigma_netgw_attach — attach an existing workload VM to a gateway.
 * Adds the workload-side veth to the workload VM's virtual NIC set.
 */
int sigma_netgw_attach(sigma_netgw_t *gw, sigma_u32 workload_vm_id);

/* Update the firewall ruleset on a live gateway (no gateway restart needed). */
int sigma_netgw_update_rules(sigma_netgw_t              *gw,
                              const sigma_netgw_ruleset_t *rules);

/* Change gateway mode (clearnet / VPN / Tor / airgap) on the fly. */
int sigma_netgw_set_mode(sigma_netgw_t *gw, sigma_netgw_mode_t mode);

/* Detach workload VM and destroy the gateway VM + veth pair. */
int sigma_netgw_destroy(sigma_netgw_t *gw);

/* Fetch runtime statistics. */
int sigma_netgw_stats(const sigma_netgw_t *gw,
                       sigma_u64 *bytes_fwd,
                       sigma_u64 *pkts_dropped,
                       sigma_u64 *dns_queries,
                       sigma_u64 *dns_blocked);

/* Default ruleset — allow HTTPS + DNS, block everything else. */
sigma_netgw_ruleset_t sigma_netgw_default_rules(void);

/* Maximum security ruleset — HTTPS-only, DoH, DNSSEC, sinkhole. */
sigma_netgw_ruleset_t sigma_netgw_secure_rules(void);
