#ifndef SIGMA_NEURAL_OPT_H
#define SIGMA_NEURAL_OPT_H

#include <stdint.h>

/* SigmaOS Neural Shard Optimizer - Phase 7 Sovereign Intelligence
 * Implements an embedded Q-Learning agent to optimize shard scheduling
 * across the 641-shard Sovereign Lattice.
 */

#define SIGMA_MAX_SHARDS 641
#define SIGMA_Q_STATES 10
#define SIGMA_Q_ACTIONS 5
#define SIGMA_LEARNING_RATE 0.1f
#define SIGMA_DISCOUNT_FACTOR 0.9f

typedef struct {
    float q_table[SIGMA_MAX_SHARDS][SIGMA_Q_STATES][SIGMA_Q_ACTIONS];
    uint32_t current_state[SIGMA_MAX_SHARDS];
    uint32_t execution_counts[SIGMA_MAX_SHARDS];
    float exploration_rate;
} sigma_neural_optimizer_t;

/* Initialize the Neural Optimizer */
void sigma_neural_init(sigma_neural_optimizer_t* opt);

/* Select best execution action for a shard (0=Suspend, 1=LowPriority, 2=Normal, 3=HighPriority, 4=Realtime) */
uint8_t sigma_neural_predict_action(sigma_neural_optimizer_t* opt, uint32_t shard_id, uint32_t current_load_state);

/* Update the Q-table based on execution reward (latency, throughput) */
void sigma_neural_feedback(sigma_neural_optimizer_t* opt, uint32_t shard_id, uint32_t state, uint8_t action, float reward, uint32_t next_state);

/* Decay exploration rate over time (annealing) */
void sigma_neural_decay_exploration(sigma_neural_optimizer_t* opt);

#endif
