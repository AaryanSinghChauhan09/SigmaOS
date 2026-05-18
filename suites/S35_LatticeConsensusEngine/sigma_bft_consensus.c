#include "sigma_bft_consensus.h"

// Simple internal memory copy to avoid libc dependencies
static void sigma_internal_sigma_memcpy(void* dest, const void* src, uint32_t len) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    while(len--) { *d++ = *s++; }
}

// Simple internal zero memory
static void sigma_internal_memzero(void* dest, uint32_t len) {
    uint8_t* d = (uint8_t*)dest;
    while(len--) { *d++ = 0; }
}

void sigma_bft_init(sigma_bft_context_t* ctx, uint32_t node_id, uint32_t total_nodes) {
    if (!ctx || total_nodes == 0) return;
    
    ctx->total_nodes = total_nodes > SIGMA_BFT_MAX_NODES ? SIGMA_BFT_MAX_NODES : total_nodes;
    ctx->f_faulty_nodes = (ctx->total_nodes - 1) / 3;
    ctx->local_node_id = node_id;
    ctx->current_view = 0;
    
    sigma_internal_memzero(ctx->prepare_votes, sizeof(ctx->prepare_votes));
    sigma_internal_memzero(ctx->commit_votes, sizeof(ctx->commit_votes));
}

int sigma_bft_process_message(sigma_bft_context_t* ctx, const sigma_bft_message_t* msg) {
    if (!ctx || !msg || msg->node_id >= ctx->total_nodes) return 0;
    if (msg->view_number < ctx->current_view) return 0; // Stale view
    
    switch (msg->current_state) {
        case BFT_STATE_PRE_PREPARE:
            // Acknowledge primary proposal and broadcast PREPARE
            ctx->prepare_votes[ctx->local_node_id] = 1; 
            break;
        case BFT_STATE_PREPARE:
            // Record votes from replica
            ctx->prepare_votes[msg->node_id] = 1;
            break;
        case BFT_STATE_COMMIT:
            // Record commit vote
            ctx->commit_votes[msg->node_id] = 1;
            break;
        case BFT_STATE_REPLY:
            // Execution finality
            break;
    }
    
    return 1;
}

void sigma_bft_generate_pre_prepare(sigma_bft_context_t* ctx, sigma_bft_message_t* out_msg, const uint8_t* payload, uint32_t payload_len) {
    if (!ctx || !out_msg || !payload) return;
    
    out_msg->view_number = ctx->current_view;
    out_msg->sequence_number = 1; // Basic seq for Phase 7 alpha
    out_msg->node_id = ctx->local_node_id;
    out_msg->current_state = BFT_STATE_PRE_PREPARE;
    
    uint32_t copy_len = payload_len > SIGMA_BFT_MSG_SIZE ? SIGMA_BFT_MSG_SIZE : payload_len;
    sigma_internal_sigma_memcpy(out_msg->payload, payload, copy_len);
    
    // Simplistic digest generation (XOR fold)
    sigma_internal_memzero(out_msg->digest, 32);
    for (uint32_t i = 0; i < copy_len; i++) {
        out_msg->digest[i % 32] ^= payload[i];
    }
}

int sigma_bft_check_quorum(sigma_bft_context_t* ctx, sigma_bft_state_t state) {
    if (!ctx) return 0;
    
    uint32_t required_votes = 2 * ctx->f_faulty_nodes + 1;
    uint32_t current_votes = 0;
    
    uint32_t* vote_array = 0;
    if (state == BFT_STATE_PREPARE) vote_array = ctx->prepare_votes;
    else if (state == BFT_STATE_COMMIT) vote_array = ctx->commit_votes;
    else return 0;
    
    for (uint32_t i = 0; i < ctx->total_nodes; i++) {
        if (vote_array[i] == 1) current_votes++;
    }
    
    return current_votes >= required_votes ? 1 : 0;
}
