#include "sigma_neural_opt.h"

// Simple LCG for pseudo-random exploration (zero-dependency)
static uint32_t sigma_rand_seed = 123456789;
static float sigma_random_float() {
    sigma_rand_seed = (1103515245 * sigma_rand_seed + 12345) % 2147483648;
    return (float)sigma_rand_seed / 2147483648.0f;
}

void sigma_neural_init(sigma_neural_optimizer_t* opt) {
    if (!opt) return;
    opt->exploration_rate = 1.0f; // Start with 100% exploration
    for (uint32_t i = 0; i < SIGMA_MAX_SHARDS; i++) {
        opt->current_state[i] = 0;
        opt->execution_counts[i] = 0;
        for (uint32_t s = 0; s < SIGMA_Q_STATES; s++) {
            for (uint32_t a = 0; a < SIGMA_Q_ACTIONS; a++) {
                opt->q_table[i][s][a] = 0.0f;
            }
        }
    }
}

uint8_t sigma_neural_predict_action(sigma_neural_optimizer_t* opt, uint32_t shard_id, uint32_t current_load_state) {
    if (!opt || shard_id >= SIGMA_MAX_SHARDS || current_load_state >= SIGMA_Q_STATES) return 2; // Default to Normal Priority

    // Epsilon-Greedy Exploration
    if (sigma_random_float() < opt->exploration_rate) {
        return (uint8_t)(sigma_random_float() * SIGMA_Q_ACTIONS);
    }

    // Exploitation: Find action with max Q-value
    uint8_t best_action = 0;
    float max_q = opt->q_table[shard_id][current_load_state][0];
    
    for (uint8_t a = 1; a < SIGMA_Q_ACTIONS; a++) {
        if (opt->q_table[shard_id][current_load_state][a] > max_q) {
            max_q = opt->q_table[shard_id][current_load_state][a];
            best_action = a;
        }
    }
    
    return best_action;
}

void sigma_neural_feedback(sigma_neural_optimizer_t* opt, uint32_t shard_id, uint32_t state, uint8_t action, float reward, uint32_t next_state) {
    if (!opt || shard_id >= SIGMA_MAX_SHARDS || state >= SIGMA_Q_STATES || next_state >= SIGMA_Q_STATES || action >= SIGMA_Q_ACTIONS) return;

    // Find max Q-value for the next state
    float max_next_q = opt->q_table[shard_id][next_state][0];
    for (uint8_t a = 1; a < SIGMA_Q_ACTIONS; a++) {
        if (opt->q_table[shard_id][next_state][a] > max_next_q) {
            max_next_q = opt->q_table[shard_id][next_state][a];
        }
    }

    // Bellman Equation update
    float current_q = opt->q_table[shard_id][state][action];
    opt->q_table[shard_id][state][action] = current_q + SIGMA_LEARNING_RATE * (reward + SIGMA_DISCOUNT_FACTOR * max_next_q - current_q);
    
    opt->execution_counts[shard_id]++;
}

void sigma_neural_decay_exploration(sigma_neural_optimizer_t* opt) {
    if (!opt) return;
    opt->exploration_rate *= 0.995f; // Annealing schedule
    if (opt->exploration_rate < 0.01f) {
        opt->exploration_rate = 0.01f; // Minimum 1% exploration to adapt to shifts
    }
}
