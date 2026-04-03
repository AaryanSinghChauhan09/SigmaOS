#ifndef SOVEREIGN_OMNI_SHARD_H
#define SOVEREIGN_OMNI_SHARD_H

#include "../libc/sigma_libc.h"

/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN OMNI-SHARD INTERFACE (v150.5 - PURE C11)
 * ========================================================================= */

typedef struct SovereignScheduler {
    const char* type_name;
    sigma_u64    ctx_switches;
    sigma_u64    deadline_misses;
} SovereignScheduler;

typedef struct SovereignCloudOrchestrator {
    const char* type_name;
    sigma_u32    active_nodes;
    sigma_u32    isolated_vpcs;
} SovereignCloudOrchestrator;

typedef struct SovereignUIEngine {
    const char* type_name;
    sigma_u64    frames_rendered;
} SovereignUIEngine;

typedef struct SovereignNetZenith {
    const char* type_name;
    sigma_u64    handshakes;
    sigma_u64    dns_queries;
} SovereignNetZenith;

/* --- Scheduler Shard Primitives --- */
void SovereignScheduler_init(SovereignScheduler* s);
void SovereignScheduler_MultilevelFeedbackQueue(SovereignScheduler* s);
void SovereignScheduler_RealTimeDeadlineSchedule(SovereignScheduler* s);
void SovereignScheduler_audit(const SovereignScheduler* s);

/* --- Cloud Shard Primitives --- */
void SovereignCloud_init(SovereignCloudOrchestrator* c);
void SovereignCloud_ElasticShardScale(SovereignCloudOrchestrator* c, int nodeCount);
void SovereignCloud_VirtualVPCIsolation(SovereignCloudOrchestrator* c, const char* tenantId);
void SovereignCloud_audit(const SovereignCloudOrchestrator* c);

/* --- UI Shard Primitives --- */
void SovereignUI_init(SovereignUIEngine* u);
void SovereignUI_RenderSovereignDOM(SovereignUIEngine* u, const char* markup);
void SovereignUI_ApplyZenithCSS(SovereignUIEngine* u, const char* styling);
void SovereignUI_audit(const SovereignUIEngine* u);

/* --- Network Shard Primitives --- */
void SovereignNet_init(SovereignNetZenith* n);
void SovereignNet_ZeroTrustHandshake(SovereignNetZenith* n);
void SovereignNet_RecursiveDNSNode(SovereignNetZenith* n, const char* domain);
void SovereignNet_audit(const SovereignNetZenith* n);

#endif // SOVEREIGN_OMNI_SHARD_H

/* --- OMNI-AGENT SOVEREIGN INTERFACE (AGENTIC CODING) --- */

typedef enum {
    AGENT_STATE_WAITING,
    AGENT_STATE_PLANNING,
    AGENT_STATE_EXECUTING,
    AGENT_STATE_AUDITING,
    AGENT_STATE_ERROR
} AgentState;

typedef enum {
    MISSION_TYPE_CODE_GEN,
    MISSION_TYPE_OPTIMIZE,
    MISSION_TYPE_QUERY,
    MISSION_TYPE_FIX
} MissionType;

typedef struct SovereignMission {
    char id[64];
    MissionType type;
    int priority;
    char intent[256];
} SovereignMission;

typedef struct SovereignOmniAgent {
    AgentState state;
    int perms;
    SovereignMission active_mission;
} SovereignOmniAgent;

#define PERM_READ   (1 << 0)
#define PERM_WRITE  (1 << 1)
#define PERM_PLAN   (1 << 2)

/**
 * Σ Sovereign Omni-Agent Prototypes
 */
void SovereignOmniAgentInit();
void SovereignOmniAgentPlan(const char* mission_id);
void SovereignOmniAgentExecute(const char* mission_id);
SovereignMission SigmaParseIntent(const char* prompt);
