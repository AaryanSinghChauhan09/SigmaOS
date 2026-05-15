// SigmaOS — sigma-net-lwip: Lightweight TCP/IP Stack
// Inspired by: lwIP — Lightweight IP stack for embedded OS
// Module: sigma-net-tcp
// USP over lwIP: No heap allocation — all connection state in static ring
// Implements TCP state machine natively, zero external network library

#ifndef SIGMA_NET_TCP_H
#define SIGMA_NET_TCP_H

#include "../../include/sigma_ring_buffer.h"
#include "../../include/sigma_spinlock.h"

#define SIGMA_TCP_MAX_CONNS  32
#define SIGMA_TCP_RX_BUF     1024
#define SIGMA_TCP_TX_BUF     1024

typedef enum SigmaTCPState {
    TCP_CLOSED      = 0,
    TCP_LISTEN      = 1,
    TCP_SYN_SENT    = 2,
    TCP_SYN_RECV    = 3,
    TCP_ESTABLISHED = 4,
    TCP_FIN_WAIT1   = 5,
    TCP_FIN_WAIT2   = 6,
    TCP_CLOSE_WAIT  = 7,
    TCP_CLOSING     = 8,
    TCP_TIME_WAIT   = 9
} SigmaTCPState;

typedef struct SigmaTCPConn {
    SigmaTCPState    state;
    unsigned int     local_ip;
    unsigned int     remote_ip;
    unsigned short   local_port;
    unsigned short   remote_port;
    unsigned int     seq_num;
    unsigned int     ack_num;
    unsigned char    rx_buf[SIGMA_TCP_RX_BUF];
    unsigned char    tx_buf[SIGMA_TCP_TX_BUF];
    unsigned int     rx_head, rx_tail;
    unsigned int     tx_head, tx_tail;
    SigmaSpinlock    lock;
} SigmaTCPConn;

typedef struct SigmaTCPStack {
    SigmaTCPConn conns[SIGMA_TCP_MAX_CONNS];
    unsigned int count;
} SigmaTCPStack;

static inline void tcp_stack_init(SigmaTCPStack* s) {
    s->count = 0;
    for (int i = 0; i < SIGMA_TCP_MAX_CONNS; i++) {
        s->conns[i].state = TCP_CLOSED;
        spinlock_init(&s->conns[i].lock);
    }
}

static inline SigmaTCPConn* tcp_alloc_conn(SigmaTCPStack* s) {
    for (int i = 0; i < SIGMA_TCP_MAX_CONNS; i++)
        if (s->conns[i].state == TCP_CLOSED) return &s->conns[i];
    return (void*)0;
}

static inline int tcp_connect(SigmaTCPConn* c,
                               unsigned int rem_ip, unsigned short rem_port,
                               unsigned int loc_ip, unsigned short loc_port) {
    c->remote_ip   = rem_ip;   c->remote_port = rem_port;
    c->local_ip    = loc_ip;   c->local_port  = loc_port;
    c->seq_num     = 0xABCD1234; // would use RDTSC-seeded ISN in production
    c->ack_num     = 0;
    c->state       = TCP_SYN_SENT;
    c->rx_head = c->rx_tail = c->tx_head = c->tx_tail = 0;
    return 0;
}

static inline int tcp_write(SigmaTCPConn* c, const unsigned char* data, unsigned int len) {
    if (c->state != TCP_ESTABLISHED) return -1;
    spinlock_acquire(&c->lock);
    for (unsigned int i = 0; i < len; i++) {
        unsigned int next = (c->tx_head + 1) % SIGMA_TCP_TX_BUF;
        if (next == c->tx_tail) { spinlock_release(&c->lock); return (int)i; }
        c->tx_buf[c->tx_head] = data[i];
        c->tx_head = next;
    }
    spinlock_release(&c->lock);
    return (int)len;
}

static inline int tcp_read(SigmaTCPConn* c, unsigned char* out, unsigned int max) {
    spinlock_acquire(&c->lock);
    unsigned int n = 0;
    while (n < max && c->rx_head != c->rx_tail) {
        out[n++] = c->rx_buf[c->rx_tail];
        c->rx_tail = (c->rx_tail + 1) % SIGMA_TCP_RX_BUF;
    }
    spinlock_release(&c->lock);
    return (int)n;
}

static inline void tcp_close(SigmaTCPConn* c) {
    c->state = TCP_FIN_WAIT1;
}

#endif /* SIGMA_NET_TCP_H */
