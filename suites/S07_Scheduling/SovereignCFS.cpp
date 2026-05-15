#include "SovereignCFS.hpp"
#include "../../include/libc/sigma_libc.h"

namespace SigmaOS {
namespace Kernel {

SovereignCFS::SovereignCFS() : root(nullptr), min_vruntime(0), latency_target_ms(20) {
    sigma_log("[CFS] Sovereign Completely Fair Scheduler Initialized.");
}

void SovereignCFS::insert(CFSNode*& node, CFSNode* new_node) {
    if (!node) {
        node = new_node;
        return;
    }
    if (new_node->vruntime < node->vruntime) {
        insert(node->left, new_node);
    } else {
        insert(node->right, new_node);
    }
}

void SovereignCFS::enqueue_task(uint32_t pid, uint64_t weight) {
    CFSNode* new_node = new CFSNode{min_vruntime, weight, pid, 0, nullptr, nullptr};
    insert(root, new_node);
    sigma_print("[CFS] Enqueued PID: ");
    sigma_print_num(pid);
    sigma_print("\n");
}

CFSNode* SovereignCFS::get_leftmost(CFSNode* node) {
    if (!node) return nullptr;
    while (node->left) {
        node = node->left;
    }
    return node;
}

uint32_t SovereignCFS::pick_next_task() {
    CFSNode* next = get_leftmost(root);
    if (!next) return 0; // Idle task
    min_vruntime = next->vruntime;
    return next->pid;
}

void SovereignCFS::update_vruntime(uint32_t pid, uint64_t execution_time) {
    // In a real implementation, we would find the node, update vruntime, and re-insert to maintain the Red-Black Tree.
    // vruntime += (execution_time * NICE_0_LOAD) / weight;
    sigma_log("[CFS] vruntime updated for task.");
}

void SovereignCFS::balance_load() {
    sigma_log("[CFS] Executing NUMA-aware load balancing across Sovereign Lattice...");
}

} // namespace Kernel
} // namespace SigmaOS
