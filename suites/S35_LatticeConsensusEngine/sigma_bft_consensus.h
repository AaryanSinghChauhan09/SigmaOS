#ifndef SIGMA_BFT_CONSENSUS_H
#define SIGMA_BFT_CONSENSUS_H

#include "sigma_libc.h"

/* SigmaOS Lattice Consensus Engine - Phase 7 Sovereign Intelligence
 * Implements Practical Byzantine Fault Tolerance (pBFT) primitives
 * for conflict-free state merging across the global mesh.
 */

#define SIGMA_BFT_MAX_NODES 256
#define SIGMA_BFT_MSG_SIZE 512

typedef enum {
    BFT_STATE_PRE_PREPARE,
    BFT_STATE_PREPARE,
    BFT_STATE_COMMIT,
    BFT_STATE_REPLY
} sigma_bft_state_t;

typedef struct {
    uint32_t view_number;
    uint32_t sequence_number;
    uint8_t digest[32]; // SHA-256 equivalent digest
    uint32_t node_id;
    sigma_bft_state_t current_state;
    uint8_t payload[SIGMA_BFT_MSG_SIZE];
} sigma_bft_message_t;

typedef struct {
    uint32_t total_nodes;
    uint32_t f_faulty_nodes; // Max faulty nodes (N = 3F + 1)
    uint32_t local_node_id;
    uint32_t current_view;
    uint32_t prepare_votes[SIGMA_BFT_MAX_NODES];
    uint32_t commit_votes[SIGMA_BFT_MAX_NODES];
} sigma_bft_context_t;

/* Initialize the BFT Context for a given node */
void sigma_bft_init(sigma_bft_context_t* ctx, uint32_t node_id, uint32_t total_nodes);

/* Process an incoming BFT message from the mesh network */
int sigma_bft_process_message(sigma_bft_context_t* ctx, const sigma_bft_message_t* msg);

/* Generate a PRE-PREPARE message (Primary Node Only) */
void sigma_bft_generate_pre_prepare(sigma_bft_context_t* ctx, sigma_bft_message_t* out_msg, const uint8_t* payload, uint32_t payload_len);

/* Check if the current state has reached consensus quorum (2F + 1) */
int sigma_bft_check_quorum(sigma_bft_context_t* ctx, sigma_bft_state_t state);

#endif
