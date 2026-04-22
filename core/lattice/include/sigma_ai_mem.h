#ifndef SIGMA_AI_MEM_H
#define SIGMA_AI_MEM_H

#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * SIGMA OS: INTELLIGENCE SUITE (S09) - LLM MEMORY ALLOCATOR
 * Direct Silicon-to-Agent neural weight caching, bypassing standard RAM blobs.
 * ========================================================================= */

#define NEURAL_BLOCK_SIZE     (1024 * 1024) // 1MB dedicated vector caching blocks
#define MAX_CONCURRENT_AGENTS 16

typedef enum {
    NEURAL_IDLE = 0,
    NEURAL_PROCESSING,
    NEURAL_AWAITING_GPU,
    NEURAL_HALTED
} ai_agent_state_t;

// Context handler representing a single active Neural AI Node running on OS
typedef struct {
    uint32_t agent_id;
    ai_agent_state_t state;
    
    // Direct pointer to pinned physical memory containing tensor vectors
    uint32_t tensor_ram_head; 
    
    // Live calculated statistics 
    uint16_t parameter_size_billion;
    uint32_t current_iq_yield; 
    
    char assigned_task[256];
} __attribute__((packed)) sigma_neural_agent_t;


void sigma_ai_memory_init(void);
int sigma_ai_allocate_swarm(const char* task, uint16_t params);
void sigma_ai_free_swarm(uint32_t agent_id);
sigma_neural_agent_t* sigma_ai_get_telemetry(uint32_t agent_id);

#endif
