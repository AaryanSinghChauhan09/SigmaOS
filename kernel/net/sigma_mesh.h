// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_mesh.h — WireGuard-based mesh VPN for sigma-fleet
 *
 * Inspired by: Tailscale, ZeroTier, WireGuard (RFC/whitepaper)
 *
 * Architecture:
 *   Each SigmaOS node generates a Curve25519 keypair at first boot.
 *   Nodes discover each other via the sigma-fleet coordinator
 *   (or a local mDNS broadcast for LAN-only meshes).
 *   All traffic is encrypted E2E with WireGuard ChaCha20-Poly1305.
 *   No central relay required — direct peer-to-peer when possible,
 *   STUN/hole-punching fallback when behind NAT.
 *
 * Use cases:
 *   - Multi-branch retail: 10 shops sync data without cloud
 *   - sigma-fleet remote management across offices
 *   - Developer SSH access to RTOS nodes from anywhere
 *   - sigma-legal: encrypted client↔CA file exchange
 *
 * CLI:
 *   sigma-mesh init                    # generate keypair, create mesh0 iface
 *   sigma-mesh peer add <pubkey> <ip>  # add a peer
 *   sigma-mesh peer list               # list connected peers + latency
 *   sigma-mesh status                  # interface stats
 *   sigma-mesh route add 10.0.0.0/24   # route a subnet through the mesh
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Key sizes (WireGuard) ───────────────────────────────────────────────── */
#define SIGMA_MESH_KEY_LEN     32   /* Curve25519 key length                */
#define SIGMA_MESH_PUBKEY_LEN  32
#define SIGMA_MESH_PSK_LEN     32   /* optional pre-shared key              */
#define SIGMA_MESH_MAX_PEERS   256

/* ── Peer endpoint ───────────────────────────────────────────────────────── */
typedef struct {
    char       hostname[64];
    char       ip4[16];          /* "10.152.1.2"                           */
    char       ip6[40];
    sigma_u16  port;             /* WireGuard UDP port (default 51820)     */
} sigma_mesh_endpoint_t;

/* ── Peer descriptor ─────────────────────────────────────────────────────── */
typedef struct {
    sigma_u8            pubkey[SIGMA_MESH_PUBKEY_LEN];
    sigma_u8            psk[SIGMA_MESH_PSK_LEN];   /* optional, zero = none */
    sigma_mesh_endpoint_t endpoint;
    char                allowed_ips[16][32];        /* CIDR routes           */
    int                 n_allowed_ips;
    sigma_u32           keepalive_s;               /* 0 = disabled          */

    /* Runtime state */
    bool                connected;
    sigma_u64           rx_bytes;
    sigma_u64           tx_bytes;
    sigma_u64           last_handshake_ns;
    sigma_u32           latency_ms;
} sigma_mesh_peer_t;

/* ── Mesh interface ──────────────────────────────────────────────────────── */
typedef struct {
    char                ifname[16];       /* "mesh0"                        */
    sigma_u8            privkey[SIGMA_MESH_KEY_LEN];
    sigma_u8            pubkey[SIGMA_MESH_PUBKEY_LEN];
    char                address[32];      /* mesh IP: "10.152.0.1/24"      */
    sigma_u16           listen_port;
    sigma_mesh_peer_t   peers[SIGMA_MESH_MAX_PEERS];
    int                 n_peers;
    bool                active;
    sigma_u64           total_rx_bytes;
    sigma_u64           total_tx_bytes;
} sigma_mesh_iface_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Generate Curve25519 keypair for this node. */
int sigma_mesh_keygen(sigma_u8 *privkey, sigma_u8 *pubkey);

/* Create and configure the WireGuard interface. */
int sigma_mesh_init(const char *ifname, const char *address,
                    sigma_u16 listen_port, sigma_mesh_iface_t *out);

/* Add a peer (triggers initial handshake). */
int sigma_mesh_peer_add(sigma_mesh_iface_t *iface,
                         const sigma_mesh_peer_t *peer);

/* Remove a peer. */
int sigma_mesh_peer_remove(sigma_mesh_iface_t *iface,
                            const sigma_u8 pubkey[SIGMA_MESH_PUBKEY_LEN]);

/* Update peer endpoint (after NAT traversal / IP change). */
int sigma_mesh_peer_update_endpoint(sigma_mesh_iface_t *iface,
                                     const sigma_u8 *pubkey,
                                     const sigma_mesh_endpoint_t *ep);

/* Route a subnet through the mesh. */
int sigma_mesh_route_add(sigma_mesh_iface_t *iface,
                          const char *cidr,
                          const sigma_u8 *via_peer_pubkey);

/* Get interface statistics. */
int sigma_mesh_stats(const sigma_mesh_iface_t *iface,
                      sigma_u64 *rx_bytes, sigma_u64 *tx_bytes,
                      int *connected_peers);

/* Tear down the mesh interface. */
int sigma_mesh_shutdown(sigma_mesh_iface_t *iface);
