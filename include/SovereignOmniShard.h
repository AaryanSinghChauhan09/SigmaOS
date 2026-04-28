/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN OMNI-SHARD (v20.0 - PURE C11 FINALITY)
 * =========================================================================
 * Domains: OS Kernel, Cloud, Web UI, Networking
 * Principle: Zero OOP runtime. Zero vtable overhead.
 * =========================================================================
 */

#ifndef SOVEREIGN_OMNI_SHARD_H
#define SOVEREIGN_OMNI_SHARD_H

#include "SovereignLibC.h"
#include "sigma_system_shards.h"

/* =========================================================================
 * DOMAIN: OS KERNEL & ADVANCED SCHEDULING
 * ========================================================================= */
void SovereignScheduler_init(SovereignScheduler* s);
void SovereignScheduler_MultilevelFeedbackQueue(SovereignScheduler* s);
void SovereignScheduler_RealTimeDeadlineSchedule(SovereignScheduler* s);
void SovereignScheduler_audit(const SovereignScheduler* s);

/* =========================================================================
 * DOMAIN: CLOUD & HYPERVISING
 * ========================================================================= */
void SovereignCloud_init(SovereignCloudOrchestrator* c);
void SovereignCloud_ElasticShardScale(SovereignCloudOrchestrator* c, int nodeCount);
void SovereignCloud_VirtualVPCIsolation(SovereignCloudOrchestrator* c, const char* tenantId);
void SovereignCloud_audit(const SovereignCloudOrchestrator* c);

/* =========================================================================
 * DOMAIN: WEB & UI ENGINE
 * ========================================================================= */
void SovereignUI_init(SovereignUIEngine* u);
void SovereignUI_RenderSovereignDOM(SovereignUIEngine* u, const char* markup);
void SovereignUI_ApplyZenithCSS(SovereignUIEngine* u, const char* styling);
void SovereignUI_Notify(SovereignUIEngine* u, const char* msg, const char* type);
void SovereignUI_audit(const SovereignUIEngine* u);

/* =========================================================================
 * DOMAIN: NETWORKING & SECURITY
 * ========================================================================= */
void SovereignNet_init(SovereignNetZenith* n);
void SovereignNet_ZeroTrustHandshake(SovereignNetZenith* n);
void SovereignNet_RecursiveDNSNode(SovereignNetZenith* n, const char* domain);
void SovereignNet_audit(const SovereignNetZenith* n);

#endif /* SOVEREIGN_OMNI_SHARD_H */
