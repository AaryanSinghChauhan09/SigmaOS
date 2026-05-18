#include "libc/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignNet.h"
#include "libc/sigma_libc.h"
#include "sigma_string.h"

#define MAX_NET_PROTOCOLS 16
static sovereign_net_protocol_t g_net_protocols[MAX_NET_PROTOCOLS];
static sigma_u32 g_net_proto_count = 0;

void SovereignNet_InitRegistry(void) {
    sigma_sigma_memset(g_net_protocols, 0, sizeof(g_net_protocols));
    g_net_proto_count = 0;
    sigma_sigma_printf("S [NET]: Sovereign Network Registry Operational.\n");
}

sigma_err_t SovereignNet_RegisterProtocol(const char* name, sigma_u16 ethertype, sigma_net_handler_fn handler) {
    if (g_net_proto_count >= MAX_NET_PROTOCOLS) return SIGMA_ENOSPC;

    sovereign_net_protocol_t* p = &g_net_protocols[g_net_proto_count++];
    sigma_strncpy(p->name, name, 32);
    p->ethertype = ethertype;
    p->handler = handler;
    
    sigma_sigma_printf("S [NET]: Registered Network Protocol Shard '%s' (Type: 0x%04x)\n", name, ethertype);
    return SIGMA_OK;
}

void SovereignNet_ProcessPacket(sigma_u16 ethertype, void* payload, sigma_sz_t size) {
    for (sigma_u32 i = 0; i < g_net_proto_count; i++) {
        if (g_net_protocols[i].ethertype == ethertype) {
            g_net_protocols[i].handler(payload, size);
            return;
        }
    }
    sigma_sigma_printf("S [NET/ERR]: Dropped unknown packet (EtherType: 0x%04x)\n", ethertype);
}



