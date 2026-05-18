/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA UNION OVERLAY FILE SYSTEM GOVERNOR (sigma_overlayfs) v1.0
 * =========================================================================
 * Mission: Zero-dependency live system directory merging and union filesystem.
 * Inspiration: Linux OverlayFS / unionfs / Alpine Live USB.
 * Principle: Instant copy-up write redirection.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

// C-bridge imports from the kernel's OverlayFS implementation
extern "C" {
    void overlay_init();
    sigma_bool overlay_mount(const char* lower, const char* upper, const char* merged);
    sigma_bool overlay_write(const char* filename, const char* content);
    void overlay_list();
}

namespace SigmaOS {
namespace Tools {

class SigmaOverlayCLI : public SigmaObject, public SigmaSingleton<SigmaOverlayCLI> {
    friend class SigmaSingleton<SigmaOverlayCLI>;
public:
    const char* type_name() const noexcept override { return "SigmaOverlayCLI"; }

    void run_command(int argc, char* argv[]) {
        if (argc < 2) {
            print_usage();
            return;
        }

        SigmaString cmd(argv[1]);

        if (sigma_strcmp(cmd.c_str(), "mount") == 0) {
            if (argc < 5) {
                sigma_log_error("[OVERLAY-CLI] Error: 'mount' requires <lowerdir> <upperdir> <mergeddir>");
                return;
            }
            const char* lower = argv[2];
            const char* upper = argv[3];
            const char* merged = argv[4];
            overlay_mount(lower, upper, merged);
        } else if (sigma_strcmp(cmd.c_str(), "write") == 0) {
            if (argc < 4) {
                sigma_log_error("[OVERLAY-CLI] Error: 'write' requires <filename> <content>");
                return;
            }
            const char* filename = argv[2];
            const char* content = argv[3];
            overlay_write(filename, content);
        } else if (sigma_strcmp(cmd.c_str(), "list") == 0) {
            overlay_list();
        } else {
            print_usage();
        }
    }

    void print_usage() {
        sigma_log_info("Σ SigmaOS Overlay Union FS Governor (sigma-overlay) v1.0");
        sigma_log_info("Usage:");
        sigma_log_info("  sigma-overlay mount <lowerdir> <upperdir> <mergeddir> Mount a merged OverlayFS partition");
        sigma_log_info("  sigma-overlay write <filename> <content>               Write data (triggers Copy-Up-On-Write)");
        sigma_log_info("  sigma-overlay list                                     List the merged filesystem layout");
    }

private:
    SigmaOverlayCLI() {}
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void overlay_cli_run(int argc, char* argv[]) {
        SigmaOS::Tools::SigmaOverlayCLI::getInstance().run_command(argc, argv);
    }
}
