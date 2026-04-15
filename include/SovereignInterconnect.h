/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN INTERCONNECT (v1.0 — MASTER REGISTRY)
 * =========================================================================
 * Mission: Unified Interface for Inter-Suite Communication.
 * =========================================================================
 */

#ifndef SOVEREIGN_INTERCONNECT_H
#define SOVEREIGN_INTERCONNECT_H

#include "sigma_base.h"

/* 
 * Σ [OMNIFABRIC] MESSAGE CLASSIFICATIONS 
 */
#define MSG_TYPE_MEM_ALLOC    0x01
#define MSG_TYPE_FS_WRITE     0x02
#define MSG_TYPE_NET_SEND     0x03
#define MSG_TYPE_SEC_AUDIT    0x04
#define MSG_TYPE_SIM_TICK     0x05
#define MSG_TYPE_ETH_STATE    0x06
#define MSG_TYPE_SYS_PANIC    0xFF

typedef struct {
    sigma_u32 sender_id;
    sigma_u32 receiver_id;
    sigma_u32 msg_type;
    sigma_u64 payload[4];
    sigma_u32 priority;
} OmniMessage;

/* 
 * Σ [S01-S33] CANONICAL SUITE IDENTIFIERS 
 */
#define SUITE_GENESIS              1
#define SUITE_ZENITHUI             2
#define SUITE_ORCHESTRATOR         3
#define SUITE_HAL                  4
#define SUITE_MEMORY               5
#define SUITE_STORAGE              6
#define SUITE_NETWORK              7
#define SUITE_SECURITY             8
#define SUITE_INTELLIGENCE         9
#define SUITE_REGISTRY             10
#define SUITE_VIRTUALIZATION       11
#define SUITE_ECOSYSTEM            12
#define SUITE_SENTIENCE            13
#define SUITE_TRANSCENDENCE        14
#define SUITE_DEVNEXUS             15
#define SUITE_SOULMOLDING          16
#define SUITE_BIONEXUS             17
#define SUITE_QUANTUMLINK          18
#define SUITE_SELFEVOLUTION        19
#define SUITE_GLOBALVFS            20
#define SUITE_ETERNALSTATE         21
#define SUITE_SIMULATIONNEXUS      22
#define SUITE_OMNINEXUS            23
#define SUITE_GLOBALDEBUGGER       24
#define SUITE_ZEROKERNEL           25
#define SUITE_OMNIFABRIC           26
#define SUITE_NEURALLINK           27
#define SUITE_OMNIBUS              28
#define SUITE_LATTICEMERGE         29
#define SUITE_SUPREMACY            30
#define SUITE_GLOBALGOVERNANCE     31
#define SUITE_UNIFIEDSOVEREIGNTY   32
#define SUITE_TERMINALFULFILLMENT  33

/* OmniFabric Interface (S26) */
void        OmniFabric_Init(void);
sigma_err_t OmniFabric_Send(sigma_u32 sender, sigma_u32 receiver, sigma_u32 type, sigma_u64* data);
sigma_bool  OmniFabric_Poll(OmniMessage* out_msg);

#endif /* SOVEREIGN_INTERCONNECT_H */
