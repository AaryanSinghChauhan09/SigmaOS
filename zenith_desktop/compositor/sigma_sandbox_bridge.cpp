/**
 * =========================================================================
 * Σ SIGMAOS: ZENITH SANDBOX BRIDGE (PHASE 4a)
 * =========================================================================
 * Acts as the policy enforcer between the Zenith UI and the Sovereign 
 * Orchestrator. When a GUI application launches, this bridge automatically 
 * requests the Orchestrator to spin up a dedicated container shard. 
 * =========================================================================
 */

#include <sigma_libc.h>

namespace Zenith {
namespace Security {

class SandboxBridge {
public:
    static SandboxBridge& getInstance() {
        static SandboxBridge instance;
        return instance;
    }

    void init() {
        sys_print("[Zenith-Sandbox] Initializing Secure App Bridge...\n");
    }

    sigma_u32 spawnSandboxedApp(const char* app_name, sigma_u32 executable_inode) {
        sys_print("[Zenith-Sandbox] Launching '%s' in an isolated container...\n", app_name);

        /* 
         * 1. Send IPC to Orchestrator (Shard ID 4) to spawn container 
         * For MVP, we pass an arbitrary 64MB memory limit.
         */
        sigma_u64 args[3] = { (sigma_u64)app_name, executable_inode, 64 * 1024 * 1024 };
        sigma_status status = sys_ipc_send(4, 1, /* MSG_SPAWN_CONTAINER */ args, sizeof(args));

        if (status != SIGMA_SUCCESS) {
            sys_print("[Zenith-Sandbox] ERROR: Orchestrator refused to spawn container.\n");
            return 0; // Invalid Container ID
        }

        /* Mock receiving the container ID back from the Orchestrator */
        sigma_u32 allocated_container_id = m_next_mock_id++;
        
        sys_print("[Zenith-Sandbox] App '%s' successfully sandboxed in Container [%u]\n", 
                  app_name, allocated_container_id);

        return allocated_container_id;
    }

private:
    SandboxBridge() : m_next_mock_id(1) {}
    sigma_u32 m_next_mock_id;
};

} // namespace Security
} // namespace Zenith

extern "C" {
    void zenith_sandbox_init(void) {
        Zenith::Security::SandboxBridge::getInstance().init();
    }

    sigma_u32 zenith_launch_app_sandboxed(const char* name, sigma_u32 inode) {
        return Zenith::Security::SandboxBridge::getInstance().spawnSandboxedApp(name, inode);
    }
}
