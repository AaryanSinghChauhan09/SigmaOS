// SigmaOS Sovereign Process Descriptor
// OOP model for process lifecycle management — zero dependency
#ifndef SIGMA_PROCESS_HPP
#define SIGMA_PROCESS_HPP

#include "libc/sigma_libc.h"

namespace sigma {
namespace proc {

enum class ProcessState : sigma_u8 {
    CREATED = 0,
    RUNNING = 1,
    BLOCKED = 2,
    ZOMBIE  = 3,
    DEAD    = 4
};

// Abstract: every process type implements this
class IProcess {
public:
    virtual ~IProcess() {}
    virtual sigma_u32 get_pid() const = 0;
    virtual ProcessState get_state() const = 0;
    virtual void run() = 0;
    virtual void block() = 0;
    virtual void terminate() = 0;
};

// Concrete: Kernel Process
class KernelProcess : public IProcess {
private:
    sigma_u32 pid;
    ProcessState state;

public:
    KernelProcess(sigma_u32 id) : pid(id), state(ProcessState::CREATED) {}

    sigma_u32 get_pid() const override { return pid; }
    ProcessState get_state() const override { return state; }

    void run() override {
        state = ProcessState::RUNNING;
        sigma_kprint("[SigmaProc] Kernel process running on bare silicon.\n");
    }

    void block() override {
        state = ProcessState::BLOCKED;
        sigma_kprint("[SigmaProc] Kernel process blocked — waiting for IRQ.\n");
    }

    void terminate() override {
        state = ProcessState::DEAD;
        sigma_kprint("[SigmaProc] Kernel process terminated. Resources freed.\n");
    }
};

// User-Defined: custom process functor for automation hooks
class AutomationProcess : public IProcess {
private:
    sigma_u32 pid;
    ProcessState state;
    const char* hook_name;

public:
    AutomationProcess(sigma_u32 id, const char* name)
        : pid(id), state(ProcessState::CREATED), hook_name(name) {}

    sigma_u32 get_pid() const override { return pid; }
    ProcessState get_state() const override { return state; }

    void run() override {
        state = ProcessState::RUNNING;
        sigma_kprint("[SigmaProc] Automation hook running: ");
        sigma_kprint(hook_name);
        sigma_kprint("\n");
    }

    void block() override { state = ProcessState::BLOCKED; }

    void terminate() override {
        state = ProcessState::DEAD;
        sigma_kprint("[SigmaProc] Automation process completed.\n");
    }
};

} // namespace proc
} // namespace sigma

#endif // SIGMA_PROCESS_HPP
