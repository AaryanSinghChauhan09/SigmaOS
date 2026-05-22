/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA LOCAL STATE BACKUP GOVERNOR (sigma_lbu) v1.0
 * =========================================================================
 * Mission: Zero-dependency diskless memory persistence manager.
 * Inspiration: Alpine Linux lbu (Alpine Local Backup) tool.
 * Principle: Encrypted state archiving with PQC-signature verification.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

// C-bridge imports from the kernel's LBU implementation
extern "C" {
    void lbu_init();
    sigma_bool lbu_track(const char* path);
    void lbu_commit();
    void lbu_restore();
    void lbu_audit();
}

namespace SigmaOS {
namespace Tools {

class SigmaLbuCLI : public SigmaObject, public SigmaSingleton<SigmaLbuCLI> {
    friend class SigmaSingleton<SigmaLbuCLI>;
public:
    const char* type_name() const noexcept override { return "SigmaLbuCLI"; }

    void run_command(int argc, char* argv[]) {
        if (argc < 2) {
            print_usage();
            return;
        }

        SigmaString cmd(argv[1]);

        if (sigma_strcmp(cmd.c_str(), "track") == 0) {
            if (argc < 3) {
                sigma_log_infoor("[LBU-CLI] Error: 'track' requires <file_path>");
                return;
            }
            const char* path = argv[2];
            lbu_track(path);
        } else if (sigma_strcmp(cmd.c_str(), "commit") == 0) {
            lbu_commit();
        } else if (sigma_strcmp(cmd.c_str(), "restore") == 0) {
            lbu_restore();
        } else if (sigma_strcmp(cmd.c_str(), "audit") == 0) {
            lbu_audit();
        } else {
            print_usage();
        }
    }

    void print_usage() {
        sigma_log_info("Î£ SigmaOS Local State Backup Governor (sigma-lbu) v1.0");
        sigma_log_info("Usage:");
        sigma_log_info("  sigma-lbu track <file_path>   Add path to persistent configuration manifest");
        sigma_log_info("  sigma-lbu commit              Compress and commit dynamic RAM state to boot flash");
        sigma_log_info("  sigma-lbu restore             Re-extract committed state back into memory ramdisk");
        sigma_log_info("  sigma-lbu audit               Verify checksums and audit persistence manifest");
    }

private:
    SigmaLbuCLI() {}
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void lbu_cli_run(int argc, char* argv[]) {
        SigmaOS::Tools::SigmaLbuCLI::getInstance().run_command(argc, argv);
    }
}

