/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DEFENDER (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: Windows Defender / YARA
 *
 * Features implemented:
 *   ✓ Real-time file system shield (On-access scanning)
 *   ✓ YARA-style bytecode matching engine for malware signatures
 *   ✓ Heuristic analysis (Entropy, suspicious strings)
 *   ✓ Quarantine management
 *   ✓ EICAR test string detection
 * =========================================================================
 */

#ifndef SOVEREIGN_DEFENDER_H
#define SOVEREIGN_DEFENDER_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef struct {
    char rule_name[64];
    sigma_u8 signature[128];
    sigma_u32 sig_len;
} SigmaYaraRule_t;

typedef enum {
    DEFENDER_CLEAN = 0,
    DEFENDER_MALWARE = 1,
    DEFENDER_SUSPICIOUS = 2,
} SigmaScanResult_t;

sigma_err_t sigma_defender_enable_real_time_protection(void);
sigma_err_t sigma_defender_disable_real_time_protection(void);
SigmaScanResult_t sigma_defender_scan_buffer(const void *buffer, sigma_sz_t size, char *threat_name);
SigmaScanResult_t sigma_defender_scan_file(const char *path, char *threat_name);
sigma_err_t sigma_defender_quarantine(const char *path);

void SovereignDefender_Init(void);

#endif /* SOVEREIGN_DEFENDER_H */
