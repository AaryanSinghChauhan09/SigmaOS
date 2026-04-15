/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN JOURNAL SHARD HEADER
 * =========================================================================
 */
#ifndef SOVEREIGN_JOURNAL_SHARD_H
#define SOVEREIGN_JOURNAL_SHARD_H

#include "sigma_types.h"

typedef enum {
    LOG_EMERG=0, LOG_ALERT=1, LOG_CRIT=2, LOG_ERR=3,
    LOG_WARNING=4, LOG_NOTICE=5, LOG_INFO=6, LOG_DEBUG=7
} SigmaLogLevel_t;

void sigma_journal_write   (SigmaLogLevel_t lvl, const char* unit, const char* msg);
void sigma_journal_follow  (SigmaLogLevel_t min_level);
void SovereignJournalShard_Init (void);
void SovereignJournal_Audit     (void);

#endif /* SOVEREIGN_JOURNAL_SHARD_H */
