#ifndef SIGMA_TRANSPILER_H
#define SIGMA_TRANSPILER_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t hardware_id;
    char target_architecture[16];
    bool transpilation_active;
} sigma_transpiler_state_t;

/* --- Transpiler Primitives --- */
void transpiler_init(void);
void transpiler_auto_map(uint32_t device_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TRANSPILER_H */
