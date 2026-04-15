/*
 * =========================================================================
 * Σ SIGMAOS kernel/suites/S15_Distributed/shards/sigma_raft.h
 * =========================================================================
 * Sovereign Distributed Consensus — gap-closes:
 *   etcd    : Raft consensus, leader election, log replication
 *   ZooKeeper: ZAB protocol, ephemeral nodes, watches
 *   Consul  : gossip (SWIM), service discovery, health checks
 *   Kubernetes: etcd-backed state, controller reconciliation loop
 *   Kafka   : distributed commit log, partition leadership
 * =========================================================================
 * Implements Raft (Ongaro & Ousterhout 2014) in pure C11.
 * Suitable for embedded cluster metadata coordination.
 * =========================================================================
 */

#ifndef SIGMA_RAFT_H
#define SIGMA_RAFT_H

typedef unsigned long long rf_u64;
typedef unsigned int       rf_u32;
typedef signed   int       rf_i32;
typedef unsigned char      rf_bool;
#define RF_TRUE  ((rf_bool)1)
#define RF_FALSE ((rf_bool)0)
#define RF_NULL  ((void*)0)
#define RF_OK    ((rf_i32) 0)
#define RF_ERR   ((rf_i32)-1)

/* ── Raft node states ────────────────────────────────────────────────────── */
typedef enum {
    RAFT_FOLLOWER  = 0,
    RAFT_CANDIDATE = 1,
    RAFT_LEADER    = 2
} raft_state_t;

#define RAFT_MAX_NODES   16
#define RAFT_LOG_MAX    512
#define RAFT_CMD_LEN     64

/* ── Log entry ───────────────────────────────────────────────────────────── */
typedef struct {
    rf_u64 term;
    rf_u64 index;
    char   command[RAFT_CMD_LEN];
} raft_log_entry_t;

/* ── Node descriptor ─────────────────────────────────────────────────────── */
typedef struct {
    rf_u32       node_id;
    char         addr[48];    /* host:port                              */
    rf_bool      voting;
    rf_bool      reachable;
    rf_u64       next_index;  /* leader: next log index to send         */
    rf_u64       match_index; /* leader: highest log index known replicated */
} raft_peer_t;

/* ── Raft FSM state ──────────────────────────────────────────────────────── */
typedef struct {
    rf_u32          self_id;
    raft_state_t    state;

    /* Persistent state (written to stable storage before RPC response) */
    rf_u64          current_term;
    rf_u32          voted_for;     /* 0 = none                         */
    raft_log_entry_t log[RAFT_LOG_MAX];
    rf_u64          log_len;

    /* Volatile state */
    rf_u64          commit_index;
    rf_u64          last_applied;

    /* Cluster membership */
    raft_peer_t     peers[RAFT_MAX_NODES];
    rf_u32          peer_count;

    /* Election */
    rf_u64          election_timeout_ms;
    rf_u64          last_heartbeat_ms;
    rf_u32          votes_received;

    /* Gossip health (Consul SWIM parity) */
    rf_u64          gossip_seq;
} raft_node_t;

/* ── RPC messages ────────────────────────────────────────────────────────── */
typedef struct {
    rf_u64 term;
    rf_u32 candidate_id;
    rf_u64 last_log_index;
    rf_u64 last_log_term;
} raft_vote_req_t;

typedef struct {
    rf_u64  term;
    rf_bool granted;
} raft_vote_resp_t;

typedef struct {
    rf_u64           term;
    rf_u32           leader_id;
    rf_u64           prev_log_index;
    rf_u64           prev_log_term;
    raft_log_entry_t entries[8];
    rf_u32           entry_count;
    rf_u64           leader_commit;
} raft_append_req_t;

typedef struct {
    rf_u64  term;
    rf_bool success;
    rf_u64  match_index;
} raft_append_resp_t;

/* ── Service registry entry (Consul parity) ─────────────────────────────── */
typedef struct {
    char   name[32];
    char   addr[48];
    rf_u32 port;
    rf_bool healthy;
    rf_u64  last_check_ns;
} sigma_service_entry_t;

#define SIGMA_SVC_MAX 128

/* ── Public API ─────────────────────────────────────────────────────────── */
void   sigma_raft_init(rf_u32 self_id, const char *addr);
rf_i32 sigma_raft_add_peer(rf_u32 id, const char *addr, rf_bool voting);
rf_i32 sigma_raft_propose(const char *command);   /* client write request */
void   sigma_raft_tick(rf_u64 elapsed_ms);         /* drive state machine  */

/* Leader election */
void   sigma_raft_start_election(void);
void   sigma_raft_handle_vote_req(raft_vote_req_t *req, raft_vote_resp_t *resp);
void   sigma_raft_handle_vote_resp(rf_u32 from, raft_vote_resp_t *resp);
void   sigma_raft_handle_append(raft_append_req_t *req, raft_append_resp_t *resp);
void   sigma_raft_send_heartbeats(void);

/* Service discovery (Consul parity) */
rf_i32 sigma_svc_register(const char *name, const char *addr, rf_u32 port);
sigma_service_entry_t *sigma_svc_lookup(const char *name);
void   sigma_svc_health_check(void);
void   sigma_svc_list(void);

void   sigma_raft_status(void);

#endif /* SIGMA_RAFT_H */
