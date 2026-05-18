#include "libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S15_DevNexus/shards/sigma_raft.c
 * =========================================================================
 * Pure C11 Raft consensus implementation + Consul-style service registry.
 * =========================================================================
 */

#include "sigma_raft.h"
#include "libc/sigma_libc.h"

static raft_node_t        s_raft;
static sigma_service_entry_t s_services[SIGMA_SVC_MAX];
static rf_u32             s_svc_count = 0;

static const char *state_str[] = {"FOLLOWER","CANDIDATE","LEADER"};

/* -- Init ------------------------------------------------------------------ */
void sigma_raft_init(rf_u32 self_id, const char *addr) {
    sigma_sigma_memset(&s_raft, 0, sizeof(s_raft));
    s_raft.self_id              = self_id;
    s_raft.state                = RAFT_FOLLOWER;
    s_raft.current_term         = 0;
    s_raft.voted_for            = 0;
    s_raft.election_timeout_ms  = 150 + (self_id * 37 % 150); /* jitter */
    s_raft.commit_index         = 0;
    s_raft.last_applied         = 0;
    (void)addr;
    sigma_sigma_printf("S [RAFT] Node %u initialized. Election timeout: %llums\n",
                 self_id, (unsigned long long)s_raft.election_timeout_ms);
}

rf_i32 sigma_raft_add_peer(rf_u32 id, const char *addr, rf_bool voting) {
    if (s_raft.peer_count >= RAFT_MAX_NODES) return RF_ERR;
    raft_peer_t *p = &s_raft.peers[s_raft.peer_count++];
    p->node_id    = id;
    p->voting     = voting;
    p->reachable  = RF_TRUE;
    p->next_index = s_raft.log_len + 1;
    p->match_index= 0;
    sigma_strncpy(p->addr, addr, 47);
    sigma_sigma_printf("S [RAFT] Peer added: node=%u addr=%s voting=%d\n",
                 id, addr, voting);
    return RF_OK;
}

/* -- Leader election ------------------------------------------------------- */
void sigma_raft_start_election(void) {
    s_raft.current_term++;
    s_raft.state          = RAFT_CANDIDATE;
    s_raft.voted_for      = s_raft.self_id;
    s_raft.votes_received = 1;  /* vote for self */

    sigma_sigma_printf("S [RAFT] Node %u starting election for term %llu\n",
                 s_raft.self_id, (unsigned long long)s_raft.current_term);

    raft_vote_req_t req = {
        .term           = s_raft.current_term,
        .candidate_id   = s_raft.self_id,
        .last_log_index = s_raft.log_len,
        .last_log_term  = s_raft.log_len > 0
                          ? s_raft.log[s_raft.log_len-1].term : 0
    };

    /* Simulate broadcasting vote requests */
    for (rf_u32 i = 0; i < s_raft.peer_count; i++) {
        if (!s_raft.peers[i].reachable) continue;
        raft_vote_resp_t resp;
        sigma_raft_handle_vote_req(&req, &resp);
        if (resp.granted) {
            s_raft.votes_received++;
            sigma_sigma_printf("S [RAFT] Vote granted by node %u\n",
                         s_raft.peers[i].node_id);
        }
        /* Check majority */
        rf_u32 quorum = (s_raft.peer_count + 2) / 2;
        if (s_raft.votes_received >= quorum && s_raft.state == RAFT_CANDIDATE) {
            s_raft.state = RAFT_LEADER;
            sigma_sigma_printf("S [RAFT] Node %u elected LEADER term=%llu\n",
                         s_raft.self_id, (unsigned long long)s_raft.current_term);
            sigma_raft_send_heartbeats();
            return;
        }
    }
}

void sigma_raft_handle_vote_req(raft_vote_req_t *req, raft_vote_resp_t *resp) {
    resp->term    = s_raft.current_term;
    resp->granted = RF_FALSE;

    /* Grant vote if term is current and we haven't voted yet */
    if (req->term < s_raft.current_term) return;
    if (req->term > s_raft.current_term) {
        s_raft.current_term = req->term;
        s_raft.state        = RAFT_FOLLOWER;
        s_raft.voted_for    = 0;
    }
    if (s_raft.voted_for == 0 || s_raft.voted_for == req->candidate_id) {
        rf_u64 my_last_term = s_raft.log_len > 0
                              ? s_raft.log[s_raft.log_len-1].term : 0;
        /* Candidate log must be at least as up-to-date as ours */
        if (req->last_log_term > my_last_term ||
           (req->last_log_term == my_last_term &&
            req->last_log_index >= s_raft.log_len)) {
            s_raft.voted_for = req->candidate_id;
            resp->granted    = RF_TRUE;
        }
    }
    resp->term = s_raft.current_term;
}

void sigma_raft_handle_vote_resp(rf_u32 from, raft_vote_resp_t *resp) {
    if (resp->term > s_raft.current_term) {
        s_raft.current_term = resp->term;
        s_raft.state        = RAFT_FOLLOWER;
    }
    if (resp->granted && s_raft.state == RAFT_CANDIDATE) {
        s_raft.votes_received++;
        sigma_sigma_printf("S [RAFT] Vote from %u  total %u\n",
                     from, s_raft.votes_received);
    }
}

/* -- AppendEntries (log replication + heartbeat) --------------------------- */
void sigma_raft_handle_append(raft_append_req_t *req, raft_append_resp_t *resp) {
    resp->term    = s_raft.current_term;
    resp->success = RF_FALSE;

    if (req->term < s_raft.current_term) return;
    s_raft.last_heartbeat_ms = 0; /* reset election timer */
    if (req->term > s_raft.current_term) {
        s_raft.current_term = req->term;
        s_raft.state        = RAFT_FOLLOWER;
    }

    /* Append entries */
    if (req->entry_count > 0) {
        for (rf_u32 i = 0; i < req->entry_count && s_raft.log_len < RAFT_LOG_MAX; i++) {
            s_raft.log[s_raft.log_len++] = req->entries[i];
        }
    }

    /* Advance commit index */
    if (req->leader_commit > s_raft.commit_index)
        s_raft.commit_index = req->leader_commit < s_raft.log_len
                              ? req->leader_commit : s_raft.log_len;

    resp->success     = RF_TRUE;
    resp->match_index = s_raft.log_len;
}

void sigma_raft_send_heartbeats(void) {
    if (s_raft.state != RAFT_LEADER) return;
    raft_append_req_t hb = {
        .term          = s_raft.current_term,
        .leader_id     = s_raft.self_id,
        .leader_commit = s_raft.commit_index,
        .entry_count   = 0
    };
    for (rf_u32 i = 0; i < s_raft.peer_count; i++) {
        if (!s_raft.peers[i].reachable) continue;
        raft_append_resp_t resp;
        sigma_raft_handle_append(&hb, &resp);
        if (resp.success) s_raft.peers[i].match_index = resp.match_index;
    }
}

/* -- Client propose -------------------------------------------------------- */
rf_i32 sigma_raft_propose(const char *command) {
    if (s_raft.state != RAFT_LEADER) {
        sigma_sigma_printf("S [RAFT] Propose rejected  not leader\n");
        return RF_ERR;
    }
    if (s_raft.log_len >= RAFT_LOG_MAX) return RF_ERR;
    raft_log_entry_t *e = &s_raft.log[s_raft.log_len++];
    e->term  = s_raft.current_term;
    e->index = s_raft.log_len;
    sigma_strncpy(e->command, command, RAFT_CMD_LEN - 1);
    sigma_sigma_printf("S [RAFT] Proposed: [%llu] %s\n",
                 (unsigned long long)e->index, command);
    sigma_raft_send_heartbeats();
    s_raft.commit_index = s_raft.log_len;
    return RF_OK;
}

/* -- Tick ------------------------------------------------------------------ */
void sigma_raft_tick(rf_u64 elapsed_ms) {
    s_raft.last_heartbeat_ms += elapsed_ms;
    if (s_raft.state != RAFT_LEADER &&
        s_raft.last_heartbeat_ms >= s_raft.election_timeout_ms)
        sigma_raft_start_election();
    if (s_raft.state == RAFT_LEADER && elapsed_ms % 50 == 0)
        sigma_raft_send_heartbeats();
    /* Apply committed entries */
    while (s_raft.last_applied < s_raft.commit_index) {
        s_raft.last_applied++;
        sigma_sigma_printf("S [RAFT] APPLIED: [%llu] %s\n",
                     (unsigned long long)s_raft.last_applied,
                     s_raft.log[s_raft.last_applied-1].command);
    }
}

/* -- Service registry (Consul parity) ------------------------------------- */
rf_i32 sigma_svc_register(const char *name, const char *addr, rf_u32 port) {
    if (s_svc_count >= SIGMA_SVC_MAX) return RF_ERR;
    sigma_service_entry_t *s = &s_services[s_svc_count++];
    sigma_strncpy(s->name, name, 31);
    sigma_strncpy(s->addr, addr, 47);
    s->port    = port;
    s->healthy = RF_TRUE;
    sigma_sigma_printf("S [SVC] REGISTER: %s @ %s:%u\n", name, addr, port);
    return RF_OK;
}

sigma_service_entry_t *sigma_svc_lookup(const char *name) {
    for (rf_u32 i = 0; i < s_svc_count; i++)
        if (sigma_streq(s_services[i].name, name) && s_services[i].healthy)
            return &s_services[i];
    return RF_NULL;
}

void sigma_svc_health_check(void) {
    for (rf_u32 i = 0; i < s_svc_count; i++) {
        /* Simulated health check  toggle based on last_check_ns */
        s_services[i].last_check_ns++;
        s_services[i].healthy = (s_services[i].last_check_ns % 10 != 0);
    }
}

void sigma_svc_list(void) {
    sigma_sigma_printf("\nS SERVICE REGISTRY (%u services)\n", s_svc_count);
    for (rf_u32 i = 0; i < s_svc_count; i++) {
        sigma_sigma_printf("  %-20s %s:%-6u %s\n",
                     s_services[i].name, s_services[i].addr, s_services[i].port,
                     s_services[i].healthy ? "[healthy]" : "[unhealthy]");
    }
}

/* -- Status ---------------------------------------------------------------- */
void sigma_raft_status(void) {
    sigma_sigma_printf("\nS RAFT STATUS\n");
    sigma_sigma_printf("  node=%u  state=%-10s  term=%llu\n",
                 s_raft.self_id, state_str[s_raft.state],
                 (unsigned long long)s_raft.current_term);
    sigma_sigma_printf("  log_len=%llu  commit=%llu  applied=%llu\n",
                 (unsigned long long)s_raft.log_len,
                 (unsigned long long)s_raft.commit_index,
                 (unsigned long long)s_raft.last_applied);
    sigma_sigma_printf("  peers=%u  votes=%u\n", s_raft.peer_count, s_raft.votes_received);
    sigma_svc_list();
}
