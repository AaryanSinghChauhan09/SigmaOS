#ifndef SOVEREIGN_OMNI_SHARD_H
#define SOVEREIGN_OMNI_SHARD_H

#include "../libc/sigma_libc.h"

/* --- OS KERNEL & SCHEDULING --- */
typedef struct SovereignScheduler {
    const char* type_name;
    sigma_u64   ctx_switches;
    sigma_u64   deadline_misses;
} SovereignScheduler;

void SovereignScheduler_init(SovereignScheduler* s);
void SovereignScheduler_MultilevelFeedbackQueue(SovereignScheduler* s);
void SovereignScheduler_RealTimeDeadlineSchedule(SovereignScheduler* s);
void SovereignScheduler_audit(const SovereignScheduler* s);

/* --- CLOUD & HYPERVISING --- */
typedef struct SovereignCloudOrchestrator {
    const char* type_name;
    sigma_u32   active_nodes;
    sigma_u32   isolated_vpcs;
} SovereignCloudOrchestrator;

void SovereignCloud_init(SovereignCloudOrchestrator* c);
void SovereignCloud_ElasticShardScale(SovereignCloudOrchestrator* c, int nodeCount);
void SovereignCloud_VirtualVPCIsolation(SovereignCloudOrchestrator* c, const char* tenantId);
void SovereignCloud_audit(const SovereignCloudOrchestrator* c);

/* --- WEB & UI ENGINE --- */
typedef struct SovereignUIEngine {
    const char* type_name;
    sigma_u64   frames_rendered;
} SovereignUIEngine;

void SovereignUI_init(SovereignUIEngine* u);
void SovereignUI_RenderSovereignDOM(SovereignUIEngine* u, const char* markup);
void SovereignUI_ApplyZenithCSS(SovereignUIEngine* u, const char* styling);
void SovereignUI_audit(const SovereignUIEngine* u);

/* --- NETWORKING & SECURITY --- */
typedef struct SovereignNetZenith {
    const char* type_name;
    sigma_u64   handshakes;
    sigma_u64   dns_queries;
} SovereignNetZenith;

void SovereignNet_init(SovereignNetZenith* n);
void SovereignNet_ZeroTrustHandshake(SovereignNetZenith* n);
void SovereignNet_RecursiveDNSNode(SovereignNetZenith* n, const char* domain);
void SovereignNet_audit(const SovereignNetZenith* n);

#endif /* SOVEREIGN_OMNI_SHARD_H */
