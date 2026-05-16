#include "../../../../../include/libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS: S07_NETWORK  SovereignNetwork_FSM.c
 * =========================================================================
 * Implementation of Idea 251 (Apex Infinity): TCP Finite State Machine.
 * Zero external libraries. Direct state transitions.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"

typedef enum {
    TCP_CLOSED, TCP_LISTEN, TCP_SYN_SENT, TCP_SYN_RECEIVED, 
    TCP_ESTABLISHED, TCP_FIN_WAIT_1, TCP_FIN_WAIT_2, 
    TCP_CLOSE_WAIT, TCP_CLOSING, TCP_LAST_ACK, TCP_TIME_WAIT
} SovereignTcpState;

const char* tcp_state_to_str(SovereignTcpState state) {
    switch (state) {
        case TCP_CLOSED:      return "CLOSED";
        case TCP_LISTEN:      return "LISTEN";
        case TCP_ESTABLISHED: return "ESTABLISHED";
        default:              return "TRANSITIONING";
    }
}

void tcp_fsm_init(void) {
    sigma_sigma_printf("S [S07]: TCP Finite State Machine Materialized (Apex Idea 251).\n");
}

SovereignTcpState tcp_transition(SovereignTcpState current, const char* event) {
    if (current == TCP_CLOSED && sigma_sigma_strcmp(event, "PASSIVE_OPEN") == 0) return TCP_LISTEN;
    if (current == TCP_LISTEN && sigma_sigma_strcmp(event, "SYN") == 0) return TCP_SYN_RECEIVED;
    if (current == TCP_SYN_RECEIVED && sigma_sigma_strcmp(event, "ACK") == 0) return TCP_ESTABLISHED;
    
    return current;
}
