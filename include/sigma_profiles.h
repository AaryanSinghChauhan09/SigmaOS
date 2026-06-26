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

typedef enum {
    SIGMA_PROFILE_STANDARD = 0,   /* Balanced desktop usage                */
    SIGMA_PROFILE_FORENSIC = 1,   /* CAINE-style strict write-protection   */
    SIGMA_PROFILE_EDUCATION = 2,  /* Sandboxed limits for safe exploration */
    SIGMA_PROFILE_ENTERPRISE = 3, /* Strict validation and VFS policies    */
    SIGMA_PROFILE_IOT = 4         /* Highly-optimized minimal foot-print  */
} sigma_system_profile_t;

#endif /* SIGMA_PROFILES_H */
