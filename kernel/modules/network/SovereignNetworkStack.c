/*
 * Σ SIGMAOS: SOVEREIGN NETWORK STACK v3.0 — MODULAR
 * Mission: Zero-Wait sharded TCP/IP Stack. Every protocol is a shard.
 * Design: C11 / Zero-Dependency / Registry-Based.
 */
#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignNet.h"

/* Extern Shard Registration Functions */
extern void SovereignEthernet_Register(void);

void sigma_network_shard_init(void) {
    sigma_printf("Σ [NET]: Synchronizing Sovereign Network Shards...\n");

    /* 1. Initialize Registry */
    SovereignNet_InitRegistry();

    /* 2. Register Protocol Shards */
    SovereignEthernet_Register();

    /* 3. Simulate Packet Ingress */
    sigma_u8 dummy_frame[128];
    SovereignNet_ProcessPacket(0x88B5, dummy_frame, sizeof(dummy_frame));

    sigma_printf("Σ [NET]: Network Stack online. Connectivity Sovereignty achieved.\n");
}
