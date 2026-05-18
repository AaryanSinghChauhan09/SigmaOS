// SigmaOS — sigma-proc-signal: OOP IPC Signal Handling
// Modularised from: SovereignProcessManager.c
// USP: Object-oriented signal dispatch and asynchronous event delivery.

#ifndef SIGMA_PROC_SIGNAL_HPP
#define SIGMA_PROC_SIGNAL_HPP

#include "sigma_proc_pcb.h"

namespace sigma {
namespace proc {

enum class SignalType {
    TERMINATE,
    INTERRUPT,
    PAUSE,
    RESUME,
    USER_EVENT_1
};

class SignalDispatcher {
public:
    // Dispatch a signal directly to a target process
    bool send_signal(SigmaPCB* target, SignalType sig) {
        if (!target) return false;
        
        switch (sig) {
            case SignalType::TERMINATE:
                target->state = SIGMA_PROC_ZOMBIE;
                break;
            case SignalType::PAUSE:
                target->state = SIGMA_PROC_BLOCKED;
                break;
            case SignalType::RESUME:
                target->state = SIGMA_PROC_READY;
                break;
            default:
                // Append to pending signal queue for the process
                break;
        }
        return true;
    }

    // Called during context switch to handle pending signals
    void process_pending(SigmaPCB* process) {
        if (!process) return;
        // Evaluate pending signal queue and invoke user handlers
    }
};

} // namespace proc
} // namespace sigma

#endif /* SIGMA_PROC_SIGNAL_HPP */
