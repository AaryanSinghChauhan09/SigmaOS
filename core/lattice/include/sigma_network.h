#ifndef SIGMA_NETWORK_H
#define SIGMA_NETWORK_H

/* =========================================================================
 * SIGMA OS: SOVEREIGN NETWORK SUBSYSTEM (SYSTEM-LEVEL HEADER)
 * ========================================================================= */

void sigma_network_init();
void sigma_network_cleanup();
int sigma_create_server_socket(int port);
void sigma_close_socket(int socket_fd);

#endif
