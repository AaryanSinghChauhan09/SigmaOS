/* Σ SIGMAOS: SOVEREIGN PRIVACY SHARD HEADER */
#ifndef SOVEREIGN_PRIVACY_SHARD_H
#define SOVEREIGN_PRIVACY_SHARD_H
#include "sigma_types.h"
typedef enum { PRIV_ACCESS_LOCATION, PRIV_ACCESS_CAMERA, PRIV_ACCESS_MICROPHONE,
               PRIV_ACCESS_CONTACTS, PRIV_ACCESS_NETWORK, PRIV_ACCESS_STORAGE,
               PRIV_ACCESS_KERNEL_MEM } SigmaPrivAccessType_t;
typedef enum { PRIV_VERDICT_ALLOW, PRIV_VERDICT_DENY,
               PRIV_VERDICT_PROMPT, PRIV_VERDICT_ANONYMISE } SigmaPrivVerdict_t;
sigma_err_t        sigma_privacy_set_policy (const char* shard, SigmaPrivAccessType_t type,
                                              SigmaPrivVerdict_t verdict, sigma_bool persist);
SigmaPrivVerdict_t sigma_privacy_check      (const char* shard, SigmaPrivAccessType_t type);
void               sigma_privacy_set_level  (sigma_u32 level);
void               sigma_privacy_report     (void);
void               SovereignPrivacyShard_Init (void);
void               SovereignPrivacy_Audit    (void);
#endif
