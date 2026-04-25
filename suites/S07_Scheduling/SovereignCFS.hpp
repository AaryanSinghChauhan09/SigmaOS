#pragma once
#include <stdint.h>

namespace SigmaOS {
namespace Kernel {

// Track 1: Kernel Maturity - Efficient Scheduling
// Completely Fair Scheduler (CFS) Node
struct CFSNode {
    uint64_t vruntime;   // Virtual runtime
    uint64_t weight;     // Process priority/weight
    uint32_t pid;
    uint32_t state;      // 0: READY, 1: RUNNING, 2: SLEEPING
    CFSNode* left;
    CFSNode* right;
};

class SovereignCFS {
private:
    CFSNode* root;
    uint64_t min_vruntime;
    uint64_t latency_target_ms;

    void insert(CFSNode*& node, CFSNode* new_node);
    CFSNode* get_leftmost(CFSNode* node);
    void remove(CFSNode*& node, uint32_t pid);

public:
    SovereignCFS();
    
    // Core CFS Operations
    void enqueue_task(uint32_t pid, uint64_t weight);
    uint32_t pick_next_task();
    void update_vruntime(uint32_t pid, uint64_t execution_time);
    void balance_load(); // NUMA-aware load balancing placeholder
};

} // namespace Kernel
} // namespace SigmaOS
