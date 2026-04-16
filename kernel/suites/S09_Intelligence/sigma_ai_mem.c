#include "../../../include/sigma_ai_mem.h"
#include "../../../include/sigma_pmm.h"
#include <string.h>

/* =========================================================================
 * SIGMA OS: INTELLIGENCE SUITE (S09) - LLM MEMORY ALLOCATOR
 * Manages physical RAM partitions for AI agents.
 * ========================================================================= */

static sigma_neural_agent_t agents[MAX_CONCURRENT_AGENTS];

void sigma_ai_memory_init(void) {
    memset(agents, 0, sizeof(agents));
}

int sigma_ai_allocate_swarm(const char* task, uint16_t params) {
    for (int i = 0; i < MAX_CONCURRENT_AGENTS; i++) {
        if (agents[i].state == NEURAL_IDLE) {
            agents[i].agent_id = i + 1;
            agents[i].state = NEURAL_PROCESSING;
            agents[i].parameter_size_billion = params;
            agents[i].current_iq_yield = 100; // Baseline IQ yield
            
            strncpy(agents[i].assigned_task, task, 255);
            
            // Allocate 1MB block via PMM for tensor residency
            void* block = sigma_pmm_allocate_block(); 
            agents[i].tensor_ram_head = (uintptr_t)block;
            
            return agents[i].agent_id;
        }
    }
    return -1; // No agents available
}

void sigma_ai_free_swarm(uint32_t agent_id) {
    if (agent_id > 0 && agent_id <= MAX_CONCURRENT_AGENTS) {
        agents[agent_id - 1].state = NEURAL_IDLE;
        // In reality, we'd tell PMM to free the block
    }
}

sigma_neural_agent_t* sigma_ai_get_telemetry(uint32_t agent_id) {
    if (agent_id > 0 && agent_id <= MAX_CONCURRENT_AGENTS) {
        return &agents[agent_id - 1];
    }
    return NULL;
}
