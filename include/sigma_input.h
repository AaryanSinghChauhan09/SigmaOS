/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN INPUT INTERFACE (SII)
 * =========================================================================
 * Mission: Zero-latency, interrupt-driven input orchestration.
 * =========================================================================
 */

#ifndef SIGMA_INPUT_H
#define SIGMA_INPUT_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_KEY_PRESS,
    SIGMA_KEY_RELEASE
} sigma_key_state_t;

typedef struct {
    sigma_u8 scancode;
    sigma_u8 char_code;
    sigma_key_state_t state;
} sigma_key_event_t;

/* --- Input Primitives --- */
void input_init(void);
void input_push_event(sigma_key_event_t* event);
bool input_pop_event(sigma_key_event_t* out_event);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_INPUT_H */
