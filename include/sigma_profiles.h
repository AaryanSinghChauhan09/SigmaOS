/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM PROFILES
 * =========================================================================
 * Formally defines isolation and sandboxing policy configurations based on
 * targeted device configurations (Desktop, Forensic, IoT, Enterprise).
 * =========================================================================
 */

#ifndef SIGMA_PROFILES_H
#define SIGMA_PROFILES_H

/* --- Software / Security Profiles --- */
typedef enum {
    SIGMA_PROFILE_STANDARD  = 0,  /* Balanced desktop usage                */
    SIGMA_PROFILE_FORENSIC  = 1,  /* CAINE-style strict write-protection   */
    SIGMA_PROFILE_EDUCATION = 2,  /* Sandboxed limits for safe exploration */
    SIGMA_PROFILE_ENTERPRISE= 3,  /* Strict validation and VFS policies    */
    SIGMA_PROFILE_IOT       = 4   /* Highly-optimized minimal footprint    */
} sigma_system_profile_t;

/* --- Hardware Platform Profiles (bitmask) ---
 * Used by the Hardware Test Suite to select which tests apply to a platform.
 */
typedef enum {
    SIGMA_HW_PROFILE_NONE       = 0x00,  /* No profile */
    SIGMA_HW_PROFILE_STANDARD   = 0x01,  /* Generic x86_64 desktop/laptop */
    SIGMA_HW_PROFILE_GAMING     = 0x02,  /* High-performance GPU platform */
    SIGMA_HW_PROFILE_SERVER     = 0x04,  /* Headless multi-NIC server */
    SIGMA_HW_PROFILE_IOT_ARM64  = 0x08,  /* ARM64 embedded / Raspberry Pi */
    SIGMA_HW_PROFILE_FORENSIC   = 0x10,  /* CAINE-style read-only imaging */
    SIGMA_HW_PROFILE_ALL        = 0xFF   /* Run on any platform */
} sigma_hw_profile_t;

#endif /* SIGMA_PROFILES_H */
