#ifndef SIGMA_SCHEDULER_H
#define SIGMA_SCHEDULER_H

#include "sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

struct lattice_task_t {
    sigma_u32 id;
    sigma_u32 priority;
    sigma_u32 silicon_affinity; // CPU core preference
    void (*entry_point)();
};

class SovereignScheduler {
public:
    static SovereignScheduler& getInstance();

    void init();
    void schedule(void (*task)(), sigma_u32 priority);

private:
    SovereignScheduler() : active_tasks(0), initialized(false) {}
    sigma_u32 active_tasks;
    bool initialized;
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
