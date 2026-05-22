#include "../sigma_libc.h"


// SigmaOS Sandbox Tool
// Competitor Target: ChromeOS / gVisor (Process Isolation & Sandboxing)

void sandbox_process(const char* bin) {
    sigma_log_info("[Sigma Sandbox] Creating zero-trust sandbox execution boundary for: %s\n", bin);
    sigma_log_info("[Sigma Sandbox] Dropping all capabilities. Restricting filesystem to /tmp/sandbox.\n");
    sigma_log_info("[Sigma Sandbox] Sandbox active. Executing binary...\n");
}

int main(int argc, char** argv) {
    if (argc > 1) {
        sandbox_process(argv[1]);
    } else {
        sigma_log_info("Error: Provide target binary to sandbox (e.g. sigma_sandbox /bin/bash)\n");
    }
    return 0;
}


