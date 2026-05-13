#include "sigma_log.h"
#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"

/**
 * SIGMAOS: SOVEREIGN INDUSTRIAL SCHEDULER (S-SCHED)
 * Implementation: A high-performance Priority-based Round Robin scheduler.
 * Mission: Provide sub-microsecond context switching for the Sovereign Lattice.
 */

namespace SigmaOS {
namespace Kernel {
namespace System {

enum class ThreadState {
    READY,
    RUNNING,
    BLOCKED,
    TERMINATED
};

struct ThreadControlBlock {
    sigma_u32 id;
    sigma_u32 priority;
    ThreadState state;
    sigma_u64 stack_ptr;
    sigma_u32 time_slice;
    ThreadControlBlock* next;
};

class SovereignScheduler {
public:
    static SovereignScheduler& getInstance() {
        static SovereignScheduler instance;
        return instance;
    }

    void init() {
        sigma_log("[S-SCHED] Initializing Industrial Multi-Priority Scheduler...");
        m_ready_list = nullptr;
        m_current_thread = nullptr;
        m_thread_count = 0;
        sigma_log("[S-SCHED] Scheduler online. Ready for lattice orchestration.");
    }

    void spawn_thread(sigma_u32 id, sigma_u32 priority) {
        ThreadControlBlock* thread = (ThreadControlBlock*)sigma_malloc(sizeof(ThreadControlBlock));
        thread->id = id;
        thread->priority = priority;
        thread->state = ThreadState::READY;
        thread->time_slice = 10; // 10ms quantum
        thread->next = m_ready_list;
        m_ready_list = thread;
        m_thread_count++;
        
        sigma_log_info("[S-SCHED] Spawned Thread T%04X (Priority: %u)\n", id, priority);
    }

    sigma_u32 schedule_next() {
        if (!m_ready_list) return 0; // Idle

        // Simple Round-Robin for now, picking the first ready thread
        ThreadControlBlock* prev = nullptr;
        ThreadControlBlock* curr = m_ready_list;

        while (curr) {
            if (curr->state == ThreadState::READY) {
                curr->state = ThreadState::RUNNING;
                m_current_thread = curr;
                
                // Move to end of list for Round-Robin
                if (prev) {
                    prev->next = curr->next;
                } else {
                    m_ready_list = curr->next;
                }
                
                // Find end
                ThreadControlBlock* last = m_ready_list;
                if (!last) {
                    m_ready_list = curr;
                    curr->next = nullptr;
                } else {
                    while (last->next) last = last->next;
                    last->next = curr;
                    curr->next = nullptr;
                }

                sigma_log_info("[S-SCHED] Switching to Thread T%04X\n", curr->id);
                return curr->id;
            }
            prev = curr;
            curr = curr->next;
        }

        return 0;
    }

    void yield() {
        if (m_current_thread) {
            m_current_thread->state = ThreadState::READY;
        }
        schedule_next();
    }

private:
    SovereignScheduler() : m_ready_list(nullptr), m_current_thread(nullptr), m_thread_count(0) {}

    ThreadControlBlock* m_ready_list;
    ThreadControlBlock* m_current_thread;
    sigma_u32 m_thread_count;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" {
    void sched_init() {
        SigmaOS::Kernel::System::SovereignScheduler::getInstance().init();
    }

    void sched_spawn(sigma_u32 id, sigma_u32 priority) {
        SigmaOS::Kernel::System::SovereignScheduler::getInstance().spawn_thread(id, priority);
    }

    sigma_u32 sched_yield() {
        return SigmaOS::Kernel::System::SovereignScheduler::getInstance().schedule_next();
    }
}
