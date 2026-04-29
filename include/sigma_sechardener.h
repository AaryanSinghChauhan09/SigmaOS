/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SECURITY HARDENER (S-SECHARDENER)  — SHARD #500
 * =========================================================================
 * Mission: A comprehensive, modular security hardening framework that
 * enforces bounds checking, ASLR, stack canaries, and seccomp-style
 * syscall filtering across all 500 sovereign shards.
 *
 * Directly addresses code-scanning alerts for:
 *  - CWE-119: Improper Restriction of Buffer Operations
 *  - CWE-732: Incorrect Permission Assignment
 *  - CWE-20:  Improper Input Validation
 * =========================================================================
 */

#ifndef SIGMA_SECHARDENER_H
#define SIGMA_SECHARDENER_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    HARDEN_LEVEL_BASELINE,   /* ASLR + stack canaries */
    HARDEN_LEVEL_STRICT,     /* + syscall filtering */
    HARDEN_LEVEL_SOVEREIGN   /* + full CIB isolation per shard */
} sigma_harden_level_t;

/* --- Security Hardener Primitives --- */
void sechardener_init(void);
void sechardener_apply_to_shard(uint32_t shard_id, sigma_harden_level_t level);
void sechardener_validate_buffer(const void* buf, uint32_t claimed_size, uint32_t actual_capacity);
void sechardener_audit_all_shards(void);

/* Secure string operations (replaces strcpy/sprintf that trigger CWE-119) */
void sigma_hardened_strcpy(char* dest, const char* src, uint32_t max_len);
int  sigma_hardened_snprintf(char* dest, uint32_t max_len, const char* fmt, ...);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SECHARDENER_H */
