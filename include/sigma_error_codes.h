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
#define ZEN_SUCCESS         0
#define ZEN_ERROR          (-1)

/* -------------------------------------------------------------------------
 * Hardware Profile Extensions
 * Extends sigma_profiles.h with driver-specific variants.
 * ------------------------------------------------------------------------- */
#define SIGMA_HW_PROFILE_STANDARD     SIGMA_PROFILE_STANDARD
#define SIGMA_HW_PROFILE_FORENSIC     SIGMA_PROFILE_FORENSIC
#define SIGMA_HW_PROFILE_EDUCATION    SIGMA_PROFILE_EDUCATION
#define SIGMA_HW_PROFILE_ENTERPRISE   SIGMA_PROFILE_ENTERPRISE
#define SIGMA_HW_PROFILE_IOT          SIGMA_PROFILE_IOT
#define SIGMA_HW_PROFILE_GAMING       5   /* High-performance gaming rig    */
#define SIGMA_HW_PROFILE_SERVER       6   /* Headless / rack-mount server   */
#define SIGMA_HW_PROFILE_IOT_ARM64    7   /* Raspberry Pi / embedded ARM64  */

/* Re-export sigma_system_profile_t as sigma_hw_profile_t for driver code */
typedef sigma_system_profile_t sigma_hw_profile_t;

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
