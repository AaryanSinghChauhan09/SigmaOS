// SigmaOS — sigma-power-scheduler: Mobile Workload Throttling
// Module: sigma-power-scheduler
// USP: Plugs into the kernel scheduler to throttle non-essential processes 
//      on mobile/IoT devices to preserve thermal headroom and battery.

#ifndef SIGMA_POWER_SCHEDULER_HPP
#define SIGMA_POWER_SCHEDULER_HPP

#include "../S01_Genesis/sigma_proc_pcb.h"

namespace sigma {
namespace power {

class PowerAwareScheduler {
public:
    static void throttle_background_tasks(SigmaPCB* pcb, bool is_critical_battery) {
        if (!pcb) return;

        // If battery is critical and process is a background task, halt it entirely
        if (is_critical_battery && pcb->state == SIGMA_PROC_READY) {
            pcb->state = SIGMA_PROC_BLOCKED;
        }
    }
};

} // namespace power
} // namespace sigma

#endif /* SIGMA_POWER_SCHEDULER_HPP */
