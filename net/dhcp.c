/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: DHCP CLIENT
 * =============================================================================
 * Inspired by: systemd-networkd (systemd/src/network/)
 *              ISC dhclient (isc-dhcp)
 *              RFC 2131 (Dynamic Host Configuration Protocol)
 * =============================================================================
 * Simulates DHCP DORA handshake (Discover → Offer → Request → Acknowledge)
 * to obtain an IP address, subnet mask, gateway, and DNS from a server.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define DHCP_STATE_INIT       0
#define DHCP_STATE_SELECTING  1
#define DHCP_STATE_REQUESTING 2
#define DHCP_STATE_BOUND      3
#define DHCP_STATE_RENEWING   4
#define DHCP_STATE_REBINDING  5

#define DHCP_MSG_DISCOVER  1
#define DHCP_MSG_OFFER     2
#define DHCP_MSG_REQUEST   3
#define DHCP_MSG_ACK       4
#define DHCP_MSG_NAK       5
#define DHCP_MSG_RELEASE   7

typedef struct {
    sigma_u32 assigned_ip;
    sigma_u32 subnet_mask;
    sigma_u32 gateway;
    sigma_u32 dns_server;
    sigma_u32 lease_time;     /* seconds */
    sigma_u32 server_ip;
    sigma_u32 state;
    sigma_u32 transaction_id;
} dhcp_client_t;

static void print_ipv4(sigma_u32 ip) {
    sigma_printf("%u.%u.%u.%u",
        (ip >> 24) & 0xFF, (ip >> 16) & 0xFF,
        (ip >> 8) & 0xFF, ip & 0xFF);
}

void dhcp_client_init(dhcp_client_t* client) {
    sigma_memset(client, 0, sizeof(*client));
    client->state = DHCP_STATE_INIT;
    client->transaction_id = 0x5349474D; /* "SIGM" */
    sigma_printf("[dhcp] Client initialized (xid=0x%08X)\n", client->transaction_id);
}

void dhcp_send_discover(dhcp_client_t* client) {
    client->state = DHCP_STATE_SELECTING;
    sigma_printf("[dhcp] DHCPDISCOVER broadcast on 255.255.255.255:67\n");
    sigma_printf("[dhcp]   xid=0x%08X, requesting IP lease\n", client->transaction_id);

    /* Simulate server response: DHCPOFFER */
    client->server_ip   = 0xC0A80101; /* 192.168.1.1 */
    client->assigned_ip = 0xC0A8010A; /* 192.168.1.10 */
    client->subnet_mask = 0xFFFFFF00; /* 255.255.255.0 */
    client->gateway     = 0xC0A80101; /* 192.168.1.1 */
    client->dns_server  = 0x08080808; /* 8.8.8.8 */
    client->lease_time  = 86400;      /* 24 hours */

    sigma_printf("[dhcp] DHCPOFFER received from ");
    print_ipv4(client->server_ip);
    sigma_printf("\n");
    sigma_printf("[dhcp]   Offered IP: ");
    print_ipv4(client->assigned_ip);
    sigma_printf(", Lease: %u seconds\n", client->lease_time);
}

void dhcp_send_request(dhcp_client_t* client) {
    client->state = DHCP_STATE_REQUESTING;
    sigma_printf("[dhcp] DHCPREQUEST sent to ");
    print_ipv4(client->server_ip);
    sigma_printf("\n");
    sigma_printf("[dhcp]   Requesting IP: ");
    print_ipv4(client->assigned_ip);
    sigma_printf("\n");

    /* Simulate server ACK */
    sigma_printf("[dhcp] DHCPACK received — lease confirmed\n");
    client->state = DHCP_STATE_BOUND;
}

void dhcp_bind(dhcp_client_t* client) {
    sigma_printf("[dhcp] Interface configured:\n");
    sigma_printf("[dhcp]   IP Address : ");  print_ipv4(client->assigned_ip); sigma_printf("\n");
    sigma_printf("[dhcp]   Subnet Mask: ");  print_ipv4(client->subnet_mask); sigma_printf("\n");
    sigma_printf("[dhcp]   Gateway    : ");  print_ipv4(client->gateway);     sigma_printf("\n");
    sigma_printf("[dhcp]   DNS Server : ");  print_ipv4(client->dns_server);  sigma_printf("\n");
    sigma_printf("[dhcp]   Lease Time : %u seconds\n", client->lease_time);
    sigma_printf("[dhcp] State: BOUND — network connectivity established\n");
}

void dhcp_release(dhcp_client_t* client) {
    sigma_printf("[dhcp] DHCPRELEASE sent for ");
    print_ipv4(client->assigned_ip);
    sigma_printf("\n");
    client->assigned_ip = 0;
    client->state = DHCP_STATE_INIT;
    sigma_printf("[dhcp] Lease released — interface deconfigured\n");
}

void dhcp_full_handshake(dhcp_client_t* client) {
    sigma_printf("\n--- Σ DHCP DORA Handshake ---\n");
    dhcp_send_discover(client);
    dhcp_send_request(client);
    dhcp_bind(client);
    sigma_printf("--- Σ DHCP Complete ---\n\n");
}
