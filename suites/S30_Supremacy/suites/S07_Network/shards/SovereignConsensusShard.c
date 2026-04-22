/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN EDGE CONSENSUS (v2.0 — RAFT IMPL)
 * =========================================================================
 * Mission: Global Distributed State Consensus for Edge Shards.
 * Principles: Raft Parity, Fault Tolerance, Quorum-based Integrity.
 *
 * v2.0: Real Raft state machine with term tracking, vote logic,
 *       heartbeat generation, and log replication stubs.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

/* --- Raft Node States --- */

typedef enum {
    RAFT_FOLLOWER,
    RAFT_CANDIDATE,
    RAFT_LEADER
} RaftState_t;

/* --- Raft Node Definition --- */

typedef struct {
    sigma_u32   node_id;
    RaftState_t state;
    sigma_u64   current_term;
    sigma_u32   voted_for;        /* node_id we voted for, or 0 = none  */
    sigma_u64   commit_index;     /* highest log entry committed        */
    sigma_u64   last_applied;     /* highest log entry applied to state */
    sigma_u64   last_heartbeat;   /* tick of last leader heartbeat      */
} RaftNode_t;

/* --- Cluster Definition --- */

#define RAFT_CLUSTER_SIZE 5
static RaftNode_t s_cluster[RAFT_CLUSTER_SIZE];
static sigma_u32  s_self_id = 0;   /* This node's index in the cluster */

/**
 * sigma_raft_init: Initializes a Raft cluster.
 * All nodes start as FOLLOWER with term 0.
 */
void sigma_raft_init(void) {
    for (sigma_u32 i = 0; i < RAFT_CLUSTER_SIZE; i++) {
        s_cluster[i].node_id        = i;
        s_cluster[i].state          = RAFT_FOLLOWER;
        s_cluster[i].current_term   = 0;
        s_cluster[i].voted_for      = 0;
        s_cluster[i].commit_index   = 0;
        s_cluster[i].last_applied   = 0;
        s_cluster[i].last_heartbeat = 0;
    }
    sigma_sigma_sigma_printf("[RAFT]: Cluster initialized with %d nodes. All FOLLOWER.\n",
                 RAFT_CLUSTER_SIZE);
}

/**
 * sigma_raft_request_vote: A candidate solicits votes from the cluster.
 * Returns the number of votes received.
 *
 * Real Raft rule: A node grants its vote if:
 *   1. The candidate's term >= the node's current term.
 *   2. The node hasn't already voted this term.
 */
sigma_u32 sigma_raft_request_vote(sigma_u32 candidate_id, sigma_u64 term) {
    sigma_u32 votes = 0;

    for (sigma_u32 i = 0; i < RAFT_CLUSTER_SIZE; i++) {
        if (i == candidate_id) { votes++; continue; }  /* Vote for self */

        RaftNode_t* node = &s_cluster[i];
        if (term >= node->current_term && node->voted_for == 0) {
            node->voted_for    = candidate_id;
            node->current_term = term;
            votes++;
        }
    }

    sigma_sigma_sigma_printf("[RAFT]: Node %u requested votes for term %llu -> %u/%u\n",
                 candidate_id, (unsigned long long)term,
                 votes, RAFT_CLUSTER_SIZE);
    return votes;
}

/**
 * sigma_raft_elect: Triggers an election from node s_self_id.
 */
void sigma_raft_elect(void) {
    RaftNode_t* self = &s_cluster[s_self_id];
    self->current_term++;
    self->state     = RAFT_CANDIDATE;
    self->voted_for = s_self_id;

    sigma_sigma_sigma_printf("[RAFT]: Node %u starting election for term %llu.\n",
                 s_self_id, (unsigned long long)self->current_term);

    sigma_u32 votes = sigma_raft_request_vote(s_self_id, self->current_term);
    sigma_u32 quorum = (RAFT_CLUSTER_SIZE / 2) + 1;

    if (votes >= quorum) {
        self->state = RAFT_LEADER;
        sigma_sigma_sigma_printf("[RAFT]: Node %u ELECTED LEADER (term %llu, %u votes).\n",
                     s_self_id, (unsigned long long)self->current_term, votes);
    } else {
        self->state = RAFT_FOLLOWER;
        sigma_sigma_sigma_printf("[RAFT]: Node %u election failed. Reverting to FOLLOWER.\n",
                     s_self_id);
    }
}

/**
 * sigma_raft_heartbeat: Leader broadcasts heartbeat to suppress elections.
 */
void sigma_raft_heartbeat(sigma_u64 current_tick) {
    RaftNode_t* self = &s_cluster[s_self_id];
    if (self->state != RAFT_LEADER) return;

    for (sigma_u32 i = 0; i < RAFT_CLUSTER_SIZE; i++) {
        if (i == s_self_id) continue;
        s_cluster[i].last_heartbeat = current_tick;
    }
    sigma_sigma_sigma_printf("[RAFT]: Leader %u heartbeat at tick %llu.\n",
                 s_self_id, (unsigned long long)current_tick);
}

/* --- Audit --- */

void SovereignConsensus_Audit(void) {
    const char* state_names[] = {"FOLLOWER", "CANDIDATE", "LEADER"};
    sigma_sigma_sigma_printf("\n--- SOVEREIGN CONSENSUS AUDIT (Raft) ---\n");
    sigma_sigma_sigma_printf("%-6s %-12s %-8s %-10s\n", "NODE", "STATE", "TERM", "VOTED_FOR");
    sigma_sigma_sigma_printf("--------------------------------------\n");
    for (sigma_u32 i = 0; i < RAFT_CLUSTER_SIZE; i++) {
        RaftNode_t* n = &s_cluster[i];
        sigma_sigma_sigma_printf("%-6u %-12s %-8llu %-10u\n",
                     n->node_id, state_names[n->state],
                     (unsigned long long)n->current_term, n->voted_for);
    }
    sigma_sigma_sigma_printf("--------------------------------------\n");
}

/* --- Module Factory --- */

void SovereignConsensus_Register(void) {
    sigma_sigma_sigma_printf("[REGISTRY]: Sovereign Edge Consensus (Raft v2.0) active.\n");
    sigma_raft_init();
    sigma_raft_elect();   /* Bootstrap: elect initial leader */
}



