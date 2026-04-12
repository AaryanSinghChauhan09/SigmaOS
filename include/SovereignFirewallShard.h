/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN FIREWALL SHARD HEADER
 * ========================================================================= */
#ifndef SOVEREIGN_FIREWALL_SHARD_H
#define SOVEREIGN_FIREWALL_SHARD_H
#include "sigma_types.h"
typedef enum { FW_PROTO_ANY=0, FW_PROTO_TCP=6, FW_PROTO_UDP=17, FW_PROTO_ICMP=1 } SigmaFWProto_t;
typedef enum { FW_ACCEPT, FW_DROP, FW_REJECT, FW_LOG_AND_ACCEPT } SigmaFWVerdict_t;
sigma_err_t      sigma_fw_add_rule  (SigmaFWProto_t proto, sigma_u32 src, sigma_u32 dst,
                                      sigma_u16 port, SigmaFWVerdict_t verdict, const char* comment);
SigmaFWVerdict_t sigma_fw_classify  (SigmaFWProto_t proto, sigma_u32 src, sigma_u32 dst, sigma_u16 dst_port);
void             SovereignFirewallShard_Init (void);
void             SovereignFirewall_Audit      (void);
#endif
