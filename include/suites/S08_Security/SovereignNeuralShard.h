/* S SIGMAOS: SOVEREIGN NEURAL SHARD HEADER */
#ifndef SOVEREIGN_NEURAL_SHARD_H
#define SOVEREIGN_NEURAL_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

typedef enum { NEURAL_OP_MATMUL, NEURAL_OP_RELOO, NEURAL_OP_CONV2D, NEURAL_OP_SOFTMAX } SigmaNeuralOp_t;

sigma_err_t sigma_neural_dispatch (SigmaNeuralOp_t op, sigma_u32 params);
void        sigma_neural_predict  (const char* context);
void        SovereignNeuralShard_Init   (void);
void        SovereignNeural_Audit       (void);

#endif
