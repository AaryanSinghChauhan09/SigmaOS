/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA ZFS-COW STORAGE GOVERNOR (sigma_zfs) v1.0
 * =========================================================================
 * Mission: Zero-dependency storage pooling and copy-on-write manager.
 * Inspiration: OpenZFS / FreeBSD zpool / ZFS filesystems.
 * Principle: Striping, mirroring, and post-quantum cryptographically-secured parity.
 * =========================================================================
 */

#include "../include/sigma_kernel_types.h"
#include "../include/sigma_log.h"
#include "../include/SigmaOOP.hpp"

// C-bridge imports from the kernel's ZFS implementation
extern "C" {
    void zfs_init();
    sigma_bool zfs_pool_add(const char* path, sigma_u32 size_gb);
    sigma_bool zfs_allocate(sigma_u32 size_gb, const char* dataset);
    void zfs_snapshot(const char* dataset, const char* snapshot);
    void zfs_audit();
}

namespace SigmaOS {
namespace Tools {

class SigmaZfsCLI : public SigmaObject, public SigmaSingleton<SigmaZfsCLI> {
    friend class SigmaSingleton<SigmaZfsCLI>;
public:
    const char* type_name() const noexcept override { return "SigmaZfsCLI"; }

    void run_command(int argc, char* argv[]) {
        if (argc < 2) {
            print_usage();
            return;
        }

        SigmaString cmd(argv[1]);

        if (sigma_strcmp(cmd.c_str(), "add") == 0) {
            if (argc < 4) {
                sigma_log_error("[ZFS-CLI] Error: 'add' requires <dev_path> <size_gb>");
                return;
            }
            const char* path = argv[2];
            sigma_u32 size = sigma_atoi(argv[3]);
            zfs_pool_add(path, size);
        } else if (sigma_strcmp(cmd.c_str(), "allocate") == 0) {
            if (argc < 4) {
                sigma_log_error("[ZFS-CLI] Error: 'allocate' requires <size_gb> <dataset_name>");
                return;
            }
            sigma_u32 size = sigma_atoi(argv[2]);
            const char* dataset = argv[3];
            zfs_allocate(size, dataset);
        } else if (sigma_strcmp(cmd.c_str(), "snapshot") == 0) {
            if (argc < 4) {
                sigma_log_error("[ZFS-CLI] Error: 'snapshot' requires <dataset_name> <snapshot_name>");
                return;
            }
            const char* dataset = argv[2];
            const char* snap = argv[3];
            zfs_snapshot(dataset, snap);
        } else if (sigma_strcmp(cmd.c_str(), "audit") == 0) {
            zfs_audit();
        } else {
            print_usage();
        }
    }

    void print_usage() {
        sigma_log_info("Σ SigmaOS ZFS Storage Pool Governor (sigma-zfs) v1.0");
        sigma_log_info("Usage:");
        sigma_log_info("  sigma-zfs add <dev_path> <size_gb>        Add a block device to the storage pool");
        sigma_log_info("  sigma-zfs allocate <size_gb> <dataset>    Allocate a new dataset transactionally");
        sigma_log_info("  sigma-zfs snapshot <dataset> <snap_name>  Create an O(1) Copy-on-Write snapshot");
        sigma_log_info("  sigma-zfs audit                           Perform storage pool diagnostics audit");
    }

private:
    SigmaZfsCLI() {}
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void zfs_cli_run(int argc, char* argv[]) {
        SigmaOS::Tools::SigmaZfsCLI::getInstance().run_command(argc, argv);
    }
}
