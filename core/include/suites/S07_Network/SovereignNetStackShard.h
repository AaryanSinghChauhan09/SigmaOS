/* S SIGMAOS: SOVEREIGN NETWORK STACK SHARD HEADER */
#ifndef SOVEREIGN_NETSTACK_SHARD_H
#define SOVEREIGN_NETSTACK_SHARD_H
#include "sigma_types.h"
typedef enum { SOCK_SIGMA_TCP=0, SOCK_SIGMA_UDP=1, SOCK_SIGMA_RAW=2 } SigmaSockType_t;
sigma_u32   sigma_socket    (SigmaSockType_t type);
sigma_err_t sigma_bind      (sigma_u32 fd, sigma_u32 addr, sigma_u16 port);
sigma_err_t sigma_listen    (sigma_u32 fd, sigma_u32 backlog);
sigma_err_t sigma_connect   (sigma_u32 fd, sigma_u32 dst, sigma_u16 port);
sigma_err_t sigma_send      (sigma_u32 fd, sigma_u32 len);
sigma_u32   sigma_recv      (sigma_u32 fd, sigma_u32 max_len);
sigma_err_t sigma_route_add (sigma_u32 dest, sigma_u32 mask, sigma_u32 gw,
                              const char* iface, sigma_u32 metric);
void        SovereignNetStackShard_Init (void);
void        SovereignNetStack_Audit      (void);
#endif
