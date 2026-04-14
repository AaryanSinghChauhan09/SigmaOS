/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN INTERCONNECT (v1.0)
 * =========================================================================
 * Mission: Unified Interface for Inter-Suite Communication.
 * =========================================================================
 */

#ifndef SOVEREIGN_INTERCONNECT_H
#define SOVEREIGN_INTERCONNECT_H

#include "sigma_base.h"

/* Message Types for OmniFabric */
#define MSG_TYPE_MEM_ALLOC    0x01
#define MSG_TYPE_FS_WRITE     0x02
#define MSG_TYPE_NET_SEND     0x03
#define MSG_TYPE_SEC_AUDIT    0x04
#define MSG_TYPE_SYS_PANIC    0xFF

typedef struct {
    sigma_u32 sender_id;
    sigma_u32 receiver_id;
    sigma_u32 msg_type;
    sigma_u64 payload[4];
} OmniMessage;

/* OmniFabric Interface (S26) */
void        OmniFabric_Init(void);
sigma_err_t OmniFabric_Send(sigma_u32 sender, sigma_u32 receiver, sigma_u32 type, sigma_u64* data);
sigma_bool  OmniFabric_Poll(OmniMessage* out_msg);

/* Suite IDs (S01-S33) */
#define SUITE_GENESIS      1
#define SUITE_MEMORY       5
#define SUITE_STORAGE      6
#define SUITE_NETWORK      7
#define SUITE_SECURITY     8
#define SUITE_OMNIFABRIC   26

#endif /* SOVEREIGN_INTERCONNECT_H */
