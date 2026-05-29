/**
 * =========================================================================
 * Σ SIGMAOS: GRANULAR SOVEREIGN ERROR CODES
 * =========================================================================
 * Inspired by Fedora CoreOS transparent error mapping and NixOS-style
 * trace logs. Categorized diagnostic error codes.
 * =========================================================================
 */

#ifndef SIGMA_ERROR_CODES_H
#define SIGMA_ERROR_CODES_H

/* Categories:
 * 1xx: Container & Orchestrator Runtime
 * 2xx: Network Sandbox & Isolation
 * 3xx: Filesystem & Zero-Trust Access
 * 4xx: UI & Compositor
 * 5xx: Boot & Hardware Fallbacks
 */

#define ZEN_SUCCESS 0

/* 1xx: Container Errors */
#define ZEN_101_CONTAINER_RUNTIME_FAILED  101
#define ZEN_102_CONTAINER_OOM             102
#define ZEN_103_CONTAINER_SCHEDULER_DENY  103

/* 2xx: Sandbox Errors */
#define ZEN_201_SANDBOX_ESCAPE_ATTEMPT    201
#define ZEN_202_SANDBOX_IPC_INTEGRITY     202
#define ZEN_203_NETWORK_SANDBOX_REFUSED   203

/* 3xx: Zero-Trust FS Errors */
#define ZEN_301_ACL_DENIAL                301
#define ZEN_302_READONLY_VFS_VIOLATION    302
#define ZEN_303_CRYPTO_SECTOR_CORRUPT     303

/* 4xx: Compositor UI Errors */
#define ZEN_401_FB_ACQUISITION_FAILED     401
#define ZEN_402_WINDOW_ALLOCATION_OOM     402
#define ZEN_403_THEME_BROADCAST_FAILURE   403

/* 5xx: Recovery & Fallback Profiles */
#define ZEN_501_FORENSIC_BOOT_ACTIVE      501
#define ZEN_502_VGA_FALLBACK_TRIGGERED    502
#define ZEN_503_SELF_HEALING_RESTART      503

#endif /* SIGMA_ERROR_CODES_H */
