/* S SIGMAOS: SOVEREIGN AUDIT SHARD HEADER */
#ifndef SOVEREIGN_AUDIT_SHARD_H
#define SOVEREIGN_AUDIT_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"
typedef enum { AUDIT_LOGIN, AUDIT_LOGOUT, AUDIT_SYSCALL, AUDIT_CFG_CHANGE,
               AUDIT_FW_RULE, AUDIT_DMA_QUARANTINE, AUDIT_PRIV_ESCALATION,
               AUDIT_PATCH_APPLY, AUDIT_SUSPICIOUS } SigmaAuditType_t;
void        sigma_audit_write        (SigmaAuditType_t type, sigma_u32 pid, sigma_u32 uid,
                                       const char* subject, const char* action);
sigma_bool  sigma_audit_verify_chain (void);
void        SovereignAuditShard_Init  (void);
void        SovereignAudit_Audit      (void);
#endif
