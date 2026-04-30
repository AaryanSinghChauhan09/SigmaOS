#ifndef SIGMA_NEURAL_H
#define SIGMA_NEURAL_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t ops_per_sec;
    bool npu_engaged;
} sigma_neural_state_t;

/* --- Neural Primitives --- */
void      neural_init(void);
void      neural_set_acceleration(sigma_u32 type);
void      neural_infer_shard(sigma_u32 model_id, const void* input, void* output);
void      neural_morph_ui(sigma_u32 widget_id, sigma_u32 cognitive_load);
sigma_u64 neural_get_inference_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NEURAL_H */
