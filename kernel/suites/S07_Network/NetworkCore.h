#ifndef SIGMA_NETWORKING_H
#define SIGMA_NETWORKING_H

// SigmaOS Networking Shard
// Absorbing robust TCP/IP features and BSD networking ideas
#include <sigma_types.h>

void net_init_custom_tcp_ip_stack();
void net_load_wifi_drivers();
void net_start_kernel_ssh_server();
void net_enable_distributed_os_nodes();

#endif // SIGMA_NETWORKING_H

