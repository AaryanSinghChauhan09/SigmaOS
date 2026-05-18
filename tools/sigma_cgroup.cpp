/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA CGROUP RESOURCE MANAGER (sigma_cgroup) v1.0
 * =========================================================================
 * Mission: Zero-dependency resource allocation and silicon governance CLI.
 * Inspiration: Linux cgroups v2 / Kubernetes ResourceQuota.
 * Principle: Enforces CPU, Memory, and I/O weights natively.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"
#include <stdlib.h>

#define sigma_atoi atoi

// C-bridge imports from the kernel's cgroup implementation
extern "C" {
    void cgroup_init();
    sigma_bool cgroup_create(const char* name, sigma_u32 cpu_pct, sigma_u32 mem_mb, sigma_u32 io_weight);
    void cgroup_enforce();
    void cgroup_audit();
}

namespace SigmaOS {
namespace Tools {

class SigmaCgroupCLI : public SigmaObject, public SigmaSingleton<SigmaCgroupCLI> {
    friend class SigmaSingleton<SigmaCgroupCLI>;
public:
    const char* type_name() const noexcept override { return "SigmaCgroupCLI"; }

    void run_command(int argc, char* argv[]) {
        if (argc < 2) {
            print_usage();
            return;
        }

        SigmaString cmd(argv[1]);

        if (sigma_strcmp(cmd.c_str(), "create") == 0) {
            if (argc < 6) {
                sigma_log_error("[CGROUP-CLI] Error: 'create' requires <name> <cpu_pct> <mem_mb> <io_weight>");
                return;
            }
            const char* name = argv[2];
            sigma_u32 cpu = sigma_atoi(argv[3]);
            sigma_u32 mem = sigma_atoi(argv[4]);
            sigma_u32 io = sigma_atoi(argv[5]);

            cgroup_create(name, cpu, mem, io);
        } else if (sigma_strcmp(cmd.c_str(), "enforce") == 0) {
            sigma_log_info("[CGROUP-CLI] Triggering automatic governor sweep...");
            cgroup_enforce();
        } else if (sigma_strcmp(cmd.c_str(), "audit") == 0) {
            cgroup_audit();
        } else {
            print_usage();
        }
    }

    void print_usage() {
        sigma_log_info("Σ SigmaOS Cgroup Manager (sigma-cgroup) v1.0");
        sigma_log_info("Usage:");
        sigma_log_info("  sigma-cgroup create <name> <cpu_pct> <mem_mb> <io_weight>");
        sigma_log_info("  sigma-cgroup enforce");
        sigma_log_info("  sigma-cgroup audit");
    }

private:
    SigmaCgroupCLI() {}
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void cgroup_cli_run(int argc, char* argv[]) {
        SigmaOS::Tools::SigmaCgroupCLI::getInstance().run_command(argc, argv);
    }
}
