#ifndef SIGMA_AI_H
#define SIGMA_AI_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* 
 * Sigma Intelligence Engine (SIE) Core API 
 * This provides the syscall interface and structs for zero-latency AI interactions
 */

typedef struct {
    const char* prompt;
    char* response_buffer;
    sigma_size_t buffer_size;
    sigma_u32 max_tokens;
    float temperature;
} sigma_inference_req_t;

/* Syscall stub to interact with the SIE daemon natively */
extern sigma_status sys_infer(sigma_inference_req_t* req);

/* Semantic Search Vectors */
typedef struct {
    float* vector;
    sigma_size_t dimensions;
} sigma_vector_t;

extern sigma_status sys_embed_text(const char* text, sigma_vector_t* out_vector);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_AI_H
