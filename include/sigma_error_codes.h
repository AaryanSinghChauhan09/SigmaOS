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

/* Pull in the sovereign libc (sys_print, sigma_malloc, sigma_strcmp, etc.)
 * and all kernel primitive types. Any file that includes sigma_error_codes.h
 * automatically gets the full sovereign runtime API. */
#include "sigma_libc.h"
#include "sigma_profiles.h"

/* -------------------------------------------------------------------------
 * Top-Level Status Aliases
 * Canonical names used across the codebase.
 * ------------------------------------------------------------------------- */
#define SIGMA_SUCCESS       0
#define SIGMA_ERROR        (-1)
#define ZEN_SUCCESS         0
#define ZEN_ERROR          (-1)

/* Kernel Error Codes */
#define K_OK                0
#define K_ERR_INVAL        (-1)
#define K_ERR_TIMEOUT      (-2)
#define K_ERR_IO           (-3)
#define K_ERR_EIO          (-3)
#define K_ERR_NO_MEM       (-4)
#define K_ERR_NOMEM        (-4)   /* Alias: out-of-memory */
#define K_ERR_NOTFOUND     (-5)   /* Resource / entry not found */
#define K_ERR_PERM         (-6)   /* Permission denied (Zero-Trust) */
#define K_ERR_BUSY         (-7)   /* Resource busy / locked */
#define K_ERR_OVERFLOW     (-8)   /* Arithmetic or buffer overflow */
#define K_ERR_UNIMPL       (-9)   /* Feature not yet implemented */
typedef int sigma_status;

/* Hardware Profile extensions moved to sigma_driver_codes.h */

/* -------------------------------------------------------------------------
 * Categories:
 *  DRV  Driver & Hardware codes
 *  NET  Network Sandbox & Isolation
 *  FS   Filesystem & Zero-Trust Access
 *  4xx  UI & Compositor
 *  5xx  Boot & Hardware Fallbacks
 *  KRN  Kernel-level panics (self-healing)
 * ------------------------------------------------------------------------- */

/* Driver codes */
#define ZEN_DRV_GPU_INIT_FAILED       0xD001
#define ZEN_DRV_GPU_FALLBACK_VGA      0xD002
#define ZEN_DRV_RECIPE_SIG_INVALID    0xD003
#define ZEN_DRV_RELOAD_FAIL           0xD004
#define ZEN_DRV_CRASH                 0xD005

/* 1xx: Container Errors */
#define ZEN_101_CONTAINER_RUNTIME_FAILED  101
#define ZEN_102_CONTAINER_OOM             102
#define ZEN_103_CONTAINER_SCHEDULER_DENY  103

/* 2xx: Sandbox / Network Errors */
#define ZEN_201_SANDBOX_ESCAPE_ATTEMPT    201
#define ZEN_202_SANDBOX_IPC_INTEGRITY     202
#define ZEN_203_NETWORK_SANDBOX_REFUSED   203
#define ZEN_NET_DOWN                      0xE001
#define ZEN_NET_RESET_FAIL                0xE002

/* 3xx: Zero-Trust FS Errors */
#define ZEN_301_ACL_DENIAL                301
#define ZEN_302_READONLY_VFS_VIOLATION    302
#define ZEN_303_CRYPTO_SECTOR_CORRUPT     303
#define ZEN_FS_CORRUPT                    0xF001

/* 4xx: Compositor UI Errors */
#define ZEN_401_FB_ACQUISITION_FAILED     401
#define ZEN_402_WINDOW_ALLOCATION_OOM     402
#define ZEN_403_THEME_BROADCAST_FAILURE   403

/* 5xx: Recovery & Fallback Profiles */
#define ZEN_501_FORENSIC_BOOT_ACTIVE      501
#define ZEN_502_VGA_FALLBACK_TRIGGERED    502
#define ZEN_503_SELF_HEALING_RESTART      503

/* KRN: Kernel panic codes */
#define ZEN_MEM_LEAK                      0xA001
#define ZEN_KRN_PANIC                     0xB001

#endif /* SIGMA_ERROR_CODES_H */
