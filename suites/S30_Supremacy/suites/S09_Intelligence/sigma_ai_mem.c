#include "../../../../include/SovereignLibC.h"
#include "../../../../include/sigma_ai_mem.h"
#include "../../../../include/hal/sigma_pmm.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* =========================================================================
 * SIGMA OS: INTELLIGENCE SUITE (S09) - AI MEMORY ALLOCATOR
 * Manages direct RAM pinning for neural agent tensors at OS level.
 * ========================================================================= */

static sigma_neural_agent_t agents[MAX_CONCURRENT_AGENTS];

void sigma_ai_memory_init(void) {
    sigma_sigma_memset(agents, 0, sizeof(agents));
    sigma_sigma_printf("[AI] Neural Agent Memory Matrix online. Ready for swarm deployment.\n");
}

int sigma_ai_allocate_swarm(const char* task, uint16_t params) {
    for (int i = 0; i < MAX_CONCURRENT_AGENTS; i++) {
        if (agents[i].state == NEURAL_IDLE) {
            agents[i].agent_id             = i + 1;
            agents[i].state                = NEURAL_PROCESSING;
            agents[i].parameter_size_billion = params;
            agents[i].current_iq_yield     = 98; // Baseline

            strncpy(agents[i].assigned_task, task, 255);

            // Pin physical memory block for the tensor weight store
            void* block = sigma_pmm_allocate_block();
            agents[i].tensor_ram_head = (uint32_t)(uintptr_t)block;

            sigma_sigma_printf("[AI] Agent %u spawned → task='%s' params=%uB\n",
                   agents[i].agent_id, task, params);
            return agents[i].agent_id;
        }
    }
    sigma_sigma_printf("[AI] WARN: Max concurrent agents reached (%d).\n", MAX_CONCURRENT_AGENTS);
    return -1;
}

void sigma_ai_free_swarm(uint32_t agent_id) {
    if (agent_id == 0 || agent_id > MAX_CONCURRENT_AGENTS) return;

    sigma_neural_agent_t* a = &agents[agent_id - 1];
    if (a->state != NEURAL_IDLE) {
        a->state = NEURAL_IDLE;
        a->tensor_ram_head = 0;
        sigma_sigma_printf("[AI] Agent %u deallocated from neural memory pool.\n", agent_id);
    }
}

sigma_neural_agent_t* sigma_ai_get_telemetry(uint32_t agent_id) {
    if (agent_id == 0 || agent_id > MAX_CONCURRENT_AGENTS) return 0;
    return &agents[agent_id - 1];
}
