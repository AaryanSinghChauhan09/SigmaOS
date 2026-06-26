// SPDX-License-Identifier: GPL-2.0-or-later
#ifndef SIGMA_DHCP_H
#define SIGMA_DHCP_H

/**
 * SigmaOS DHCP Client (RFC 2131 / RFC 2132)
 * Full DISCOVER → OFFER → REQUEST → ACK flow with lease management.
 */

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── Constants ─────────────────────────────────────────────────────────────── */

#define DHCP_CLIENT_PORT          68
#define DHCP_SERVER_PORT          67

/* Message types (option 53) */
#define DHCP_MSG_DISCOVER          1
#define DHCP_MSG_OFFER             2
#define DHCP_MSG_REQUEST           3
#define DHCP_MSG_DECLINE           4
#define DHCP_MSG_ACK               5
#define DHCP_MSG_NAK               6
#define DHCP_MSG_RELEASE           7
#define DHCP_MSG_INFORM            8
#define DHCP_MSG_FORCE_RENEW       9
#define DHCP_MSG_LEASE_QUERY       10
#define DHCP_MSG_LEASE_UNASSIGNED  11
#define DHCP_MSG_LEASE_UNKNOWN     12
#define DHCP_MSG_LEASE_ACTIVE      13

/* Standard options */
#define DHCP_OPT_PAD               0
#define DHCP_OPT_SUBNET_MASK       1
#define DHCP_OPT_ROUTER            3
#define DHCP_OPT_DNS_SERVER        6
#define DHCP_OPT_HOST_NAME         12
#define DHCP_OPT_DOMAIN_NAME       15
#define DHCP_OPT_BROADCAST_ADDR    28
#define DHCP_OPT_REQUESTED_IP      50
#define DHCP_OPT_LEASE_TIME        51
#define DHCP_OPT_MSG_TYPE          53
#define DHCP_OPT_SERVER_ID         54
#define DHCP_OPT_PARAM_LIST        55
#define DHCP_OPT_MAX_MSG_SIZE      57
#define DHCP_OPT_RENEWAL_TIME      58
#define DHCP_OPT_REBIND_TIME       59
#define DHCP_OPT_VENDOR_CLASS      60
#define DHCP_OPT_CLIENT_ID         61
#define DHCP_OPT_TFTP_SERVER       66
#define DHCP_OPT_BOOT_FILE         67
#define DHCP_OPT_END               255

/* Defaults */
#define DHCP_DISCOVER_TIMEOUT_MS   2000
#define DHCP_REQUEST_TIMEOUT_MS    2000
#define DHCP_DEFAULT_LEASE         86400  /* 24 hours */
#define DHCP_MIN_LEASE             60

/* Magic cookie (RFC 2131 §3) */
#define DHCP_MAGIC_COOKIE          0x63825363U

/* Op codes */
#define DHCP_OP_BOOTREQUEST        1
#define DHCP_OP_BOOTREPLY          2

/* Hardware types */
#define DHCP_HTYPE_ETHERNET        1

/* Flags */
#define DHCP_FLAG_BROADCAST        0x8000

/* ── Wire message ───────────────────────────────────────────────────────────── */

typedef struct __attribute__((packed)) {
    uint8_t  op;           /* BOOTREQUEST / BOOTREPLY                     */
    uint8_t  htype;        /* Hardware type (1 = Ethernet)                */
    uint8_t  hlen;         /* Hardware address length                     */
    uint8_t  hops;         /* Relay agent hops                            */
    uint32_t xid;          /* Transaction ID                              */
    uint16_t secs;         /* Seconds since address acquisition began     */
    uint16_t flags;        /* DHCP_FLAG_BROADCAST or 0                    */
    uint32_t ciaddr;       /* Client IP (already bound, or 0)             */
    uint32_t yiaddr;       /* Your (offered/assigned) IP                  */
    uint32_t siaddr;       /* Next-server IP                              */
    uint32_t giaddr;       /* Relay agent IP                              */
    uint8_t  chaddr[16];   /* Client hardware address                     */
    uint8_t  sname[64];    /* Optional server name                        */
    uint8_t  file[128];    /* Boot file name                              */
    uint32_t magic;        /* DHCP_MAGIC_COOKIE                           */
    uint8_t  options[312]; /* Variable-length options                     */
} sigma_dhcp_message_t;

/* ── Option descriptor ─────────────────────────────────────────────────────── */

typedef struct {
    uint8_t  code;
    uint8_t  length;
    uint8_t* data;
} sigma_dhcp_option_t;

/* ── Lease state machine ────────────────────────────────────────────────────── */

typedef enum {
    DHCP_LEASE_STATE_INIT       = 0,
    DHCP_LEASE_STATE_SELECTING,
    DHCP_LEASE_STATE_REQUESTING,
    DHCP_LEASE_STATE_BOUND,
    DHCP_LEASE_STATE_RENEWING,
    DHCP_LEASE_STATE_REBINDING,
    DHCP_LEASE_STATE_EXPIRED,
    DHCP_LEASE_STATE_RELEASED,
} sigma_dhcp_lease_state_t;

typedef struct {
    char     interface[32];
    uint8_t  hwaddr[6];

    /* Assigned networking */
    uint32_t ip_address;
    uint32_t subnet_mask;
    uint32_t broadcast_addr;
    uint32_t server_id;
    uint32_t router;
    uint32_t dns_servers[4];
    uint8_t  dns_count;

    /* Domain */
    char     hostname[64];
    char     domain_name[64];

    /* Timing */
    uint32_t lease_time;
    uint32_t renewal_time;
    uint32_t rebind_time;
    time_t   lease_obtained;
    time_t   lease_expires;

    /* State */
    sigma_dhcp_lease_state_t state;
    uint32_t xid;
    uint8_t  retries;

    /* Optional boot parameters */
    char     tftp_server[128];
    char     boot_file[128];
} sigma_dhcp_lease_t;

/* ── Client configuration ──────────────────────────────────────────────────── */

typedef struct {
    char     interface[32];
    uint8_t  hwaddr[6];
    char     hostname[64];
    char     client_id[64];
    uint32_t requested_ip;
    char     vendor_class[64];

    uint8_t* param_list;
    uint8_t  param_list_len;

    uint32_t discover_timeout_ms;
    uint32_t request_timeout_ms;
    uint8_t  max_retries;

    bool     broadcast_flag;
    bool     rapid_commit;
    bool     persistent;

    /* Event callbacks */
    void (*on_bound)  (const sigma_dhcp_lease_t* lease, void* ctx);
    void (*on_expired)(const char* interface, void* ctx);
    void (*on_error)  (const char* interface, int error,
                       const char* msg, void* ctx);
    void* cb_ctx;
} sigma_dhcp_config_t;

/* ── Client context ────────────────────────────────────────────────────────── */

typedef struct {
    sigma_dhcp_config_t config;
    sigma_dhcp_lease_t  lease;
    uint32_t            xid_counter;
    bool                initialized;
    int                 socket_fd;
    bool                socket_bound;
    time_t              last_tx_time;
    time_t              lease_timer;

    /* Statistics */
    uint64_t discover_sent;
    uint64_t offer_received;
    uint64_t request_sent;
    uint64_t ack_received;
    uint64_t nak_received;
    uint64_t release_sent;
} sigma_dhcp_client_t;

/* ── Public API ─────────────────────────────────────────────────────────────── */

sigma_dhcp_client_t* sigma_dhcp_client_new(const char* interface);
sigma_dhcp_client_t* sigma_dhcp_client_new_with_config(const sigma_dhcp_config_t* cfg);
void                 sigma_dhcp_client_free(sigma_dhcp_client_t* client);

int sigma_dhcp_discover(sigma_dhcp_client_t* client);
int sigma_dhcp_request (sigma_dhcp_client_t* client, uint32_t req_ip, uint32_t srv_id);
int sigma_dhcp_renew   (sigma_dhcp_client_t* client);
int sigma_dhcp_release (sigma_dhcp_client_t* client);
int sigma_dhcp_decline (sigma_dhcp_client_t* client, uint32_t ip, const char* reason);

int sigma_dhcp_process_message(sigma_dhcp_client_t* client,
                                const sigma_dhcp_message_t* msg, size_t len);
int sigma_dhcp_tick(sigma_dhcp_client_t* client);

const sigma_dhcp_lease_t* sigma_dhcp_get_lease(const sigma_dhcp_client_t* client);
bool    sigma_dhcp_lease_is_valid(const sigma_dhcp_client_t* client);
int64_t sigma_dhcp_lease_remaining(const sigma_dhcp_client_t* client);

/* Message building / parsing */
int sigma_dhcp_build_discover(const sigma_dhcp_client_t* c,
                               sigma_dhcp_message_t* msg,
                               uint8_t* opts, size_t* opts_len);
int sigma_dhcp_build_request (const sigma_dhcp_client_t* c,
                               sigma_dhcp_message_t* msg,
                               uint8_t* opts, size_t* opts_len,
                               uint32_t req_ip, uint32_t srv_id);
int sigma_dhcp_build_release (const sigma_dhcp_client_t* c,
                               sigma_dhcp_message_t* msg,
                               uint8_t* opts, size_t* opts_len);
int sigma_dhcp_parse_message  (const uint8_t* buf, size_t len,
                               sigma_dhcp_message_t* msg);
int sigma_dhcp_parse_options  (const uint8_t* opts, size_t len,
                               sigma_dhcp_option_t** parsed, size_t* count);

/* Option helpers */
uint8_t* sigma_dhcp_find_option    (const sigma_dhcp_message_t* msg, uint8_t code);
int      sigma_dhcp_add_option     (uint8_t* opts, size_t* off, size_t max,
                                    uint8_t code, const void* data, uint8_t dlen);
int      sigma_dhcp_add_msg_type   (uint8_t* opts, size_t* off, uint8_t type);
int      sigma_dhcp_add_client_id  (uint8_t* opts, size_t* off,
                                    const uint8_t* hw, uint8_t hlen);
int      sigma_dhcp_add_requested_ip(uint8_t* opts, size_t* off, uint32_t ip);
int      sigma_dhcp_add_server_id  (uint8_t* opts, size_t* off, uint32_t ip);
int      sigma_dhcp_add_param_list (uint8_t* opts, size_t* off,
                                    const uint8_t* list, uint8_t len);
int      sigma_dhcp_add_hostname   (uint8_t* opts, size_t* off, const char* h);

/* Utilities */
const char* sigma_dhcp_msg_type_to_string(uint8_t type);
const char* sigma_dhcp_state_to_string(sigma_dhcp_lease_state_t state);
void        sigma_dhcp_ip_to_string(uint32_t ip, char* buf, size_t len);
uint32_t    sigma_dhcp_string_to_ip(const char* str);
void        sigma_dhcp_hwaddr_to_string(const uint8_t* hw, char* buf, size_t len);
int         sigma_dhcp_string_to_hwaddr(const char* str, uint8_t* hw);

/* Platform socket ops */
int     sigma_dhcp_socket_open (const char* interface);
int     sigma_dhcp_socket_bind (int fd, const char* interface);
ssize_t sigma_dhcp_socket_send (int fd, const uint8_t* data, size_t len,
                                 uint32_t dest_ip);
ssize_t sigma_dhcp_socket_recv (int fd, uint8_t* data, size_t max,
                                 uint32_t* src_ip);
void    sigma_dhcp_socket_close(int fd);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DHCP_H */
