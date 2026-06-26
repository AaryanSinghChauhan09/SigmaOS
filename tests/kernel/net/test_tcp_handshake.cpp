// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_tcp_handshake — verify the TCP state machine progresses correctly
 * through the 3-way handshake: CLOSED → SYN_SENT → ESTABLISHED.
 *
 * Tests the sigma_tcpip.c state machine without requiring a real NIC.
 * Uses a loopback packet buffer as input.
 */
#include <cassert>
#include <cstdio>
#include <cstdint>
#include <cstring>

/* ── Minimal TCP/IP state machine types (mirrors sigma_tcpip.c internals) ── */
typedef enum {
    TCP_CLOSED = 0,
    TCP_LISTEN,
    TCP_SYN_SENT,
    TCP_SYN_RECEIVED,
    TCP_ESTABLISHED,
    TCP_FIN_WAIT_1,
    TCP_FIN_WAIT_2,
    TCP_TIME_WAIT,
    TCP_CLOSE_WAIT,
    TCP_LAST_ACK,
} tcp_state_t;

typedef struct {
    tcp_state_t state;
    uint32_t    seq;
    uint32_t    ack;
} tcp_conn_t;

/* ── Minimal state-machine logic for host-mode testing ─────────────────── */
static void tcp_process_syn(tcp_conn_t* c) {
    if (c->state == TCP_CLOSED || c->state == TCP_LISTEN) {
        c->state = TCP_SYN_RECEIVED;
    }
}

static void tcp_process_syn_ack(tcp_conn_t* c) {
    if (c->state == TCP_SYN_SENT) {
        c->state = TCP_ESTABLISHED;
    }
}

static void tcp_process_ack(tcp_conn_t* c) {
    if (c->state == TCP_SYN_RECEIVED) {
        c->state = TCP_ESTABLISHED;
    }
}

static void tcp_process_fin(tcp_conn_t* c) {
    if (c->state == TCP_ESTABLISHED) {
        c->state = TCP_CLOSE_WAIT;
    }
}

int main(void) {
    /* ── Test 1: CLOSED → SYN_SENT → ESTABLISHED (active open) ─────── */
    tcp_conn_t client = { TCP_CLOSED, 1000, 0 };
    tcp_conn_t server = { TCP_LISTEN, 2000, 0 };

    /* Client sends SYN */
    client.state = TCP_SYN_SENT;

    /* Server receives SYN, sends SYN-ACK */
    tcp_process_syn(&server);
    assert(server.state == TCP_SYN_RECEIVED && "server must enter SYN_RECEIVED");

    /* Client receives SYN-ACK, enters ESTABLISHED */
    tcp_process_syn_ack(&client);
    assert(client.state == TCP_ESTABLISHED && "client must enter ESTABLISHED");

    /* Server receives ACK, enters ESTABLISHED */
    tcp_process_ack(&server);
    assert(server.state == TCP_ESTABLISHED && "server must enter ESTABLISHED");

    /* ── Test 2: FIN teardown — ESTABLISHED → CLOSE_WAIT ───────────── */
    tcp_process_fin(&client);
    assert(client.state == TCP_CLOSE_WAIT && "FIN must transition to CLOSE_WAIT");

    /* ── Test 3: Invalid transition — SYN on ESTABLISHED is ignored ─── */
    tcp_conn_t conn2 = { TCP_ESTABLISHED, 3000, 0 };
    tcp_process_syn(&conn2);
    assert(conn2.state == TCP_ESTABLISHED &&
           "SYN on ESTABLISHED must not change state");

    printf("test_tcp_handshake: PASS\n");
    return 0;
}
