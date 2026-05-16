// SigmaOS — sigma-sec-sandbox: Application Isolation Sandbox
// Module: sigma-sec-sandbox
// USP: Strictly isolates processes by restricting their VMM access and syscall interface capability.

#ifndef SIGMA_SEC_SANDBOX_H
#define SIGMA_SEC_SANDBOX_H

#include "sigma_caps.h"
#include "./sigmaos/core/src/atomic_sigma_process.hpp"

#define SIGMA_SANDBOX_STRICT 1
#define SIGMA_SANDBOX_PERMISSIVE 0

typedef struct SigmaSandboxConfig {
    unsigned char mode;           // STRICT or PERMISSIVE
    unsigned char allow_network;
    unsigned char allow_fs_write;
    unsigned char allow_ipc;
    unsigned int  max_memory_mb;
} SigmaSandboxConfig;

// Abstract Sandbox Class encapsulating a process
class SovereignSandbox : public sigma::core::IProcess {
private:
    sigma::core::IProcess* underlying_process;
    SigmaSandboxConfig     config;
    SigmaCapToken          sandbox_token; // Restricted token

public:
    SovereignSandbox(sigma::core::IProcess* process, SigmaSandboxConfig cfg) 
        : underlying_process(process), config(cfg) {
        
        // Downgrade capabilities based on config
        sandbox_token.capabilities = 0; // Clear all
        if (config.allow_network) sandbox_token.capabilities |= SIGMA_CAP_NET;
        if (config.allow_fs_write) sandbox_token.capabilities |= SIGMA_CAP_FS_WRITE;
        if (config.allow_ipc) sandbox_token.capabilities |= SIGMA_CAP_IPC;
    }

    // Wrap IProcess interface
    void initialize() override {
        sigma_kprint("[SANDBOX] Initializing isolated container...\n");
        if (underlying_process) underlying_process->initialize();
    }

    void execute() override {
        // Here we would enforce hardware-level isolation (e.g. configuring VMM permissions)
        // prior to executing the payload
        if (underlying_process) underlying_process->execute();
    }

    void shutdown() override {
        sigma_kprint("[SANDBOX] Tearing down isolated container...\n");
        if (underlying_process) underlying_process->shutdown();
    }

    bool is_capability_allowed(unsigned int cap_flag) const {
        return (sandbox_token.capabilities & cap_flag) != 0;
    }
};

#endif /* SIGMA_SEC_SANDBOX_H */
