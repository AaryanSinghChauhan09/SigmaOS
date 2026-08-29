/* SPDX-License-Identifier: MIT */
/*
 * =========================================================================
 * Σ SIGMAOS: NETWORK SUBSYSTEM KERNEL INTERFACE (S-NET)
 * =========================================================================
 * Socket definitions, network protocol families, netlink bus,
 * and FreeBSD VNET stack kernel header definitions.
 * =========================================================================
 */

#ifndef SIGMA_NET_H
#define SIGMA_NET_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Address Families --- */
#define SIGMA_AF_UNSPEC   0
#define SIGMA_AF_UNIX     1
#define SIGMA_AF_LOCAL    SIGMA_AF_UNIX
#define SIGMA_AF_INET     2
#define SIGMA_AF_INET6    10
#define SIGMA_AF_NETLINK  16
#define SIGMA_AF_PACKET   17

/* --- Socket Types --- */
#define SIGMA_SOCK_STREAM    1
#define SIGMA_SOCK_DGRAM     2
#define SIGMA_SOCK_RAW       3
#define SIGMA_SOCK_SEQPACKET 5
#define SIGMA_SOCK_NONBLOCK  00004000
#define SIGMA_SOCK_CLOEXEC   02000000

/* --- IP Protocols --- */
#define SIGMA_IPPROTO_IP    0
#define SIGMA_IPPROTO_ICMP  1
#define SIGMA_IPPROTO_TCP   6
#define SIGMA_IPPROTO_UDP   17
#define SIGMA_IPPROTO_IPV6  41
#define SIGMA_IPPROTO_RAW   255

/* --- Socket Options --- */
#define SIGMA_SOL_SOCKET    1
#define SIGMA_SO_DEBUG      1
#define SIGMA_SO_REUSEADDR  2
#define SIGMA_SO_TYPE       3
#define SIGMA_SO_ERROR      4
#define SIGMA_SO_DONTROUTE  5
#define SIGMA_SO_BROADCAST  6
#define SIGMA_SO_SNDBUF     7
#define SIGMA_SO_RCVBUF     8
#define SIGMA_SO_KEEPALIVE  9
#define SIGMA_SO_OOBINLINE  10
#define SIGMA_SO_NO_CHECK   11
#define SIGMA_SO_PRIORITY   12
#define SIGMA_SO_LINGER     13
#define SIGMA_SO_BSDCOMPAT  14
#define SIGMA_SO_REUSEPORT  15
#define SIGMA_SO_BINDTODEVICE 25

/* --- FreeBSD VNET Stack Identifier Structure --- */
struct sigma_vnet {
    sigma_u32       vnet_id;
    char            vnet_name[32];
    sigma_u32       flags;
    sigma_uintptr_t ifnet_head;
};

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NET_H */
