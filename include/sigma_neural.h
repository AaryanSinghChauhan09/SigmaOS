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
void neural_init(void);
void neural_morph_ui(uint32_t widget_id, uint32_t cognitive_load);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NEURAL_H */
