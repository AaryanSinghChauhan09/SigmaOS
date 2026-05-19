/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: NETLINK SOCKETS (IPC)
 * =============================================================================
 * Inspired by: Linux kernel net/netlink/af_netlink.c
 * =============================================================================
 * Provides a datagram-oriented service for kernel-to-user routing and IPC.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define NETLINK_ROUTE       0
#define NETLINK_FIREWALL    3
#define NETLINK_KOBJECT_UEVENT 15

#define NLM_F_REQUEST       1
#define NLM_F_ACK           4

typedef struct {
    sigma_u32 nlmsg_len;
    sigma_u16 nlmsg_type;
    sigma_u16 nlmsg_flags;
    sigma_u32 nlmsg_seq;
    sigma_u32 nlmsg_pid;
} __attribute__((packed)) nlmsghdr_t;

typedef void (*netlink_recv_cb)(sigma_u32 pid, const nlmsghdr_t* nlh, const void* payload);

typedef struct {
    sigma_u32 protocol;
    sigma_u32 port_id;
    netlink_recv_cb callback;
    sigma_bool active;
} sigma_netlink_sock_t;

#define MAX_NETLINK_SOCKS 16
static sigma_netlink_sock_t nl_sockets[MAX_NETLINK_SOCKS];

void netlink_init(void) {
    sigma_memset(nl_sockets, 0, sizeof(nl_sockets));
    sigma_printf("[netlink] Netlink IPC subsystem initialized\n");
}

int netlink_bind(sigma_u32 protocol, sigma_u32 port_id, netlink_recv_cb cb) {
    for (sigma_u32 i = 0; i < MAX_NETLINK_SOCKS; i++) {
        if (!nl_sockets[i].active) {
            nl_sockets[i].protocol = protocol;
            nl_sockets[i].port_id  = port_id;
            nl_sockets[i].callback = cb;
            nl_sockets[i].active   = SIGMA_TRUE;
            sigma_printf("[netlink] Socket bound to protocol %u, port %u\n", protocol, port_id);
            return (int)i;
        }
    }
    return -1;
}

void netlink_kernel_send(sigma_u32 protocol, sigma_u32 dst_port, sigma_u16 type, const void* data, sigma_u32 len) {
    nlmsghdr_t hdr;
    hdr.nlmsg_len = sizeof(nlmsghdr_t) + len;
    hdr.nlmsg_type = type;
    hdr.nlmsg_flags = 0;
    hdr.nlmsg_seq = 0;
    hdr.nlmsg_pid = 0; /* Kernel sends as PID 0 */
    
    sigma_printf("[netlink] Kernel dispatching message (type %u, len %u) to port %u (proto %u)\n", 
                 type, len, dst_port, protocol);
                 
    /* Deliver to bound sockets */
    for (sigma_u32 i = 0; i < MAX_NETLINK_SOCKS; i++) {
        if (nl_sockets[i].active && nl_sockets[i].protocol == protocol && 
            (nl_sockets[i].port_id == dst_port || dst_port == 0xFFFFFFFF)) {
            if (nl_sockets[i].callback) {
                nl_sockets[i].callback(0, &hdr, data);
            }
        }
    }
}
