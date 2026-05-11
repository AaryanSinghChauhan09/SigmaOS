#include "sigma_libc.h"
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Q-Learning AI Scheduler Algorithm
// USP: Reinforcement learning determines optimal CPU timeslices
// based on IPC frequency, page faults, and cache misses.
// ---------------------------------------------------------

#define MAX_PROCESSES   128
#define Q_STATES        4   // Low, Medium, High, Realtime Workload
#define Q_ACTIONS       3   // Decrease Slice, Keep Slice, Increase Slice
#define ALPHA           0.1f // Learning Rate
#define GAMMA           0.9f // Discount Factor
#define REWARD_MAX      100.0f

typedef struct {
    uint32_t pid;
    float    q_table[Q_STATES][Q_ACTIONS];
    uint8_t  current_state;
    uint8_t  last_action;
    float    last_reward;
    uint32_t timeslice_ms;
} ai_proc_state_t;

static ai_proc_state_t ai_state_table[MAX_PROCESSES];

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Heuristic to determine the current state of a process
static uint8_t determine_state(uint32_t page_faults, uint32_t ipc_calls) {
    if (page_faults > 100) return 3; // High memory pressure (Realtime needed)
    if (ipc_calls > 500)   return 2; // High IPC (High workload)
    if (ipc_calls > 50)    return 1; // Medium workload
    return 0; // Low/Idle
}

// Compute the reward based on how smoothly the process ran
static float compute_reward(uint32_t cpu_stalls, uint32_t ui_latency_ms) {
    float reward = REWARD_MAX - (cpu_stalls * 2.0f) - (ui_latency_ms * 5.0f);
    if (reward < -REWARD_MAX) return -REWARD_MAX;
    return reward;
}

// Get the best action from the Q-table (Exploitation)
static uint8_t get_best_action(ai_proc_state_t* p, uint8_t state) {
    uint8_t best_action = 1; // Default to 'Keep Slice'
    float max_q = p->q_table[state][1];

    for (uint8_t a = 0; a < Q_ACTIONS; a++) {
        if (p->q_table[state][a] > max_q) {
            max_q = p->q_table[state][a];
            best_action = a;
        }
    }
    return best_action;
}

// Update the Q-table based on the reward received
static void update_q_table(ai_proc_state_t* p, uint8_t new_state, float reward) {
    float max_next_q = p->q_table[new_state][get_best_action(p, new_state)];
    float old_q = p->q_table[p->current_state][p->last_action];
    
    // Q-Learning Equation: Q(s,a) = Q(s,a) + alpha * (reward + gamma * max(Q(s',a')) - Q(s,a))
    p->q_table[p->current_state][p->last_action] = old_q + ALPHA * (reward + GAMMA * max_next_q - old_q);
}

// Main Scheduler Policy Hook (Replaces static priority scheduling)
uint32_t ai_scheduler_pick_next(uint32_t* runqueue, uint32_t queue_len, 
                                uint32_t* out_timeslice_ms,
                                uint32_t current_cpu_stalls, uint32_t current_ui_latency) {
    if (queue_len == 0) return UINT32_MAX;

    // We will pick the first ready process, but we use AI to determine its timeslice
    uint32_t next_pid = runqueue[0];
    
    // Find AI state
    ai_proc_state_t* p = SIGMA_NULL;
    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (ai_state_table[i].pid == next_pid) { p = &ai_state_table[i]; break; }
    }
    if (!p) {
        // Init new process
        p = &ai_state_table[0]; // Simplified for mock
        p->pid = next_pid;
        p->timeslice_ms = 10;
        p->current_state = 0;
    }

    // 1. Observe Reward from last run
    float reward = compute_reward(current_cpu_stalls, current_ui_latency);
    
    // 2. Observe New State
    uint8_t new_state = determine_state(0 /* mock page faults */, 0 /* mock ipc */);
    
    // 3. Update Q-Table
    update_q_table(p, new_state, reward);
    
    // 4. Decide Next Action (Decrease=0, Keep=1, Increase=2)
    uint8_t action = get_best_action(p, new_state);
    if (action == 0 && p->timeslice_ms > 2) p->timeslice_ms -= 2;
    if (action == 2 && p->timeslice_ms < 50) p->timeslice_ms += 2;
    
    p->current_state = new_state;
    p->last_action = action;
    
    *out_timeslice_ms = p->timeslice_ms;
    return next_pid;
}
