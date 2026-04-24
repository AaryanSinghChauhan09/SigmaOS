#include "sigma_libc.h"
#include "sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Consensus-Driven Module Updater
// Distributed nodes vote on module updates before applying
// ---------------------------------------------------------

#define MAX_NODES         32
#define MAX_PROPOSALS     16
#define NODE_ID_LEN       16
#define QUORUM_PERCENT    66  // >66% vote = update accepted

typedef struct {
    uint8_t  node_id[NODE_ID_LEN];
    uint8_t  public_key[32]; // Ed25519 for vote signing
    uint8_t  alive;
} consensus_node_t;

typedef enum {
    PROPOSAL_PENDING,
    PROPOSAL_ACCEPTED,
    PROPOSAL_REJECTED,
    PROPOSAL_APPLIED
} proposal_state_t;

typedef struct {
    uint32_t         proposal_id;
    char             module_name[32];
    uint32_t         new_version;     // Packed semantic version
    uint8_t          module_hash[32]; // SHA-256 of new module binary
    proposal_state_t state;
    uint32_t         votes_for;
    uint32_t         votes_against;
    uint32_t         total_nodes;
} update_proposal_t;

static consensus_node_t nodes[MAX_NODES];
static uint32_t node_count = 0;
static update_proposal_t proposals[MAX_PROPOSALS];
static uint32_t proposal_count = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);
extern int capsule_hotswap(uint32_t old_id, uint32_t new_id);

// Register a node in the consensus cluster
int consensus_register_node(const uint8_t* node_id, const uint8_t* public_key) {
    if (node_count >= MAX_NODES) return -1;
    consensus_node_t* n = &nodes[node_count++];
    memcpy(n->node_id, node_id, NODE_ID_LEN);
    memcpy(n->public_key, public_key, 32);
    n->alive = 1;
    return 0;
}

// Propose a module update (initiated by any node)
int consensus_propose_update(const char* module_name, uint32_t new_version,
                              const uint8_t module_hash[32]) {
    if (proposal_count >= MAX_PROPOSALS) return -1;
    update_proposal_t* p = &proposals[proposal_count];
    p->proposal_id   = proposal_count++;
    p->new_version   = new_version;
    p->state         = PROPOSAL_PENDING;
    p->votes_for     = 0;
    p->votes_against = 0;
    p->total_nodes   = node_count;
    strncpy(p->module_name, module_name, 31);
    memcpy(p->module_hash, module_hash, 32);

    audit_chain_append(0, 1, "CONSENSUS_UPDATE_PROPOSED");
    return p->proposal_id;
}

// Cast a vote for a proposal (called when a node receives and validates a proposal)
int consensus_vote(uint32_t proposal_id, const uint8_t* node_id, uint8_t approve) {
    if (proposal_id >= proposal_count) return -1;
    update_proposal_t* p = &proposals[proposal_id];
    if (p->state != PROPOSAL_PENDING) return -2;

    // In production: verify vote is signed by node's Ed25519 key
    if (approve) p->votes_for++;
    else         p->votes_against++;

    // Check for quorum
    uint32_t total_votes = p->votes_for + p->votes_against;
    if (total_votes == p->total_nodes || p->votes_for * 100 / p->total_nodes >= QUORUM_PERCENT) {
        if (p->votes_for * 100 / p->total_nodes >= QUORUM_PERCENT) {
            p->state = PROPOSAL_ACCEPTED;
            audit_chain_append(0, 1, "CONSENSUS_UPDATE_ACCEPTED");
            // capsule_hotswap(old_capsule_id, new_capsule_id);
            p->state = PROPOSAL_APPLIED;
        } else {
            p->state = PROPOSAL_REJECTED;
            audit_chain_append(0, 2, "CONSENSUS_UPDATE_REJECTED");
        }
    }
    return 0;
}
