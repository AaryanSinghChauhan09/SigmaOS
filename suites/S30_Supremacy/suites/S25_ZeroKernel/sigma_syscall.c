#include "../../../include/sigma_syscall.h"
#include "../../../include/sigma_pmm.h"
#include "../../../include/sigma_process.h"
#include "../../../include/sigma_fs.h"
#include "../../../include/sigma_security.h"
#include "../../../include/sigma_ai_mem.h"
#include "../../../include/sigma_virtio.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* =========================================================================
 * SIGMA OS: SOVEREIGN SYSCALL DISPATCHER (S25 - ZeroKernel)
 * Central dispatch table replacing Linux's 450+ POSIX syscalls.
 * Each call maps to a dedicated Sigma kernel shard — zero ABI overhead.
 * ========================================================================= */

int64_t sigma_syscall_dispatch(sigma_syscall_frame_t* frame) {
    if (!frame) return -1;

    switch (frame->syscall_num) {

        case SYS_SIGMA_EXIT:
            // Terminate the calling process
            if (frame->arg0 < MAX_PROCESSES)
                sigma_ai_free_swarm((uint32_t)frame->arg0);
            return 0;

        case SYS_SIGMA_WRITE: {
            // Write bytes to VGA or kernel log
            const char* buf = (const char*)(uintptr_t)frame->arg1;
            if (buf) sigma_sigma_printf("[WRITE] %s\n", buf);
            return sigma_sigma_strlen(buf);
        }

        case SYS_SIGMA_ALLOC: {
            // Allocate 4KB physical page via PMM
            void* block = sigma_pmm_allocate_block();
            frame->ret = (uint64_t)(uintptr_t)block;
            return (int64_t)frame->ret;
        }

        case SYS_SIGMA_FREE:
            // Return physical page to PMM
            sigma_pmm_free_block((void*)(uintptr_t)frame->arg0);
            return 0;

        case SYS_SIGMA_SPAWN: {
            // Spawn a new process thread
            int pid = sigma_process_spawn(
                (void (*)(void))(uintptr_t)frame->arg0,
                (const char*)(uintptr_t)frame->arg1,
                (uint32_t)frame->arg2
            );
            return pid;
        }

        case SYS_SIGMA_YIELD:
            sigma_scheduler_tick();
            return 0;

        case SYS_SIGMA_AI_SPAWN: {
            // Allocate a neural agent swarm slot
            int id = sigma_ai_allocate_swarm(
                (const char*)(uintptr_t)frame->arg0,
                (uint16_t)frame->arg1
            );
            return id;
        }

        case SYS_SIGMA_AI_FREE:
            sigma_ai_free_swarm((uint32_t)frame->arg0);
            return 0;

        case SYS_SIGMA_VM_CREATE: {
            int vm_id = sigma_virt_create_vm(
                (const char*)(uintptr_t)frame->arg0,
                (uint64_t)frame->arg1,
                (uint8_t)frame->arg2
            );
            return vm_id;
        }

        case SYS_SIGMA_VM_START:
            return sigma_virt_start_vm((uint32_t)frame->arg0);

        case SYS_SIGMA_FS_READ:
            return sigma_fs_read_file(
                (uint32_t)frame->arg0,
                (uint8_t*)(uintptr_t)frame->arg1,
                (size_t)frame->arg2
            );

        case SYS_SIGMA_FS_WRITE:
            return sigma_fs_write_file(
                (uint32_t)frame->arg0,
                (const uint8_t*)(uintptr_t)frame->arg1,
                (size_t)frame->arg2
            );

        case SYS_SIGMA_SECURITY:
            return sigma_security_check(
                (uint32_t)frame->arg0,
                (sigma_permission_t)frame->arg1
            );

        default:
            sigma_sigma_printf("[SYSCALL] Unknown syscall: 0x%llx\n",
                   (unsigned long long)frame->syscall_num);
            return -EINVAL;
    }
}
