#ifndef SIGMA_APCB_H
#define SIGMA_APCB_H

#include <string>
#include <vector>
#include <iostream>

namespace sigma {
namespace kernel {

// Bare-metal implementation of the Agentic Process Control Block
class NativeAPCB {
public:
    int pid;
    std::string intent;
    std::string state;
    std::vector<std::string> error_history;

    NativeAPCB(int process_id, const std::string& process_intent) 
        : pid(process_id), intent(process_intent), state("READY") {}

    void handle_crash(const std::string& traceback) {
        state = "PAUSED_FOR_AI_FIX";
        error_history.push_back(traceback);
        std::cout << "[NativeAPCB] PID " << pid << " crashed. Handoff to AI." << std::endl;
    }

    void resume() {
        state = "RUNNING";
        std::cout << "[NativeAPCB] PID " << pid << " resumed by AI." << std::endl;
    }
};

} // namespace kernel
} // namespace sigma

extern "C" {
    // C-ABI for FFI bindings (allows Python/Rust to call directly without overhead)
    void* apcb_create(int pid, const char* intent) {
        return new sigma::kernel::NativeAPCB(pid, std::string(intent));
    }
    
    void apcb_handle_crash(void* apcb_ptr, const char* traceback) {
        auto* apcb = static_cast<sigma::kernel::NativeAPCB*>(apcb_ptr);
        apcb->handle_crash(std::string(traceback));
    }
    
    void apcb_destroy(void* apcb_ptr) {
        delete static_cast<sigma::kernel::NativeAPCB*>(apcb_ptr);
    }
}

#endif // SIGMA_APCB_H
