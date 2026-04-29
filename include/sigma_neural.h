/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NEURAL ENGINE (S-NEURAL)
 * =========================================================================
 * Mission: Silicon-native AI acceleration and neural shard orchestration.
 * =========================================================================
 */

#ifndef SIGMA_NEURAL_H
#define SIGMA_NEURAL_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t model_id;
    uint32_t layer_count;
    uint32_t silicon_accel_type; // 0: CPU, 1: GPU, 2: NPU/Tensor
} sigma_neural_context_t;

/* --- Neural Primitives --- */
void neural_init(void);
void neural_infer_shard(uint32_t model_id, const void* input, void* output);
void neural_set_acceleration(uint32_t type);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NEURAL_H */
