/* S SIGMAOS: SOVEREIGN SESSION SHARD HEADER */
#ifndef SOVEREIGN_SESSION_SHARD_H
#define SOVEREIGN_SESSION_SHARD_H
#include "sigma_types.h"

typedef enum { AUTH_PIN, AUTH_BIOMETRIC, AUTH_FIDO2_KEY, AUTH_QUORUM } SigmaAuthType_t;

sigma_err_t sigma_session_login   (const char* name, SigmaAuthType_t method);
void        sigma_session_lock    (void);
sigma_err_t sigma_session_elevate (void);
void        SovereignSessionShard_Init (void);
void        SovereignSession_Audit     (void);

#endif
