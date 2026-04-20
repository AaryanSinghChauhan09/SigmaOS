/* --------------------------------------------------------------------------
 * SIGMA OS SOVEREIGN HTTP ENGINE (SYSTEM-LEVEL ARCHITECTURE)
 * --------------------------------------------------------------------------
 * This utilizes strict separation of concerns into network and http 
 * modular C sub-systems, representing pure OS-level design methodology.
 * -------------------------------------------------------------------------- */

#include <stdio.h>
#include <stdlib.h>
#include "../include/sigma_network.h"
#include "../include/sigma_http.h"

#ifdef _WIN32
    #include <winsock2.h>
#else
    #include <sys/socket.h>
#endif

#define PORT 3334

int main() {
    sigma_network_init();

    int server_socket = sigma_create_server_socket(PORT);

    printf("====================================================\n");
    printf("[KERNEL] Σ SIGMA OS SOVEREIGN C-WEB ENGINE\n");
    printf("====================================================\n");
    printf("[KERNEL] System C-Level Modular Architecture Active.\n");
    printf("[KERNEL] Network & HTTP protocols securely sharded.\n");
    printf("[KERNEL] Awaiting incoming connections (Port %d)...\n\n", PORT);

    while (1) {
        struct sockaddr_in client_addr;
        int client_len = sizeof(client_addr);
        int client_socket = accept(server_socket, (struct sockaddr*)&client_addr, &client_len);

        if (client_socket >= 0) {
            sigma_handle_client_connection(client_socket);
        }
    }

    sigma_network_cleanup();
    return 0;
}
