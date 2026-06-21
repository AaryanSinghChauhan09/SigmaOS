/*
 * SigmaOS — sovereign socket ABI (userland + syscall surface)
 * Canonical names: sigma_socket_open / send / recv
 */
#ifndef SIGMA_SOCKET_ABI_H
#define SIGMA_SOCKET_ABI_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef sigma_u32 sigma_sock_t;

#define SIGMA_AF_INET  2u
#define SIGMA_SOCK_STREAM 1u
#define SIGMA_SOCK_DGRAM  2u
#define SIGMA_IPPROTO_TCP 6u
#define SIGMA_IPPROTO_UDP 17u

sigma_sock_t sigma_socket_open(sigma_u32 domain, sigma_u32 type, sigma_u32 protocol);
sigma_i32    sigma_socket_send(sigma_sock_t sock, const void* data, sigma_u32 len);
sigma_i32    sigma_socket_recv(sigma_sock_t sock, void* buffer, sigma_u32 max_len);
void         sigma_socket_close(sigma_sock_t sock);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SOCKET_ABI_H */
