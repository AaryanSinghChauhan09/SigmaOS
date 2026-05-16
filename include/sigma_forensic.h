/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN FORENSIC TOOLKIT (S-FORENSIC)
 * =========================================================================
 * Mission: Non-invasive silicon auditing and hardware evidence preservation.
 * Inspired by CAINE / SystemRescue / Forensic-OS.
 * =========================================================================
 */

#ifndef SIGMA_FORENSIC_H
#define SIGMA_FORENSIC_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char device_id[64];
    bool write_block_active;
    sigma_u64 total_blocks;
} sigma_forensic_device_t;

/* --- Forensic Primitives --- */
void      forensic_init(void);
void      forensic_enable_write_block(const char* device_id);
void      forensic_dump_memory(void* buffer, sigma_size_t size);
void      forensic_analyze_lattice_integrity(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Forensics {

class SovereignForensicEngine {
public:
    static SovereignForensicEngine& getInstance() {
        static SovereignForensicEngine instance;
        return instance;
    }

    void init();
    void setWriteBlock(const char* id, bool enable);
    void dumpMemory(void* buf, sigma_size_t size);
    void auditLattice();

private:
    SovereignForensicEngine() {}
};

} // namespace Forensics
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_FORENSIC_H */
