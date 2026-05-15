#ifndef SIGMA_SCHEDULER_H
#define SIGMA_SCHEDULER_H

#include "../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

struct SovereignTask {
    sigma_u32 id;
    sigma_u64 vruntime; // Virtual Runtime for CFS fairness
    sigma_u32 priority;
    bool active;
};

class SovereignScheduler : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignScheduler> {
    friend class SigmaOS::SigmaSingleton<SovereignScheduler>;
public:
    const char* type_name() const noexcept override { return "SovereignScheduler"; }
    
    void init();
    void schedule(void (*task)(), sigma_u32 priority);

private:
    SovereignScheduler() : m_task_count(0), m_min_vruntime(0) {}
    SovereignTask m_tasks[1024];
    sigma_u32 m_task_count;
    sigma_u64 m_min_vruntime;
};

} // namespace Orchestration
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void scheduler_init(void);
void scheduler_push(void (*task)(), sigma_u32 priority);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SCHEDULER_H */
