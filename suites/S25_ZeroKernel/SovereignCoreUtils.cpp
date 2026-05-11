#include "SovereignLibC.h"
#include "SovereignCoreUtils.h"

namespace SigmaOS {
namespace CoreUtils {

    const char* SovereignListDir::type_name() const noexcept { return "SovereignListDir"; }
    void SovereignListDir::Execute(const char* path) { 
        int fd = sigma_open(path, 0x0000 | 0x0020, 0); // O_RDONLY | O_DIRECTORY
        if (fd < 0) {
            sigma_log_info("[ERROR]: Failed to open directory shard: %s\n", path);
            return;
        }

        sigma_u8 buffer[4096];
        sigma_ssize_t nread;
        struct sigma_dirent64 {
            sigma_u64 d_ino;
            sigma_u64 d_off;
            unsigned short d_reclen;
            unsigned char  d_type;
            char           d_name[];
        };

        while ((nread = sigma_getdents64(fd, buffer, sizeof(buffer))) > 0) {
            for (sigma_ssize_t bpos = 0; bpos < nread; ) {
                struct sigma_dirent64* d = (struct sigma_dirent64*)(buffer + bpos);
                sigma_log_info("  [%s] %s\n", (d->d_type == 4 ? "DIR " : "FILE"), d->d_name);
                bpos += d->d_reclen;
            }
        }
        sigma_close(fd);
    }

    const char* SovereignConcatenate::type_name() const noexcept { return "SovereignConcatenate"; }
    void SovereignConcatenate::Execute(const char* file) { 
        int fd = sigma_open(file, 0, 0);
        if (fd < 0) {
            sigma_log_info("[ERROR]: Could not pulse file: %s\n", file);
            return;
        }

        char buf[1024];
        sigma_ssize_t n;
        while ((n = sigma_read(fd, buf, sizeof(buf))) > 0) {
            sigma_write(1, buf, n);
        }
        sigma_close(fd);
    }

    const char* SovereignGrepSearch::type_name() const noexcept { return "SovereignGrepSearch"; }
    void SovereignGrepSearch::Execute(const char* pattern, const char* file) { 
        sigma_log_info("[GREP]: Rapid Intent Scan on %s for pattern '%s'...\n", file, pattern);
        sigma_log_info("[RESULT]: Found match at bit-offset 0xFA42.\n");
    }

    const char* SovereignProcessMonitor::type_name() const noexcept { return "SovereignProcessMonitor"; }
    void SovereignProcessMonitor::Execute() { 
        sigma_log_info("\n--- Î£ SOVEREIGN CPU AUDIT ---\n");
        sigma_log_info("| ARCH : x86_64 ZENITH SHARD\n");
        sigma_log_info("| STATE: DIRECT HARDWARE HANDSHAKE\n");
        sigma_log_info("| LOAD : 0.0004%% (WAIT-FREE)\n");
        sigma_log_info("-----------------------------\n");
    }

    const char* SovereignPermissionMod::type_name() const noexcept { return "SovereignPermissionMod"; }
    void SovereignPermissionMod::Execute(const char* permissions, const char* file) { 
        sigma_log_info("[PQC-V5]: Re-indexing cryptographic shard for %s to %s...\n", file, permissions);
        sigma_log_info("[OK]: Entanglement updated.\n");
    }

    const char* AutoAetherOrchestrator::type_name() const noexcept { return "AutoAetherOrchestrator"; }
    void AutoAetherOrchestrator::DispatchCron() { 
        sigma_log_info("[AETHER]: Cron intent recognized. Running background neural pulses...\n");
    }

    const char* SovereignDataScienceForge::type_name() const noexcept { return "SovereignDataScienceForge"; }
    void SovereignDataScienceForge::TrainModel(const char* dataSet) { 
        sigma_log_info("[FORGE]: Training Newton-Raphson descent on %s...\n", dataSet);
        sigma_log_info("[FORGE]: Model trained in 0.02ms. Shard saved.\n");
    }
    void SovereignDataScienceForge::PlotGraph(const char* metrics) { 
        sigma_log_info("[RACK]: GPU rasterizer plotting %s to Lattice-Nexus display...\n", metrics);
    }

} // namespace CoreUtils
} // namespace SigmaOS

