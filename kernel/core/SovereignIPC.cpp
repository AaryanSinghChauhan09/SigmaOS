/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN IPC SUBSYSTEM (v1.0)
 * =========================================================================
 * Ring buffer message queues, shared memory segments with ref counting,
 * and signal delivery between processes.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_ipc_manager.h"

namespace SigmaOS {
namespace Kernel {

class SovereignIPC {
public:
    static SovereignIPC& getInstance() {
        static SovereignIPC instance;
        return instance;
    }

    void init() {
        m_queue_count = 0;
        m_shm_count = 0;
        m_total_messages = 0;
        for (sigma_u32 i = 0; i < IPC_MAX_QUEUES; i++) {
            m_queues[i].queue_id = 0;
            m_queues[i].count = 0;
        }
        for (sigma_u32 i = 0; i < IPC_MAX_SHM_SEGMENTS; i++) {
            m_shm[i].shm_id = 0;
            m_shm[i].ref_count = 0;
        }
        sigma_log("[IPC] Sovereign IPC Subsystem initialized.");
        sigma_log("[IPC] Message queues: 128 max | SHM segments: 64 max | Signals: active");
    }

    sigma_u32 createQueue(sigma_u32 owner_pid) {
        if (m_queue_count >= IPC_MAX_QUEUES) return 0;

        sigma_u32 id = ++m_queue_count;
        sigma_msg_queue_t& q = m_queues[id - 1];
        q.queue_id = id;
        q.owner_pid = owner_pid;
        q.head = 0;
        q.tail = 0;
        q.count = 0;

        sigma_log_info("[IPC] Queue %u created (owner PID %u)\n", id, owner_pid);
        return id;
    }

    int destroyQueue(sigma_u32 queue_id) {
        sigma_msg_queue_t* q = findQueue(queue_id);
        if (!q) return K_ERR_NOTFOUND;

        sigma_log_info("[IPC] Queue %u destroyed (%u messages discarded)\n",
                       queue_id, q->count);
        q->queue_id = 0;
        q->count = 0;
        return K_OK;
    }

    int send(sigma_u32 queue_id, const sigma_msg_t* msg) {
        sigma_msg_queue_t* q = findQueue(queue_id);
        if (!q) return K_ERR_NOTFOUND;
        if (q->count >= IPC_QUEUE_CAPACITY) return K_ERR_BUSY;

        q->messages[q->tail] = *msg;
        q->tail = (q->tail + 1) % IPC_QUEUE_CAPACITY;
        q->count++;
        m_total_messages++;
        return K_OK;
    }

    int receive(sigma_u32 queue_id, sigma_msg_t* out_msg) {
        sigma_msg_queue_t* q = findQueue(queue_id);
        if (!q) return K_ERR_NOTFOUND;
        if (q->count == 0) return K_ERR_NOTFOUND;

        *out_msg = q->messages[q->head];
        q->head = (q->head + 1) % IPC_QUEUE_CAPACITY;
        q->count--;
        return K_OK;
    }

    sigma_u32 queueCount(sigma_u32 queue_id) {
        sigma_msg_queue_t* q = findQueue(queue_id);
        return q ? q->count : 0;
    }

    sigma_u32 shmCreate(const char* name, sigma_usize size, sigma_u32 owner_pid) {
        if (m_shm_count >= IPC_MAX_SHM_SEGMENTS) return 0;

        sigma_u32 id = ++m_shm_count;
        sigma_shm_t& s = m_shm[id - 1];
        s.shm_id = id;
        sigma_strncpy(s.name, name, IPC_SHM_NAME_LEN);
        s.size = ALIGN_UP(size, PAGE_SIZE);
        s.phys_base = 0x40000000ULL + (sigma_u64)(id - 1) * s.size;
        s.ref_count = 1;
        s.owner_pid = owner_pid;
        s.writable = SIGMA_TRUE;

        sigma_log_info("[IPC] SHM '%s' created: %lluKB @ phys 0x%llx (owner PID %u)\n",
                       name, (unsigned long long)(s.size / 1024),
                       (unsigned long long)s.phys_base, owner_pid);
        return id;
    }

    int shmAttach(sigma_u32 shm_id, sigma_u32 pid) {
        sigma_shm_t* s = findShm(shm_id);
        if (!s) return K_ERR_NOTFOUND;
        s->ref_count++;
        sigma_log_info("[IPC] SHM %u attached by PID %u (refcount=%u)\n",
                       shm_id, pid, s->ref_count);
        return K_OK;
    }

    int shmDetach(sigma_u32 shm_id, sigma_u32 pid) {
        sigma_shm_t* s = findShm(shm_id);
        if (!s || s->ref_count == 0) return K_ERR_NOTFOUND;
        s->ref_count--;
        sigma_log_info("[IPC] SHM %u detached by PID %u (refcount=%u)\n",
                       shm_id, pid, s->ref_count);
        return K_OK;
    }

    int shmDestroy(sigma_u32 shm_id) {
        sigma_shm_t* s = findShm(shm_id);
        if (!s) return K_ERR_NOTFOUND;
        if (s->ref_count > 0) {
            sigma_log("[IPC] WARNING: Destroying SHM with active references.");
        }
        sigma_log_info("[IPC] SHM '%s' destroyed.\n", s->name);
        s->shm_id = 0;
        return K_OK;
    }

    int signalSend(sigma_u32 target_pid, sigma_signal_t sig) {
        const char* sig_name = "UNKNOWN";
        switch (sig) {
            case SIGMA_SIG_TERM: sig_name = "SIGTERM"; break;
            case SIGMA_SIG_KILL: sig_name = "SIGKILL"; break;
            case SIGMA_SIG_STOP: sig_name = "SIGSTOP"; break;
            case SIGMA_SIG_CONT: sig_name = "SIGCONT"; break;
            case SIGMA_SIG_USR1: sig_name = "SIGUSR1"; break;
            case SIGMA_SIG_USR2: sig_name = "SIGUSR2"; break;
            case SIGMA_SIG_CHLD: sig_name = "SIGCHLD"; break;
            case SIGMA_SIG_PIPE: sig_name = "SIGPIPE"; break;
        }
        sigma_log_info("[IPC] Signal %s → PID %u\n", sig_name, target_pid);
        return K_OK;
    }

    void printStatus() {
        sigma_log("\n--- IPC SUBSYSTEM STATUS ---");
        sigma_log_info("| Message queues : %u active\n", m_queue_count);
        sigma_log_info("| SHM segments   : %u active\n", m_shm_count);
        sigma_log_info("| Total messages : %llu sent\n", (unsigned long long)m_total_messages);

        for (sigma_u32 i = 0; i < m_queue_count; i++) {
            if (m_queues[i].queue_id != 0) {
                sigma_log_info("|  Queue %u: owner=%u, msgs=%u/%u\n",
                    m_queues[i].queue_id, m_queues[i].owner_pid,
                    m_queues[i].count, (sigma_u32)IPC_QUEUE_CAPACITY);
            }
        }
        for (sigma_u32 i = 0; i < m_shm_count; i++) {
            if (m_shm[i].shm_id != 0) {
                sigma_log_info("|  SHM '%s': %lluKB, refs=%u\n",
                    m_shm[i].name,
                    (unsigned long long)(m_shm[i].size / 1024),
                    m_shm[i].ref_count);
            }
        }
        sigma_log("----------------------------");
    }

private:
    SovereignIPC() : m_queue_count(0), m_shm_count(0), m_total_messages(0) {}

    sigma_msg_queue_t* findQueue(sigma_u32 id) {
        if (id == 0 || id > m_queue_count) return SIGMA_NULL;
        sigma_msg_queue_t& q = m_queues[id - 1];
        return (q.queue_id == id) ? &q : SIGMA_NULL;
    }

    sigma_shm_t* findShm(sigma_u32 id) {
        if (id == 0 || id > m_shm_count) return SIGMA_NULL;
        sigma_shm_t& s = m_shm[id - 1];
        return (s.shm_id == id) ? &s : SIGMA_NULL;
    }

    sigma_msg_queue_t m_queues[IPC_MAX_QUEUES];
    sigma_shm_t       m_shm[IPC_MAX_SHM_SEGMENTS];
    sigma_u32         m_queue_count;
    sigma_u32         m_shm_count;
    sigma_u64         m_total_messages;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void ipc_init(void) { SigmaOS::Kernel::SovereignIPC::getInstance().init(); }

sigma_u32 ipc_create_queue(sigma_u32 owner_pid) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().createQueue(owner_pid);
}
int ipc_destroy_queue(sigma_u32 queue_id) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().destroyQueue(queue_id);
}
int ipc_send(sigma_u32 queue_id, const sigma_msg_t* msg) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().send(queue_id, msg);
}
int ipc_receive(sigma_u32 queue_id, sigma_msg_t* out_msg) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().receive(queue_id, out_msg);
}
sigma_u32 ipc_queue_count(sigma_u32 queue_id) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().queueCount(queue_id);
}
sigma_u32 shm_create(const char* name, sigma_usize size, sigma_u32 owner_pid) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().shmCreate(name, size, owner_pid);
}
int shm_attach(sigma_u32 shm_id, sigma_u32 pid) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().shmAttach(shm_id, pid);
}
int shm_detach(sigma_u32 shm_id, sigma_u32 pid) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().shmDetach(shm_id, pid);
}
int shm_destroy(sigma_u32 shm_id) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().shmDestroy(shm_id);
}
int signal_send(sigma_u32 target_pid, sigma_signal_t sig) {
    return SigmaOS::Kernel::SovereignIPC::getInstance().signalSend(target_pid, sig);
}
void ipc_print_status(void) {
    SigmaOS::Kernel::SovereignIPC::getInstance().printStatus();
}

} // extern "C"
